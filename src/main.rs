use std::fs::{File, OpenOptions};
use std::io;
use std::io::SeekFrom;
use std::io::prelude::*;

use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Extent {
    fileoffset: u64,
    startblock: u64,
    byteoffset: u64,
}

#[derive(Deserialize,Debug)]
struct ExtentInfo {
    extent: Vec<Extent>
}

#[derive(Deserialize,Debug)]
struct LTFSFile {
    name: String,
    extentinfo: ExtentInfo
}

#[derive(Deserialize,Debug)]
struct Contents {
    file: Vec<LTFSFile>,
}

#[derive(Deserialize,Debug)]
struct Directory {
    contents: Contents
}

#[derive(Deserialize,Debug)]
struct Index {
    directory: Directory
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let index_name = args.next().unwrap();
    let mut f = File::open(index_name)?;

    let index: Index = serde_xml_rs::from_reader(&mut f).expect("xml");

    //println!("index {:?}", index);

    for f in index.directory.contents.file {
        let mut output = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&f.name)?;

        println!("Restoring {}", f.name);

        for extent in f.extentinfo.extent {
            output.seek(SeekFrom::Start(extent.fileoffset))?;
            assert_eq!(extent.byteoffset, 0);

            let extent_name = format!("data-{}", extent.startblock);
            if let Ok(mut input) = File::open(extent_name) {
                io::copy(&mut input, &mut output)?;
            } else {
                eprintln!("Failed to open data block {}, that part of the file will be blank", extent.startblock);
            }
        }
    }

    Ok(())
}
