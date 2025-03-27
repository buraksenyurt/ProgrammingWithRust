use sysinfo::System;

pub fn print_all_metrics(system: &mut System) {
    print_memory(system);
    print_cpu(system);
}

pub fn print_memory(system: &mut System) {
    system.refresh_memory();
    println!("{} Mb", system.free_memory() / (1024 * 1024));
}

pub fn print_cpu(system: &mut System) {
    system.refresh_cpu_usage();
    for (id, cpu) in system.cpus().iter().enumerate() {
        print!("{} ({:2.2} %)", id, cpu.cpu_usage());
    }
    println!();
}

pub fn show_usages() {
    //todo@buraksenyurt json veya xml formatta çıktı işlevi ekleyelim
    println!(
        r"
    A tiny system metrics collector about
    Cpu or memory usages

    Commands    :

    kind        : cpu or mem
    count       : Number of total metric
    period      : Number of periodic metric in seconds
    format      : Output format (json / xml)

    Usages      :

    kind cpu count 100 period 2 format json
    (Get cpu usage every 2 seconds for 100 times and save all into json/xml file)
    "
    );
}
