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
use crate::dioxus_sortable::*;

use chrono::offset::Local;
use dioxus::prelude::*;
use kbo::variant_calling::Variant;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum CallResultField {
    Chrom,
    #[default]
    Pos,
    Id,
    Ref,
    Alt,
    Qual,
    Filter,
    Info,
    Format,
    Unknown,
}

impl PartialOrdBy<CallResult> for CallResultField {
    fn partial_cmp_by(&self, a: &CallResult, b: &CallResult) -> Option<std::cmp::Ordering> {
        match self {
            CallResultField::Chrom => a.chromosome.partial_cmp(&b.chromosome),
            CallResultField::Pos => a.position.partial_cmp(&b.position),
            CallResultField::Id => a.id.partial_cmp(&b.id),
            CallResultField::Ref => a.ref_base.partial_cmp(&b.ref_base),
            CallResultField::Alt => a.alt_base.partial_cmp(&b.alt_base),
            CallResultField::Qual => a.qual.partial_cmp(&b.qual),
            CallResultField::Filter => a.filter.partial_cmp(&b.filter),
            CallResultField::Info => a.info.partial_cmp(&b.info),
            CallResultField::Format => a.format.partial_cmp(&b.format),
            CallResultField::Unknown => a.unknown.partial_cmp(&b.unknown),
        }
    }
}

/// This trait decides how fields (columns) may be sorted
impl Sortable for CallResultField {
    fn sort_by(&self) -> Option<SortBy> {
        SortBy::increasing_or_decreasing()
    }

    fn null_handling(&self) -> NullHandling {
        NullHandling::Last
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallResult {
    chromosome: String,
    position: u64,
    id: String,
    ref_base: String,
    alt_base: String,
    qual: String,
    filter: String,
    info: String,
    format: String,
    unknown: String,
}

#[component]
pub fn SortableCallResultTable(
    data: Vec::<CallResult>,
) -> Element {
    let sorter = use_sorter::<CallResultField>();
    sorter.read().sort(data.as_mut_slice());

    rsx! {
        table {
            thead {
                tr {
                    Th { sorter: sorter, field: CallResultField::Chrom, "CHROM" }
                    Th { sorter: sorter, field: CallResultField::Pos, "POS" }
                    Th { sorter: sorter, field: CallResultField::Id, "ID" }
                    Th { sorter: sorter, field: CallResultField::Ref, "REF" }
                    Th { sorter: sorter, field: CallResultField::Alt, "ALT" }
                    Th { sorter: sorter, field: CallResultField::Qual, "QUAL" }
                    Th { sorter: sorter, field: CallResultField::Filter, "FILTER" }
                    Th { sorter: sorter, field: CallResultField::Info, "INFO" }
                    Th { sorter: sorter, field: CallResultField::Format, "FORMAT" }
                    Th { sorter: sorter, field: CallResultField::Unknown, "unknown" }
                }
            }
            tbody {
                {
                    data.iter().map(|row| {
                        rsx! {
                            tr {
                                td { "{row.chromosome}" }
                                td { "{row.position}" }
                                td { "{row.id}" }
                                td { "{row.ref_base}" }
                                td { "{row.alt_base}" }
                                td { "{row.qual}" }
                                td { "{row.filter}" }
                                td { "{row.info}" }
                                td { "{row.format}" }
                                td { "{row.unknown}" }
                            }
                        }
                    })
                }
            }
        }
    }
}

#[component]
fn CopyableCallResultTable(
    data: Vec::<CallResult>,
    ref_path: String,
    contig_info: Vec<(String, usize)>,
) -> Element {

    let display = format_call_header(&ref_path, &contig_info) +
        &data.iter().map(|x| {
        x.chromosome.clone() + "\t" +
            &x.position.to_string() + "\t" +
            &x.id.to_string() + "\t" +
            &x.ref_base.to_string() + "\t" +
            &x.alt_base.to_string() + "\t" +
            &x.qual.to_string() + "\t" +
            &x.filter.to_string() + "\t" +
            &x.info.to_string() + "\t" +
            &x.format.clone() + "\t" +
            &x.unknown.clone() + "\n"
    }).collect::<String>();

    rsx! {
        textarea {
            id: "call-result",
            name: "call-result",
            value: display,
            rows: data.len() + 10,
            width: "98%",
        },
    }
}

fn split_flanking_variants(
    ref_var: &[u8],
    query_var: &[u8],
    query_pos: usize,
) -> Option<(Variant, Variant)> {
    let ref_len = ref_var.len();
    if ref_len != query_var.len() || ref_len == 1 {
        return None
    }

    let first_mismatch = ref_var[0] != query_var[0];
    let last_mismatch = ref_var[ref_len - 1] != query_var[ref_len - 1];

    let mut middle_match = true;
    for pos in 1..(ref_len - 1) {
        middle_match &= ref_var[pos] == query_var[pos];
    }

    if first_mismatch && last_mismatch && middle_match {
        Some(
            (Variant{query_chars: vec![query_var[0]], ref_chars: vec![ref_var[0]], query_pos},
             Variant{query_chars: vec![query_var[ref_len - 1]], ref_chars: vec![ref_var[ref_len - 1]], query_pos: query_pos + ref_len - 1})
        )
    } else {
        None
    }
}

fn format_call_result(
    variant: &Variant,
    ref_seq: &[u8],
    contig: &str,
) -> CallResult {
    let is_indel = variant.ref_chars.len() != variant.query_chars.len();
    let mut pos = variant.query_pos as u64;

    let (alt_bases, ref_bases) = if is_indel {
        // Add nucleotide preceding an indel to the output
        // (.vcf does not like empty bases in REF or ALT)
        //
        let alt_bases = (ref_seq[variant.query_pos - 1] as char).to_string() + &variant.ref_chars.iter().map(|nt| *nt as char).collect::<String>();
        let ref_bases = (ref_seq[variant.query_pos - 1] as char).to_string() + &variant.query_chars.iter().map(|nt| *nt as char).collect::<String>();
        // We added 1 base so decrement position by 1
        pos -= 1;
        (alt_bases, ref_bases)
    } else {
        let alt_bases = variant.ref_chars.iter().map(|nt| *nt as char).collect::<String>();
        let ref_bases = variant.query_chars.iter().map(|nt| *nt as char).collect::<String>();
        (alt_bases, ref_bases)
    };

    let info = if variant.ref_chars.len() != 1 || variant.query_chars.len() != 1 {
        "INDEL"
    } else {
        "."
    }.to_string();

    CallResult {
        chromosome: contig.to_string(),
        position: pos,
        id: ".".to_string(),
        ref_base: ref_bases,
        alt_base: alt_bases,
        qual: ".".to_string(),
        filter: ".".to_string(),
        info,
        format: "GT".to_string(),
        unknown: "1".to_string(),
    }

}

fn format_call_header(
    ref_file: &str,
    contig_info: &[(String, usize)],
) -> String {
    let current_date = Local::now().format("%Y%m%d").to_string();
    "##fileformat=VCFv4.4\n".to_string() +
        &contig_info.iter().map(|(name, length)| {
            let mut header_contents = name.split_whitespace();
            let contig_name = header_contents.next().expect("Contig name");
            "##contig=<ID=".to_owned() + contig_name + ",length=" + &length.to_string() + ">\n"
        }).collect::<String>() +
        "##contig=<ID=PLACEHOLDER,length=99999>\n" +
        "##fileDate=" + &current_date.to_string() + "\n" +
        "##source=kbo-gui v" + env!("CARGO_PKG_VERSION") + "\n" +
        "##reference=" + ref_file + "\n" +
        "##phasing=none\n"
}

#[component]
pub fn CallOptsSelector(
    max_error_prob: Signal<f64>,
) -> Element {
    rsx! {
        div { class: "row-contents",
              div { class: "column-right",
                    "Error tolerance",
              }
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
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct CallRunnerErr {
    code: usize,
    message: String,
}

async fn call_runner(
    reference: &[(String, Vec<crate::util::ContigData>)],
    queries: &[(String, Vec<crate::util::ContigData>)],
    call_opts: kbo::CallOpts,
) -> Result<(Vec<(String, usize)>, Vec<CallResult>), CallRunnerErr>{

    if reference.is_empty() {
        return Err(CallRunnerErr{ code: 2, message: "Argument `reference` is empty.".to_string() })
    }
    if queries.is_empty() {
        return Err(CallRunnerErr{ code: 3, message: "Argument `queries` is empty.".to_string() })
    }

    let query_data: Vec<Vec<u8>> = queries[0].1.iter().map(|x| x.seq.clone()).collect();
    let index = crate::util::sbwt_builder(&query_data, call_opts.sbwt_build_opts.clone()).await;

    match index {
        Ok((sbwt_query, sbwt_lcs)) => {

            let mut contig_info: Vec<(String, usize)> = Vec::with_capacity(reference[0].1.len());
            let mut res: Vec<CallResult> = Vec::new();

            reference[0].1.iter().for_each(|contig| {
                let mut header_contents = contig.name.split_whitespace();
                let contig_name = header_contents.next().expect("Contig name");
                contig_info.push((contig.name.clone(), contig.seq.len()));
                let variants = kbo::call(&sbwt_query, &sbwt_lcs, &contig.seq, call_opts.clone());

                res.extend(variants.iter().flat_map(|variant| {

                    let flanking = split_flanking_variants(&variant.ref_chars, &variant.query_chars, variant.query_pos);
                    if flanking.is_some() {
                        let (var1, var2) = flanking.unwrap();
                        let record1 = format_call_result(&var1, &contig.seq, contig_name);
                        let record2 = format_call_result(&var2, &contig.seq, contig_name);
                        vec![record1, record2]
                    } else {
                        vec![format_call_result(variant, &contig.seq, contig_name)]
                    }
                }));
            });
            if !res.is_empty() {
                Ok((contig_info, res))
            } else {
                Err(CallRunnerErr{ code: 0, message: "No variants detected.".to_string() })
            }
        },
        Err(_) => Err(CallRunnerErr{ code: 1, message: "Variant calling error.".to_string() })
    }
}

#[component]
pub fn Call(
    ref_contigs: ReadOnlySignal<Vec<(String, Vec<crate::util::ContigData>)>>,
    query_contigs: ReadOnlySignal<Vec<(String, Vec<crate::util::ContigData>)>>,
    interactive: ReadOnlySignal<bool>,
    call_opts: kbo::CallOpts,
) -> Element {

    let variants = use_resource(move || {
        let opts = call_opts.clone();
        async move {
            // Delay start to render a loading spinner
            gloo_timers::future::TimeoutFuture::new(100).await;
            call_runner(&ref_contigs.read(), &query_contigs.read(), opts).await
        }
    }).suspend()?;

    match &*variants.read_unchecked() {
        Ok(data) => {
            let ref_path = ref_contigs.read()[0].0.clone();
            rsx! {
                if *interactive.read() {
                    SortableCallResultTable { data: data.1.to_vec() }
                } else {
                    CopyableCallResultTable { data: data.1.to_vec(), ref_path, contig_info: data.0.to_vec() }
                }
            }
        },
        Err(e) => rsx! { { "Error: ".to_string() + &e.message } }
    }
}
