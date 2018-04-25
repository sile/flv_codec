use bytecodec::{ByteCount, Decode, Encode, Eos, ExactBytesEncode, Result};
use bytecodec::bytes::CopyableBytesDecoder;
use bytecodec::combinator::{Buffered, Length, SkipRemaining};

#[derive(Debug, Default, Clone)]
pub struct FlvHeader {
    pub audio_flag: bool,
    pub video_flag: bool,
}

#[derive(Debug, Default)]
pub struct FlvHeaderDecoder {
    bytes: Buffered<CopyableBytesDecoder<[u8; 9]>>,
    rest: Length<SkipRemaining<NullDecoder>>,
}
impl Decode for FlvHeaderDecoder {
    type Item = FlvHeader;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<(usize, Option<Self::Item>)> {
        let mut offset = 0;
        if !self.bytes.has_item() {
            offset += track!(self.bytes.decode(buf, eos))?.0;
            if !self.bytes.has_item() {
                return Ok((offset, None));
            }
        }
        unimplemented!()
    }

    fn has_terminated(&self) -> bool {
        false
    }

    fn requiring_bytes(&self) -> ByteCount {
        self.bytes
            .requiring_bytes()
            .add_for_decoding(self.rest.requiring_bytes())
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
impl ExactBytesEncode for FlvHeaderEncoder {
    fn exact_requiring_bytes(&self) -> u64 {
        unimplemented!()
    }
}

// TODO: move to bytecodec
#[derive(Debug, Default)]
struct NullDecoder;
impl Decode for NullDecoder {
    type Item = ();

    fn decode(&mut self, _buf: &[u8], _eos: Eos) -> Result<(usize, Option<Self::Item>)> {
        Ok((0, Some(())))
    }

    fn has_terminated(&self) -> bool {
        false
    }

    fn requiring_bytes(&self) -> ByteCount {
        ByteCount::Finite(0)
    }
}
