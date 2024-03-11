use sysctl::Sysctl;

fn main() {
    let cpu_brand = sysctl::Ctl::new("machdep.cpu.brand_string").unwrap();
    let ncpus = sysctl::Ctl::new("hw.ncpu").unwrap();
    let perf_cores = sysctl::Ctl::new("hw.perflevel0.physicalcpu").unwrap();
    let eff_cores = sysctl::Ctl::new("hw.perflevel1.physicalcpu").unwrap();
    let mem_size = sysctl::Ctl::new("hw.memsize").unwrap();

    println!(
        "machdep.cpu.brand_string: {}",
        cpu_brand.value_string().unwrap()
    );
    println!("hw.ncpu: {}", ncpus.value_string().unwrap());
    println!(
        "hw.perflevel0.physicalcpu: {}",
        perf_cores.value_string().unwrap()
    );
    println!(
        "hw.perflevel1.physicalcpu: {}",
        eff_cores.value_string().unwrap()
    );
    println!("hw.memsize: {}", mem_size.value_string().unwrap());
}
