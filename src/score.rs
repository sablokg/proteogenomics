use crate::proteome::Proteinanalyze;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn hmmscore(path: &str, score: &str) -> Result<(String, Vec<Proteinanalyze>), Box<dyn Error>> {
    let mut hmmvec: Vec<Proteinanalyze> = Vec::new();
    let mut filterscore: Vec<Proteinanalyze> = Vec::new();
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            let proteinline: Vec<_> = line
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            hmmvec.push(Proteinanalyze {
                targetname: proteinline[0].to_string(),
                tlength: proteinline[2].parse::<usize>().unwrap(),
                queryname: proteinline[3].to_string(),
                querylength: proteinline[5].parse::<usize>().unwrap(),
                evalue: proteinline[6].parse::<f32>().unwrap(),
                score: proteinline[13].parse::<f32>().unwrap(),
                val1: proteinline[15].parse::<usize>().unwrap(),
                val2: proteinline[16].parse::<usize>().unwrap(),
                ed1: proteinline[17].parse::<usize>().unwrap(),
                ed2: proteinline[18].parse::<usize>().unwrap(),
                al1: proteinline[19].parse::<usize>().unwrap(),
                al2: proteinline[20].parse::<usize>().unwrap(),
                desc: proteinline[22].to_string(),
            });
        }
    }
    for i in hmmvec.iter() {
        if i.score >= score.parse::<f32>().unwrap() {
            filterscore.push(Proteinanalyze {
                targetname: i.targetname.clone(),
                tlength: i.tlength,
                queryname: i.queryname.clone(),
                querylength: i.querylength,
                evalue: i.evalue,
                score: i.score,
                val1: i.val1,
                val2: i.val2,
                ed1: i.ed1,
                ed2: i.ed2,
                al1: i.al1,
                al2: i.al2,
                desc: i.desc.clone(),
            });
        }
    }

    let mut filteredscore_target: Vec<Proteinanalyze> = Vec::new();
    for i in hmmvec.iter() {
        if i.score >= score.parse::<f32>().unwrap() && i.desc == "-" {
            continue;
        } else if i.score >= score.parse::<f32>().unwrap() && i.desc != "-" {
            filteredscore_target.push(Proteinanalyze {
                targetname: i.targetname.clone(),
                tlength: i.tlength,
                queryname: i.queryname.clone(),
                querylength: i.querylength,
                evalue: i.evalue,
                score: i.score,
                val1: i.val1,
                val2: i.val2,
                ed1: i.ed1,
                ed2: i.ed2,
                al1: i.al1,
                al2: i.al2,
                desc: i.desc.clone(),
            });
        }
    }

    let mut filewrite = File::create("filtered-score.txt").expect("file not found");
    for i in filterscore.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.targetname,
            i.tlength,
            i.queryname,
            i.querylength,
            i.evalue,
            i.score,
            i.val1,
            i.val2,
            i.ed1,
            i.ed2,
            i.al1,
            i.al2,
            i.desc
        )
        .expect("line not found");
    }

    let mut fileadd = File::create("filtered-scores-with-all-targets.txt").expect("file not found");
    for i in filteredscore_target.iter() {
        writeln!(
            fileadd,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.targetname,
            i.tlength,
            i.queryname,
            i.querylength,
            i.evalue,
            i.score,
            i.val1,
            i.val2,
            i.ed1,
            i.ed2,
            i.al1,
            i.al2,
            i.desc
        )
        .expect("description not found");
    }

    Ok::<(String, Vec<Proteinanalyze>), Box<dyn Error>>((
        "results have been written to the file and also printed here".to_string(),
        filterscore,
    ))
}
