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

use crate::components::common::*;
use crate::components::call::*;
use crate::components::find::*;
use crate::components::map::*;

use crate::common::*;

use crate::opts::GuiOpts;

static CSS: Asset = asset!("/assets/main.css");

struct ResultCache {
    pub call: Signal<Result<CallResults, CallRunnerErr>>,
    pub find: Signal<Result<Vec<FindResult>, FindRunnerErr>>,
    pub map: Signal<Result<Vec<MapResult>, MapRunnerErr>>,
}

impl Default for ResultCache {
    fn default() -> ResultCache {
        ResultCache {
            call: use_signal(|| Err(CallRunnerErr{ code: 99, message: "Waiting for data.".to_string() })),
            find: use_signal(|| Err(FindRunnerErr{ code: 99, message: "Waiting for data.".to_string() })),
            map: use_signal(|| Err(MapRunnerErr{ code: 99, message: "Waiting for data.".to_string() })),
        }
    }
}

#[component]
pub fn Kbo() -> Element {
    // Input data
    let reference: Signal<Vec<SeqData>> = use_signal(Vec::new);
    let queries: Signal<Vec<SeqData>> = use_signal(Vec::new);

    // Cached SBWT
    let index: Signal<Vec<IndexData>> = use_signal(Vec::new);

    // Options
    let kbo_mode: Signal<KboMode> = use_signal(KboMode::default);
    let gui_opts: Signal<GuiOpts> = use_signal(GuiOpts::default);

    // Cached results
    let results: ResultCache = ResultCache::default();

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
                          FastaFileSelector { multiple: false, out_data: reference },

                          div { class: "row",
                                details {
                                    summary { "Indexing options" },
                                    BuildOptsSelector { opts: gui_opts },
                                }
                          },

                          div { class: "row-contents",
                                InteractivitySwitcher { kbo_mode, opts: gui_opts },
                          },
                    }

                    div { class: "column-right",
                          div { class: "row",
                                strong { { "Query file".to_string() + if *kbo_mode.read() != KboMode::Call { "(s)" } else { "" } } }
                          }
                          FastaFileSelector { multiple: *kbo_mode.read() != KboMode::Call, out_data: queries },

                          div { class: "row",
                                details {
                                    summary { "Alignment options" }
                                    match *kbo_mode.read() {
                                        KboMode::Call => rsx! { CallOptsSelector { opts: gui_opts } },
                                        KboMode::Find => rsx! { FindOptsSelector { opts: gui_opts } },
                                        KboMode::Map => rsx! { MapOptsSelector { opts: gui_opts } },
                                    }
                                },
                          }

                          div { class: "row-contents",
                                DetailSwitcher { kbo_mode, opts: gui_opts },
                          },
                    }
              }

              // Dynamically rendered components,
              // based on which KboMode is selected.
              div { class: "row-results",
                    SuspenseBoundary {
                        fallback: |_| rsx! {
                            span { class: "loader" },
                        },

                        // Build index
                        IndexBuilder { seq_data: queries, gui_opts, cached_index: index }

                        // Run commands
                        match *kbo_mode.read() {
                            KboMode::Call => {
                                rsx!{ Call { ref_contigs: reference, index: index, opts: gui_opts, result: results.call } }
                            },
                            KboMode::Find => {
                                rsx! { Find { indexes: index, query_contigs: reference, opts: gui_opts, result: results.find } }
                            },
                            KboMode::Map => {
                                rsx! { Map { ref_contigs: reference, indexes: index, opts: gui_opts, result: results.map } }
                            },
                        }
                    }
              },
              div { class: "row-results",
                    // Render results
                    match *kbo_mode.read() {
                        KboMode::Call => {
                            rsx! { CallRenderer { result: results.call, opts: gui_opts } }
                        },
                        KboMode::Find => {
                            rsx! { FindRenderer { result: results.find, opts: gui_opts } }
                        },
                        KboMode::Map => {
                            rsx! { MapRenderer { result: results.map, opts: gui_opts } }
                        },
                    }
              }
        }
        footer { class: "footer",
                 div { class: "row-footer",
                       div { class: "column-footer",
                             { "kbo-gui v".to_string() + env!("CARGO_PKG_VERSION")  },
                       }
                       div { class: "column-footer",
                             a { href: env!("CARGO_PKG_HOMEPAGE").to_string(), "About" },
                       }
                       div { class: "column-footer",
                             a { href: env!("CARGO_PKG_REPOSITORY").to_string(), "Report issues" },
                       }
                 }
        }
    }
}
