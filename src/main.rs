#[macro_use]
extern crate structopt;

use structopt::StructOpt;

use csv::*;
use std::io;
use csv::Reader;
use std::collections::{HashSet, HashMap};
use std::env;
use std::path::Path;
use std::fs::OpenOptions;
use std::net::Shutdown::Write;

#[derive(Debug, StructOpt)]
#[structopt(name = "csv_spkit")]
struct Opt {
    #[structopt(short = "c", long = "column")]
    column: String
}

fn split(column: &str) -> () {
    let mut reader = csv::Reader::from_reader(io::stdin());

    let id_index: usize = reader.headers().expect("").iter().enumerate().filter(|(i, header)| {
        *header == column
    }).map(|(i, _)| {
        i
    }).collect::<Vec<usize>>()[0];
    let header = reader.headers().expect("").clone();

    reader.records().filter(|r| {r.is_ok()}).map(|r|{r.expect("")}).for_each( |row| {
        let id = row.get(id_index).expect("").to_string();
        let path_string = format!("{}.csv", id).replace("/", "_");
        let path = Path::new(path_string.as_str());
        if path.exists() {
            let file = OpenOptions::new().append(true).open(path).expect(format!("Could not create writer {}", path_string).as_str());
            let mut writer= Writer::from_writer(file);
            writer.write_record(row.iter());
            writer.flush();
        } else {
            let mut writer = Writer::from_path(path).expect(format!("Could not create new writer {}", path_string).as_str());
            writer.write_record(header.iter());
            writer.write_record(row.iter());
            writer.flush();
        }
    });
}


fn main() {
    let opt = Opt::from_args();
    let arguments: Vec<String> = env::args().collect();
    split(opt.column.as_str());
}