use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "proteogenomics",
    version = "1.0",
    about = "hmm proteome analyze
    ************************************************
    Author Gaurav Sablok,
    Email: gsablok@proton.me
   ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// select the targets from the hmm.
    TargetHMM {
        /// provide the path to the hmm file
        hmmpath: String,
        /// filter according to the target description
        targetdesc: String,
        /// threads for the analysis
        thread: String,
    },
    /// select the scores from the hmm
    ScoreHMM {
        /// provide the path to the hmm file
        hmmpath: String,
        /// filter according to the score
        score: String,
        /// threads for the analysis
        thread: String,
    },
    /// select the evalues from the hmm
    EvalueHMM {
        /// provide the path to the hmm file
        hmmpath: String,
        /// filter according to the evalue
        evalue: String,
        /// threads for the analysis
        thread: String,
    },
    /// select the targets based on evalue and target
    EvalTarget {
        /// provide the path to the hmm file
        hmmpath: String,
        /// provide the evalue selector
        evalue: String,
        /// provide the target selector
        target: String,
        /// threads for the analysis
        thread: String,
    },
    /// prepare the unique counts of the domains
    Targetcounts {
        /// provide the path to the hmm file
        hmmpath: String,
        /// thread for the analysis
        thread: String,
    },
    PDBId {
        /// please provide the path to the pdb file
        pdbfile: String,
        /// please provide the name of the output file
        output: String,
        /// threads for the analysis
        thread: String,
    },
    /// extract the sequence of the pdf file
    PDBSequence {
        /// pdf file for the analysis
        pdbfile: String,
        /// thread for the analysis
        thread: String,
    },
    /// calculate the euclidean distance bettwen two chain coordinates
    EuclideanComparative {
        /// provide the pdb file
        pdbfile: String,
        /// provide the chain to be selected
        chain: String,
        /// provide the residue 1
        residue1: String,
        /// provide the resiude 2
        residue2: String,
        /// provide the atom1
        atom1: String,
        /// provide the atom2
        atom2: String,
        /// threads for the analysis
        thread: String,
    },
    /// calculates the euclidean distance for all chain atoms
    EuclideanAll {
        /// provide the pdb file
        pdbfile: String,
        /// provide the chain
        chain: String,
        /// threads for the analysis
        thread: String,
    },
}
