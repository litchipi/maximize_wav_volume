use wav::{Header, BitDepth, read, write};

fn maximize_bitdepth_thirtytwof(data: Vec<f32>) -> Vec<f32> {
    println!("BitDepth: 32f");
    let abs_max : f32 = data.iter().fold(0.0,
        |acc, &s|
            if s.abs() > acc {
                s
            } else {
                acc
            }
    );
    let vol_ratio = abs_max / (f32::MAX - f32::MIN);
    println!("Current volume ratio: {:.2}%", vol_ratio);
    let ampl_ratio = 1.0 / vol_ratio;

    data.iter().map(|s| *s * ampl_ratio).collect()
}

fn maximize_bitdepth_twentyfour(data: Vec<i32>) -> Vec<i32> {
    let bitdepth_absmax_val = 2^24;
    println!("BitDepth: 24");
    let val_min = data.iter().min().expect("No data");
    let abs_max = val_min.abs().max(*data.iter().max().unwrap());

    let vol_ratio = (abs_max as f64) / (2.0*(bitdepth_absmax_val as f64));
    println!("Current volume ratio: {:.2}%", vol_ratio);

    let ampl_ratio = 1.0 / vol_ratio;

    data.iter().map(|s|
        ((*s as f64) * ampl_ratio) as i32
    ).collect()
}

fn maximize_bitdepth_sixteen(data: Vec<i16>) -> Vec<i16> {
    println!("BitDepth: 16");
    let val_min = data.iter().min().expect("No data");
    let abs_max = val_min.abs().max(*data.iter().max().unwrap());

    let vol_ratio = (abs_max as f64) / (i16::MAX as f64);
    println!("Current volume ratio: {:.2}%", vol_ratio);

    let ampl_ratio = 1.0 / vol_ratio;

    data.iter().map(|s|
        ((*s as f64) * ampl_ratio) as i16
    ).collect()
}

fn maximize_bitdepth_eight(data: Vec<u8>) -> Vec<u8> {
    println!("BitDepth: 8");
    let val_min = data.iter().min().expect("No data");
    let abs_max = (u8::MAX - val_min).max(*data.iter().max().unwrap());

    let vol_ratio = (abs_max as f64) / ((u8::MAX as f64) - (u8::MIN as f64));
    println!("Current volume ratio: {:.2}%", vol_ratio);

    let ampl_ratio = 1.0 / vol_ratio;

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
    let outf = fpath.with_file_name("out.wav");
    println!("Writing to {}", outf.to_str().unwrap());
    let mut wavfile = std::fs::File::create(outf).expect("Unable to open wav file");
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
        println!("Treating {}", f);
        let (header, data) = {
            let mut wavfile = std::fs::File::open(&fpath).expect("Unable to open wav file");
            read(&mut wavfile).expect("Unable to read WAV data from file")
        };
        maximize_volume(&fpath, header, data);
        println!();
    }

}
