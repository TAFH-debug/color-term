use core::slice::SlicePattern;
use std::fs::File;
use std::io::Read;
use crate::png::chuncks::*;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

pub fn png<S: AsRef<str>>(filename: S) -> Result<(), String> {
    let mut file = File::open(filename.as_ref()).expect("Invalid filename");
    let mut chuncks = Vec::new();
    let mut signature: [u8; 8] = [0; 8];

    file.read(&mut signature).expect("OS Error");

    if signature != PNG_SIGNATURE {
        return Err("Invalid signature.".to_string());
    }

    println!("Start reading chuncks...");
    read_chuncks_rec(&mut file, &mut chuncks);

    //Read IHDR chunck
    let (ihdr_n, ihdr_d) = &chuncks[0];
    assert_eq!(ihdr_n, "IHDR");
    let ihdr = IhdrChunck::from_bytes(ihdr_d.clone());
    ihdr.check().expect("Oh no");

    //TODO
    Ok(())
}

pub fn read_chuncks_rec(f: &mut File, chuncks: &mut Vec<(String, Vec<u8>)>) {
    let mut ln_buf: [u8; 4] = [0; 4];
    let mut ct_buf: [u8; 4] = [0; 4];
    let mut data: Vec<u8> = Vec::new();

    f.read(&mut ln_buf).expect("OS Error");
    f.read(&mut ct_buf).expect("OS Error");

    let name = String::from_utf8(Vec::from(ct_buf)).unwrap();

    if name == "IEND" {
        return;
    }

    for _ in 0..u32::from_be_bytes(ln_buf) {
        let mut buf = [0; 1];
        f.read(&mut buf).expect("OS Error");
        data.push(*buf.first().unwrap());
    }

    f.read(&mut [0; 4]);

    chuncks.push(
        (name, data)
    );
    read_chuncks_rec(f, chuncks)
}