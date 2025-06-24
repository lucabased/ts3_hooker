mod process_checker;
mod network_sniffer;
mod packet_handler;

fn main() {
    if let Some(pid) = process_checker::get_ts3_server_pid() {
        println!("TeamSpeak 3 server is running with PID: {}. Starting network sniffer...", pid);
        let handles = network_sniffer::start_sniffing(pid);
        for handle in handles {
            handle.join().unwrap();
        }
    } else {
        println!("TeamSpeak 3 server is not running.");
    }
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
