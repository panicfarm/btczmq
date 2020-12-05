use error::Error;
use std::convert::TryFrom;

mod error;

fn main() -> Result<(), Error> {
    println!("connecting ...");
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB)?;
    subscriber.connect("tcp://127.0.0.1:28332")?;
    subscriber.set_subscribe(b"hashblock")?;
    //subscriber.set_subscribe(b"hashtx")?;
    //subscriber.set_subscribe(b"rawblock")?;
    //subscriber.set_subscribe(b"rawtx")?;
    let mut msg = zmq::Message::new();
    loop {
        subscriber.recv(&mut msg, 0)?;
        //println!("{:?} {:?} {}", msg, msg.as_str(), msg.get_more());
        match msg.as_str() {
            Some(channel) => {
                //i guess we always have two more parts
                let mut n = 0;
                while subscriber.get_rcvmore()? {
                    n += 1;
                    let v = subscriber.recv_bytes(0)?;
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
            _ => Err(Error::Zmq {
                reason: "Unexpected zmq part".into(),
            })?,
            //println!("{:?}", subscriber.recv_string(0)?);
        }
    }
}
