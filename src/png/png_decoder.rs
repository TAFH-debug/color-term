use std::fs::File;
use std::io::Read;
use crate::deflate;
use crate::png::chuncks::*;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const BYTES_PER_PIXEL: i32 = 4;

struct Wrapper {
    vec: Vec<u8>,
    counter: usize,
}
impl Read for Wrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for i in 0..buf.len() {
            match self.vec.get(self.counter) {
                Some(n) => buf[i] = *n,
                None => return Ok(0),
            }
            self.counter += 1;
        }
        Ok(buf.len())
    }
}

pub fn png<S: AsRef<str>>(filename: S) -> Result<(Vec<u8>, u32, u32), String> {
    let mut file = File::open(filename.as_ref()).expect("Invalid filename");
    let mut chuncks = Vec::new();
    let mut signature: [u8; 8] = [0; 8];

    file.read(&mut signature).expect("OS Error");

    if signature != PNG_SIGNATURE {
        return Err("Invalid signature.".to_string());
    }

    println!("Start reading chuncks...");
    read_chuncks_rec(&mut file, &mut chuncks);

    //Read and Check IHDR chunck
    let (ihdr_n, ihdr_d) = &chuncks[0];
    assert_eq!(ihdr_n, "IHDR");
    let ihdr = IhdrChunck::from_bytes(ihdr_d.clone());
    ihdr.check().expect("Oh no");

    //Read IDAT chunks.
    let mut idat_data = Vec::new();
    for i in chuncks {
        if i.0 == "IDAT" {
            idat_data.append(&mut i.1.clone());
        }
    }

    let mut decoder = compress::zlib::Decoder::new(Wrapper {
        vec: idat_data,
        counter: 0,
    });
    let mut idata = Vec::new();
    decoder.read_to_end(&mut idata).expect("TODO: panic message");

    //Reverse filtering.
    println!("Reverse filtering...");
    let mut recon: Vec<u8> = Vec::new();
    let stride = ihdr.width as i32 * BYTES_PER_PIXEL;

    pub fn recon_a(r: i32, c: i32, recon: &Vec<u8>, strd: i32) -> u8 {
        return if c >= BYTES_PER_PIXEL {
            recon[(r * strd + c - BYTES_PER_PIXEL) as usize]
        }
        else { 0 }
    }

    pub fn recon_b(r: i32, c: i32, recon: &Vec<u8>, strd: i32) -> u8 {
        return if r > 0 {
            recon[((r-1) * strd + c) as usize]
        }
        else { 0 }
    }

    pub fn recon_c(r: i32, c: i32, recon: &Vec<u8>, strd: i32) -> u8 {
        return if c >= BYTES_PER_PIXEL && r > 0 {
            recon[(((r-1) * strd) + c - BYTES_PER_PIXEL) as usize]
        }
        else { 0 }
    }

    let mut i = 0;
    for r in 0..ihdr.height as i32 {
        let filter_type = idata[i];
        i += 1;
        for c in 0..stride {
            let filt_x = idata[i] as u32;
            i += 1;
            let recon_x: u32;
            match filter_type {
                0 => recon_x = filt_x,
                1 => recon_x = filt_x + recon_a(r, c, &recon, stride) as u32,
                2 => recon_x = filt_x + recon_b(r, c, &recon, stride) as u32,
                3 => recon_x = filt_x +
                    (recon_a(r, c, &recon, stride) as u32 + recon_b(r, c, &recon, stride) as u32) / 2,
                4 => {
                    recon_x = filt_x + paeth_predictor(
                        recon_a(r, c, &recon, stride) as i32,
                        recon_b(r, c, &recon, stride) as i32,
                        recon_c(r, c, &recon, stride) as i32
                    ) as u32;
                },
                _ => return Err(format!("Type: {}\nRecon: {:?}\nCounter: {}", filter_type, recon, i)),
            }
            recon.push((recon_x & 0xff) as u8);
        }
    }
    println!("Recon: {:?}", recon);
    Ok((recon, ihdr.width, ihdr.height))
}

fn paeth_predictor(a: i32, b: i32, c: i32) -> u8 {
    let p = (a + b - c);
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc { a as u8 } else if pb <= pc { b as u8 } else { c as u8 }
}

fn read_chuncks_rec(f: &mut File, chuncks: &mut Vec<(String, Vec<u8>)>) {
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