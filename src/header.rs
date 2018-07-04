use bytecodec::bytes::CopyableBytesDecoder;
use bytecodec::combinator::Length;
use bytecodec::padding::PaddingDecoder;
use bytecodec::{ByteCount, Decode, Encode, Eos, ErrorKind, Result, SizedEncode};

#[derive(Debug, Default, Clone)]
pub struct FlvHeader {
    pub audio_flag: bool,
    pub video_flag: bool,
}

#[derive(Debug, Default)]
pub struct FlvHeaderDecoder {
    bytes: CopyableBytesDecoder<[u8; 9]>,
    padding: Length<PaddingDecoder>,
}
impl FlvHeaderDecoder {
    pub fn new() -> Result<Self> {
        track_panic!(ErrorKind::Other, "unimplemented");
    }
}
impl Decode for FlvHeaderDecoder {
    type Item = FlvHeader;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        // let mut offset = 0;
        // if !self.bytes.has_item() {
        //     offset += track!(self.bytes.decode(buf, eos))?.0;
        //     if !self.bytes.has_item() {
        //         return Ok((offset, None));
        //     }
        // }
        track_panic!(ErrorKind::Other, "unimplemented");
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        unimplemented!()
    }

    fn is_idle(&self) -> bool {
        unimplemented!()
    }

    fn requiring_bytes(&self) -> ByteCount {
        self.bytes
            .requiring_bytes()
            .add_for_decoding(self.padding.requiring_bytes())
    }
}

#[derive(Debug, Default)]
pub struct FlvHeaderEncoder {}
impl Encode for FlvHeaderEncoder {
    type Item = FlvHeader;

    fn encode(&mut self, buf: &mut [u8], eos: Eos) -> Result<usize> {
        unimplemented!()
    }

    fn start_encoding(&mut self, item: Self::Item) -> Result<()> {
        unimplemented!()
    }

    fn is_idle(&self) -> bool {
        unimplemented!()
    }

    fn requiring_bytes(&self) -> ByteCount {
        unimplemented!()
    }
}
impl SizedEncode for FlvHeaderEncoder {
    fn exact_requiring_bytes(&self) -> u64 {
        unimplemented!()
    }
}
