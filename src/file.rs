use bytecodec::combinator::{Last, MaybeEos, Peekable};
use bytecodec::fixnum::{U32beDecoder, U32beEncoder};
use bytecodec::tuple::{TupleDecoder, TupleEncoder};
use bytecodec::{ByteCount, Decode, Encode, EncodeExt, Eos, ErrorKind, Result, SizedEncode};

use header::{Header, HeaderDecoder, HeaderEncoder};
use tag::{Tag, TagDecoder, TagEncoder};

/// FLV file encoder.
///
/// See the [specification] about the format of FLV file.
///
/// [specification]: https://wwwimages2.adobe.com/content/dam/acom/en/devnet/flv/video_file_format_spec_v10.pdf
#[derive(Debug)]
pub struct FileEncoder<Data> {
    header: Last<TupleEncoder<(HeaderEncoder, U32beEncoder)>>,
    tag: TagEncoder<Data>,
    prev_tag_size: U32beEncoder,
}
impl<Data> FileEncoder<Data> {
    /// Makes a new `FileEncoder` instance.
    pub fn new(header: Header) -> Self {
        FileEncoder {
            header: TupleEncoder::default().last((header, 0)),
            tag: TagEncoder::default(),
            prev_tag_size: U32beEncoder::default(),
        }
    }
}
impl<Data: AsRef<[u8]>> Encode for FileEncoder<Data> {
    type Item = Tag<Data>;

    fn encode(&mut self, buf: &mut [u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        bytecodec_try_encode!(self.header, offset, buf, eos);
        bytecodec_try_encode!(self.tag, offset, buf, eos);
        bytecodec_try_encode!(self.prev_tag_size, offset, buf, eos);
        Ok(offset)
    }

    fn start_encoding(&mut self, item: Self::Item) -> Result<()> {
        let tag_size = item.tag_size();
        track!(self.tag.start_encoding(item))?;
        track!(self.prev_tag_size.start_encoding(tag_size))?;
        Ok(())
    }

    fn requiring_bytes(&self) -> ByteCount {
        ByteCount::Finite(self.exact_requiring_bytes())
    }

    fn is_idle(&self) -> bool {
        self.header.is_idle() && self.tag.is_idle() && self.prev_tag_size.is_idle()
    }
}
impl<Data: AsRef<[u8]>> SizedEncode for FileEncoder<Data> {
    fn exact_requiring_bytes(&self) -> u64 {
        self.header.exact_requiring_bytes()
            + self.tag.exact_requiring_bytes()
            + self.prev_tag_size.exact_requiring_bytes()
    }
}
impl<Data> Default for FileEncoder<Data> {
    fn default() -> Self {
        FileEncoder::new(Header {
            has_audio: true,
            has_video: true,
        })
    }
}

/// FLV file decoder.
///
/// See the [specification] about the format of FLV file.
///
/// [specification]: https://wwwimages2.adobe.com/content/dam/acom/en/devnet/flv/video_file_format_spec_v10.pdf
#[derive(Debug, Default)]
pub struct FileDecoder {
    header: Peekable<TupleDecoder<(HeaderDecoder, U32beDecoder)>>,
    tag: MaybeEos<TagDecoder>,
    prev_tag_size: U32beDecoder,
}
impl FileDecoder {
    /// Makes a new `FileDecoder` instance.
    pub fn new() -> Self {
        FileDecoder::default()
    }

    /// Returns the header of the FLV file.
    ///
    /// If the header has not been decoded yet, it will return `None`.
    pub fn header(&self) -> Option<&Header> {
        self.header.peek().map(|t| &t.0)
    }
}
impl Decode for FileDecoder {
    type Item = Tag;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        if !self.header.is_idle() {
            bytecodec_try_decode!(self.header, offset, buf, eos);

            let prev_tag_size = self.header.peek().map(|t| t.1);
            track_assert_eq!(prev_tag_size, Some(0), ErrorKind::InvalidInput);
        }
        bytecodec_try_decode!(self.tag, offset, buf, eos);
        bytecodec_try_decode!(self.prev_tag_size, offset, buf, eos);
        Ok(offset)
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let tag = track!(self.tag.finish_decoding())?;
        let prev_tag_size = track!(self.prev_tag_size.finish_decoding())?;
        track_assert_eq!(tag.tag_size(), prev_tag_size, ErrorKind::InvalidInput; tag.kind());
        Ok(tag)
    }

    fn is_idle(&self) -> bool {
        self.tag.is_idle() && self.prev_tag_size.is_idle()
    }

    fn requiring_bytes(&self) -> ByteCount {
        self.header
            .requiring_bytes()
            .add_for_decoding(self.tag.requiring_bytes())
            .add_for_decoding(self.prev_tag_size.requiring_bytes())
    }
}
