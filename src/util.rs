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

#[derive(Clone, PartialEq)]
pub struct ContigData {
    pub name: String,
    pub seq: Vec<u8>,
}

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
) -> Result<Vec<(String, Vec<ContigData>)>, ParseError> {
    let mut contigs: Vec<(String, Vec<crate::util::ContigData>)> = Vec::with_capacity(files.len());
    for (filename, contents) in files {
        let data = crate::util::read_seq_data(&contents).await?;
        contigs.push((filename.to_string(), data));
    };

    Ok(contigs)
}

pub async fn sbwt_builder(
    seq_data: &[Vec<u8>],
    build_opts: kbo::BuildOpts,
) -> Result<(sbwt::SbwtIndexVariant, sbwt::LcsArray), BuilderErr> {
    Ok(crate::util::build_sbwt(seq_data, Some(build_opts)))
}
