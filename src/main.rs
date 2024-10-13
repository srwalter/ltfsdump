use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_file(f: &mut File) -> io::Result<String> {
    let mut result = "".to_string();

    loop {
        let mut buf = vec![0; 512 * 1024];
        let bytes = f.read(&mut buf)?;
        result.push_str(&String::from_utf8(buf).expect("bad string"));
        if bytes < 512 * 1024 {
            break;
        }
    }
    Ok(result)
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
    // XXX: set to partition 1
    loop {
        let mut f = File::open("/dev/nst0")?;
        println!("file: {}", read_file(&mut f)?);
    }
    Ok(())
}
