use std::net::{TcpStream, ToSocketAddrs, SocketAddr};
use std::time::Duration;
use std::thread;


fn scan_ports(url: &str, start: u16, end: u16) -> Vec<u16> {
    let mut checked_count = 0;
    let total_count = end - start;

    (start..end).filter_map(|port| {
        checked_count += 1;
        let progress = (checked_count as f64 / total_count as f64) * 100.0;
        print!("Checking port {} ({:.2}%)", port, progress);

        let socket_addr = (url, port);
        let timeout  = Duration::from_secs(1);

        let mut socket_addrs = socket_addr.to_socket_addrs().unwrap();
        let socket_addr: SocketAddr = socket_addrs.next().unwrap();

        match TcpStream::connect_timeout(&socket_addr, timeout) {
            Ok(_) => Some(port),
            Err(_) => None,
        }
    }).collect()
}

fn main() {
    let url = "testphp.vulnweb.com";
    let port_range = (1, 1000);

    let chunk_size = 100;
    let num_threads = (port_range.1 - port_range.0) / chunk_size;

    let mut threads = Vec::new();
    for i in 0..num_threads {
        let start = port_range.0 + (i * chunk_size);
        let end = start + chunk_size;

        threads.push(thread::spawn(move || {
            scan_ports(url, start, end)
        }));
    }

    let mut open_ports = Vec::new();
    for thread in threads {
        open_ports.extend(thread.join().unwrap());
    }

    print!("Open ports on {}:{:?}", url, open_ports);
}
