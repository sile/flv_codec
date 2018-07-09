use bytecodec::combinator::{MaybeEos, Peekable};
use bytecodec::fixnum::U32beDecoder;
use bytecodec::{ByteCount, Decode, Eos, ErrorKind, Result};

use header::{Header, HeaderDecoder};
use tag::{Tag, TagDecoder};

/// FLV file decoder.
///
/// See the [specification] about the format of FLV file.
///
/// [specification]: https://wwwimages2.adobe.com/content/dam/acom/en/devnet/flv/video_file_format_spec_v10.pdf
#[derive(Debug, Default)]
pub struct FileDecoder {
    header: Peekable<HeaderDecoder>,
    tag: MaybeEos<TagDecoder>,
    prev_tag_size: U32beDecoder,
    is_first_prev_tag_size_read: bool,
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
        self.header.peek()
    }
}
impl Decode for FileDecoder {
    type Item = Tag;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        if !self.is_first_prev_tag_size_read {
            bytecodec_try_decode!(self.header, offset, buf, eos);
            bytecodec_try_decode!(self.prev_tag_size, offset, buf, eos);

            self.is_first_prev_tag_size_read = true;
            let prev_tag_size = track!(self.prev_tag_size.finish_decoding())?;
            track_assert_eq!(prev_tag_size, 0, ErrorKind::InvalidInput);
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
        let size = self.tag
            .requiring_bytes()
            .add_for_decoding(self.prev_tag_size.requiring_bytes());
        if self.is_first_prev_tag_size_read {
            size
        } else {
            size.add_for_decoding(self.header.requiring_bytes())
                .add_for_decoding(self.prev_tag_size.requiring_bytes())
        }
    }
}
