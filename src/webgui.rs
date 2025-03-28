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
use crate::components::call::Call;
use crate::components::find::Find;
use crate::components::map::Map;

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
                        // Mode `Find`
                        rsx! {
                          Call {
                            queries: queries.as_ref().unwrap().clone(),
                            reference: reference.as_ref().unwrap()[0].clone(),
                          }
                        }
                      },

                      KboMode::Find => {
                        // Mode `Find`
                        rsx! {
                          Find {
                            queries: queries.as_ref().unwrap().clone(),
                            reference: reference.as_ref().unwrap()[0].clone(),
                          }
                        }
                      },

                      KboMode::Map => {
                        rsx! {
                          Map {
                            queries: queries.as_ref().unwrap().clone(),
                            reference: reference.as_ref().unwrap()[0].clone(),
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
