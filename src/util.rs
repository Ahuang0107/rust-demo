#[allow(dead_code)]
fn format_memory(memory: u64) -> String {
    let mut used_memory = memory as f64;
    let mut unit_index = 0;
    let unit_format = vec!["B", "KB", "MB", "GB"];
    while used_memory > 1024.0 {
        used_memory /= 1024.0;
        unit_index += 1;
    }
    used_memory = used_memory.round();
    used_memory.to_string() + unit_format[unit_index]
}
