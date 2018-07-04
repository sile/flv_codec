extern crate flv_codec;
#[macro_use]
extern crate trackable;

use flv_codec::FlvHeaderDecoder;
use trackable::error::MainError;

fn main() -> Result<(), MainError> {
    track!(FlvHeaderDecoder::new())?;
    track_any_err!(std::fs::File::open("foo"))?;
    Ok(())
}
