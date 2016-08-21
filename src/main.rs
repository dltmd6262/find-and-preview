extern crate rustc_serialize;
extern crate docopt;
extern crate ansi_term;

use docopt::Docopt;
use std::process::Command;
use ansi_term::Style;
use ansi_term::Colour::RGB;

const USAGE: &'static str = "
Find files and preview them in pretty format

Usage:
    find <dir> <name> [-f]

Options:
    -f, --full  Print full preview for each find result
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_f: bool,
    arg_dir: String,
    arg_name: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let output = Command::new("find")
        .args(&[args.arg_dir, String::from("-name"), args.arg_name])
        .output()
        .unwrap();

    let result_string = String::from_utf8_lossy(&output.stdout);
    let mut split: Vec<&str> = result_string.split("\n").collect();
    split.pop();

    for s in split {
        let mut parse_vec: Vec<&str> = s.split("/").collect();
        let filename = parse_vec.pop().unwrap();

        let preview = Command::new("head").args(&[String::from("-n 1"), String::from(s)]).output().unwrap();

        println!("
            {}: {}
            {}: {}
            {}: {}
        ", Style::new().bold().fg(RGB(192, 123, 142)).paint("Name: "), filename,
        Style::new().bold().fg(RGB(192, 123, 142)).paint("Path: "), s,
        Style::new().bold().fg(RGB(192, 123, 142)).paint("Top: "), String::from_utf8_lossy(&preview.stdout));
    }
}
