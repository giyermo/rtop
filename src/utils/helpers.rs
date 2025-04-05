pub fn byte_to_readable_size(size_in_bytes: u64) -> (f64, String) {
    let mut new_size: f64 = size_in_bytes as f64;
    let mut i = 0;

    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

    while new_size >= 1024.0 {
        new_size /= 1024.0;
        i += 1;
    }

    let unit = if i < units.len() {
        units[i].to_string()
    } else {
        "YiB".to_string() // Default to "YiB" for very large values
    };

    (new_size, unit)
}
