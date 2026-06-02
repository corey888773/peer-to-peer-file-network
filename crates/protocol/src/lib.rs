struct Bitfield {
    mask: Vec<u8>,
}
impl Bitfield {
    fn has_piece(&self, index: u32) -> bool {
        let byte = index / 8;
        let bit = index % 8;

        (1 << 7 - bit) & self.mask[byte as usize] != 0
    }

    fn set_piece(&mut self, index: u32) {
        let byte = index / 8;
        let bit = index % 8;

        self.mask[byte as usize] |= 1 << (7 - bit)
    }

    fn payload(&self) -> Vec<u8> {
        self.mask.clone()
    }

    fn from_buffer(buffer: Vec<u8>) -> Self {
        Self { mask: buffer }
    }
}

struct Request {
    index: u32,
    begin: u32,
    length: u32,
}

impl Request {
    fn payload(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.index.to_be_bytes());
        buffer.extend_from_slice(&self.begin.to_be_bytes());
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer
    }

    fn from_buffer(buffer: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let index: u32 = u32::from_be_bytes(buffer[0..4].try_into()?);
        let begin: u32 = u32::from_be_bytes(buffer[4..8].try_into()?);
        let length: u32 = u32::from_be_bytes(buffer[8..12].try_into()?);

        Ok(Self {
            index,
            begin,
            length,
        })
    }
}

struct Piece {
    index: u32,
    begin: u32,
    block: Vec<u8>,
}

impl Piece {
    fn payload(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.index.to_be_bytes());
        buffer.extend_from_slice(&self.begin.to_be_bytes());
        buffer.extend_from_slice(&self.block);
        buffer
    }

    fn from_buffer(buffer: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let index: u32 = u32::from_be_bytes(buffer[0..4].try_into()?);
        let begin: u32 = u32::from_be_bytes(buffer[4..8].try_into()?);
        let block: Vec<u8> = buffer[8..].to_vec();

        Ok(Self {
            index,
            begin,
            block,
        })
    }
}

enum PeerMessage {
    Choke(),
    Unchoke(),
    Interested(),
    NotInterested(),
    Have(u32),
    Bitfield(Bitfield),
    Request(Request),
    Piece(Piece),
    Cancel(Request),
}

impl PeerMessage {
    fn serialize_without_payload(id: u8) -> Vec<u8> {
        let length: i32 = 1;
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&length.to_be_bytes());
        buffer.push(id);
        buffer
    }

    fn serialize_with_payload(id: u8, paylaod: Vec<u8>) -> Vec<u8> {
        let length = 1 + paylaod.len();
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&length.to_be_bytes());
        buffer.push(id);
        buffer.extend_from_slice(&paylaod);
        buffer
    }

    pub fn serialize(self) -> Vec<u8> {
        match self {
            PeerMessage::Choke() => Self::serialize_without_payload(0),
            PeerMessage::Unchoke() => Self::serialize_without_payload(1),
            PeerMessage::Interested() => Self::serialize_without_payload(2),
            PeerMessage::NotInterested() => Self::serialize_without_payload(3),
            PeerMessage::Have(index) => Self::serialize_with_payload(4, index.to_be_bytes().into()),
            PeerMessage::Bitfield(bitfield) => Self::serialize_with_payload(5, bitfield.payload()),
            PeerMessage::Request(request) => Self::serialize_with_payload(6, request.payload()),
            PeerMessage::Piece(piece) => Self::serialize_with_payload(7, piece.payload()),
            PeerMessage::Cancel(request) => Self::serialize_with_payload(8, request.payload()),
        }
    }

    pub fn deserialize(buffer: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let id: u8 = buffer[4];
        let payload: Vec<u8> = buffer[5..].to_vec();

        match id {
            0 => Ok(PeerMessage::Choke()),
            1 => Ok(PeerMessage::Unchoke()),
            2 => Ok(PeerMessage::Interested()),
            3 => Ok(PeerMessage::NotInterested()),
            4 => Ok(PeerMessage::Have(u32::from_be_bytes(
                buffer[5..9].try_into()?,
            ))),
            5 => Ok(PeerMessage::Bitfield(Bitfield::from_buffer(payload))),
            6 => Ok(PeerMessage::Request(Request::from_buffer(payload)?)),
            7 => Ok(PeerMessage::Piece(Piece::from_buffer(payload)?)),
            8 => Ok(PeerMessage::Cancel(Request::from_buffer(payload)?)),
            _ => Err(format!("unsupported message id {id}").into()),
        }
    }
}

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
