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

#[derive(Default, PartialEq)]
enum KboMode {
    #[default]
    Find,
    Map,
}

#[component]
pub fn Kbo() -> Element {
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
            crate::components::FastaFileSelector { multiple: false, seq_data: ref_files }
            {
                if ref_files.read().len() > 0 {
                    ref_files.read().iter().for_each(|seq| {
                        refseqs.extend(crate::util::read_seq_data(seq));
                    });
                }
            }
        }

        div {
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

        {
            if *kbo_mode.read() == KboMode::Map {
                rsx! {
                    crate::components::Map {}
                }
            } else if *kbo_mode.read() == KboMode::Find {
                rsx! {
                    crate::components::Find {
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
