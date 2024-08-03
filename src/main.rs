use std::env;    
use std::net::IpAddr;    
use std::process::Command;    
use std::str;    
use std::thread;    
use std::sync::mpsc::channel;    
    
fn main() {    
    print_banner();    
    let args: Vec<String> = env::args().collect();    
    if args.len() != 3 {    
        println!("Usage: {} <start_ip> <end_ip>", args[0]);    
        println!("Example: {} 192.168.1.1 192.168.1.255", args[0]);    
        return;    
    }    
    
    let start_ip = args[1].parse::<IpAddr>().expect("Invalid start IP address");    
    let end_ip = args[2].parse::<IpAddr>().expect("Invalid end IP address");    
    
    let (tx, rx) = channel();    
    
    let mut current_ip = start_ip;    
    
    while current_ip <= end_ip {    
        let tx = tx.clone();    
        let ip = current_ip.to_string();    
    
        thread::spawn(move || {    
            let output = Command::new("ping")    
                .arg("-c")    
                .arg("1")    
                .arg("-W")    
                .arg("1")    
                .arg(&ip)    
                .output()    
                .expect("failed to execute process");    
    
            let result = str::from_utf8(&output.stdout);    
    
            if let Ok(output_str) = result {    
                if output_str.contains("1 packets received") || output_str.contains("1 received") {    
                    tx.send(ip).expect("Failed to send IP");    
                }    
            }    
        });    
    
        current_ip = increment_ip(current_ip);    
    }    
    
    drop(tx); // Close the sending side of the channel    
    
    for received_ip in rx {    
        println!("Host up: {}", received_ip);    
    }    
}    
    
fn increment_ip(ip: IpAddr) -> IpAddr {    
    let mut octets = match ip {    
        IpAddr::V4(ipv4) => ipv4.octets().to_vec(),    
        IpAddr::V6(_) => panic!("IPv6 is not supported"),    
    };    
    
    for i in (0..octets.len()).rev() {    
        if octets[i] < 255 {    
            octets[i] += 1;    
            break;    
        } else {    
            octets[i] = 0;    
        }    
    }    
    
    IpAddr::from([octets[0], octets[1], octets[2], octets[3]])    
}    
    
fn print_banner() {    
    // ANSI color codes    
    let cyan = "x1b[36m";    
    let yellow = "x1b[33m";    
    let reset = "x1b[0m"; // Resets the color to default    
    
    println!("{}    ____                  _   __     __  _____                {}", cyan, reset);    
    println!("{}   / __ )__  __________  / | / /__  / /_/ ___/_________ _____ {}", cyan, reset);    
    println!("{}  / __  / / / /_  /_  / /  |/ / _ / __/__ / ___/ __ `/ __ {}", yellow, reset);    
    println!("{} / /_/ / /_/ / / /_/ /_/ /|  /  __/ /_ ___/ / /__/ /_/ / / / /{}", yellow, reset);    
    println!("{}/_____/__,_/ /___/___/_/ |_/___/__//____/___/__,_/_/ /_/ {}", cyan, reset);    
    println!("{}                                                              {}", cyan, reset);    
    println!("{}Author: Abeis Nelson{}", yellow, reset);    
}    
