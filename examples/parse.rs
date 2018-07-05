extern crate bytecodec;
extern crate flv_codec;
#[macro_use]
extern crate trackable;

use bytecodec::io::IoDecodeExt;
use flv_codec::FlvHeaderDecoder;
use trackable::error::MainError;

fn main() -> Result<(), MainError> {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();

    let mut decoder = FlvHeaderDecoder::new();
    let header = track!(decoder.decode_exact(&mut input))?;
    println!("# {:?}", header);

    Ok(())
}
