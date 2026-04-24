use sysinfo::System;
use std::{thread, time};

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_all();

        // Clear screen (simple cross-platform-ish)
        print!("\x1B[2J\x1B[1;1H");

        println!("=== System Monitor ===\n");

        // CPU
        let cpu_usage = system.global_cpu_info().cpu_usage();
        println!("CPU Usage: {:.2}%", cpu_usage);

        // Memory
        let total_mem = system.total_memory() / 1024;
        let used_mem = system.used_memory() / 1024;

        println!("Memory: {} MB / {} MB", used_mem, total_mem);

        // Top processes (by CPU)
        println!("\nTop Processes:");

        let mut processes: Vec<_> = system.processes().values().collect();

        processes.sort_by(|a, b| {
            b.cpu_usage()
                .partial_cmp(&a.cpu_usage())
                .unwrap()
        });

        for process in processes.iter().take(5) {
            println!(
                "{} | CPU: {:.2}% | MEM: {} KB",
                process.name(),
                process.cpu_usage(),
                process.memory()
            );
        }

        // Refresh every 1 second
        thread::sleep(time::Duration::from_secs(1));
    }
}