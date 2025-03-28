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

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "div-box",
              // kbo title + space for logo
              div { class: "row-logo",
                    h1 { "kbo"}
              }

              div { class: "row-header",
                    RunModeSelector { kbo_mode }
              }

              // Input selectors
              div { class: "row",
                    // Reference file
                    div { class: "column-left",
                          h3 {
                              "Reference file"
                          }
                          FastaFileSelector { multiple: false, seq_data: ref_files }
                    }

                    // Query file(s)
                    div { class: "column-right",
                          h3 { { "Query file".to_string() + if *kbo_mode.read() != KboMode::Call { "(s)" } else { "" } } }
                          FastaFileSelector { multiple: *kbo_mode.read() != KboMode::Call, seq_data: query_files }
                    }
              }

              // fastX parser errors
              div { class: "row",
                    div { class: "column-left",
                          if ref_error.read().len() > 0 {
                              { ref_error.read().to_string() }
                          }
                    }
                    div { class: "column-right",
                          if query_error.read().len() > 0 {
                              { query_error.read().to_string() }
                          }
                    }
              }

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
                              match *kbo_mode.read() {
                                  KboMode::Call => rsx! { CallOptsSelector { max_error_prob } },
                                  KboMode::Find => rsx! { FindOptsSelector { min_len, max_gap_len, max_error_prob } },
                                  KboMode::Map => rsx! { MapOptsSelector { max_error_prob, do_vc, do_gapfill } },
                              }
                          }
                    }
              }

              // Dynamically rendered components,
              // based on which KboMode is selected.
              {

                  let query_contigs = use_resource(move || async move {
                      crate::util::read_fasta_files(&query_files.read()).await
                  });

                  let ref_contigs = use_resource(move || async move {
                      crate::util::read_fasta_files(&ref_files.read()).await
                  });

                  if ref_files.read().len() > 0 {
                      if let Some(queries) = &*query_contigs.read_unchecked() {
                          if let Some(reference) = &*ref_contigs.read_unchecked() {
                              match *kbo_mode.read() {

                                  KboMode::Call => {
                                      let mut call_opts = kbo::CallOpts::default();
                                      call_opts.max_error_prob = *max_error_prob.read();
                                      call_opts.sbwt_build_opts.k = *kmer_size.read() as usize;
                                      call_opts.sbwt_build_opts.dedup_batches = *dedup_batches.read();
                                      call_opts.sbwt_build_opts.prefix_precalc = *prefix_precalc.read() as usize;

                                      // Mode `Find`
                                      rsx! {
                                          Call {
                                              queries: queries.as_ref().unwrap().clone(),
                                              reference: reference.as_ref().unwrap()[0].clone(),
                                              call_opts,
                                          }
                                      }
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

                                      rsx! {
                                          Find {
                                              queries: queries.as_ref().unwrap().clone(),
                                              reference: reference.as_ref().unwrap()[0].clone(),
                                              find_opts,
                                              build_opts,
                                              min_len,
                                          }
                                      }
                                  },

                                  KboMode::Map => {
                                      let mut map_opts = kbo::MapOpts::default();
                                      map_opts.max_error_prob = *max_error_prob.read();
                                      map_opts.call_variants = *do_vc.read();
                                      map_opts.fill_gaps = *do_vc.read();

                                      // Options for indexing reference
                                      let mut build_opts = kbo::BuildOpts::default();
                                      build_opts.build_select = true;
                                      build_opts.k = *kmer_size.read() as usize;
                                      build_opts.dedup_batches = *dedup_batches.read();
                                      build_opts.prefix_precalc = *prefix_precalc.read() as usize;

                                      rsx! {
                                          Map {
                                              queries: queries.as_ref().unwrap().clone(),
                                              reference: reference.as_ref().unwrap()[0].clone(),
                                              map_opts,
                                              build_opts,
                                          }
                                      }
                                  },
                              }
                          } else {
                              rsx! { { "loading" } }
                          }
                      } else {
                          rsx! { { "loading" } }
                      }
                  } else {
                      rsx! { { "nothing" } }
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
