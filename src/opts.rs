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

#[derive(Clone, Copy, Default, PartialEq)]
pub struct GuiOpts {
    pub out_opts: OutOpts,
    pub build_opts: BuildOpts,
    pub aln_opts: AlnOpts,
}

impl GuiOpts {
    pub fn to_kbo_call(self) -> kbo::CallOpts {
        let mut call_opts = kbo::CallOpts::default();
        call_opts.max_error_prob = self.aln_opts.max_error_prob;
        call_opts.sbwt_build_opts = self.build_opts.to_kbo();
        call_opts
    }

    pub fn to_kbo_find(self) -> kbo::FindOpts {
        let mut find_opts = kbo::FindOpts::default();
        find_opts.max_error_prob = self.aln_opts.max_error_prob;
        find_opts.max_gap_len = self.aln_opts.max_gap_len as usize;
        find_opts
    }

    pub fn to_kbo_map(self) -> kbo::MapOpts {
        let mut map_opts = kbo::MapOpts::default();
        map_opts.max_error_prob = self.aln_opts.max_error_prob;
        map_opts.call_variants = self.aln_opts.do_vc;
        map_opts.fill_gaps = self.aln_opts.do_gapfill;
        map_opts.sbwt_build_opts = self.build_opts.to_kbo();
        map_opts
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct OutOpts {
    pub interactive: bool,
    pub detailed: bool,
}

impl Default for OutOpts {
    fn default() -> OutOpts {
        OutOpts {
            interactive: true,
            detailed: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct BuildOpts {
    pub kmer_size: u32,
    pub dedup_batches: bool,
    pub prefix_precalc: u32,
}

impl Default for BuildOpts {
    fn default() -> BuildOpts {
        BuildOpts {
            kmer_size: 51,
            dedup_batches: true,
            prefix_precalc: 8,
        }
    }
}

impl BuildOpts {
    pub fn to_kbo(self) -> kbo::BuildOpts {
        let mut kbo_opts = kbo::BuildOpts::default();
        kbo_opts.build_select = true;
        kbo_opts.k = self.kmer_size as usize;
        kbo_opts.dedup_batches = self.dedup_batches;
        kbo_opts.prefix_precalc = self.prefix_precalc as usize;
        kbo_opts
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct AlnOpts {
    pub max_error_prob: f64,
    pub min_len: u64,
    pub max_gap_len: u64,
    pub do_vc: bool,
    pub do_gapfill: bool,
}

impl Default for AlnOpts {
    fn default() -> AlnOpts {
        AlnOpts {
            max_error_prob: 0.0000001,
            min_len: 100,
            max_gap_len: 0,
            do_vc: true,
            do_gapfill: true
        }
    }
}
