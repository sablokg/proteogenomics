mod args;
mod count;
mod euclidean;
mod evaltarget;
mod evalue;
mod proteome;
mod score;
mod target;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::count::hmmcount;
use crate::euclidean::eucledianall;
use crate::euclidean::euclediancomparativetwo;
use crate::euclidean::pdbchainwrite;
use crate::euclidean::pdbsequence;
use crate::evaltarget::hmmevaltarget;
use crate::evalue::hmmevalue;
use crate::score::hmmscore;
use crate::target::hmmtarget;
use clap::Parser;
use figlet_rs::FIGfont;

/*
Gaurav Sablok,
gsablok@proton.me
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("proteogenomics");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::TargetHMM {
            hmmpath,
            targetdesc,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = hmmtarget(hmmpath, &targetdesc.clone()).unwrap();
                println!(
                    "The target from the corresponding hmm files have been filtered: {:?}",
                    command
                );
            });
        }
        Commands::ScoreHMM {
            hmmpath,
            score,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = hmmscore(hmmpath, score).unwrap();
                println!(
                    "The values after applying the score filter are:{:?}",
                    command
                );
            });
        }
        Commands::EvalueHMM {
            hmmpath,
            evalue,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = hmmevalue(hmmpath, evalue).unwrap();
                println!(
                    "The values after applying the evalue filter are: {:?}",
                    command
                );
            });
        }
        Commands::Targetcounts { hmmpath, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = hmmcount(hmmpath).unwrap();
                println!(
                    "The domain aligned targets for the enrichment are: {:?}",
                    command
                );
            });
        }
        Commands::EvalTarget {
            hmmpath,
            evalue,
            target,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = hmmevaltarget(hmmpath, evalue, target).unwrap();
                println!(
                    "The filtered evalues along with the target have been written: {:?}",
                    command
                );
            });
        }
        Commands::PDBId {
            pdbfile,
            output,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = pdbchainwrite(pdbfile, output).unwrap();
                println!(
                    "The atom coordinates for the following pdbf file are: {:?}",
                    command
                );
            });
        }
        Commands::PDBSequence { pdbfile, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = pdbsequence(pdbfile);
                println!("The protein sequence is as follows: {:?}", command);
            });
        }
        Commands::EuclideanComparative {
            pdbfile,
            chain,
            residue1,
            residue2,
            atom1,
            atom2,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command =
                    euclediancomparativetwo(pdbfile, chain, residue1, residue2, atom1, atom2)
                        .unwrap();
                println!(
                    "The eucledian distance between to given coordinates of the same chain is {:?}",
                    command
                );
            });
        }
        Commands::EuclideanAll {
            pdbfile,
            chain,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = eucledianall(pdbfile, chain).unwrap();
                println!(
                    "The vector containing the eucleadian distance for those chain atoms are: {:?}",
                    command
                );
            });
        }
    }
}
