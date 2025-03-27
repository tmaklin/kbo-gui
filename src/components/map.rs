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

use crate::components::common::BuildOptsSelector;

#[component]
pub fn Map(
    queries: Vec<(String, Vec<crate::util::ContigData>)>,
    reference: (String, Vec<crate::util::ContigData>),
) -> Element {

    let mut res = use_signal(Vec::<(String, Vec<u8>)>::new);

    // Options for running queries
    let mut max_error_prob: Signal<f64> = use_signal(|| 0.0000001_f64);
    let mut do_vc: Signal<bool> = use_signal(|| true);
    let mut do_gapfill: Signal<bool> = use_signal(|| true);

    // Options for indexing reference
    let kmer_size: Signal<u32> = use_signal(|| 31);
    let dedup_batches: Signal<bool> = use_signal(|| true);
    let prefix_precalc: Signal<u32> = use_signal(|| 8);

    let mut res_error: Signal<String> = use_signal(String::new);

    rsx! {
        div { class: "row",
              div { class: "column-left", br {} },
              div { class: "column-right" },
        }

        div { class: "row",
              div { class: "column-left",
                    details {
                        summary { "Indexing options" }
                        BuildOptsSelector { kmer_size, dedup_batches, prefix_precalc }
                    }
              }
              div { class: "column-right",
                    details {
                        summary { "Alignment options" }
                        div { class: "row-contents",
                        div { class: "column",
                              "Error tolerance"
                        },
                        div { class: "column",
                              input {
                                  r#type: "number",
                                  id: "min_len",
                                  name: "min_len",
                                  min: "0",
                                  max: "1.00",
                                  value: "0.0000001",
                                  onchange: move |event| {
                                      let new = event.value().parse::<f64>();
                                      if let Ok(new_prob) = new { max_error_prob.set(new_prob.clamp(0_f64 + f64::EPSILON, 1_f64 - f64::EPSILON)) };
                                  }
                              },
                        }
                        }
                        div { class: "row-contents",
                        div { class: "column",
                              "Variant calling"
                        },
                        div { class: "column",
                              input {
                                  r#type: "checkbox",
                                  id: "do_vc",
                                  name: "do_vc",
                                  checked: *do_vc.read(),
                                  onchange: move |_| {
                                      let old: bool = *do_vc.read();
                                      *do_vc.write() = !old;
                                  }
                              },
                        }
                        }
                        div { class: "row-contents",
                        div { class: "column",
                              "Gap filling"
                        },
                        div { class: "column",
                              input {
                                  r#type: "checkbox",
                                  id: "do_gapfill",
                                  name: "do_gapfill",
                                  checked: *do_gapfill.read(),
                                  onchange: move |_| {
                                      let old: bool = *do_gapfill.read();
                                      *do_gapfill.write() = !old;
                                  }
                              },
                        }
                        }
                    }
              }
        }

        div { class: "row-run",
              div { class: "column",
                    button {
                        onclick: move |_event| {
                            if !reference.1.is_empty() && !queries.is_empty() {
                                // Clear old results
                                res.write().clear();
                                *res_error.write() = String::new();

                                let mut map_opts = kbo::MapOpts::default();
                                map_opts.max_error_prob = *max_error_prob.read();
                                map_opts.call_variants = *do_vc.read();
                                map_opts.fill_gaps = *do_vc.read();

                                queries.iter().for_each(|(query_file, query_contig)| {
                                    // Options for indexing reference
                                    let mut build_opts = kbo::BuildOpts::default();
                                    build_opts.build_select = true;
                                    build_opts.k = *kmer_size.read() as usize;
                                    build_opts.dedup_batches = *dedup_batches.read();
                                    build_opts.prefix_precalc = *prefix_precalc.read() as usize;

                                    let (sbwt, lcs) = crate::util::build_sbwt(
                                        &query_contig.iter().map(|x| x.seq.clone()).collect::<Vec<Vec<u8>>>(),
                                        Some(build_opts),
                                    );

                                    let my_res: Vec<u8> = reference.1.iter().flat_map(|ref_contig| {
                                        kbo::map(&ref_contig.seq, &sbwt, &lcs, map_opts.clone())
                                    }).collect();

                                    if my_res.is_empty() && res.read().is_empty() {
                                        *res_error.write() = "Nothing to report!".to_string();
                                    } else {
                                        res.write().push((">".to_string() + query_file, my_res));
                                    }
                                });
                            }
                        },
                        "Run analysis",
                    }
              }
              div { class: "column" }
              //
              // TODO Look into implementing an interactive alignment view for map.
              //
              // div { class: "column",
              //       input {
              //           r#type: "checkbox",
              //           name: "interactive",
              //           id: "interactive",
              //           checked: true,
              //           onchange: move |_| {
              //               let old: bool = *interactive.read();
              //               *interactive.write() = !old;
              //           }
              //       },
              //       "Interactive output",
              // }
        }
        div { class: "row-results",
              if res.read().len() > 0 && res_error.read().is_empty() {
                  {
                      rsx! {
                          CopyableMapResult { data: res.read().to_vec() }
                      }
                  }
              } else if !res_error.read().is_empty() {
                  div {
                      { res_error.read().to_string() }
                  }
              }
        }
    }
}

#[component]
fn CopyableMapResult(
    data: Vec<(String, Vec::<u8>)>,
) -> Element {

    let mut total_len = 0;
    let display = data.iter().map(|(seq, contents)| {
        let mut counter = 0;
        total_len += contents.len();
        seq.clone() + "\n" + &contents.iter().map(|x| {
            counter += 1;
            if counter % 80 == 0 {
                counter = 0;
                (*x as char).to_string() + "\n"
            } else {
                (*x as char).to_string()
            }
        }).collect::<String>()
            + "\n"
    }).collect::<String>();

    rsx! {
        textarea {
            id: "find-result",
            name: "find-result",
            value: display,
            rows: total_len.div_ceil(80),
            width: "95%",
        },
    }
}
