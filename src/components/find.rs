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
use crate::dioxus_sortable::*;

use needletail::Sequence;

use crate::common::*;
use crate::opts::GuiOpts;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum FindResultField {
    Query,
    Ref,
    #[default]
    QStart,
    QEnd,
    Strand,
    Length,
    Mismatches,
    GapBases,
    GapOpens,
    Identity,
    Coverage,
    QContig,
    RContig,
}

impl PartialOrdBy<FindResult> for FindResultField {
    fn partial_cmp_by(&self, a: &FindResult, b: &FindResult) -> Option<std::cmp::Ordering> {
        match self {
            FindResultField::Query => a.query_file.partial_cmp(&b.query_file),
            FindResultField::Ref => a.ref_file.partial_cmp(&b.ref_file),
            FindResultField::QStart => a.start.partial_cmp(&b.start),
            FindResultField::QEnd => a.end.partial_cmp(&b.end),
            FindResultField::Strand => a.strand.partial_cmp(&b.strand),
            FindResultField::Length => a.length.partial_cmp(&b.length),
            FindResultField::Mismatches => a.mismatches.partial_cmp(&b.mismatches),
            FindResultField::GapBases => a.gap_bases.partial_cmp(&b.gap_bases),
            FindResultField::GapOpens => a.gap_opens.partial_cmp(&b.gap_opens),
            FindResultField::Identity => a.identity.partial_cmp(&b.identity),
            FindResultField::Coverage => a.coverage.partial_cmp(&b.coverage),
            FindResultField::QContig => a.query_contig.partial_cmp(&b.query_contig),
            FindResultField::RContig => a.ref_contig.partial_cmp(&b.ref_contig),
        }
    }
}

/// This trait decides how fields (columns) may be sorted
impl Sortable for FindResultField {
    fn sort_by(&self) -> Option<SortBy> {
        SortBy::increasing_or_decreasing()
    }

    fn null_handling(&self) -> NullHandling {
        NullHandling::Last
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FindResult {
    query_file: String,
    ref_file: String,
    start: u64,
    end: u64,
    strand: char,
    length: u64,
    mismatches: u64,
    gap_bases: u64,
    gap_opens: u64,
    identity: f64,
    coverage: f64,
    query_contig: String,
    ref_contig: String,
}

#[component]
fn SortableFindResultTable(
    data: Vec::<FindResult>,
) -> Element {
    let sorter = use_sorter::<FindResultField>();
    sorter.read().sort(data.as_mut_slice());

    rsx! {
        table {
            thead {
                tr {
                    Th { sorter: sorter, field: FindResultField::Query, "ref" }
                    Th { sorter: sorter, field: FindResultField::Ref, "query" }
                    Th { sorter: sorter, field: FindResultField::QStart, "r.start" }
                    Th { sorter: sorter, field: FindResultField::QEnd, "r.end" }
                    Th { sorter: sorter, field: FindResultField::Strand, "strand" }
                    Th { sorter: sorter, field: FindResultField::Length, "length" }
                    Th { sorter: sorter, field: FindResultField::Mismatches, "mismatches" }
                    Th { sorter: sorter, field: FindResultField::GapBases, "gap_bases" }
                    Th { sorter: sorter, field: FindResultField::GapOpens, "gap_opens" }
                    Th { sorter: sorter, field: FindResultField::Identity, "identity" }
                    Th { sorter: sorter, field: FindResultField::Coverage, "coverage" }
                    Th { sorter: sorter, field: FindResultField::QContig, "ref.contig" }
                    Th { sorter: sorter, field: FindResultField::RContig, "query.contig" }
                }
            }
            tbody {
                {
                    data.iter().map(|row| {
                        let identity_rounded: String = format!("{:.2}", row.identity);
                        let coverage_rounded: String = format!("{:.2}", row.coverage);
                        rsx! {
                            tr {
                                td { "{row.query_file}" }
                                td { "{row.ref_file}" }
                                td { "{row.start}" }
                                td { "{row.end}" }
                                td { "{row.strand}" }
                                td { "{row.length}" }
                                td { "{row.mismatches}" }
                                td { "{row.gap_bases}" }
                                td { "{row.gap_opens}" }
                                td { "{identity_rounded}" }
                                td { "{coverage_rounded}" }
                                td { "{row.query_contig}" }
                                td { "{row.ref_contig}" }
                            }
                        }
                    })
                }
            }
        }
    }
}

#[component]
fn CopyableFindResultTable(
    data: Vec::<FindResult>,
) -> Element {

    let header = "ref\tquery\tr.start\tr.end\tstrand\tlength\tmismatches\tgap_bases\tgap_opens\tidentity\tcoverage\tref.contig\tquery.contig";
    let display = header.to_string() + &data.iter().map(|x| {
        let identity_rounded: String = format!("{:.2}", x.identity);
        let coverage_rounded: String = format!("{:.2}", x.coverage);

            x.query_file.clone() + "\t" +
            &x.ref_file.clone() + "\t" +
            &x.start.to_string() + "\t" +
            &x.end.to_string() + "\t" +
            &x.strand.to_string() + "\t" +
            &x.length.to_string() + "\t" +
            &x.mismatches.to_string() + "\t" +
            &x.gap_bases.to_string() + "\t" +
            &x.gap_opens.to_string() + "\t" +
            &identity_rounded.to_string() + "\t" +
            &coverage_rounded.to_string() + "\t" +
            &x.query_contig.clone() + "\t" +
            &x.ref_contig.clone() + "\n"
    }).collect::<String>();

    rsx! {
        textarea {
            id: "find-result",
            name: "find-result",
            value: display,
            rows: data.len(),
            width: "99%",
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn format_find_result(
    result: &kbo::format::RLE,
    query_file: String,
    ref_file: String,
    query_contig: String,
    ref_contig: String,
    query_bases: usize,
    ref_bases: usize,
    strand: char,
) -> FindResult {
    let aln_len = result.end - result.start;
    let aln_start = if strand == '+' { result.start } else { query_bases - result.end } + 1;
    let aln_end = if strand == '+' { result.end } else { query_bases - result.start };
    let coverage = (result.matches as f64 + result.mismatches as f64)/(ref_bases as f64) * 100_f64;
    let identity = (result.matches as f64)/(aln_len as f64) * 100_f64;

    FindResult {
        query_file,
        ref_file,
        start: aln_start as u64,
        end: aln_end as u64,
        strand,
        length: aln_len as u64,
        mismatches: result.mismatches as u64,
        gap_bases: result.gap_bases as u64,
        gap_opens: result.gap_opens as u64,
        identity,
        coverage,
        query_contig,
        ref_contig,
    }

}

#[component]
pub fn FindOptsSelector(
    opts: Signal<GuiOpts>,
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
                    "Max gap len",
              }
              div { class: "column-left",
                    input {
                        r#type: "number",
                        id: "max_gap_len",
                        name: "max_gap_len",
                        min: "0",
                        max: "5000",
                        value: opts.read().aln_opts.max_gap_len.to_string(),
                        onchange: move |event| {
                            let new = event.value().parse::<u64>();
                            if let Ok(new_len) = new { opts.write().aln_opts.max_gap_len = new_len };
                        }
                    },
              }
        }
        div { class: "row-contents",
              div { class: "column-right",
                    "Min length",
              }
              div { class: "column-left",
                    input {
                        r#type: "number",
                        id: "min_len",
                        name: "min_len",
                        min: "0",
                        max: "5000",
                        value: opts.read().aln_opts.min_len.to_string(),
                        onchange: move |event| {
                            let new = event.value().parse::<u64>();
                            if let Ok(new_len) = new { opts.write().aln_opts.min_len = new_len };
                        }
                    }
              }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct FindRunnerErr {
    pub code: usize,
    pub message: String,
}

async fn find_runner(
    indexes: &[IndexData],
    queries: &[SeqData],
    ref_file: &str,
    find_opts: kbo::FindOpts,
) -> Result<Vec<FindResult>, FindRunnerErr> {

    if ref_file.is_empty() {
        return Err(FindRunnerErr{ code: 1, message: "Argument `ref_file` is empty.".to_string() })
    }
    if queries.is_empty() {
        return Err(FindRunnerErr{ code: 1, message: "Argument `queries` is empty.".to_string() })
    }
    if indexes.is_empty() {
        return Err(FindRunnerErr{ code: 1, message: "Argument `indexes` is empty.".to_string() })
    }

    let res = indexes.iter().flat_map(|index| {
        queries.iter().flat_map(|query| {
            let mut run_lengths: Vec<FindResult> = Vec::new();

            // Get local alignments for forward strand
            query.contigs.iter().for_each(|contig| {
                let query_bases = contig.seq.len();
                let run_lengths_fwd = kbo::find(&contig.seq, &index.sbwt, &index.lcs, find_opts);
                run_lengths.extend(run_lengths_fwd.iter().map(|x| {
                    format_find_result(x, query.file_name.clone(), ref_file.to_string(), contig.name.clone(), index.file_name.clone(), query_bases, index.bases, '+')
                }));

                // Add local alignments for reverse complement
                let run_lengths_rev = kbo::find(&contig.seq.reverse_complement(), &index.sbwt, &index.lcs, find_opts);
                run_lengths.extend(run_lengths_rev.iter().map(|x| {
                    format_find_result(x, query.file_name.clone(), ref_file.to_string(), contig.name.clone(), index.file_name.clone(), query_bases, index.bases, '-')
                }));

            });

            run_lengths

        }).collect::<Vec<FindResult>>()
    }).collect::<Vec<FindResult>>();

    if !res.is_empty() {
        return Ok(res)
    }

    Err(FindRunnerErr{ code: 0, message: "No alignments detected.".to_string() })
}

#[component]
pub fn Find(
    indexes: ReadOnlySignal<Vec<IndexData>>,
    query_contigs: ReadOnlySignal<Vec<SeqData>>,
    opts: ReadOnlySignal<GuiOpts>,
    result: Signal<Result<Vec<FindResult>, FindRunnerErr>>,
) -> Element {

    if indexes.read().is_empty() {
        return rsx! { { "".to_string() } }
    }
    if query_contigs.read().is_empty() {
        return rsx! { { "".to_string() } }
    }

    let _ = use_resource(move || {
        let ref_name = indexes.read()[0].file_name.clone();
        async move {
            let res = find_runner(&indexes.read(), &query_contigs.read(), &ref_name, opts.read().to_kbo_find()).await;
            result.set(res);
        }
    }).suspend()?;

    rsx!{ br {} }
}

#[component]
pub fn FindRenderer(
    result: ReadOnlySignal<Result<Vec<FindResult>, FindRunnerErr>>,
    opts: ReadOnlySignal<GuiOpts>,
) -> Element {
    match &*result.read() {
        Ok(data) => {
            let req_len = opts.read().aln_opts.min_len;
            let filtered = data.iter().filter_map(|x| if x.length >= req_len{ Some(x.clone()) } else { None } ).collect::<Vec<FindResult>>();
            rsx! {
                if opts.read().out_opts.interactive {
                    SortableFindResultTable { data: filtered }
                } else {
                    CopyableFindResultTable { data: filtered }
                }
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
