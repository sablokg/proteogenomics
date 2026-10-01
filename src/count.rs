use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn hmmcount(path: &str) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut functionhold: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            functionhold.push(
                line.split(" ")
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>()[22]
                    .to_string(),
            );
        }
    }
    let functionhashset: HashSet<_> = functionhold.iter().collect::<HashSet<_>>();
    let filteredhashset: HashSet<_> = functionhashset
        .into_iter()
        .filter(|x| x.to_string() != "-")
        .collect::<HashSet<_>>();

    let mut unique: HashMap<String, usize> = HashMap::new();
    for i in filteredhashset {
        *unique.entry(i.to_string()).or_default() += 1;
    }

    Ok::<HashMap<String, usize>, Box<dyn Error>>(unique)
}
