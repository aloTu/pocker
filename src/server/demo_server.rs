mod command;
use crate::command::FromClientMessage;
use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node::{self};

use std::collections::HashMap;

struct ClientInfo {
    count: usize,
}

fn main() {
    let (handler, listener) = node::split::<()>();

    let mut clients: HashMap<Endpoint, ClientInfo> = HashMap::new();

    match handler
        .network()
        .listen(Transport::FramedTcp, "0.0.0.0:3042")
    {
        Ok((_id, real_addr)) => println!("Server running at {}", real_addr,),
        Err(_) => return println!("Can not listening at 0.0.0.0:3042"),
    }

    // Read incoming network events.
    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => unreachable!(), // Used for explicit connections.
        NetEvent::Accepted(endpoint, _listener) => {
            clients.insert(endpoint, ClientInfo { count: 0 });
            println!(
                "Client ({}) connected (total clients: {})",
                endpoint.addr(),
                clients.len()
            );
        }
        NetEvent::Message(endpoint, input_data) => {
            let message: FromClientMessage = bincode::deserialize(&input_data).unwrap();
            println!("Received: {:?}", message);
            handler.network().send(endpoint, input_data);
        }
        NetEvent::Disconnected(endpoint) => {
            clients.remove(&endpoint).unwrap();
            println!(
                "Client ({}) disconnected (total clients: {})",
                endpoint.addr(),
                clients.len()
            );
        }
    });
}
