fn main() {}

#[cfg(test)]
mod integration_test {
    use protocol::Handshake;
    use std::io::Write;
    use std::{
        io::Read,
        net::{TcpListener, TcpStream},
    };

    #[test]
    fn basic_hadnshake_test() -> Result<(), Box<dyn std::error::Error>> {
        let expected_handshake = Handshake::new([0u8; 20], [1u8; 20]).serialize();

        let listener = TcpListener::bind("0.0.0.0:6397")?;
        let h = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();

            let mut read_buffer = [0u8; 68];
            stream.read_exact(&mut read_buffer).unwrap();

            let received_handshake = Handshake::deserialize(read_buffer).unwrap();
            assert_eq!(received_handshake.serialize(), expected_handshake)
        });

        let mut client = TcpStream::connect("0.0.0.0:6397")?;
        client.write(&expected_handshake).unwrap();
        h.join().unwrap();

        Ok(())
    }
}
