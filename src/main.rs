use std::convert::TryFrom;
fn main() {
    println!("connecting ...");
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://127.0.0.1:28332").unwrap();
    subscriber.set_subscribe(b"hashblock").unwrap();
    subscriber.set_subscribe(b"hashtx").unwrap();
    subscriber.set_subscribe(b"rawblock").unwrap();
    subscriber.set_subscribe(b"rawtx").unwrap();
    let mut msg = zmq::Message::new();
    loop {
        subscriber.recv(&mut msg, 0).unwrap();
        //println!("{:?} {:?} {}", msg, msg.as_str(), msg.get_more());
        match msg.as_str() {
            Some(channel) => {
                //i guess we always have two more parts
                let mut n = 0;
                while subscriber.get_rcvmore().unwrap() {
                    n += 1;
                    let v = subscriber.recv_bytes(0).unwrap();
                    if n == 1 {
                        println!("{}: {:?}", channel, hex::encode(v));
                    } else if n == 2 {
                        println!(
                            "id: {:?}",
                            u32::from_le_bytes(<[u8; 4]>::try_from(v).unwrap())
                        );
                    }
                }
            }
            _ => break,
            //println!("{:?}", subscriber.recv_string(0).unwrap());
        }
    }
}
