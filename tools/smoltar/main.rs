use std::fs::{create_dir_all, File};

fn main() -> std::io::Result<()> {
    // usage:
    // smoltar -x archive.tar.zst outdir
    // smoltar -l archive.tar.zst
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: smoltar -x archive.tar.gz outdir");
        println!("Usage: smoltar -l archive.tar.gz");
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid arguments",
        ));
    }

    let file = File::open(&args[2]).unwrap();
    let unzipped = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(unzipped);
    if args[1] == "-x" {
        // go ahead and do create outdir (and any parents)
        let outdir = &args[3];
        create_dir_all(outdir).unwrap();
        archive.unpack(&args[3]).unwrap();
    } else if args[1] == "-l" {
        for file in archive.entries().unwrap() {
            let file = file.unwrap();
            println!("{}", file.path().unwrap().display());
        }
    } else {
        println!("Usage: smoltar -x archive.tar.gz outdir");
        println!("Usage: smoltar -l archive.tar.gz");
    }

    Ok(())
}
