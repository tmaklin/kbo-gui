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

      // Mode `Call`
        input {
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
                          {
                              if ref_files.read().len() > 0 {
                                  ref_files.read().iter().for_each(|seq| {
                                    let data = crate::util::read_seq_data(seq);
                                    match data {
                                        Ok(data) => { *ref_error.write() = String::new(); refseqs.extend(data) },
                                        Err(e) => { *ref_error.write() = e.msg; },
                                    }
                                  });
                              }
                          }
                    }

                    // Query file(s)
                    div { class: "column-right",
                          h3 { { "Query file".to_string() + if *kbo_mode.read() != KboMode::Call { "(s)" } else { "" } } }
                          FastaFileSelector { multiple: *kbo_mode.read() != KboMode::Call, seq_data: query_files }
                          {
                              if query_files.read().len() > 0 {
                                  query_files.read().iter().for_each(|query| {
                                    let data = crate::util::read_seq_data(query);
                                    match data {
                                        Ok(data) => { *query_error.write() = String::new(); queries.extend(data)},
                                        Err(e) => { *query_error.write() = e.msg; },
                                    }
                                  })
                              }
                          }
                    }
              }
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
                  match *kbo_mode.read() {

                    KboMode::Call => {
                          // Mode `Find`
                          rsx! {
                              Call {
                                  ref_files,
                                  query_files,
                                  queries,
                                  refseqs,
                              }
                          }
                      },

                      KboMode::Find => {
                          // Mode `Find`
                          rsx! {
                              Find {
                                  ref_files,
                                  query_files,
                                  queries,
                                  refseqs,
                              }
                          }
                      },

                      KboMode::Map => {
                          rsx! {
                              Map {
                                  ref_files,
                                  query_files,
                                  queries,
                                  refseqs,
                              }
                          }
                      },
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
