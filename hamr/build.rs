use std::{env, fs::File, io::Write, path::Path};

use serde_json::Value;

fn main() {
    println!("cargo:rerun-if-changed=data/");

    let out_dir_str = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    generate_code_from_dict(
        "Domain",
        "data/domain_encode.json",
        out_dir.join("domain.rs"),
    );
    generate_code_from_dict("Path", "data/path_encode.json", out_dir.join("path.rs"));
    generate_code_from_dict("Sld", "data/sld_encode.json", out_dir.join("sld.rs"));
    generate_code_from_dict("Tld", "data/tld_encode.json", out_dir.join("tld.rs"));
}

fn generate_code_from_dict(
    struct_name: &str,
    json_path: impl AsRef<Path>,
    out_file_path: impl AsRef<Path>,
) {
    let mut out_file = File::create(out_file_path).unwrap();

    let json_path = json_path.as_ref();
    let json_file = File::open(json_path).unwrap();
    let Value::Object(dict) = serde_json::from_reader(json_file).unwrap() else {
        panic!(
            "json file '{}' is not a top-level dict!",
            json_path.display()
        );
    };

    // encode
    writeln!(out_file, "pub struct {struct_name}Encode;").unwrap();
    writeln!(out_file, "impl {struct_name}Encode {{").unwrap();
    writeln!(
        out_file,
        "pub fn lookup(key: &str) -> Option<&'static str> {{"
    )
    .unwrap();
    writeln!(out_file, "match key {{").unwrap();
    for (key, val) in &dict {
        writeln!(out_file, "\"{key}\" => Some({val}),").unwrap();
    }
    writeln!(out_file, "_ => None,").unwrap();
    writeln!(out_file, "}}").unwrap();
    writeln!(out_file, "}}").unwrap();
    writeln!(out_file, "}}").unwrap();

    // decode
    writeln!(out_file, "pub struct {struct_name}Decode;").unwrap();
    writeln!(out_file, "impl {struct_name}Decode {{").unwrap();
    writeln!(
        out_file,
        "pub fn lookup(key: &str) -> Option<&'static str> {{"
    )
    .unwrap();
    writeln!(out_file, "match key {{").unwrap();
    for (key, val) in &dict {
        writeln!(out_file, "{val} => Some(\"{key}\"),").unwrap();
    }
    writeln!(out_file, "_ => None,").unwrap();
    writeln!(out_file, "}}").unwrap();
    writeln!(out_file, "}}").unwrap();
    writeln!(out_file, "}}").unwrap();

    if struct_name == "Sld" {
        let mut slds: Vec<_> = dict.keys().collect();
        // BUG: maybe should be reversed?
        slds.sort_by_key(|s| s.len());

        writeln!(out_file, "pub const SLD_LIST: &[&str] = &[").unwrap();
        for sld in slds {
            writeln!(out_file, "\"{sld}\",").unwrap();
        }
        writeln!(out_file, "];").unwrap();
    }
}
