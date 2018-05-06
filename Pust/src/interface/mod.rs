extern crate pnet;
extern crate rand;
extern crate pnet_datalink;

use pnet::datalink::{self};

pub fn handle_interface_request(arg:&String) {
    if arg.eq("list") {

        for interfaces in datalink::interfaces() {
            println!("{}", interfaces);
        }
    }
}

pub fn get_active_interface() -> Option<pnet_datalink::NetworkInterface> {

    let interfaces = datalink::interfaces();

    let mut active_interface : Option<datalink::NetworkInterface> = None;

    //currently just selects the last one in the list.
    for inter in interfaces {
        let thing = inter.mac_address();
        let thing2 = inter.is_up();
        let thing3 = inter.ips.first();
        if let Some(thing3) = thing3 {
            println!("{}",thing3);
        }
        active_interface = Some(inter.clone());
    }

    return active_interface; 
}