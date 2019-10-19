extern crate lapp;
extern crate unicode_normalization;
use std::io::{BufRead, BufReader, BufWriter, Write};
use unicode_normalization::UnicodeNormalization;

// usize because that's the max number of bools in the slice
fn trues(bools: &[bool]) -> usize {
    let mut n = 0;
    for b in bools {
        if *b { n += 1 }
    }
    n
}

fn main() {
    let usage = include_str!("../README");
    let args = lapp::parse_args(&usage);

    let infile = BufReader::new(args.get_infile("infile"));
    let mut outfile = BufWriter::new(args.get_outfile("outfile"));    

    if args.get_bool("version") {
        println!("Version {} licensed GPLv3. (C) 2019 Leonora Tindall <nora@nora.codes>",
            env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let nfd = args.get_bool("nfd");
    let nfkd = args.get_bool("nfkd");
    let nfc = args.get_bool("nfc");
    let nfkc = args.get_bool("nfkc");
    let ss = args.get_bool("stream-safe");

    if trues(&[nfd, nfkd, nfc, nfkc]) > 1 {
        eprintln!("--nfd, --nfkd, --nfc, and --nfkc are mutually exclusive.");
        std::process::exit(1);
    } 

    for line in infile.lines() {
        let mut line = line.expect("Could not read line from file. Error").clone();
        if args.get_bool("crlf") {
            line.push('\x0D');
        }
        line.push('\x0A');

        let normalized: Box<dyn Iterator<Item=char>>;
        if nfd {
            normalized = Box::new(line.chars().nfd());
        } else if nfkc {
            normalized = Box::new(line.chars().nfkc());
        } else if nfkd {
            normalized = Box::new(line.chars().nfkd());
        } else {
            normalized = Box::new(line.chars().nfc());
        }

        let output: String;
        if ss {
            output = normalized.stream_safe().collect();
        } else {
            output = normalized.collect();
        }

        write!(&mut outfile, "{}", output).expect("Could not write to output. Error");
    }
}
