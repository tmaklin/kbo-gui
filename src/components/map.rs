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

#[component]
pub fn MapOptsSelector(
    opts: Signal<GuiOpts>,
) -> Element {
    rsx! {
        div { class: "row-contents",
              div { class: "column-right",
                    "Error tolerance"
              },
              div { class: "column-left",
                    input {
                        r#type: "number",
                        id: "min_len",
                        name: "min_len",
                        min: "0",
                        max: "1.00",
                        value: opts.read().aln_opts.max_error_prob.to_string(),
                        onchange: move |event| {
                            let new = event.value().parse::<f64>();
                            if let Ok(new_prob) = new { opts.write().aln_opts.max_error_prob = new_prob.clamp(0_f64 + f64::EPSILON, 1_f64 - f64::EPSILON) };
                        }
                    },
              }
        }
        div { class: "row-contents",
              div { class: "column-right",
                    "Call variants"
              },
              div { class: "column-left",
                    input {
                        r#type: "checkbox",
                        id: "do_vc",
                        name: "do_vc",
                        checked: opts.read().aln_opts.do_vc,
                        onchange: move |_| {
                            let old: bool = opts.read().aln_opts.do_vc;
                            opts.write().aln_opts.do_vc = !old;
                        }
                    },
              }
        }
        div { class: "row-contents",
              div { class: "column-right",
                    "Fill gaps"
              },
              div { class: "column-left",
                    input {
                        r#type: "checkbox",
                        id: "do_gapfill",
                        name: "do_gapfill",
                        checked: opts.read().aln_opts.do_gapfill,
                        onchange: move |_| {
                            let old: bool = opts.read().aln_opts.do_gapfill;
                            opts.write().aln_opts.do_gapfill = !old;
                        }
                    },
              }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct MapRunnerErr {
    code: usize,
    message: String,
}

async fn map_runner(
    reference: &[SeqData],
    queries: &[IndexData],
    map_opts: kbo::MapOpts,
) -> Result<Vec<(String, Vec<u8>)>, MapRunnerErr> {

    if reference.is_empty() {
        return Err(MapRunnerErr{ code: 1, message: "Argument `reference` is empty.".to_string() })
    }

    if queries.is_empty() {
        return Err(MapRunnerErr{ code: 1, message: "Argument `queries` is empty.".to_string() })
    }

    let ref_contigs = reference.first().unwrap();
    let aln = queries.iter().map(|index| {

        let res: Vec<u8> = ref_contigs.contigs.iter().flat_map(|ref_contig| {
                                    kbo::map(&ref_contig.seq, &index.sbwt, &index.lcs, map_opts.clone())
                                }).collect();
        (index.file_name.clone(), res)
    }).collect::<Vec<(String, Vec<u8>)>>();

    if !aln.is_empty() {
        return Ok(aln)
    }

    Err(MapRunnerErr{ code: 0, message: "Mapping error.".to_string() })
}

#[component]
pub fn Map(
    ref_contigs: ReadOnlySignal<Vec<SeqData>>,
    indexes: ReadOnlySignal<Vec<IndexData>>,
    opts: ReadOnlySignal<GuiOpts>,
) -> Element {

    if ref_contigs.read().is_empty() {
        return rsx! { { "".to_string() } }
    }
    if indexes.read().is_empty() {
        return rsx! { { "".to_string() } }
    }

    let aln = use_resource(move || {
        async move {
            gloo_timers::future::TimeoutFuture::new(100).await;
            map_runner(&ref_contigs.read(), &indexes.read(), opts.read().to_kbo_map()).await
        }
    }).suspend()?;

    match &*aln.read_unchecked() {
        Ok(data) => {
            rsx! {
                CopyableMapResult { data: data.to_vec() }
            }
        },
        Err(e) => {
            match e.code {
                0 => rsx! { { "Error: ".to_string() + &e.message } },
                _ => rsx! { { "" } },
            }
        },
    }
}

#[component]
fn CopyableMapResult(
    data: Vec<(String, Vec<u8>)>,
) -> Element {

    let display = data.iter().map(|(file, aln)| {
        let mut counter = 0;
        let mut out = [">".to_owned() + file + &'\n'.to_string(),
             aln.iter().flat_map(|x| {
                 counter += 1;
                 if counter % 80 == 0 {
                     counter = 0;
                     vec![*x as char, '\n']
                 } else {
                     vec![*x as char]
                 }
             }).collect::<String>()].concat();
        if out.as_bytes()[out.len() - 1] != b'\n' {
            out += "\n";
        }
        out
    }).collect::<String>();

    let rows = display.len().div_ceil(80);

    rsx! {
        textarea {
            id: "find-result",
            name: "find-result",
            value: display,
            rows: rows,
            width: "95%",
        },
    }
}
