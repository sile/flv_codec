use bytecodec::combinator::MaybeEos;
use bytecodec::fixnum::U32beDecoder;
use bytecodec::{ByteCount, Decode, DecodeExt, Eos, ErrorKind, Result};

use {FlvTag, FlvTagDecoder};

#[derive(Debug)]
pub struct FlvBodyDecoder {
    is_head: bool,
    tag: MaybeEos<FlvTagDecoder>,
    prev_tag_size: U32beDecoder,
}
impl FlvBodyDecoder {
    pub fn new() -> Self {
        FlvBodyDecoder::default()
    }
}
impl Default for FlvBodyDecoder {
    fn default() -> Self {
        FlvBodyDecoder {
            is_head: true,
            tag: FlvTagDecoder::default().maybe_eos(),
            prev_tag_size: U32beDecoder::default(),
        }
    }
}
impl Decode for FlvBodyDecoder {
    type Item = FlvTag;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        if self.is_head {
            bytecodec_try_decode!(self.prev_tag_size, offset, buf, eos);
            let tag_size = track!(self.prev_tag_size.finish_decoding())?;
            track_assert_eq!(tag_size, 0, ErrorKind::InvalidInput);
            self.is_head = false;
        }
        bytecodec_try_decode!(self.tag, offset, buf, eos);
        bytecodec_try_decode!(self.prev_tag_size, offset, buf, eos);
        Ok(offset)
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        // FIXME: check actual tag size
        let tag = track!(self.tag.finish_decoding())?;
        let _tag_size = track!(self.prev_tag_size.finish_decoding())?;
        Ok(tag)
    }

    fn is_idle(&self) -> bool {
        self.tag.is_idle() && self.prev_tag_size.is_idle()
    }

    fn requiring_bytes(&self) -> ByteCount {
        if self.is_head {
            self.prev_tag_size.requiring_bytes()
        } else {
            self.tag
                .requiring_bytes()
                .add_for_decoding(self.prev_tag_size.requiring_bytes())
        }
    }
}
