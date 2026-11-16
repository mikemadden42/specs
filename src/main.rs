#[cfg(target_os = "macos")]
fn main() {
    use sysctl::Sysctl;

    let keys = [
        "machdep.cpu.brand_string",
        "hw.ncpu",
        "hw.perflevel0.physicalcpu",
        "hw.perflevel1.physicalcpu",
        "hw.memsize",
    ];

    for key in keys {
        let value = sysctl::Ctl::new(key).and_then(|ctl| ctl.value_string());
        let value = match value {
            Ok(v) if key == "hw.memsize" => format_bytes(&v),
            Ok(v) => v,
            Err(_) => "n/a".to_string(),
        };
        println!("{key}: {value}");
    }
}

#[cfg(target_os = "macos")]
fn format_bytes(raw: &str) -> String {
    match raw.trim().parse::<u64>() {
        Ok(bytes) => format!("{:.1} GiB", bytes as f64 / (1u64 << 30) as f64),
        Err(_) => raw.to_string(),
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("specs only supports macOS");
    std::process::exit(1);
}
