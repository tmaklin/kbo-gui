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

use crate::common::*;
use crate::opts::GuiOpts;
use crate::util::build_indexes;

#[component]
pub fn BuildOptsSelector(
    opts: Signal<GuiOpts>
) -> Element {
    rsx! {
        div { class: "row-contents",
              div { class: "column-right",
                    "k-mer size",
              }
              div { class: "column-left",
                    input {
                        r#type: "number",
                        id: "kmer_size",
                        name: "kmer_size",
                        min: "2",
                        max: "256",
                        value: opts.read().build_opts.kmer_size.to_string(),
                        onchange: move |event| {
                            let new = event.value().parse::<u32>();
                            if let Ok(new_k) = new { opts.write().build_opts.kmer_size = new_k.clamp(2, 255) };
                        }
                    },
              }
        }
        div { class: "row-contents",
              div { class: "column-right",
                    "Prefix precalc",
              }
              div { class: "column-left",
                  input {
                      r#type: "number",
                      id: "prefix_precalc",
                      name: "prefix_precalc",
                      min: "1",
                      max: "255",
                      value: opts.read().build_opts.prefix_precalc.to_string(),
                      onchange: move |event| {
                          let new = event.value().parse::<u32>();
                          if let Ok(new_precalc) = new { opts.write().build_opts.prefix_precalc = new_precalc };
                      }
                  },
              }
        }
        div { class: "row-contents",
              div { class: "column",
                    "Deduplicate",
              }
              div { class: "column-left",
                  input {
                      r#type: "checkbox",
                      name: "dedup_batches",
                      id: "dedup_batches",
                      checked: opts.read().build_opts.dedup_batches.to_string(),
                      onchange: move |_| {
                          let old: bool = opts.read().build_opts.dedup_batches;
                          opts.write().build_opts.dedup_batches = !old;
                      }
                  },
              }
        }
    }
}

#[component]
pub fn DetailSwitcher(
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
                checked: opts.read().out_opts.interactive,
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
pub fn FastaFileSelector(
    multiple: bool,
    out_data: Signal<Vec<SeqData>>,
) -> Element {
    let mut error: Signal<String> = use_signal(String::new);

    rsx! {
        div { class: "row",
              input {
                  // tell the input to pick a file
                  r#type: "file",
                  // list the accepted extensions
                  accept: ".fasta,.fas,.fa,.fna,.ffn,.faa,.mpfa,.frn,.fasta.gz,.fas.gz,.fa.gz,.fna.gz,.ffn.gz,.faa.gz,.mpfa.gz,.frn.gz",
                  // pick multiple files
                  multiple: multiple,
                  onchange: move |evt| {
                      error.set(String::new());
                      async move {
                          if let Some(file_engine) = &evt.files() {
                              let files = file_engine.files();
                              let mut seq_data: Vec<(String, Vec<u8>)> = Vec::new();
                              for file_name in &files {
                                  if let Some(file) = file_engine.read_file(file_name).await
                                  {
                                      seq_data.push((file_name.clone(), file));
                                  }
                              }

                              let ref_contigs = crate::util::read_fasta_files(&seq_data).await;

                              match &ref_contigs {
                                  Ok(ref_data) => out_data.set(ref_data.clone()),
                                  Err(e) => error.set("Error: ".to_string() + &e.msg),
                              }
                          }
                      }
                  },
              }
        },
        div { class: "row",
              { (*error.read()).clone() },
        },
    }
}

#[component]
pub fn IndexBuilder(
    seq_data: ReadOnlySignal<Vec<SeqData>>,
    gui_opts: ReadOnlySignal<GuiOpts>,
    cached_index: Signal<Vec<IndexData>>,
) -> Element {

  if seq_data.is_empty() {
      return rsx! { { "".to_string() } }
  }

  let indexes = use_resource(move || async move {
        // Delay start to render a loading spinner
        let mut indexes: Vec<IndexData> = Vec::new();
        if gui_opts.read().out_opts.detailed {
            let tmp = crate::util::build_runner(&seq_data.read(), gui_opts.read().build_opts.to_kbo(), true).await;
            if let Ok(mut data) = tmp {
                indexes.append(&mut data);
            }
        } else {
            let mut tmp = build_indexes(&seq_data.read(), gui_opts.read().build_opts.to_kbo()).await;
            indexes.append(&mut tmp);
        }
        indexes
    }).suspend()?;

    use_effect(move || {
        cached_index.set(indexes.read().clone());
    });

    rsx! {
        { "".to_string() },
    }
}

#[component]
pub fn InteractivitySwitcher(
    kbo_mode: Signal<KboMode>,
    opts: Signal<GuiOpts>,
) -> Element {
    rsx! {
        if *kbo_mode.read() == KboMode::Find {
            input {
                r#type: "checkbox",
                name: "detailed",
                id: "detailed",
                checked: opts.read().out_opts.detailed,
                onchange: move |_| {
                    let old: bool = opts.read().out_opts.detailed;
                    opts.write().out_opts.detailed = !old;
                }
            },
            "Split query by contig",
        } else {
            br {},
        }
    }
}

#[component]
pub fn RunModeSelector(
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
