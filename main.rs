extern crate md5;
extern crate clap;

use std::fs::File;
use std::io::{self, Read};
use md5::Digest;
use clap::{App, Arg};

fn main() -> io::Result<()> {
    let matches = App::new("Checksum")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("PATH")
            .help("Specify file location")
            .takes_value(true)
            .required(true))
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let mut file = File::open(path)?;
    let mut data = Vec::new();
	
    file.read_to_end(&mut data)?;
	
    let md5_digest = md5::compute(&data);
	
    println!("MD5 Checksum: {:x}", md5_digest);
    Ok(())
}