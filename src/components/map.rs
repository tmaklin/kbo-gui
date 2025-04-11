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
pub fn MapOptsSelector(
    max_error_prob: Signal<f64>,
    do_vc: Signal<bool>,
    do_gapfill: Signal<bool>,
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
                        value: "0.0000001",
                        onchange: move |event| {
                            let new = event.value().parse::<f64>();
                            if let Ok(new_prob) = new { max_error_prob.set(new_prob.clamp(0_f64 + f64::EPSILON, 1_f64 - f64::EPSILON)) };
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
                        checked: *do_vc.read(),
                        onchange: move |_| {
                            let old: bool = *do_vc.read();
                            *do_vc.write() = !old;
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
                        checked: *do_gapfill.read(),
                        onchange: move |_| {
                            let old: bool = *do_gapfill.read();
                            *do_gapfill.write() = !old;
                        }
                    },
              }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct MapRunnerErr {
    code: usize,
    message: String,
}

async fn map_runner(
    reference: &[(String, Vec<crate::util::ContigData>)],
    queries: &[(String, Vec<crate::util::ContigData>)],
    map_opts: kbo::MapOpts,
) -> Result<Vec<(String, Vec<u8>)>, MapRunnerErr> {

    if reference.is_empty() {
        return Err(MapRunnerErr{ code: 1, message: "Argument `reference` is empty.".to_string() })
    }

    let aln = queries.iter().map(|(query_file, query_contig)| {

        let (sbwt, lcs) = crate::util::build_sbwt(
            &query_contig.iter().map(|x| x.seq.clone()).collect::<Vec<Vec<u8>>>(),
            Some(map_opts.sbwt_build_opts.clone()),
        );

        let res: Vec<u8> = reference[0].1.iter().flat_map(|ref_contig| {
                                    kbo::map(&ref_contig.seq, &sbwt, &lcs, map_opts.clone())
                                }).collect();
        (query_file.clone(), res)
    }).collect::<Vec<(String, Vec<u8>)>>();

    if !aln.is_empty() {
        return Ok(aln)
    }

    Err(MapRunnerErr{ code: 1, message: "Mapping error.".to_string() })
}

#[component]
pub fn Map(
    ref_contigs: ReadOnlySignal<Vec<(String, Vec<crate::util::ContigData>)>>,
    query_contigs: ReadOnlySignal<Vec<(String, Vec<crate::util::ContigData>)>>,
    map_opts: kbo::MapOpts,
) -> Element {

    let aln = use_resource(move || {
        let opts = map_opts.clone();
        async move {
            // Delay start to render a loading spinner
            gloo_timers::future::TimeoutFuture::new(100).await;
            map_runner(&ref_contigs.read(), &query_contigs.read(), opts).await
        }
    }).suspend()?;

    match &*aln.read_unchecked() {
        Ok(data) => {
            rsx! {
                CopyableMapResult { data: data.to_vec() }
            }
        },
        Err(e) => rsx! { { "Error: ".to_string() + &e.message } },
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
