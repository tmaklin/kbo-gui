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

use crate::components::common::FastaFileSelector;
use crate::components::common::BuildOptsSelector;
use crate::components::call::*;
use crate::components::find::*;
use crate::components::map::*;
use crate::util::SeqData;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Default, PartialEq)]
enum KboMode {
    #[default]
    Call,
    Find,
    Map,
}

#[component]
fn RunModeSelector(
    kbo_mode: Signal<KboMode>,
) -> Element {
    rsx! {

      // Mode `Call`
        input {
            class: if *kbo_mode.read() == KboMode::Call { "test-active"} else { "test" },
            r#type: "button",
            name: "kbo-mode",
            value: "Call",
            onclick: move |_| {
                *kbo_mode.write() = KboMode::Call;
            },
        }
        " "
        // Mode `Find`
        input {
            class: if *kbo_mode.read() == KboMode::Find { "test-active"} else { "test" },
            r#type: "button",
            name: "kbo-mode",
            value: "Find",
            onclick: move |_| {
                *kbo_mode.write() = KboMode::Find;
            },
        }
        " "
        // Mode `Map`
        input {
            class: if *kbo_mode.read() == KboMode::Map { "test-active"} else { "test" },
            r#type: "button",
            name: "kbo-mode",
            value: "Map",
            onclick: move |_| {
                *kbo_mode.write() = KboMode::Map;
            },
        }
    }
}

#[component]
pub fn Kbo() -> Element {
    let ref_files: Signal<Vec<(String, Vec<u8>)>> = use_signal(|| { Vec::with_capacity(1) });
    let query_files: Signal<Vec<(String, Vec<u8>)>> = use_signal(Vec::new);

    let mut n_refs: Signal<usize> = use_signal(|| 0);
    let mut n_queries: Signal<usize> = use_signal(|| 0);

    let mut reference: Signal<SeqData> = use_signal(SeqData::default);
    let mut queries: Signal<Vec<SeqData>> = use_signal(Vec::new);

    let kbo_mode: Signal<KboMode> = use_signal(KboMode::default);

    let version = env!("CARGO_PKG_VERSION").to_string();
    let footer_string = "kbo-gui v".to_string() + &version;
    let repository = env!("CARGO_PKG_REPOSITORY").to_string();
    let homepage = env!("CARGO_PKG_HOMEPAGE").to_string();

    let mut ref_error: Signal<String> = use_signal(String::new);
    let mut query_error: Signal<String> = use_signal(String::new);


    // Options for indexing reference
    let kmer_size: Signal<u32> = use_signal(|| 51);
    let dedup_batches: Signal<bool> = use_signal(|| true);
    let prefix_precalc: Signal<u32> = use_signal(|| 8);

    // Alignment options
    let max_error_prob: Signal<f64> = use_signal(|| 0.0000001_f64);
    let min_len: Signal<u64> = use_signal(|| 100_u64);
    let max_gap_len: Signal<u64> = use_signal(|| 0_u64);
    let do_vc: Signal<bool> = use_signal(|| true);
    let do_gapfill: Signal<bool> = use_signal(|| true);

    // Output options
    let mut interactive: Signal<bool> = use_signal(|| true);
    let mut detailed: Signal<bool> = use_signal(|| false);

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "div-box",

              div { class: "row-header",
                    h1 { "kbo"},
                    RunModeSelector { kbo_mode },
              }

              div { class: "row",
                    div { class: "column-left",
                          div { class: "row",
                                strong { "Reference file" },
                          }
                          div { class: "row",
                                FastaFileSelector { multiple: false, seq_data: ref_files },
                          }

                          div { class: "row",
                                {
                                    let ref_contigs = use_resource(move || async move {
                                        let res = crate::util::read_fasta_files(&ref_files.read()).await;
                                        n_refs.set(ref_files.read().len());
                                        res
                                    });
                                    use_effect(move || {
                                        if *n_refs.read() > 0 {
                                            if let Ok(ref_data) = (*ref_contigs.read()).as_ref().unwrap() {
                                                reference.set((*ref_data.clone())[0].clone());
                                            }
                                        }
                                    });
                                }

                                if ref_error.read().len() > 0 {
                                    { ref_error.read().to_string() }
                                } else {
                                    br {}
                                }
                          },

                          div { class: "row",
                                details {
                                    summary { "Indexing options" },
                                    BuildOptsSelector { kmer_size, dedup_batches, prefix_precalc },
                                }
                          },

                          div { class: "row-contents",
                                if *kbo_mode.read() == KboMode::Find {
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
                                    "Split reference by contig",
                                } else {
                                    br {},
                                }
                          }

                    }

                    div { class: "column-right",
                          div { class: "row",
                                strong { { "Query file".to_string() + if *kbo_mode.read() != KboMode::Call { "(s)" } else { "" } } }
                          }
                          div { class: "row",
                                FastaFileSelector { multiple: *kbo_mode.read() != KboMode::Call, seq_data: query_files }
                          }

                          div { class: "row",
                                {
                                    let query_contigs = use_resource(move || async move {
                                        let res = crate::util::read_fasta_files(&query_files.read()).await;
                                        n_queries.set(query_files.read().len());
                                        res
                                    });
                                    use_effect(move || {
                                        if *n_queries.read() > 0 {
                                            if let Ok(query_data) = (*query_contigs.read()).as_ref().unwrap() {
                                                queries.set((*query_data.clone()).to_vec());
                                            }
                                        }
                                    });
                                }
                                if query_error.read().len() > 0 {
                                    { query_error.read().to_string() }
                                } else {
                                    br {}
                                }
                          },

                          div { class: "row",
                                details {
                                    summary { "Alignment options" }
                                    match *kbo_mode.read() {
                                        KboMode::Call => rsx! { CallOptsSelector { max_error_prob } },
                                        KboMode::Find => rsx! { FindOptsSelector { min_len, max_gap_len, max_error_prob } },
                                        KboMode::Map => rsx! { MapOptsSelector { max_error_prob, do_vc, do_gapfill } },
                                    }
                                },
                          }

                          div { class: "row-contents",
                                if *kbo_mode.read() == KboMode::Map {
                                    br {}
                                } else {
                                    input {
                                        r#type: "checkbox",
                                        name: "interactive",
                                        id: "interactive",
                                        checked: true,
                                        onchange: move |_| {
                                            let old: bool = *interactive.read();
                                            *interactive.write() = !old;
                                        }
                                    },
                                    "Interactive output",
                                }
                          }
                    }
              }

              // Dynamically rendered components,
              // based on which KboMode is selected.
              {

                  if *n_refs.read() > 0 && *n_queries.read() > 0 {
                      rsx! {
                          div { class: "row-results",
                                SuspenseBoundary {
                                    fallback: |_| rsx! {
                                        span { class: "loader" },
                                    },
                                    match *kbo_mode.read() {
                                        KboMode::Call => {
                                            let mut call_opts = kbo::CallOpts::default();
                                            call_opts.max_error_prob = *max_error_prob.read();
                                            call_opts.sbwt_build_opts.k = *kmer_size.read() as usize;
                                            call_opts.sbwt_build_opts.dedup_batches = *dedup_batches.read();
                                            call_opts.sbwt_build_opts.prefix_precalc = *prefix_precalc.read() as usize;
                                            rsx!{ Call { ref_contigs: reference, query_contigs: queries, interactive, call_opts } }
                                        },
                                        KboMode::Find => {
                                            // Mode `Find`
                                            let mut find_opts = kbo::FindOpts::default();
                                            find_opts.max_error_prob = *max_error_prob.read();
                                            find_opts.max_gap_len = *max_gap_len.read() as usize;

                                            // Options for indexing reference
                                            let mut build_opts = kbo::BuildOpts::default();
                                            build_opts.k = *kmer_size.read() as usize;
                                            build_opts.dedup_batches = *dedup_batches.read();
                                            build_opts.prefix_precalc = *prefix_precalc.read() as usize;

                                            rsx! { Find { ref_contigs: reference, query_contigs: queries, interactive, min_len, detailed, find_opts, build_opts } }
                                        },
                                        KboMode::Map => {
                                            // Options for indexing reference
                                            let mut build_opts = kbo::BuildOpts::default();
                                            build_opts.build_select = true;
                                            build_opts.k = *kmer_size.read() as usize;
                                            build_opts.dedup_batches = *dedup_batches.read();
                                            build_opts.prefix_precalc = *prefix_precalc.read() as usize;

                                            let mut map_opts = kbo::MapOpts::default();
                                            map_opts.max_error_prob = *max_error_prob.read();
                                            map_opts.call_variants = *do_vc.read();
                                            map_opts.fill_gaps = *do_vc.read();
                                            map_opts.sbwt_build_opts = build_opts;

                                            rsx! { Map { ref_contigs: reference, query_contigs: queries, map_opts } }
                                        },
                                    }
                                }
                          }
                      }
                  } else {
                      rsx! { { "" } }
                  }
              }
        }
        footer { class: "footer",
                 div { class: "row-footer",
                       div { class: "column-footer",
                             { footer_string },
                       }
                       div { class: "column-footer",
                             a { href: homepage, "About" },
                       }
                       div { class: "column-footer",
                             a { href: repository, "Report issues" },
                       }
                 }
        }
    }
}
