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
use dioxus::prelude::*;
use needletail::Sequence;

#[component]
pub fn FastaFileSelector(
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

#[component]
pub fn Map() -> Element {
    rsx! {
        div {
            { "Map is not yet implemented; select \"Mode: Find\" to continue." }
        }
    }
}

#[component]
pub fn Find(
    ref_files: Signal<Vec<Vec<u8>>>,
    query_files: Signal<Vec<Vec<u8>>>,
    queries: Vec<crate::util::ContigData>,
    refseqs: Vec<crate::util::ContigData>,
) -> Element {
    let mut res = use_signal(Vec::<(String, String, String, String, String, String, String, String, String)>::new);

    let mut detailed:Signal<bool> = use_signal(|| false);

    rsx! {
        div {
            h2 { "Options" }
            input {
                r#type: "checkbox",
                name: "detailed",
                id: "detailed",
                checked: false,
                onchange: move |_| {
                    *detailed.write() = true;
                }
            },
            "Detailed"
        }
        div {
            h2 { "Result" }
            button {
                onclick: move |_event| {
                    if ref_files.read().len() > 0 && query_files.read().len() > 0 {
                        let mut indexes: Vec<((sbwt::SbwtIndexVariant, sbwt::LcsArray), String)> = Vec::new();

                        if !*detailed.read() {

                            // TODO Clone here should be made unnecessary

                            indexes.push((crate::util::build_sbwt(&[refseqs.iter().flat_map(|contig| contig.seq.clone()).collect()]), "ref_file".to_string()));
                        } else {
                            refseqs.iter().for_each(|contig| {
                                indexes.push((crate::util::build_sbwt(&[contig.seq.clone()]), contig.name.clone()));
                            });
                        }

                        indexes.iter().for_each(|((sbwt, lcs), ref_contig)| {
                            queries.iter().for_each(|contig| {
                                // Get local alignments for forward strand
                                let mut run_lengths = kbo::find(&contig.seq, &sbwt, &lcs, kbo::FindOpts::default());

                                // Add local alignments for reverse complement
                                run_lengths.append(&mut kbo::find(&contig.seq.reverse_complement(), &sbwt, &lcs, kbo::FindOpts::default()));

                                // Sort by q.start
                                run_lengths.sort_by_key(|x| x.start);

                                // Print results with query and ref name added
                                run_lengths.iter().for_each(|x| {
                                    res.write().push((
                                        "query".to_string(),
                                        "ref".to_string(),
                                        x.start.to_string(),
                                        x.end.to_string(),
                                        (x.end - x.start + 1).to_string(),
                                        x.mismatches.to_string(),
                                        x.gap_opens.to_string(),
                                        contig.name.clone(),
                                        ref_contig.clone(),
                                    ));
                                });
                            });
                        });
                    }
                },
                "run!",
            }
            if res.read().len() > 0 {
                table {
                    thead {
                        tr {
                            td { "query" }
                            td { "ref" }
                            td { "q.start" }
                            td { "q.end" }
                            td { "length" }
                            td { "mismatches" }
                            td { "gap_opens" }
                            td { "ref_contig" }
                            td { "query_contig" }
                        }
                    }
                    tbody {
                        {
                            res.read().iter().map(|row| {
                                rsx! {
                                    tr {
                                        td { "{row.0}" }
                                        td { "{row.1}" }
                                        td { "{row.2}" }
                                        td { "{row.3}" }
                                        td { "{row.4}" }
                                        td { "{row.5}" }
                                        td { "{row.6}" }
                                        td { "{row.8}" }
                                        td { "{row.7}" }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}
