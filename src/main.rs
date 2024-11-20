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

#[derive(Default, PartialEq)]
enum KboMode {
    #[default]
    Find,
    Map,
}

fn build_sbwt(ref_data: &[Vec<u8>]) -> (sbwt::SbwtIndexVariant, sbwt::LcsArray) {
    kbo::index::build_sbwt_from_vecs(ref_data, &Some(kbo::index::BuildOpts::default()))
}

fn read_seq_data(file_contents: &Vec<u8>) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut seq_data: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    let mut reader = needletail::parse_fastx_reader(file_contents.deref()).expect("valid fastX data");
    while let Some(rec) = reader.next() {
        let seqrec = rec.expect("Valid fastX record");
        let contig = seqrec.id();
        let seq = seqrec.normalize(true);
        seq_data.push((contig.to_vec(), seq.to_vec()));
    }
    seq_data
}

#[component]
fn Map() -> Element {
    rsx! {
        div {
            { "Map is not yet implemented; select \"Mode: Find\" to continue." }
        }
    }
}

#[component]
fn Find(
    ref_files: Signal<Vec<Vec<u8>>>,
    query_files: Signal<Vec<Vec<u8>>>,
    queries: Vec<(Vec<u8>, Vec<u8>)>,
    refseqs: Vec<(Vec<u8>, Vec<u8>)>,
) -> Element {
    let mut res = use_signal(Vec::<String>::new);

    rsx! {
        div {
            h2 { "Result" }
            button {
                onclick: move |_event| {
                    if ref_files.read().len() > 0 && query_files.read().len() > 0 {
                        let index = build_sbwt(&[refseqs.iter().flat_map(|x| x.1.clone()).collect()]);
                        queries.iter().for_each(|(contig, seq)| {
                            // Get local alignments for forward strand
                            let mut run_lengths = kbo::find(seq, &index.0, &index.1, kbo::FindOpts::default());

                            // Add local alignments for reverse complement
                            run_lengths.append(&mut kbo::find(&seq.reverse_complement(), &index.0, &index.1, kbo::FindOpts::default()));

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

#[component]
fn Home() -> Element {
    let ref_files: Signal<Vec<Vec<u8>>> = use_signal(Vec::new);
    let query_files: Signal<Vec<Vec<u8>>> = use_signal(Vec::new);

    let mut kbo_mode: Signal<KboMode> = use_signal(KboMode::default);

    let mut queries: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    let mut refseqs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

    rsx! {
        // Run mode selector
        // supported: see KboMode
        div {
            h2 { "Mode" }
            input {
                r#type: "radio",
                name: "kbo-mode",
                value: "find",
                checked: true,
                onchange: move |_| {
                    *kbo_mode.write() = KboMode::Find;
                },
            }
            input {
                r#type: "radio",
                name: "kbo-mode",
                value: "map",
                onchange: move |_| {
                    *kbo_mode.write() = KboMode::Map;
                },
            }
        }

        div {
            h2 { "Reference file" }
            FastaFileSelector { multiple: false, seq_data: ref_files }
            {
                if ref_files.read().len() > 0 {
                    ref_files.read().iter().for_each(|seq| {
                        refseqs.extend(read_seq_data(seq));
                    });
                }
            }
        }

        div {
            h2 { "Query file(s)" }
            FastaFileSelector { multiple: true, seq_data: query_files }
            {
                if query_files.read().len() > 0 {
                    query_files.read().iter().for_each(|query| {
                        queries.extend(read_seq_data(query));
                    })
                }
            }
        }

        {
            if *kbo_mode.read() == KboMode::Map {
                rsx! {
                    Map {}
                }
            } else if *kbo_mode.read() == KboMode::Find {
                rsx! {
                    Find {
                        ref_files,
                        query_files,
                        queries,
                        refseqs,
                    }
                }
            } else {
                rsx! {
                    div {
                        { "Unknown mode; check your selection." }
                    }
                }
            }
        }
    }
}
