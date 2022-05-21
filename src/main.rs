use pnet::datalink;
use std::io;

fn main() {
    let interfaces = datalink::interfaces();

    println!("Please select an interface to listen on.");
    for (i, iface) in interfaces.iter().enumerate() {
        println!("{}: {}", i, iface.description);
    }

    let mut iface_ind_str = String::new();
    io::stdin().read_line(&mut iface_ind_str).unwrap();
    println!("Got: \"{}\"", iface_ind_str);

    let iface_ind = iface_ind_str.parse::<usize>().unwrap();
    let selected_iface = &interfaces[iface_ind];

    let (_, mut rx) = match datalink::channel(selected_iface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_)  => panic!("Unknown channel type"),
        Err(e) => panic!("Error openning channel: {}", e),
    };

    println!("Listening for packets...");
    while let Ok(_) = rx.next() {
        println!("Got packet!")
    }
}
