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

use crate::opts::GuiOpts;

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

    let mut reference: Signal<SeqData> = use_signal(SeqData::default);
    let tmp_data: Signal<Vec<SeqData>> = use_signal(Vec::new);
    let queries: Signal<Vec<SeqData>> = use_signal(Vec::new);

    let kbo_mode: Signal<KboMode> = use_signal(KboMode::default);

    let ref_error: Signal<String> = use_signal(String::new);
    let query_error: Signal<String> = use_signal(String::new);

    // Options
    let mut gui_opts: Signal<GuiOpts> = use_signal(GuiOpts::default);

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
                                FastaFileSelector { multiple: false, out_data: tmp_data, out_text: ref_error },
                          }

                          div { class: "row",
                                if !ref_error.read().is_empty() {
                                    { ref_error.read().to_string() }
                                } else {
                                    br {}
                                }
                          },

                          div { class: "row",
                                details {
                                    summary { "Indexing options" },
                                    BuildOptsSelector { opts: gui_opts },
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
                                            let old: bool = gui_opts.read().out_opts.detailed;
                                            gui_opts.write().out_opts.detailed = !old;
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
                                FastaFileSelector { multiple: *kbo_mode.read() != KboMode::Call, out_data: queries, out_text: query_error }
                          }

                          div { class: "row",
                                if !query_error.read().is_empty() {
                                    { query_error.read().to_string() }
                                } else {
                                    br {}
                                }
                          },

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
                                if *kbo_mode.read() == KboMode::Map {
                                    br {}
                                } else {
                                    input {
                                        r#type: "checkbox",
                                        name: "interactive",
                                        id: "interactive",
                                        checked: true,
                                        onchange: move |_| {
                                            let old: bool = gui_opts.read().out_opts.interactive;
                                            gui_opts.write().out_opts.interactive = !old;
                                        }
                                    },
                                    "Interactive output",
                                }
                          }
                    }
              }

              // Dynamically rendered components,
              // based on which KboMode is selected.
              div { class: "row-results",
                    {
                        if !tmp_data.read().is_empty() {
                            use_effect(move || {
                                reference.set(tmp_data.read()[0].clone());
                            });
                        }
                    }

                    SuspenseBoundary {
                        fallback: |_| rsx! {
                            span { class: "loader" },
                        },
                        match *kbo_mode.read() {
                            KboMode::Call => {
                                rsx!{ Call { ref_contigs: reference, query_contigs: queries, opts: gui_opts } }
                            },
                            KboMode::Find => {
                                rsx! { Find { ref_contigs: reference, query_contigs: queries, opts: gui_opts } }
                            },
                            KboMode::Map => {
                                rsx! { Map { ref_contigs: reference, query_contigs: queries, opts: gui_opts } }
                            },
                        }
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
