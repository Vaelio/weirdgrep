use clap::Parser;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;


#[derive(Parser)]
#[clap(author = "Vaelio <archelio@protonmail.com>")]
#[clap(about = "Regex parser to search through files.")]
#[clap(version, long_about = None)]
struct Args {
    /// Regex to apply
    regex: String,

    /// End Tag for the match
    endtag: String,

    /// File to parse
    path: String,

    /// Switch to scope mode, and use this regex as a search and (regex, endtag) as boundaries of the search afterwards 
    #[clap(short, value_parser)]
    within: Option<String>,

    /// Print line numbers for each printed lines starting from 0
    #[clap(short, long, action)]
    numbers: bool
}


fn open(f: &String) -> Result<File, Box<dyn Error>> {
    Ok(File::open(f)?)
}


fn read_file_with_regex_tag_and_within(f: File, rin_s: &str, rout_s: &str, within: &str, numbers: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let bufreader = BufReader::new(f);
    let mut content: Vec<String> = Vec::new();
    let mut resv: Vec<usize> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    let rin = Regex::new(rin_s)?;
    let rout = Regex::new(rout_s)?;
    let rwithin = Regex::new(within)?;

    for (line_count, line) in bufreader.lines().enumerate() {
        let line = line?;

        if rwithin.is_match(&line) {
            resv.push(line_count);
        }

        content.push(line);
    }

    for res in resv {
        /* go forward until rout matches */
        let mut after : Vec<String> = {
            let mut after: Vec<String> = Vec::new();
            for (c, line) in content.iter().skip(res).enumerate() {
                if numbers {
                    after.push(format!("{}: {}", res+c, line));
                } else {
                    after.push(line.to_string());
                }

                if rout.is_match(line) {
                    break
                }

            }

            after
        };
        
        /* go backward until rin matches */
        let mut before : Vec<String> = {
            let mut before: Vec<String> = Vec::new();
            for (c, line) in content.iter().rev().skip(content.len() - res).enumerate() {
                if numbers {
                    before.insert(0, format!("{}: {}", res-c, line));
                } else {
                    before.insert(0, line.to_string());
                }

                if rin.is_match(line) {
                    break
                }
            }

            before
        };

        out.append(&mut before);
        out.append(&mut after);

    }

    Ok(out)

    
}


fn read_file_with_regex_and_tag(f: File, rin_s: &str, rout_s: &str, numbers: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let bufreader = BufReader::new(f);
    let mut content: Vec<String> = Vec::new();
    let rin = Regex::new(rin_s)?;
    let rout = Regex::new(rout_s)?;
    let mut appending = false;

    for (idx, line) in bufreader.lines().enumerate() {
        let line = line?;
        if !appending && rin.is_match(&line) {
            appending = true;
        }
        if appending {
            if rout.is_match(&line) {
                appending = false;
            }

            if numbers {
                content.push(format!("{}: {}", idx, line));
            } else {
                content.push(line);
            }
        }
    }

    Ok(content)
}

fn main() {
    let args = Args::parse();

    if let Ok(file) = open(&args.path) {
        if let Some(within) = args.within {
            match read_file_with_regex_tag_and_within(file, &args.regex, &args.endtag, &within, args.numbers) {
                Ok(v) => {
                    for line in v.iter() {
                        println!("{}", line);
                    }
                },
                Err(e) => println!("Got error while parsing: {:?}", e),
            }
        } else {
            match read_file_with_regex_and_tag(file, &args.regex, &args.endtag, args.numbers) {
                Ok(v) => {
                    for line in v.iter() {
                        println!("{}", line);
                    }
                },
                Err(e) => println!("Got error while parsing: {:?}", e),
            }
        }
        
    } else {
        println!("Either the file doesn't exists or you don't have access to it");
    }
}