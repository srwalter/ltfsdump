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
            .open(f.name)?;

        for extent in f.extentinfo.extent {
            output.seek(SeekFrom::Start(extent.fileoffset))?;
            assert_eq!(extent.byteoffset, 0);

            let extent_name = format!("data-{}", extent.startblock);
            let mut input = File::open(extent_name)?;

            io::copy(&mut input, &mut output)?;
        }
    }



    Ok(())
}
