use bytecodec::bytes::{BytesEncoder, CopyableBytesDecoder};
use bytecodec::combinator::{Length, Peekable};
use bytecodec::fixnum::{U32beDecoder, U32beEncoder, U8Decoder, U8Encoder};
use bytecodec::padding::PaddingDecoder;
use bytecodec::{ByteCount, Decode, Encode, Eos, ErrorKind, Result, SizedEncode};

const SIGNATURE: [u8; 3] = *b"FLV";
const VERSION: u8 = 1;
const HEADER_SIZE: usize = 9;

const FLAG_AUDIO: u8 = 0b0000_0100;
const FLAG_VIDEO: u8 = 0b0000_0001;

/// FLV header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// Whether audio tags are present in the FLV file.
    pub has_audio: bool,

    /// Whether video tags are present in the FLV file.
    pub has_video: bool,
}

#[derive(Debug, Default)]
pub struct HeaderEncoder {
    signature: BytesEncoder<[u8; 3]>,
    version: U8Encoder,
    flags: U8Encoder,
    data_offset: U32beEncoder,
}
impl Encode for HeaderEncoder {
    type Item = Header;

    fn encode(&mut self, buf: &mut [u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        bytecodec_try_encode!(self.signature, offset, buf, eos);
        bytecodec_try_encode!(self.version, offset, buf, eos);
        bytecodec_try_encode!(self.flags, offset, buf, eos);
        bytecodec_try_encode!(self.data_offset, offset, buf, eos);
        Ok(offset)
    }

    fn start_encoding(&mut self, item: Self::Item) -> Result<()> {
        let flags = (item.has_audio as u8 * FLAG_AUDIO) | (item.has_video as u8 * FLAG_VIDEO);
        track!(self.signature.start_encoding(SIGNATURE))?;
        track!(self.version.start_encoding(VERSION))?;
        track!(self.flags.start_encoding(flags))?;
        track!(self.data_offset.start_encoding(HEADER_SIZE as u32))?;
        Ok(())
    }

    fn requiring_bytes(&self) -> ByteCount {
        ByteCount::Finite(self.exact_requiring_bytes())
    }

    fn is_idle(&self) -> bool {
        self.signature.is_idle()
            && self.version.is_idle()
            && self.flags.is_idle()
            && self.data_offset.is_idle()
    }
}
impl SizedEncode for HeaderEncoder {
    fn exact_requiring_bytes(&self) -> u64 {
        self.signature.exact_requiring_bytes()
            + self.version.exact_requiring_bytes()
            + self.flags.exact_requiring_bytes()
            + self.data_offset.exact_requiring_bytes()
    }
}

#[derive(Debug, Default)]
pub struct HeaderDecoder {
    signature: CopyableBytesDecoder<[u8; 3]>,
    version: U8Decoder,
    flags: U8Decoder,
    data_offset: Peekable<U32beDecoder>,
    padding: Length<PaddingDecoder>,
}
impl Decode for HeaderDecoder {
    type Item = Header;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        bytecodec_try_decode!(self.signature, offset, buf, eos);
        bytecodec_try_decode!(self.version, offset, buf, eos);
        bytecodec_try_decode!(self.flags, offset, buf, eos);
        if !self.data_offset.is_idle() {
            bytecodec_try_decode!(self.data_offset, offset, buf, eos);

            let offset = self.data_offset.peek().cloned().expect("Never fails") as usize;
            track_assert!(offset >= HEADER_SIZE, ErrorKind::InvalidInput; offset);

            let padding_size = (offset - HEADER_SIZE) as u64;
            track!(self.padding.set_expected_bytes(padding_size))?;
        }
        bytecodec_try_decode!(self.padding, offset, buf, eos);
        Ok(offset)
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let signature = track!(self.signature.finish_decoding())?;
        track_assert_eq!(
            signature,
            SIGNATURE,
            ErrorKind::InvalidInput,
            "Not a FLV file"
        );

        let version = track!(self.version.finish_decoding())?;
        track_assert_eq!(
            version,
            VERSION,
            ErrorKind::InvalidInput,
            "Unknown FLV version"
        );

        let flags = track!(self.flags.finish_decoding())?;
        let has_audio = (flags & FLAG_AUDIO) != 0;
        let has_video = (flags & FLAG_VIDEO) != 0;

        let _ = track!(self.data_offset.finish_decoding());
        let _ = track!(self.padding.finish_decoding());
        track!(self.padding.set_expected_bytes(0))?;

        Ok(Header {
            has_audio,
            has_video,
        })
    }

    fn is_idle(&self) -> bool {
        self.signature.is_idle()
            && self.version.is_idle()
            && self.flags.is_idle()
            && self.data_offset.is_idle()
            && self.padding.is_idle()
    }

    fn requiring_bytes(&self) -> ByteCount {
        self.signature
            .requiring_bytes()
            .add_for_decoding(self.version.requiring_bytes())
            .add_for_decoding(self.flags.requiring_bytes())
            .add_for_decoding(self.data_offset.requiring_bytes())
            .add_for_decoding(self.padding.requiring_bytes())
    }
}
