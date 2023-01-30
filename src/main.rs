use std::io::Write;

use sysinfo::{CpuExt, SystemExt};

fn main() {
    std::fs::remove_file("metrics.csv").unwrap_or(());
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("metrics.csv")
        .unwrap();
    writeln!(&mut file, "cpu_usage,memory_usage").unwrap();

    let mut sys = sysinfo::System::new_all();

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();

        let cu = sys.global_cpu_info().cpu_usage();
        let mu = ((sys.used_memory() as f64) / (sys.total_memory() as f64)) * 100.0;

        writeln!(&mut file, "{:.2},{:.2}", cu, mu).unwrap();

        println!("{:.2},{:.2}", cu, mu);

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
