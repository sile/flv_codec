extern crate bytecodec;
extern crate flv_codec;
#[macro_use]
extern crate trackable;

use bytecodec::io::{IoDecodeExt, ReadBuf};
use bytecodec::Decode;
use flv_codec::{FileDecoder, FrameType, Tag};
use trackable::error::MainError;

fn main() -> Result<(), MainError> {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut buf = ReadBuf::new(vec![0; 1024]);
    let mut decoder = FileDecoder::new();

    while !buf.stream_state().is_eos() {
        track!(buf.fill(&mut input))?;
        track!(decoder.decode_from_read_buf(&mut buf))?;
        if decoder.is_idle() {
            let tag = track!(decoder.finish_decoding())?;
            println!(
                "[{}] timestamp={} key={:5} size={}",
                tag_type(&tag),
                tag.timestamp().value(),
                is_key_frame(&tag),
                tag.tag_size(),
            );
        }
    }

    Ok(())
}

fn tag_type(tag: &Tag) -> &'static str {
    match tag {
        Tag::Audio(_) => "audio",
        Tag::Video(_) => "video",
        Tag::ScriptData(_) => "script_data",
    }
}

fn is_key_frame(tag: &Tag) -> bool {
    match tag {
        Tag::Audio(_) => true,
        Tag::Video(tag) => tag.frame_type == FrameType::KeyFrame,
        Tag::ScriptData(_) => false,
    }
}
