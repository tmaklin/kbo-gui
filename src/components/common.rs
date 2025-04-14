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

#[component]
pub fn FastaFileSelector(
    multiple: bool,
    seq_data: Signal<Vec<(String, Vec<u8>)>>,
    out_text: Signal<String>,
) -> Element {
    rsx! {
        input {
            // tell the input to pick a file
            r#type: "file",
            // list the accepted extensions
            accept: ".fasta,.fas,.fa,.fna,.ffn,.faa,.mpfa,.frn,.fasta.gz,.fas.gz,.fa.gz,.fna.gz,.ffn.gz,.faa.gz,.mpfa.gz,.frn.gz",
            // pick multiple files
            multiple: multiple,
            onchange: move |evt| {
              out_text.set(String::new());
                async move {
                    if let Some(file_engine) = &evt.files() {
                        let files = file_engine.files();
                        let mut data: Vec<(String, Vec<u8>)> = Vec::new();
                        for file_name in &files {
                            if let Some(file) = file_engine.read_file(file_name).await
                            {
                                data.push((file_name.clone(), file));
                            }
                        }
                        *seq_data.write() = data;
                    }
                }
            },
        }
    }
}


#[component]
pub fn BuildOptsSelector(
    kmer_size: Signal<u32>,
    dedup_batches: Signal<bool>,
    prefix_precalc: Signal<u32>,
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
                        value: kmer_size.read().to_string(),
                        onchange: move |event| {
                            let new = event.value().parse::<u32>();
                            if let Ok(new_k) = new { kmer_size.set(new_k.clamp(2, 255)) };
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
                      value: prefix_precalc.read().to_string(),
                      onchange: move |event| {
                          let new = event.value().parse::<u32>();
                          if let Ok(new_precalc) = new { prefix_precalc.set(new_precalc) };
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
                      checked: dedup_batches.read().to_string(),
                      onchange: move |_| {
                          let old: bool = *dedup_batches.read();
                          *dedup_batches.write() = !old;
                      }
                  },
              }
        }
    }
}
