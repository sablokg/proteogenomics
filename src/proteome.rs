#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Proteinanalyze {
    pub targetname: String,
    pub tlength: usize,
    pub queryname: String,
    pub querylength: usize,
    pub evalue: f32,
    pub score: f32,
    pub val1: usize,
    pub val2: usize,
    pub ed1: usize,
    pub ed2: usize,
    pub al1: usize,
    pub al2: usize,
    pub desc: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pdbanalyze {
    pub adtype: String,
    pub dnumber: String,
    pub itype: String,
    pub gtype: String,
    pub ntype: String,
    pub atype: String,
    pub btype: String,
    pub ctype: String,
    pub utype: String,
    pub attype: String,
    pub lttype: String,
    pub uchar: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pdbsequence {
    pub adtype: String,
    pub dnumber: String,
    pub gtype: String,
    pub ntype: String,
    pub atype: String,
    pub btype: String,
    pub ctype: String,
    pub utype: String,
    pub attype: String,
    pub lttype: String,
    pub uchar: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Endchain {
    pub tab0: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Comprativedrive {
    pub chain: String,
    pub residue: usize,
    pub coordinate1: f32,
    pub coordinate2: f32,
    pub coordinate3: f32,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct EuclideanWrite {
    pub tab1: f32,
    pub tab2: f32,
    pub tab3: f32,
    pub tab4: f32,
    pub tab5: f32,
    pub tab6: f32,
    pub tab7: f32,
}
