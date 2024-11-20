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

struct FindResult {
    pub query_file: String,
    pub ref_file: String,
    pub start: u64,
    pub end: u64,
    pub strand: char,
    pub length: u64,
    pub mismatches: u64,
    pub gap_opens: u64,
    pub identity: f64,
    pub coverage: f64,
    pub query_contig: String,
    pub ref_contig: String,
}

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
    // let mut res = use_signal(Vec::<(String, String, String, String, String, String, String, String, String)>::new);
    let mut res = use_signal(Vec::<FindResult>::new);

    let mut detailed:Signal<bool> = use_signal(|| false);
    // let mut min_len:Signal<u64> = use_signal(|| 100_u64);
    // let mut max_gap_len:Signal<u64> = use_signal(|| 0_u64);
    // let mut max_error_prob:Signal<f64> = use_signal(|| 0.0000001_f64);

    rsx! {
        div {
            h2 { "Options" }
            input {
                r#type: "checkbox",
                name: "detailed",
                id: "detailed",
                checked: false,
                onchange: move |_| {
                    let old: bool = *detailed.read();
                    *detailed.write() = !old;
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
                                let mut run_lengths: Vec<FindResult> = Vec::new();

                                // Get local alignments for forward strand
                                let run_lengths_fwd = kbo::find(&contig.seq, &sbwt, &lcs, kbo::FindOpts::default());
                                run_lengths.extend(run_lengths_fwd.iter().map(|x| {
                                    FindResult {
                                        query_file: "query".to_string(),
                                        ref_file: "ref".to_string(),
                                        start: x.start as u64,
                                        end: x.end as u64,
                                        strand: '+',
                                        length: (x.end - x.start + 1) as u64,
                                        mismatches: x.mismatches as u64,
                                        gap_opens: x.gap_opens as u64,
                                        identity: -1_f64,
                                        coverage: -1_f64,
                                        query_contig: contig.name.clone(),
                                        ref_contig: ref_contig.clone(),
                                    }
                                }));

                                // Add local alignments for reverse complement
                                let run_lengths_rev = kbo::find(&contig.seq.reverse_complement(), &sbwt, &lcs, kbo::FindOpts::default());
                                run_lengths.extend(run_lengths_rev.iter().map(|x| {
                                    FindResult {
                                        query_file: "query".to_string(),
                                        ref_file: "ref".to_string(),
                                        start: x.start as u64,
                                        end: x.end as u64,
                                        strand: '-',
                                        length: (x.end - x.start + 1) as u64,
                                        mismatches: x.mismatches as u64,
                                        gap_opens: x.gap_opens as u64,
                                        identity: -1_f64,
                                        coverage: -1_f64,
                                        query_contig: contig.name.clone(),
                                        ref_contig: ref_contig.clone(),
                                    }
                                }));

                                // Sort by q.start
                                run_lengths.sort_by_key(|x| x.start);

                                // Print results with query and ref name added
                                res.write().extend(run_lengths);
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
                            td { "strand" }
                            td { "length" }
                            td { "mismatches" }
                            td { "gap_opens" }
                            td { "identity" }
                            td { "coverage" }
                            td { "query.contig" }
                            td { "ref.contig" }
                        }
                    }
                    tbody {
                        {
                            res.read().iter().map(|row| {
                                rsx! {
                                    tr {
                                        td { "{row.query_file}" }
                                        td { "{row.ref_file}" }
                                        td { "{row.start}" }
                                        td { "{row.end}" }
                                        td { "{row.strand}" }
                                        td { "{row.length}" }
                                        td { "{row.mismatches}" }
                                        td { "{row.gap_opens}" }
                                        td { "{row.identity}" }
                                        td { "{row.coverage}" }
                                        td { "{row.query_contig}" }
                                        td { "{row.ref_contig}" }
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
