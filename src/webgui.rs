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

static CSS: Asset = asset!("/assets/main.css");

#[derive(Default, PartialEq)]
enum KboMode {
    #[default]
    Find,
    Map,
}

#[component]
fn RunModeSelector(
    kbo_mode: Signal<KboMode>,
) -> Element {
    rsx! {
        // Run mode selector,
        // for supported see KboMode
        // // Mode `Call`
        // input {
        //     r#type: "button",
        //     name: "kbo-mode",
        //     value: "Call",
        //     onclick: move |_| {
        //         *kbo_mode.write() = KboMode::Call;
        //     },
        // }

        // Mode `Find`
        input {
            r#type: "button",
            name: "kbo-mode",
            value: "Find",
            onclick: move |_| {
                *kbo_mode.write() = KboMode::Find;
            },
        }

        // Mode `Map`
        input {
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
    let ref_files: Signal<Vec<Vec<u8>>> = use_signal(Vec::new);
    let query_files: Signal<Vec<Vec<u8>>> = use_signal(Vec::new);

    let kbo_mode: Signal<KboMode> = use_signal(KboMode::default);

    let mut queries: Vec<crate::util::ContigData> = Vec::new();
    let mut refseqs: Vec<crate::util::ContigData> = Vec::new();

    let version = env!("CARGO_PKG_VERSION").to_string();
    let footer_string = "kbo-gui v".to_string() + &version;

    rsx! {
        document::Stylesheet { href: CSS }

        // kbo title + space for logo
        div { class: "row",
              div { class: "column",
                    h1 { "kbo" }
              }
        }

        div { class: "row",
              div { class: "column",
                    RunModeSelector { kbo_mode }
              }
        }

        // Input selectors
        div { class: "row",
              // Reference file
              div { class: "column",
                    h2 {
                        "Reference file"
                    }
                    crate::components::FastaFileSelector { multiple: false, seq_data: ref_files }
                    {
                        if ref_files.read().len() > 0 {
                            ref_files.read().iter().for_each(|seq| {
                                refseqs.extend(crate::util::read_seq_data(seq));
                            });
                        }
                    }
              }

              // Query file(s)
              div { class: "column",
                    h2 { "Query file(s)" }
                    crate::components::FastaFileSelector { multiple: true, seq_data: query_files }
                    {
                        if query_files.read().len() > 0 {
                            query_files.read().iter().for_each(|query| {
                                queries.extend(crate::util::read_seq_data(query));
                            })
                        }
                    }
              }
        }

        // Dynamically rendered components,
        // based on which KboMode is selected.
        {
            match *kbo_mode.read() {

                // Why does this complain of unreachable patterns?
                KboMode::Find => {
                    // Mode `Find`
                    rsx! {
                        crate::components::Find {
                            ref_files,
                            query_files,
                            queries,
                            refseqs,
                        }
                    }
                },

                KboMode::Map => {
                    rsx! {
                        crate::components::Map {
                            ref_files,
                            query_files,
                            queries,
                            refseqs,
                        }
                    }
                },

                _ => {
                    rsx! {
                        div {
                            { "Unknown mode; check your selection." }
                        }
                    }
                },
            }
        }
        footer { { footer_string } }
    }
}
