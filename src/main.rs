use zip::ZipWriter;
use zip::write::FileOptions;
use std::env;
use std::fs::File;
use std::time::Instant;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 3 {
//         eprintln!("Usage: `Source` `target`");
//         return;
//     }

//     let mut input = File::open(args[1].clone()).unwrap();
//     let mut output = File::create(args[2].clone()).unwrap();
//     let mut encoder = GzEncoder::new(&mut output, Compression::default());
//     let start = Instant::now();
//     copy(&mut input, &mut encoder).unwrap();
//     encoder.finish().unwrap();
//     println!(
//         "Time elapsed: {:?}",
//         input.metadata().unwrap().len() as f64 / output.metadata().unwrap().len() as f64
//     );

//     println!("Target len {:?}", output.metadata().unwrap().len());
//     println!("Elapsed {:?}", start.elapsed());
// }













fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: `Source` `target`");
        return;
    }

    let mut input = File::open(args[1].clone()).unwrap();
    let output = File::create(args[2].clone()).unwrap();
    let mut zip = ZipWriter::new(output);

    let start = Instant::now();
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file(args[1].clone(), options).unwrap();
    std::io::copy(&mut input, &mut zip).unwrap();
    zip.finish().unwrap();

    println!("Time elapsed: {:?}", start.elapsed());
    println!("Target len {:?}", std::fs::metadata(args[2].clone()).unwrap().len());
}
