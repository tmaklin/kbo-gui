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

#[derive(Clone, PartialEq)]
pub struct ContigData {
    pub name: String,
    pub seq: Vec<u8>,
}

pub fn build_sbwt(ref_data: &[Vec<u8>]) -> (sbwt::SbwtIndexVariant, sbwt::LcsArray) {
    kbo::index::build_sbwt_from_vecs(ref_data, &Some(kbo::index::BuildOpts::default()))
}

pub fn read_seq_data(file_contents: &Vec<u8>) -> Vec<ContigData> {
    let mut seq_data: Vec<ContigData> = Vec::new();
    let mut reader = needletail::parse_fastx_reader(file_contents.deref()).expect("valid fastX data");
    while let Some(rec) = reader.next() {
        let seqrec = rec.expect("Valid fastX record");
        let contig = seqrec.id();
        let seq = seqrec.normalize(true);
        seq_data.push(
            ContigData {
                name: std::str::from_utf8(contig).expect("UTF-8").to_string(),
                seq: seq.to_vec(),
            });
    }
    seq_data
}
