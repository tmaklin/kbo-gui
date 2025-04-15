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

use crate::util::IndexData;
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
fn InteractivitySwitcher(
    kbo_mode: Signal<KboMode>,
    opts: Signal<GuiOpts>,
) -> Element {
    rsx! {
        if *kbo_mode.read() == KboMode::Find {
            input {
                r#type: "checkbox",
                name: "detailed",
                id: "detailed",
                checked: false,
                onchange: move |_| {
                    let old: bool = opts.read().out_opts.detailed;
                    opts.write().out_opts.detailed = !old;
                }
            },
            "Split reference by contig",
        } else {
            br {},
        }
    }
}

#[component]
fn DetailSwitcher(
    kbo_mode: Signal<KboMode>,
    opts: Signal<GuiOpts>,
) -> Element {
    rsx! {
        if *kbo_mode.read() == KboMode::Map {
            br {}
        } else {
            input {
                r#type: "checkbox",
                name: "interactive",
                id: "interactive",
                checked: true,
                onchange: move |_| {
                    let old: bool = opts.read().out_opts.interactive;
                    opts.write().out_opts.interactive = !old;
                }
            },
            "Interactive output",
        }
    }
}

#[component]
fn QueryIndexBuilder(
    queries: ReadOnlySignal<Vec<SeqData>>,
    gui_opts: ReadOnlySignal<GuiOpts>,
    cached_index: Signal<Vec<IndexData>>,
) -> Element {
    let _ = use_resource(move || async move {
        // Delay start to render a loading spinner
        gloo_timers::future::TimeoutFuture::new(100).await;

        let query_data: Vec<(String, Vec<Vec<u8>>)> = queries.read().iter()
                                                                    .map(|query| { (
                                                                        query.file_name.clone(),
                                                                        query.contigs.iter().map(|contig| {
                                                                            contig.seq.clone()
                                                                        }).collect::<Vec<Vec<u8>>>()
                                                                    )
                                                                    }).collect();
        let mut indexes: Vec<IndexData> = Vec::with_capacity(query_data.len());
        for (file_name, seq_data) in query_data {
            let (sbwt, lcs) = crate::util::sbwt_builder(&seq_data, gui_opts.read().build_opts.to_kbo()).await.unwrap();
            let index = IndexData { sbwt, lcs, file_name: file_name.clone(), bases: seq_data.iter().map(|x| x.len()).sum() };
            indexes.push(index);
        }
        cached_index.set(indexes);
    }).suspend()?;

    rsx! {
        { "".to_string() },
    }
}

#[component]
pub fn Kbo() -> Element {
    // Input data
    let reference: Signal<Vec<SeqData>> = use_signal(Vec::new);
    let queries: Signal<Vec<SeqData>> = use_signal(Vec::new);

    // Cached SBWT
    let cached_index: Signal<Vec<IndexData>> = use_signal(Vec::new);

    // Options
    let kbo_mode: Signal<KboMode> = use_signal(KboMode::default);
    let gui_opts: Signal<GuiOpts> = use_signal(GuiOpts::default);

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
                        QueryIndexBuilder { queries, gui_opts, cached_index },
                        match *kbo_mode.read() {
                            KboMode::Call => {
                                rsx!{ Call { ref_contigs: reference, index: cached_index, opts: gui_opts } }
                            },
                            KboMode::Find => {
                                rsx! { Find { ref_contigs: reference, query_contigs: queries, opts: gui_opts } }
                            },
                            KboMode::Map => {
                                rsx! { Map { ref_contigs: reference, indexes: cached_index, opts: gui_opts } }
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
