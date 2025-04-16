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
use std::ops::Deref;

use needletail::Sequence;
use needletail::errors::ParseError;

use crate::common::*;

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct BuilderErr {
    code: usize,
    message: String,
}

pub fn build_sbwt(
    ref_data: &[Vec<u8>],
    opts: Option<kbo::BuildOpts>,
) -> (sbwt::SbwtIndexVariant, sbwt::LcsArray) {
    let build_opts = if opts.is_some() { opts } else { Some(kbo::BuildOpts::default()) };
    kbo::index::build_sbwt_from_vecs(ref_data, &build_opts)
}

pub async fn build_indexes(
    queries: &[SeqData],
    build_opts: kbo::BuildOpts,
) -> Vec<IndexData> {
    let query_data: Vec<(String, Vec<Vec<u8>>)> = queries.iter()
                                                                .map(|query| { (
                                                                    query.file_name.clone(),
                                                                    query.contigs.iter().map(|contig| {
                                                                        contig.seq.clone()
                                                                    }).collect::<Vec<Vec<u8>>>()
                                                                )
                                                                }).collect();
    let mut indexes: Vec<IndexData> = Vec::with_capacity(query_data.len());
    for (file_name, seq_data) in query_data {
        let (sbwt, lcs) = crate::util::sbwt_builder(&seq_data, build_opts.clone()).await.unwrap();
        let index = IndexData { sbwt, lcs, file_name: file_name.clone(), bases: seq_data.iter().map(|x| x.len()).sum() };
        indexes.push(index);
    };
    indexes
}

pub async fn build_runner(
    reference: &[SeqData],
    build_opts: kbo::BuildOpts,
    separately: bool,
) -> Result<Vec<IndexData>, BuilderErr> {

    if reference.is_empty() {
        return Err(BuilderErr{ code: 1, message: "Argument `reference` is empty.".to_string() })
    }

    let ref_contigs = reference.first().unwrap();

    let res = if !separately {
        let seq_data: Vec<u8> = ref_contigs.contigs.iter().flat_map(|contig| contig.seq.clone()).collect::<Vec<u8>>();
        let bases: usize = seq_data.len();
        let data = &[seq_data];
        let index = crate::util::sbwt_builder(
            data,
            build_opts.clone(),
        );
        let index = index.await.unwrap();
        vec![IndexData { sbwt: index.0, lcs: index.1, file_name: ref_contigs.file_name.clone(), bases }]
    } else {
        let seq_data: Vec<(String, Vec<u8>)> = ref_contigs.contigs.iter().map(|contig| (contig.name.clone(), contig.seq.clone())).collect::<Vec<(String, Vec<u8>)>>();

        let mut indexes: Vec<IndexData> = Vec::new();
        for (contig_name, contig_seq) in seq_data {
            let bases = contig_seq.len();
            let data = &[contig_seq];
            let index = crate::util::sbwt_builder(
                data,
                build_opts.clone(),
            );
            let index = index.await.unwrap();
            indexes.push(IndexData { sbwt: index.0, lcs: index.1, file_name: contig_name, bases });
        }
        indexes
    };

    if !res.is_empty() {
        return Ok(res)
    }
    Err(BuilderErr{ code: 0, message: "Couldn't index reference data.".to_string() })
}

pub async fn read_seq_data(file_contents: &Vec<u8>) -> Result<Vec<ContigData>, ParseError> {
    let mut seq_data: Vec<ContigData> = Vec::new();
    let mut reader = needletail::parse_fastx_reader(file_contents.deref())?;
    while let Some(rec) = reader.next() {
        let seqrec = rec?;
        let contig = seqrec.id();
        if let Ok(contig_name) = std::str::from_utf8(contig) {
            let seq = seqrec.normalize(true);
            seq_data.push(
                ContigData {
                    name: contig_name.to_string(),
                    seq: seq.to_vec(),
                }
            );
        }
    }
    Ok(seq_data)
}

pub async fn read_fasta_files(
    files: &Vec<(String, Vec<u8>)>,
) -> Result<Vec<SeqData>, ParseError> {
    let mut contigs: Vec<SeqData> = Vec::with_capacity(files.len());
    for (filename, contents) in files {
        let data = crate::util::read_seq_data(contents).await?;
        contigs.push(SeqData { contigs: data, file_name: filename.clone() });
    };

    Ok(contigs)
}

pub async fn sbwt_builder(
    seq_data: &[Vec<u8>],
    build_opts: kbo::BuildOpts,
) -> Result<(sbwt::SbwtIndexVariant, sbwt::LcsArray), BuilderErr> {
    Ok(crate::util::build_sbwt(seq_data, Some(build_opts)))
}
