use wav::{Header, BitDepth, read, write};

// TODO     Untested
fn maximize_bitdepth_thirtytwof(data: Vec<f32>) -> Vec<f32> {
    let abs_max : f32 = data.iter().fold(0.0,
        |acc, &s|
            if s.abs() > acc {
                s
            } else {
                acc
            }
    );
    if abs_max == 0.0 {
        return data
    }
    let ampl_ratio = f32::MAX / abs_max;
    println!("BitDepth: 32f, amplify the sound by {:.2}%", (ampl_ratio-1.0)*100.0);

    data.iter().map(|s| *s * ampl_ratio).collect()
}

fn maximize_bitdepth_twentyfour(data: Vec<i32>) -> Vec<i32> {
    let val_min = data.iter().min().expect("No data");
    let abs_max = val_min.abs().max(*data.iter().max().unwrap());
    if abs_max == 0 {
        return data
    }


    let ampl_ratio = (i32::MAX as f64) / (abs_max as f64);
    println!("BitDepth: 24, amplify the sound by {:.2}%", (ampl_ratio-1.0) * 100.0);

    data.iter().map(|s|
        ((*s as f64) * ampl_ratio) as i32
    ).collect()
}

fn maximize_bitdepth_sixteen(data: Vec<i16>) -> Vec<i16> {
    let val_min = data.iter().min().expect("No data");
    let abs_max = val_min.abs().max(*data.iter().max().unwrap());
    if abs_max == 0 {
        return data
    }

    let ampl_ratio = (i16::MAX as f64) / (abs_max as f64);
    println!("BitDepth: 16, amplify the sound by {:.2}%", (ampl_ratio-1.0) * 100.0);

    data.iter().map(|s|
        ((*s as f64) * ampl_ratio) as i16
    ).collect()
}

// TODO     Untested
fn maximize_bitdepth_eight(data: Vec<u8>) -> Vec<u8> {
    let val_min = data.iter().min().expect("No data");
    let abs_max = (u8::MAX - val_min).max(*data.iter().max().unwrap());
    if abs_max == 0 {
        return data
    }

    let ampl_ratio = (u8::MAX as f64) / (abs_max as f64);
    println!("BitDepth: 8, amplify the sound by {:.2}%", (ampl_ratio-1.0) * 100.0);

    let mid = u8::MAX / 2;
    data.iter().map(|s|
        if *s >= mid {
            mid + ((((*s - mid) as f64) * ampl_ratio) as u8)
        } else {
            mid - ((((mid - *s) as f64) * ampl_ratio) as u8)
        }
    ).collect()
}

fn maximize_volume(fpath: &std::path::Path, hdr: Header, data: BitDepth) {
    match data {
        BitDepth::Empty => {},
        BitDepth::Eight(vec) => {
            let res = maximize_bitdepth_eight(vec);
            write_wav_file(fpath, hdr, BitDepth::Eight(res));
        },
        BitDepth::Sixteen(vec) => {
            let res = maximize_bitdepth_sixteen(vec);
            write_wav_file(fpath, hdr, BitDepth::Sixteen(res));
        },
        BitDepth::TwentyFour(vec) => {
            let res = maximize_bitdepth_twentyfour(vec);
            write_wav_file(fpath, hdr, BitDepth::TwentyFour(res));
        },
        BitDepth::ThirtyTwoFloat(vec) => {
            let res = maximize_bitdepth_thirtytwof(vec);
            write_wav_file(fpath, hdr, BitDepth::ThirtyTwoFloat(res));
        },
    }
}

fn write_wav_file(fpath: &std::path::Path, hdr: Header, data: BitDepth) {
    let mut wavfile = std::fs::File::create(fpath).expect("Unable to open wav file");
    write(hdr, &data, &mut wavfile).expect("Unable to write WAV file")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut args = args.iter();
    args.next();
    for f in args {
        let fpath = std::path::Path::new(f);
        if !fpath.exists() {
            println!("Input file {} not found, ignoring ...", f);
        }
        let (header, data) = {
            let mut wavfile = std::fs::File::open(&fpath).expect("Unable to open wav file");
            println!("\n{}", &fpath.to_str().unwrap());
            match read(&mut wavfile) {
                Ok(res) => res,
                Err(e) => {
                    println!("Unable to read WAV file {}: {}, skipping...", fpath.to_str().unwrap(), e);
                    continue;
                },
            }
        };
        maximize_volume(&fpath, header, data);
    }

}
