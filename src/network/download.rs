use std::time::Instant;
use std::io::Read;
use reqwest::blocking::get;

pub fn measure_download() -> Result<f64, String> {
    let download_url = "http://ipv4.download.thinkbroadband.com/10MB.zip";
    println!("Starting download");

    let start = Instant::now();

    let mut response = get(download_url)
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Server returned status code: {}", response.status()));
    }

    // Read response in chunks
    let mut total_bytes = 0;
    let mut buffer = [0u8; 8 * 1024];
    loop {
        let bytes_read = response.read(&mut buffer)
            .map_err(|e| format!("Read failed: {}", e))?;
        if bytes_read == 0 { break; }
        total_bytes += bytes_read;
    }

    let elapsed_secs = start.elapsed().as_secs_f64();
    let megabits = (total_bytes * 8) as f64 / 1_000_000.0;
    let speed_mbps = megabits / elapsed_secs;

    Ok(speed_mbps)
}
