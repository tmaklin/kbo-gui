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

use crate::util::SeqData;
use crate::opts::GuiOpts;

#[component]
pub fn FastaFileSelector(
    multiple: bool,
    out_data: Signal<Vec<SeqData>>,
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
                        let mut seq_data: Vec<(String, Vec<u8>)> = Vec::new();
                        for file_name in &files {
                            if let Some(file) = file_engine.read_file(file_name).await
                            {
                                seq_data.push((file_name.clone(), file));
                            }
                        }

                        out_text.set(String::new());
                        let ref_contigs = crate::util::read_fasta_files(&seq_data).await;

                        use_effect(move || {
                            match &ref_contigs {
                                Ok(ref_data) => out_data.set(ref_data.clone()),
                                Err(e) => out_text.set("Error: ".to_string() + &e.msg.to_string()),
                            }
                        });

                    }
                }
            },
        }
    }
}


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
                          let old: bool = (*opts.read()).build_opts.dedup_batches;
                          (*opts.write()).build_opts.dedup_batches = !old;
                      }
                  },
              }
        }
    }
}
