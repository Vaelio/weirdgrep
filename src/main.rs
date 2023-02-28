use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;
use std::io::BufReader;
use glob::glob;

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
    numbers: bool,

    /// Add markers to better show which line matched
    #[clap(short, long, action)]
    add_markers: bool,

    /// Recursive search through directories
    #[clap(short, long, action)]
    recursive: bool,
}

fn open(f: &String) -> Result<File, Box<dyn Error>> {
    Ok(File::open(f)?)
}

fn format_line_with_markers(line: &str) -> String {
    format!("{} <------- XXXXXXXXXXXXX", line)
}

fn read_file_with_regex_tag_and_within(
    f: File,
    rin_s: &str,
    rout_s: &str,
    within: &str,
    numbers: bool,
    add_markers: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let bufreader = BufReader::new(f);
    let mut content: Vec<String> = Vec::new();
    let mut resv: Vec<usize> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    let rin = Regex::new(rin_s)?;
    let rout = Regex::new(rout_s)?;
    let rwithin = Regex::new(within)?;

    for (line_count, line) in bufreader.lines().enumerate() {
        let mut line = line?;

        if rwithin.is_match(&line) {
            resv.push(line_count);

            if add_markers {
                line = format_line_with_markers(&line);
            }
        }

        content.push(line);
    }

    for res in resv {
        /* go forward until rout matches */
        let mut after: Vec<String> = {
            let mut after: Vec<String> = Vec::new();
            for (c, line) in content.iter().skip(res).enumerate() {
                if numbers {
                    after.push(format!("{}: {}", res + c, line));
                } else {
                    after.push(line.to_string());
                }

                if rout.is_match(line) {
                    break;
                }
            }

            after
        };

        /* go backward until rin matches */
        let mut before: Vec<String> = {
            let mut before: Vec<String> = Vec::new();
            for (c, line) in content.iter().rev().skip(content.len() - res).enumerate() {
                if numbers {
                    before.insert(0, format!("{}: {}", res - c, line));
                } else {
                    before.insert(0, line.to_string());
                }

                if rin.is_match(line) {
                    break;
                }
            }

            before
        };

        out.append(&mut before);
        out.append(&mut after);
    }

    Ok(out)
}

fn read_file_with_regex_and_tag(
    f: File,
    rin_s: &str,
    rout_s: &str,
    numbers: bool,
    add_markers: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let bufreader = BufReader::new(f);
    let mut content: Vec<String> = Vec::new();
    let rin = Regex::new(rin_s)?;
    let rout = Regex::new(rout_s)?;
    let mut appending = false;

    for (idx, line) in bufreader.lines().enumerate() {
        let mut line = line?;
        if !appending && rin.is_match(&line) {
            appending = true;
            if add_markers {
                line = format_line_with_markers(&line);
            }
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

fn recursive_grep(args: &Args) -> Result<Vec<String>, Box<dyn Error>> {
    let mut recurs_content = vec![];
    for entry in glob(&format!("{}/**/*", &args.path))? {
        let entry = format!("{}", entry?.display());
        if metadata(&entry)?.is_file() {
            if let Ok(file) = open(&entry) {
                let mut file_content = if let Some(within) = &args.within {
                    read_file_with_regex_tag_and_within(
                        file,
                        &args.regex,
                        &args.endtag,
                        &within,
                        args.numbers,
                        args.add_markers,
                    )?
                } else {
                    read_file_with_regex_and_tag(
                        file,
                        &args.regex,
                        &args.endtag,
                        args.numbers,
                        args.add_markers,
                    )?
                };
                if file_content.len() > 0 {
                    recurs_content.push(format!("-------- {} --------", &entry));
                }
                recurs_content.append(&mut file_content);
            } else {
                println!("Possible race conditions while recursive search: file \"{}\" does not exist or you don't have access to it", &entry);
            }
        }
    }

    Ok(recurs_content)
}


fn main() {
    let args = Args::parse();
    if args.recursive {
        match recursive_grep(&args) {
            Ok(v) => {
                for line in v.iter() {
                    println!("{}", line);
                }
            },
            Err(e) => println!("Got error while parsing recursively: {:?}", e),
        }
    } else {
        if let Ok(file) = open(&args.path) {
            if let Some(within) = args.within {
                match read_file_with_regex_tag_and_within(
                    file,
                    &args.regex,
                    &args.endtag,
                    &within,
                    args.numbers,
                    args.add_markers,
                ) {
                    Ok(v) => {
                        for line in v.iter() {
                            println!("{}", line);
                        }
                    }
                    Err(e) => println!("Got error while parsing: {:?}", e),
                }
            } else {
                match read_file_with_regex_and_tag(
                    file,
                    &args.regex,
                    &args.endtag,
                    args.numbers,
                    args.add_markers,
                ) {
                    Ok(v) => {
                        for line in v.iter() {
                            println!("{}", line);
                        }
                    }
                    Err(e) => println!("Got error while parsing: {:?}", e),
                }
            }
        } else {
            println!("Either the file doesn't exists or you don't have access to it");
        }
    }
}
