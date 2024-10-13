use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;

use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Extent {
    fileoffset: u64,
    startblock: u64,
    byteoffset: u64,
    bytecount: u64,
}

#[derive(Deserialize,Debug)]
struct Extent2 {
    extent: Vec<Extent>
}

#[derive(Deserialize,Debug)]
struct LTFSFile {
    name: String,
    extentinfo: Extent2
}

#[derive(Deserialize,Debug)]
struct File2 {
    file: Vec<LTFSFile>,
}

#[derive(Deserialize,Debug)]
struct Directory {
    contents: File2
}

#[derive(Deserialize,Debug)]
struct Index {
    creator: String,
    directory: Directory
}

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut f = File::open("index-10")?;

    let index: Index = serde_xml_rs::from_reader(&mut f).expect("xml");

    println!("index {:?}", index);



    Ok(())
}
