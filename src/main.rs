// kbo-gui: Graphical user interface for kbo built with Dioxus.
//
// Copyright 2024 Tommi Mäklin [tommi@maklin.fi].

// Copyrights in this project are retained by contributors. No copyright assignment
// is required to contribute to this project.

// Except as otherwise noted (below and/or in individual files), this
// project is licensed under the Apache License, Version 2.0
// <LICENSE-APACHE> or <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license, <LICENSE-MIT> or <http://opensource.org/licenses/MIT>,
// at your option.
//
#![allow(non_snake_case)]
use std::io::stdout;
use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use needletail::Sequence;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn FastaFileSelector(
    multiple: bool,
    seq_data: Signal<Vec<Vec<u8>>>) -> Element {
    rsx! {
        input {
            // tell the input to pick a file
            r#type: "file",
            // list the accepted extensions
            accept: ".fasta,.fas,.fa,.fna,.ffn,.faa,.mpfa,.frn,.fasta.gz,.fas.gz,.fa.gz,.fna.gz,.ffn.gz,.faa.gz,.mpfa.gz,.frn.gz",
            // pick multiple files
            multiple: multiple,
            onchange: move |evt| {
                async move {
                    if let Some(file_engine) = &evt.files() {
                        let files = file_engine.files();
                        for file_name in &files {
                            if let Some(file) = file_engine.read_file(file_name).await
                            {
                                seq_data.write().push(file.to_vec());
                            }
                        }
                    }
                }
            },
        }
    }
}

fn build_sbwt(file_contents: &[Vec<u8>]) -> (sbwt::SbwtIndexVariant, sbwt::LcsArray) {
    let mut ref_data: Vec<Vec<u8>> = Vec::new();
    file_contents.iter().for_each(|ref_file| {
        let mut reader = needletail::parse_fastx_reader(ref_file.deref()).expect("valid fastX data");
        while let Some(rec) = reader.next() {
            let seqrec = rec.expect("Valid fastX record");
            let _contig = seqrec.id();
            let seq = seqrec.normalize(true);
            ref_data.push(seq.to_vec());
        }
    });
    kbo::index::build_sbwt_from_vecs(&ref_data, &Some(kbo::index::BuildOpts::default()))
}

#[component]
fn Home() -> Element {
    let ref_files: Signal<Vec<Vec<u8>>> = use_signal(Vec::new);
    let query_files: Signal<Vec<Vec<u8>>> = use_signal(Vec::new);
    let mut res = use_signal(Vec::<String>::new);

    let mut indexes: Vec<(sbwt::SbwtIndexVariant, sbwt::LcsArray)> = Vec::new();

    rsx! {
        div {
            h2 { "Reference file" }
            FastaFileSelector { multiple: false, seq_data: ref_files }
            {
                // Build the index immediately after data is uploaded
                if ref_files.read().len() > 0 {
                    indexes.push(build_sbwt(ref_files.read().as_ref()));
                }
            }
        }

        div {
            h2 { "Query file(s)" }
            FastaFileSelector { multiple: true, seq_data: query_files }
        }

        div {
            h2 { "Result" }
            button {
                onclick: move |_event| {
                    if ref_files.read().len() > 0 && query_files.read().len() > 0 {

                        query_files.read().iter().for_each(|file| {
                            let mut reader = needletail::parse_fastx_reader(file.deref()).expect("valid fastX data");
                            while let Some(rec) = reader.next() {
                                let seqrec = rec.expect("Valid fastX record");
                                let contig = seqrec.id();
                                let seq = seqrec.normalize(true);

                                // Get local alignments for forward strand
                                let mut run_lengths = kbo::find(&seq, &indexes[0].0, &indexes[0].1, kbo::FindOpts::default());

                                // Add local alignments for reverse _complement
                                run_lengths.append(&mut kbo::find(&seq.reverse_complement(), &indexes[0].0, &indexes[0].1, kbo::FindOpts::default()));

                                // Sort by q.start
                                run_lengths.sort_by_key(|x| x.start);

                                // Print results with query and ref name added
                                run_lengths.iter().for_each(|x| {
                                    res.write().push(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                                                             "query", "ref",
                                                             x.start,
                                                             x.end,
                                                             x.end - x.start + 1,
                                                             x.mismatches,
                                                             x.gap_opens,
                                                             std::str::from_utf8(contig).expect("UTF-8")));
                                });
                            }
                        });
                    }
                },
                "run!",
            }
            for result in res() {
                // Notice the body of this for loop is rsx code, not an expression
                div {
                    { result }
                }
            }
        }
    }
}
