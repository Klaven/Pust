extern crate oping;
use oping::{Ping, PingResult};

fn main() {
    println!("Hello, world!");

    let mut ping = Ping::new();
    try!(ping.add_host("google.com"));
    let response = try!(ping.send());

    for resp in responses {
        if resp.dropped > 0 {
            println!("No response from host: {}", resp.hostname);
        } else {
            println!("Response from host {} (address {}): latency {} ms",
                     resp.hostname,
                     resp.address,
                     resp.latency_ms);
            println!("  all details: {:?}", resp);
        }
    }
}
