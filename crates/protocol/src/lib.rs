pub struct Handshake {
    reserved: [u8; 8],
    info_hash: [u8; 20],
    peer_id: [u8; 20],
}

impl Handshake {
    const PSTRLEN: u8 = 19;
    const PSTR: &[u8] = b"BitTorrent protocol";

    fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            reserved: [0u8; 8], // empty according to https://bittorrent.org/beps/bep_0003.html
            info_hash,
            peer_id,
        }
    }

    fn serialize(self) -> [u8; 68] {
        let mut buffer = [0u8; 68];
        buffer[0] = Self::PSTRLEN;
        buffer[1..=19].copy_from_slice(Self::PSTR);
        buffer[20..=27].copy_from_slice(&self.reserved);
        buffer[28..=47].copy_from_slice(&self.info_hash);
        buffer[48..=67].copy_from_slice(&self.peer_id);
        buffer
    }

    fn deserialize(buffer: [u8; 68]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            reserved: buffer[20..=27].try_into()?,
            info_hash: buffer[28..=47].try_into()?,
            peer_id: buffer[48..=67].try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happypath_serialize() {
        let expected_reserved = [0u8; 8];
        let expected_info_hash = [1u8; 20];
        let expected_peer_id = [2u8; 20];
        let sut = Handshake::new(expected_info_hash, expected_peer_id);

        let buffer = sut.serialize();

        assert_eq!(buffer[0], Handshake::PSTRLEN);
        assert_eq!(&buffer[1..=19], Handshake::PSTR);
        assert_eq!(&buffer[20..=27], expected_reserved);
        assert_eq!(&buffer[28..=47], expected_info_hash);
        assert_eq!(&buffer[48..=67], expected_peer_id);
    }

    #[test]
    fn happypath_deserialize() {
        let expected_reserved = [0u8; 8];
        let expected_info_hash = [1u8; 20];
        let expected_peer_id = [2u8; 20];

        let mut buffer = [0u8; 68];
        buffer[0] = Handshake::PSTRLEN;
        buffer[1..=19].copy_from_slice(Handshake::PSTR);
        buffer[20..=27].copy_from_slice(&expected_reserved);
        buffer[28..=47].copy_from_slice(&expected_info_hash);
        buffer[48..=67].copy_from_slice(&expected_peer_id);

        match Handshake::deserialize(buffer) {
            Ok(sut) => {
                assert_eq!(sut.reserved, expected_reserved);
                assert_eq!(sut.info_hash, expected_info_hash);
                assert_eq!(sut.peer_id, expected_peer_id);
            }
            Err(err) => {
                panic!("test failed, this should not happen {err}")
            }
        }
    }
}
