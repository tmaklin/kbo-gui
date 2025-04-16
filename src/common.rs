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

use sbwt::LcsArray;
use sbwt::SbwtIndexVariant;

#[derive(Default, PartialEq)]
pub enum KboMode {
    #[default]
    Call,
    Find,
    Map,
}

#[derive(Clone, PartialEq)]
pub struct ContigData {
    pub name: String,
    pub seq: Vec<u8>,
}

pub struct IndexData {
    pub sbwt: SbwtIndexVariant,
    pub lcs: LcsArray,
    pub file_name: String,
    pub bases: usize,
}

impl Clone for IndexData {
    fn clone(&self) -> IndexData {
        IndexData {
            sbwt: match &self.sbwt {
                SbwtIndexVariant::SubsetMatrix(index) => {
                    sbwt::SbwtIndexVariant::SubsetMatrix(index.clone())
                },
            },
            lcs: self.lcs.clone(),
            file_name: self.file_name.clone(),
            bases: self.bases,
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct SeqData {
    pub contigs: Vec<ContigData>,
    pub file_name: String,
}
