use std::env;
use std::io::{Read, stdout, Write};

enum Mode {
    Encode,
    Decode,
}

fn main() {
    let mut args = env::args().skip(1);
    let mode = match args.next().as_ref().map(String::as_ref) {
        Some("e" | "encode") => {
            Mode::Encode
        },
        Some("d" | "decode") => Mode::Decode,
        _ => {
            eprintln!("jiaozi e/d input");
            return;
        },
    };

    let input = args.collect::<String>();

    match mode {
        Mode::Encode => {
            let mut encoder = snap::read::FrameEncoder::new(input.as_bytes());

            let mut buffer = [0u8; 4096];
            let mut s = String::new();

            while let Ok(len) = encoder.read(&mut buffer) {
                if len == 0 { break; }

                for i in &buffer[..len] {
                    let mut i = *i;
                    for _ in 0..u8::BITS {
                        if (i & 1) == 0 {
                            s.push('饺');
                        } else {
                            s.push('子');
                        }
                        i >>= 1;
                    }
                }
            }

            let mut sout = stdout().lock();
            sout.write_all(s.as_bytes()).unwrap();
            sout.write_all(&[b'\n']).unwrap();
        }
        Mode::Decode => {
            let mut buf = Vec::<u8>::with_capacity(input.len() >> 1);

            let mut chars = input.chars();

            let mut flag = false;
            loop {
                let mut n = 0u8;

                for i in 0..u8::BITS {
                    n >>= 1;
                    match chars.next() {
                        Some('饺') => {
                            const JIAO: u8 = 0 << (u8::BITS - 1);
                            n |= JIAO;
                        },
                        Some('子') => {
                            const ZI: u8 = 1 << (u8::BITS - 1);
                            n |= ZI;
                        },
                        Some(ch) => {
                            eprintln!("未知的字符: {ch}");
                            return;
                        }
                        None => {
                            if i == 0 {
                                flag = true;
                                break;
                            }
                        },
                    }
                }

                if flag { break; }

                buf.push(n);
            }

            let mut decoder = snap::read::FrameDecoder::new(buf.as_slice());

            let mut temp = input;
            temp.clear();

            decoder.read_to_string(&mut temp).unwrap();

            let mut sout = stdout().lock();
            sout.write_all(temp.as_bytes()).unwrap();
            sout.write_all(&[b'\n']).unwrap();
        }
    }
}
