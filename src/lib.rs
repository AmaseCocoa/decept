use flate2::Compression;
use wasm_bindgen::prelude::*;
use rhai::{Engine, Scope, packages::Package};
use rhai_rand::RandomPackage;
use regex::Regex;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

#[wasm_bindgen]
pub fn run_wdec(logic_bin: &[u8], dsl_code: &str) -> Result<String, JsValue> {
    let de_obfuscated: Vec<u8> = logic_bin.iter().map(|&b| b ^ 0xFF).collect();
    let mut decoder = ZlibDecoder::new(&de_obfuscated[..]);
    let mut logic_script = String::new();
    
    decoder.read_to_string(&mut logic_script)
        .map_err(|e| JsValue::from_str(&format!("Logic Decompression Error: {}", e)))?;

    let mut engine = Engine::new();
    let random = RandomPackage::new();
    random.register_into_engine(&mut engine);
    engine.set_max_expr_depths(128, 128);

    let output = Arc::new(Mutex::new(String::new()));
    let output_captured = output.clone();
    engine.on_print(move |s| {
        let mut lock = output_captured.lock().unwrap();
        lock.push_str(s);
        lock.push('\n'); 
    });

    let mut rhai_lines = Vec::new();
    let re_vali = Regex::new(r"vali\s*\(\s*([^,)]+)\s*,\s*([^,)]+)\s*,\s*([^)]+)\s*\)").unwrap();

    for line in dsl_code.trim().split(';') {
        let line = line.trim();
        if line.is_empty() { continue; }

        let converted = if let Some(caps) = re_vali.captures(line) {
            let s_var = caps.get(1).unwrap().as_str().trim();
            let k_var = caps.get(2).unwrap().as_str().trim();
            let t_var = caps.get(3).unwrap().as_str().trim();
            format!("{} = multi_layout_shift({}, {}, {});", s_var, s_var, k_var, t_var)
        } else if line.starts_with("out(") {
            format!("print({});", &line[4..line.len()-1])
        } else {
            line.replace("def ", "let ")
        };
        rhai_lines.push(converted);
    }

    let full_script = format!("{}\n{}", logic_script, rhai_lines.join(";\n"));
    let mut scope = Scope::new();
    
    engine.run_with_scope(&mut scope, &full_script)
        .map_err(|e| JsValue::from_str(&format!("Rhai Runtime Error: {}", e)))?;

    let final_output = output.lock().unwrap().clone();
    Ok(final_output)
}

#[wasm_bindgen]
pub fn pack_logic(content: &str) -> Result<Vec<u8>, JsValue> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    
    encoder.write_all(content.as_bytes())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let compressed = encoder.finish()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let obfuscated: Vec<u8> = compressed.into_iter().map(|b| b ^ 0xFF).collect();

    Ok(obfuscated)
}