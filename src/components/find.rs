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

use crate::components::common::BuildOptsSelector;

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
struct FindResult {
    query_file: String,
    ref_file: String,
    start: u64,
    end: u64,
    strand: char,
    length: u64,
    mismatches: u64,
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
                    Th { sorter: sorter, field: FindResultField::Query, "query" }
                    Th { sorter: sorter, field: FindResultField::Ref, "ref" }
                    Th { sorter: sorter, field: FindResultField::QStart, "q.start" }
                    Th { sorter: sorter, field: FindResultField::QEnd, "q.end" }
                    Th { sorter: sorter, field: FindResultField::Strand, "strand" }
                    Th { sorter: sorter, field: FindResultField::Length, "length" }
                    Th { sorter: sorter, field: FindResultField::Mismatches, "mismatches" }
                    Th { sorter: sorter, field: FindResultField::GapOpens, "gap_opens" }
                    Th { sorter: sorter, field: FindResultField::Identity, "identity" }
                    Th { sorter: sorter, field: FindResultField::Coverage, "coverage" }
                    Th { sorter: sorter, field: FindResultField::QContig, "query.contig" }
                    Th { sorter: sorter, field: FindResultField::RContig, "ref.contig" }
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

    let display = data.iter().map(|x| {
        let identity_rounded: String = format!("{:.2}", x.identity);
        let coverage_rounded: String = format!("{:.2}", x.coverage);

            x.query_file.clone() + "," +
            &x.ref_file.clone() + "," +
            &x.start.to_string() + "," +
            &x.end.to_string() + "," +
            &x.strand.to_string() + "," +
            &x.length.to_string() + "," +
            &x.mismatches.to_string() + "," +
            &x.gap_opens.to_string() + "," +
            &identity_rounded.to_string() + "," +
            &coverage_rounded.to_string() + "," +
            &x.query_contig.clone() + "," +
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

fn format_find_result(
    result: &kbo::format::RLE,
    query_contig: String,
    ref_contig: String,
    ref_bases: u64,
    strand: char,
) -> FindResult {
    let aln_len = result.end - result.start + 1;
    let coverage = (aln_len as f64)/(ref_bases as f64) * 100_f64;
    let identity = (result.matches as f64)/(aln_len as f64) * 100_f64;

    FindResult {
        query_file: "query".to_string(),
        ref_file: "ref".to_string(),
        start: result.start as u64,
        end: result.end as u64,
        strand,
        length: (result.end - result.start + 1) as u64,
        mismatches: result.mismatches as u64,
        gap_opens: result.gap_opens as u64,
        identity,
        coverage,
        query_contig,
        ref_contig,
    }

}

#[component]
pub fn FindOptsSelector(
    min_len: Signal<u64>,
    max_gap_len: Signal<u64>,
    max_error_prob: Signal<f64>,
) -> Element {
    rsx! {
        div { class: "row-contents",
              div { class: "column",
                    "Max. gap size",
              }
              div { class: "column",
                    input {
                        r#type: "number",
                        id: "max_gap_len",
                        name: "max_gap_len",
                        min: "0",
                        max: "5000",
                        value: "0",
                        onchange: move |event| {
                            let new = event.value().parse::<u64>();
                            if let Ok(new_len) = new { max_gap_len.set(new_len) };
                        }
                    },
              }
        }
        div { class: "row-contents",
              div { class: "column",
                    "Error tolerance",
              }
              div { class: "column",
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
              div { class: "column",
                    "Min. length",
              }
              div { class: "column",
                    input {
                        r#type: "number",
                        id: "min_len",
                        name: "min_len",
                        min: "0",
                        max: "5000",
                        value: "100",
                        onchange: move |event| {
                            let new = event.value().parse::<u64>();
                            if let Ok(new_len) = new { min_len.set(new_len) };
                        }
                    }
              }
        }
    }
}

#[component]
pub fn Find(
    ref_files: Signal<Vec<Vec<u8>>>,
    query_files: Signal<Vec<Vec<u8>>>,
    queries: Vec<crate::util::ContigData>,
    refseqs: Vec<crate::util::ContigData>,
) -> Element {

    let mut res = use_signal(Vec::<FindResult>::new);

    // Options for running queries
    let mut detailed: Signal<bool> = use_signal(|| false);
    let mut interactive: Signal<bool> = use_signal(|| true);
    let min_len: Signal<u64> = use_signal(|| 100_u64);
    let max_gap_len: Signal<u64> = use_signal(|| 0_u64);
    let max_error_prob: Signal<f64> = use_signal(|| 0.0000001_f64);

    // Options for indexing reference
    let kmer_size: Signal<u32> = use_signal(|| 31);
    let dedup_batches: Signal<bool> = use_signal(|| true);
    let prefix_precalc: Signal<u32> = use_signal(|| 8);

    rsx! {
        div { class: "row",
              div { class: "column-left",
                    input {
                        r#type: "checkbox",
                        name: "detailed",
                        id: "detailed",
                        checked: false,
                        onchange: move |_| {
                            let old: bool = *detailed.read();
                            *detailed.write() = !old;
                        }
                    },
                    "Split reference by contig",
              }

              div { class: "column-right" }
        }

        div { class: "row",
              div { class: "column-left",
                    details {
                        summary { "Indexing options" }
                        BuildOptsSelector { kmer_size, dedup_batches, prefix_precalc }
                    }
              }
              div { class: "column-right",
                    details {
                        summary { "Alignment options" }
                        FindOptsSelector { min_len, max_gap_len, max_error_prob }
                    }
              }
        }

        div { class: "row-run",
              div { class: "column",
                    button {
                        onclick: move |_event| {
                            if ref_files.read().len() > 0 && query_files.read().len() > 0 {
                                let mut find_opts = kbo::FindOpts::default();
                                find_opts.max_error_prob = *max_error_prob.read();
                                find_opts.max_gap_len = *max_gap_len.read() as usize;

                                let mut indexes: Vec<((sbwt::SbwtIndexVariant, sbwt::LcsArray), String, u64)> = Vec::new();

                                // Options for indexing reference
                                let mut build_opts = kbo::BuildOpts::default();
                                build_opts.k = *kmer_size.read() as usize;
                                build_opts.dedup_batches = *dedup_batches.read();
                                build_opts.prefix_precalc = *prefix_precalc.read() as usize;

                                if !*detailed.read() {

                                    // TODO Work around cloning reference contig data in Find

                                    let bases: u64 = refseqs.iter().map(|contig| contig.seq.len() as u64).reduce(|a, b| a + b).unwrap();
                                    indexes.push((crate::util::build_sbwt(
                                        &[refseqs.iter().flat_map(|contig| contig.seq.clone()).collect()],
                                        Some(build_opts),
                                    ), "ref_file".to_string(), bases));
                                } else {
                                    refseqs.iter().for_each(|contig| {
                                        let bases: u64 = contig.seq.len() as u64;
                                        indexes.push((crate::util::build_sbwt(
                                            &[contig.seq.clone()],
                                            Some(build_opts.clone()),
                                        ), contig.name.clone(), bases));
                                    });
                                }

                                *res.write() = Vec::<FindResult>::new();
                                indexes.iter().for_each(|((sbwt, lcs), ref_contig, ref_bases)| {
                                    queries.iter().for_each(|contig| {
                                        let mut run_lengths: Vec<FindResult> = Vec::new();

                                        // Get local alignments for forward strand
                                        let run_lengths_fwd = kbo::find(&contig.seq, sbwt, lcs, find_opts);
                                        run_lengths.extend(run_lengths_fwd.iter().map(|x| {
                                            format_find_result(x, contig.name.clone(), ref_contig.clone(), *ref_bases, '+')
                                        }));

                                        // Add local alignments for reverse complement
                                        let run_lengths_rev = kbo::find(&contig.seq.reverse_complement(), sbwt, lcs, find_opts);
                                        run_lengths.extend(run_lengths_rev.iter().map(|x| {
                                            format_find_result(x, contig.name.clone(), ref_contig.clone(), *ref_bases, '-')
                                        }));

                                        // Print results with query and ref name added
                                        res.write().extend(run_lengths);

                                    });
                                });
                            }
                        },
                        "Run analysis",
                    }
              }
              div { class: "column",
                    input {
                        r#type: "checkbox",
                        name: "interactive",
                        id: "interactive",
                        checked: true,
                        onchange: move |_| {
                            let old: bool = *interactive.read();
                            *interactive.write() = !old;
                        }
                    },
                    "Interactive output",
              }
        }

        div { class: "row-results",
              if res.read().len() > 0 {
                  {
                      let data = res.read()
                                    .to_vec()
                                    .iter()
                                    .filter_map(|x|
                                                if x.length >= *min_len.read() {
                                                    Some(x.clone())
                                                } else {
                                                    None
                                                }
                                    ).collect::<Vec<_>>();

                      rsx! {
                          if *interactive.read() {
                              SortableFindResultTable { data }
                          } else {
                              CopyableFindResultTable { data }
                          }
                      }
                  }
              }
        }
    }
}
