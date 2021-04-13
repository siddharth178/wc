use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::{BufReader, Error};

fn print_usage_and_exit() {
    println!("wc <file1> <file2>...");
    std::process::exit(1);
}

#[derive(Debug)]
struct WcStats {
    file_name: String,
    lines: u64,
    words: u64,
    chars: u64,
}

fn word_count(fname: String, file: BufReader<File>) -> Result<WcStats, Error> {
    let mut ls = 0;
    let mut cs = 0;
    let mut ws = 0;

    for line in file.lines() {
        match line {
            Ok(l) => {
                ls += 1;
                cs += l.len() as u64;

                let mut word_start = false;
                for c in l.chars() {
                    if c.is_whitespace() {
                        if word_start {
                            ws += 1;
                            word_start = false;
                        }
                    } else {
                        word_start = true;
                    }
                }
                if word_start {
                    ws += 1;
                }
            }
            Err(_) => {}
        }
    }

    Ok(WcStats {
        file_name: fname,
        lines: ls,
        chars: cs + ls,
        words: ws,
    })
}

fn main() -> Result<(), Error> {
    let args = env::args();
    if args.len() < 2 {
        print_usage_and_exit()
    }

    let mut combined_wc = WcStats {
        file_name: "total".to_string(),
        lines: 0,
        words: 0,
        chars: 0,
    };
    for file_name in args.skip(1) {
        let file = File::open(&file_name)?;
        let file = BufReader::new(file);
        let wcs = word_count(file_name, file)?;
        println!("stats: {:?}", wcs);

        combined_wc.chars += wcs.chars;
        combined_wc.words += wcs.words;
        combined_wc.lines += wcs.lines;
    }
    println!("stats: {:?}", combined_wc);

    Ok(())
}
