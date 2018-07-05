extern crate bytecodec;
extern crate flv_codec;
#[macro_use]
extern crate trackable;

use bytecodec::io::IoDecodeExt;
use bytecodec::Error;
use flv_codec::{FlvBodyDecoder, FlvHeaderDecoder};
use std::io::Read;
use trackable::error::MainError;

fn main() -> Result<(), MainError> {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    track!(parse_header(&mut input))?;
    track!(parse_tags(&mut input))?;
    Ok(())
}

fn parse_header<R: Read>(mut input: R) -> Result<(), Error> {
    let mut decoder = FlvHeaderDecoder::new();
    let header = track!(decoder.decode_exact(&mut input))?;
    println!("[header]");
    println!("has_audio = {}", header.has_audio);
    println!("has_video = {}", header.has_video);
    println!("");
    Ok(())
}

fn parse_tags<R: Read>(mut input: R) -> Result<(), Error> {
    let mut decoder = FlvBodyDecoder::new();
    loop {
        let mut peek = [0];
        let size = track!(input.read(&mut peek).map_err(Error::from))?;
        if size == 0 {
            break;
        }
        let tag = track!(decoder.decode_exact(peek.chain(&mut input)))?;
        println!("[[tags]]");
        println!("type = {:?}", tag.tag_type());
        println!("timestamp = {:?}", tag.timestamp());
        println!("stream_id = {:?}", tag.stream_id());
        println!("");
    }
    Ok(())
}
