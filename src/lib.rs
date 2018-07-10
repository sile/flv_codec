//! Decoders and encoders for [FLV] file format.
//!
//! # Examples
//!
//! ```
//! # extern crate bytecodec;
//! # extern crate flv_codec;
//! use bytecodec::io::IoDecodeExt;
//! use flv_codec::{FileDecoder, Header, Tag};
//!
//! // Reads FLV file content
//! let mut flv = &include_bytes!("../black_silent.flv")[..];
//! let mut decoder = FileDecoder::new();
//!
//! // Decodes the first FLV tag
//! let tag = decoder.decode_exact(&mut flv).unwrap();
//! let header = decoder.header().cloned().unwrap();
//! assert_eq!(header, Header { has_audio: true, has_video: true });
//! assert_eq!(tag.timestamp().value(), 0);
//! assert_eq!(tag.stream_id().value(), 0);
//! match tag {
//!     Tag::Audio(_) => println!("audio tag"),
//!     Tag::Video(_) => println!("video tag"),
//!     Tag::ScriptData(_) => println!("script data tag"),
//! }
//!
//! // Decodes the second FLV tag
//! let tag = decoder.decode_exact(&mut flv).unwrap();
//! ```
//!
//! See [examples/] directory for more examples.
//!
//! # Reference
//!
//! - [Video File Format Specification][FLV]
//!
//! [FLV]: https://wwwimages2.adobe.com/content/dam/acom/en/devnet/flv/video_file_format_spec_v10.pdf
//! [examples/]: https://github.com/sile/flv_codec/tree/master/examples
#![warn(missing_docs)]

#[macro_use]
extern crate bytecodec;
#[macro_use]
extern crate trackable;

pub use audio::{AacPacketType, SoundFormat, SoundRate, SoundSize, SoundType};
pub use file::{FileDecoder, FileEncoder};
pub use header::Header;
pub use stream::StreamId;
pub use tag::{AudioTag, ScriptDataTag, Tag, TagDecoder, TagEncoder, TagKind, VideoTag};
pub use time::{TimeOffset, Timestamp};
pub use video::{AvcPacketType, CodecId, FrameType};

mod audio;
mod file;
mod header;
mod stream;
mod tag;
mod time;
mod video;

#[cfg(test)]
mod test {
    use bytecodec::io::{IoDecodeExt, IoEncodeExt};
    use bytecodec::Encode;

    use super::*;

    #[test]
    fn it_works() {
        let mut flv = &include_bytes!("../black_silent.flv")[..];
        let mut decoder = FileDecoder::new();

        let tag = track_try_unwrap!(decoder.decode_exact(&mut flv));
        assert_eq!(
            decoder.header().cloned(),
            Some(Header {
                has_audio: true,
                has_video: true
            })
        );
        assert_eq!(tag.timestamp(), Timestamp::new(0));
        assert_eq!(tag.stream_id(), StreamId::default());
        if let Tag::ScriptData(_) = tag {
        } else {
            panic!();
        }

        let tag = track_try_unwrap!(decoder.decode_exact(&mut flv));
        assert_eq!(tag.timestamp(), Timestamp::new(0));
        assert_eq!(tag.stream_id(), StreamId::default());
        if let Tag::Audio(_) = tag {
        } else {
            panic!();
        }

        let tag = track_try_unwrap!(decoder.decode_exact(&mut flv));
        assert_eq!(tag.timestamp(), Timestamp::new(25));
        assert_eq!(tag.stream_id(), StreamId::default());
        if let Tag::Video(_) = tag {
        } else {
            panic!();
        }
    }

    #[test]
    fn file_encoder_works() {
        let mut flv = &include_bytes!("../black_silent.flv")[..];
        let mut buf = Vec::new();
        let mut decoder = FileDecoder::default();
        let mut encoder = FileEncoder::default();

        while !flv.is_empty() {
            let tag = track_try_unwrap!(decoder.decode_exact(&mut flv));
            track_try_unwrap!(encoder.start_encoding(tag));
            track_try_unwrap!(encoder.encode_all(&mut buf));
        }
        assert_eq!(buf, &include_bytes!("../black_silent.flv")[..]);
    }
}
