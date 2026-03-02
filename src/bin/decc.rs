use std::fs;
use std::io::Write;
use flate2::write::ZlibEncoder;
use flate2::Compression;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = clap::Command::new("decc")
        .about("Pack Rhai script into .decc")
        .arg(clap::arg!(<INPUT> "Source .rhai file"))
        .get_matches();

    let input = args.get_one::<String>("INPUT").unwrap();
    let content = fs::read_to_string(input)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content.as_bytes())?;
    let compressed = encoder.finish()?;

    let obfuscated: Vec<u8> = compressed.into_iter().map(|b| b ^ 0xFF).collect();

    let mut out_path = std::path::PathBuf::from(input);
    out_path.set_extension("decc");
    fs::write(out_path, obfuscated)?;

    println!("Success: Logic packed into .decc");
    Ok(())
}