use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use sysinfo::{System, RefreshKind, MemoryRefreshKind};
use std::time::Duration;
use std::thread::sleep;

const CHUNK_SIZE: u64 = 4 * 1024 * 1024 * 1024; // 4GB por parte

fn get_total_ram() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::new()));
    sys.total_memory()
}

fn dump_memory(output_prefix: &str) {
    let total_ram = get_total_ram();
    let mut file = File::open("/dev/mem").expect("Falha ao acessar /dev/mem. Execute como root.");
    let mut buffer = vec![0u8; 4096];
    let mut offset: u64 = 0;
    let mut part = 1;

    while offset < total_ram {
        let part_size = std::cmp::min(CHUNK_SIZE, total_ram - offset);
        let mut output = OpenOptions::new().write(true).create(true).open(format!("{}_part_{}.bin", output_prefix, part)).expect("Falha ao criar arquivo");
        file.seek(SeekFrom::Start(offset)).unwrap();

        let mut remaining = part_size;
        let mut written: u64 = 0;

        while remaining > 0 {
            let to_read = std::cmp::min(buffer.len() as u64, remaining) as usize;
            let bytes_read = file.read(&mut buffer[..to_read]).unwrap_or(0);
            if bytes_read == 0 { break; }
            output.write_all(&buffer[..bytes_read]).unwrap();
            written += bytes_read as u64;
            remaining -= bytes_read as u64;

            // Atualiza a barra de progresso
            let progress = (written as f64 / part_size as f64) * 100.0;
            print!("\rExtração do dump: {:.2}% concluído", progress);
            std::io::stdout().flush().unwrap();
        }

        println!("\nParte {} salva.", part);
	sleep(Duration::from_millis(100));
        offset += part_size;
        part += 1;
    }
}

fn main() {
    let total_ram = get_total_ram();
    println!("Memória total: {} GB", total_ram / (1024 * 1024 * 1024));
    dump_memory("ram_dump");
}
