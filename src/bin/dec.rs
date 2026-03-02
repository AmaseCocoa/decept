use clap::Parser;
use regex::Regex;
use rhai::{Engine, Scope};
use rhai::packages::Package;
use rhai_rand::RandomPackage;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use flate2::read::ZlibDecoder;

#[derive(Parser)]
#[command(name = "dec")]
struct Cli {
    input: PathBuf,

    #[arg(short, long, default_value = "logic.decc")]
    logic: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let bin_data = fs::read(&cli.logic)?;
    let de_obfuscated: Vec<u8> = bin_data.into_iter().map(|b| b ^ 0xFF).collect();

    let mut decoder = ZlibDecoder::new(&de_obfuscated[..]);
    let mut logic_script = String::new();
    decoder.read_to_string(&mut logic_script).ok();
    
    let mut decoder = ZlibDecoder::new(&de_obfuscated[..]);
    let mut logic_script = String::new();
    decoder.read_to_string(&mut logic_script)?;

    let dsl_code = fs::read_to_string(&cli.input)?;
    let mut engine = Engine::new();
    let random = RandomPackage::new();
    let mut scope = Scope::new();
    engine.set_max_expr_depths(128, 128);
    random.register_into_engine(&mut engine);

    let mut rhai_lines = Vec::new();
    let re_vali = Regex::new(r"vali\s*\(\s*([^,)]+)\s*,\s*([^,)]+)\s*,\s*([^)]+)\s*\)")?;

    for line in dsl_code.trim().split(';') {
        let line = line.trim();
        if line.is_empty() { continue; }

        let converted = if let Some(caps) = re_vali.captures(line) {
            format!("{} = multi_layout_shift({}, {}, {});", &caps[1], &caps[1], &caps[2], &caps[3])
        } else if line.starts_with("out(") {
            format!("{};", line.replace("out(", "print("))
        } else {
            let mut l = line.replace("def ", "let ");
            if !l.ends_with(';') { l.push(';'); }
            l
        };
        rhai_lines.push(converted);
    }

    let full_script = format!("{}\n{}", logic_script, rhai_lines.join("\n"));
    engine.run_with_scope(&mut scope, &full_script)?;

    Ok(())
}