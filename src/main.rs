mod network;
use network::download::measure_download;

fn main() {
    match measure_download() {
        Ok(speed) => println!(" Download speed: {:.2} Mbps", speed),
        Err(e) => println!(" Download failed: {}", e),
    }
}
