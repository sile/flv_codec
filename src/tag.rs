use bytecodec::bytes::RemainingBytesDecoder;
use bytecodec::combinator::{Length, Peekable};
use bytecodec::fixnum::{U24beDecoder, U8Decoder};
use bytecodec::{ByteCount, Decode, DecodeExt, Eos, ErrorKind, Result};

use {
    AacPacketType, AvcPacketType, CodecId, FrameType, SoundFormat, SoundRate, SoundSize, SoundType,
    StreamId, TimeOffset, Timestamp,
};

const TAG_TYPE_AUDIO: u8 = 8;
const TAG_TYPE_VIDEO: u8 = 9;
const TAG_TYPE_SCRIPT_DATA: u8 = 18;

/// FLV tag.
#[derive(Debug, Clone)]
pub enum Tag {
    /// Audio tag.
    Audio(AudioTag),

    /// Video tag.
    Video(VideoTag),

    /// Script data tag.
    ScriptData(ScriptDataTag),
}
impl Tag {
    /// Returns the kind of the tag.
    pub fn kind(&self) -> TagKind {
        match self {
            Tag::Audio(_) => TagKind::Audio,
            Tag::Video(_) => TagKind::Video,
            Tag::ScriptData(_) => TagKind::ScriptData,
        }
    }

    /// Returns the timestamp of the tag.
    pub fn timestamp(&self) -> Timestamp {
        match self {
            Tag::Audio(t) => t.timestamp,
            Tag::Video(t) => t.timestamp,
            Tag::ScriptData(t) => t.timestamp,
        }
    }

    /// Returns the stream identifier of the tag.
    pub fn stream_id(&self) -> StreamId {
        match self {
            Tag::Audio(t) => t.stream_id,
            Tag::Video(t) => t.stream_id,
            Tag::ScriptData(t) => t.stream_id,
        }
    }

    /// Returns the number of bytes required to encode this tag.
    pub fn tag_size(&self) -> u32 {
        match self {
            Tag::Audio(t) => t.tag_size(),
            Tag::Video(t) => t.tag_size(),
            Tag::ScriptData(t) => t.tag_size(),
        }
    }
}
impl From<AudioTag> for Tag {
    fn from(f: AudioTag) -> Self {
        Tag::Audio(f)
    }
}
impl From<VideoTag> for Tag {
    fn from(f: VideoTag) -> Self {
        Tag::Video(f)
    }
}
impl From<ScriptDataTag> for Tag {
    fn from(f: ScriptDataTag) -> Self {
        Tag::ScriptData(f)
    }
}

/// Tag kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum TagKind {
    Audio = TAG_TYPE_AUDIO as isize,
    Video = TAG_TYPE_VIDEO as isize,
    ScriptData = TAG_TYPE_SCRIPT_DATA as isize,
}

/// Audio tag.
#[derive(Debug, Clone)]
pub struct AudioTag {
    /// Timestamp.
    pub timestamp: Timestamp,

    /// Stream identifier.
    pub stream_id: StreamId,

    /// Sound format.
    pub sound_format: SoundFormat,

    /// Sound rate.
    pub sound_rate: SoundRate,

    /// Sound size.
    pub sound_size: SoundSize,

    /// Sound yype.
    pub sound_type: SoundType,

    /// AAC packet type.
    ///
    /// This is only present if `sound_format == SoundFormat::Aac`.
    pub aac_packet_type: Option<AacPacketType>,

    /// Audio data.
    pub data: Vec<u8>,
}
impl AudioTag {
    /// Returns the number of bytes required to encode this tag.
    pub fn tag_size(&self) -> u32 {
        let mut size = TagHeader::SIZE + 1 + self.data.len() as u32;
        if self.aac_packet_type.is_some() {
            size += 1;
        }
        size
    }
}

/// Video tag.
#[derive(Debug, Clone)]
pub struct VideoTag {
    /// Timestamp.
    pub timestamp: Timestamp,

    /// Stream identifier.
    pub stream_id: StreamId,

    /// Frame type.
    pub frame_type: FrameType,

    /// Codec identifier.
    pub codec_id: CodecId,

    /// AAC packet type.
    ///
    /// This is only present if `codec_id == CodecId::Avc` and
    /// `frame_type != FrameType::VideoInfoOrCommandFrame`.
    pub avc_packet_type: Option<AvcPacketType>,

    /// Composition time offset.
    ///
    /// This is only present if `codec_id == CodecId::Avc` and
    /// `frame_type != FrameType::VideoInfoOrCommandFrame`.
    pub composition_time: Option<TimeOffset>,

    /// Video data.
    pub data: Vec<u8>,
}
impl VideoTag {
    /// Returns the number of bytes required to encode this tag.
    pub fn tag_size(&self) -> u32 {
        let mut size = TagHeader::SIZE + 1 + self.data.len() as u32;
        if self.avc_packet_type.is_some() {
            size += 4;
        }
        size
    }
}

/// Script data tag.
#[derive(Debug, Clone)]
pub struct ScriptDataTag {
    /// Timestamp.
    pub timestamp: Timestamp,

    /// Stream identifier.
    pub stream_id: StreamId,

    /// [AMF 0] encoded data.
    ///
    /// [AMF 0]: https://wwwimages2.adobe.com/content/dam/acom/en/devnet/pdf/amf0-file-format-specification.pdf
    pub data: Vec<u8>,
}
impl ScriptDataTag {
    /// Returns the number of bytes required to encode this tag.
    pub fn tag_size(&self) -> u32 {
        TagHeader::SIZE + self.data.len() as u32
    }
}

/// FLV tag decoder.
#[derive(Debug, Default)]
pub struct TagDecoder {
    header: Peekable<TagHeaderDecoder>,
    data: Length<TagDataDecoder>,
}
impl TagDecoder {
    /// Makes a new `TagDecoder` instance.
    pub fn new() -> Self {
        TagDecoder::default()
    }
}
impl Decode for TagDecoder {
    type Item = Tag;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        if !self.header.is_idle() {
            bytecodec_try_decode!(self.header, offset, buf, eos);
            let header = self.header.peek().expect("Never fails");
            let data = match header.tag_type {
                TagKind::Audio => TagDataDecoder::Audio(Default::default()),
                TagKind::Video => TagDataDecoder::Video(Default::default()),
                TagKind::ScriptData => TagDataDecoder::ScriptData(Default::default()),
            };
            self.data = data.length(u64::from(header.data_size));
        }
        bytecodec_try_decode!(self.data, offset, buf, eos);
        Ok(offset)
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let header = track!(self.header.finish_decoding())?;
        let data = track!(self.data.finish_decoding())?;
        let tag = match data {
            TagData::Audio(d) => Tag::from(AudioTag {
                timestamp: header.timestamp,
                stream_id: header.stream_id,
                sound_format: d.sound_format,
                sound_rate: d.sound_rate,
                sound_size: d.sound_size,
                sound_type: d.sound_type,
                aac_packet_type: d.aac_packet_type,
                data: d.data,
            }),
            TagData::Video(d) => Tag::from(VideoTag {
                timestamp: header.timestamp,
                stream_id: header.stream_id,
                frame_type: d.frame_type,
                codec_id: d.codec_id,
                avc_packet_type: d.avc_packet_type,
                composition_time: d.composition_time,
                data: d.data,
            }),
            TagData::ScriptData(d) => Tag::from(ScriptDataTag {
                timestamp: header.timestamp,
                stream_id: header.stream_id,
                data: d.data,
            }),
        };
        Ok(tag)
    }

    fn is_idle(&self) -> bool {
        self.header.is_idle() && self.data.is_idle()
    }

    fn requiring_bytes(&self) -> ByteCount {
        if self.header.is_idle() {
            self.data.requiring_bytes()
        } else {
            self.header.requiring_bytes()
        }
    }
}

#[derive(Debug)]
struct TagHeader {
    tag_type: TagKind,
    data_size: u32, // u24
    timestamp: Timestamp,
    stream_id: StreamId,
}
impl TagHeader {
    const SIZE: u32 = 11;
}

#[derive(Debug, Default)]
struct TagHeaderDecoder {
    tag_type: U8Decoder,
    data_size: U24beDecoder,
    timestamp: U24beDecoder,
    timestamp_extended: U8Decoder,
    stream_id: U24beDecoder,
}
impl Decode for TagHeaderDecoder {
    type Item = TagHeader;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        bytecodec_try_decode!(self.tag_type, offset, buf, eos);
        bytecodec_try_decode!(self.data_size, offset, buf, eos);
        bytecodec_try_decode!(self.timestamp, offset, buf, eos);
        bytecodec_try_decode!(self.timestamp_extended, offset, buf, eos);
        bytecodec_try_decode!(self.stream_id, offset, buf, eos);
        Ok(offset)
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let tag_type = track!(self.tag_type.finish_decoding())?;
        let data_size = track!(self.data_size.finish_decoding())?;
        let timestamp = track!(self.timestamp.finish_decoding())?;
        let timestamp_extended = track!(self.timestamp_extended.finish_decoding())?;
        let stream_id = track!(self.stream_id.finish_decoding())?;

        let tag_type = match tag_type {
            TAG_TYPE_AUDIO => TagKind::Audio,
            TAG_TYPE_VIDEO => TagKind::Video,
            TAG_TYPE_SCRIPT_DATA => TagKind::ScriptData,
            _ => track_panic!(
                ErrorKind::InvalidInput,
                "Unknown FLV tag type: {}",
                tag_type
            ),
        };
        track_assert!(
            data_size <= 0x00FF_FFFF,
            ErrorKind::InvalidInput,
            "Too large FLV tag data size: {}",
            data_size
        );
        let timestamp = Timestamp::new((timestamp as i32) | i32::from(timestamp_extended) << 24);
        Ok(TagHeader {
            tag_type,
            data_size,
            timestamp,
            stream_id: track!(StreamId::new(stream_id))?,
        })
    }

    fn is_idle(&self) -> bool {
        self.stream_id.is_idle()
    }

    fn requiring_bytes(&self) -> ByteCount {
        self.tag_type
            .requiring_bytes()
            .add_for_decoding(self.data_size.requiring_bytes())
            .add_for_decoding(self.timestamp.requiring_bytes())
            .add_for_decoding(self.timestamp_extended.requiring_bytes())
            .add_for_decoding(self.stream_id.requiring_bytes())
    }
}

#[derive(Debug)]
enum TagData {
    Audio(AudioTagData),
    Video(VideoTagData),
    ScriptData(ScriptDataTagData),
}

#[derive(Debug)]
struct AudioTagData {
    pub sound_format: SoundFormat,
    pub sound_rate: SoundRate,
    pub sound_size: SoundSize,
    pub sound_type: SoundType,
    pub aac_packet_type: Option<AacPacketType>,
    pub data: Vec<u8>,
}

#[derive(Debug)]
struct VideoTagData {
    frame_type: FrameType,
    codec_id: CodecId,
    avc_packet_type: Option<AvcPacketType>,
    composition_time: Option<TimeOffset>,
    data: Vec<u8>,
}

#[derive(Debug)]
struct ScriptDataTagData {
    data: Vec<u8>,
}

#[derive(Debug)]
enum TagDataDecoder {
    Audio(AudioTagDataDecoder),
    Video(VideoTagDataDecoder),
    ScriptData(ScriptDataTagDataDecoder),
    None,
}
impl Decode for TagDataDecoder {
    type Item = TagData;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        match self {
            TagDataDecoder::Audio(d) => track!(d.decode(buf, eos)),
            TagDataDecoder::Video(d) => track!(d.decode(buf, eos)),
            TagDataDecoder::ScriptData(d) => track!(d.decode(buf, eos)),
            TagDataDecoder::None => track_panic!(ErrorKind::InconsistentState),
        }
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let data = match self {
            TagDataDecoder::Audio(d) => TagData::Audio(track!(d.finish_decoding())?),
            TagDataDecoder::Video(d) => TagData::Video(track!(d.finish_decoding())?),
            TagDataDecoder::ScriptData(d) => TagData::ScriptData(track!(d.finish_decoding())?),
            TagDataDecoder::None => track_panic!(ErrorKind::InconsistentState),
        };
        *self = TagDataDecoder::None;
        Ok(data)
    }

    fn is_idle(&self) -> bool {
        match self {
            TagDataDecoder::Audio(d) => d.is_idle(),
            TagDataDecoder::Video(d) => d.is_idle(),
            TagDataDecoder::ScriptData(d) => d.is_idle(),
            TagDataDecoder::None => true,
        }
    }

    fn requiring_bytes(&self) -> ByteCount {
        match self {
            TagDataDecoder::Audio(d) => d.requiring_bytes(),
            TagDataDecoder::Video(d) => d.requiring_bytes(),
            TagDataDecoder::ScriptData(d) => d.requiring_bytes(),
            TagDataDecoder::None => ByteCount::Finite(0),
        }
    }
}
impl Default for TagDataDecoder {
    fn default() -> Self {
        TagDataDecoder::None
    }
}

#[derive(Debug, Default)]
struct AudioTagDataDecoder {
    header: Peekable<U8Decoder>,
    aac_packet_type: U8Decoder,
    data: RemainingBytesDecoder,
}
impl AudioTagDataDecoder {
    fn is_aac_packet(&self) -> bool {
        self.header.peek().map_or(false, |&b| (b >> 4) == 10)
    }
}
impl Decode for AudioTagDataDecoder {
    type Item = AudioTagData;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        bytecodec_try_decode!(self.header, offset, buf, eos);
        if self.is_aac_packet() {
            bytecodec_try_decode!(self.aac_packet_type, offset, buf, eos);
        }
        bytecodec_try_decode!(self.data, offset, buf, eos);
        Ok(offset)
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let b = track!(self.header.finish_decoding())?;
        let sound_format = track!(SoundFormat::from_u8(b >> 4))?;
        let sound_rate = track!(SoundRate::from_u8((b >> 2) & 0b11))?;
        let sound_size = SoundSize::from_bool((b & 0b10) != 0);
        let sound_type = SoundType::from_bool((b & 0b01) != 0);

        let aac_packet_type = if let SoundFormat::Aac = sound_format {
            let b = track!(self.aac_packet_type.finish_decoding())?;
            Some(track!(AacPacketType::from_u8(b))?)
        } else {
            None
        };

        let data = track!(self.data.finish_decoding())?;
        Ok(AudioTagData {
            sound_format,
            sound_rate,
            sound_size,
            sound_type,
            aac_packet_type,
            data,
        })
    }

    fn is_idle(&self) -> bool {
        self.data.is_idle()
    }

    fn requiring_bytes(&self) -> ByteCount {
        ByteCount::Unknown
    }
}

#[derive(Debug, Default)]
struct FrameTypeAndCodecDecoder(U8Decoder);
impl Decode for FrameTypeAndCodecDecoder {
    type Item = (FrameType, CodecId);

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        track!(self.0.decode(buf, eos))
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let b = track!(self.0.finish_decoding())?;
        let frame_type = track!(FrameType::from_u8(b >> 4))?;
        let codec_id = track!(CodecId::from_u8(b & 0b1111))?;
        Ok((frame_type, codec_id))
    }

    fn is_idle(&self) -> bool {
        self.0.is_idle()
    }

    fn requiring_bytes(&self) -> ByteCount {
        self.0.requiring_bytes()
    }
}

#[derive(Debug, Default)]
struct VideoTagDataDecoder {
    frame_type_and_codec: Peekable<FrameTypeAndCodecDecoder>,
    avc_packet_type: U8Decoder,
    composition_time: U24beDecoder,
    data: RemainingBytesDecoder,
}
impl VideoTagDataDecoder {
    fn is_avc_packet(&self) -> bool {
        self.frame_type_and_codec.peek().map_or(false, |t| {
            t.0 != FrameType::VideoInfoOrCommandFrame && t.1 == CodecId::Avc
        })
    }
}
impl Decode for VideoTagDataDecoder {
    type Item = VideoTagData;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        let mut offset = 0;
        bytecodec_try_decode!(self.frame_type_and_codec, offset, buf, eos);
        if self.is_avc_packet() {
            bytecodec_try_decode!(self.avc_packet_type, offset, buf, eos);
            bytecodec_try_decode!(self.composition_time, offset, buf, eos);
        }
        bytecodec_try_decode!(self.data, offset, buf, eos);
        Ok(offset)
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let is_avc_packet = self.is_avc_packet();

        let (frame_type, codec_id) = track!(self.frame_type_and_codec.finish_decoding())?;
        let avc_packet_type = if is_avc_packet {
            let avc_packet_type = track!(self.avc_packet_type.finish_decoding())?;
            let avc_packet_type = track!(AvcPacketType::from_u8(avc_packet_type))?;
            Some(avc_packet_type)
        } else {
            None
        };
        let composition_time = if is_avc_packet {
            let composition_time = track!(self.composition_time.finish_decoding())?;
            let composition_time = TimeOffset::from_u24(composition_time);
            Some(composition_time)
        } else {
            None
        };
        let data = track!(self.data.finish_decoding())?;
        Ok(VideoTagData {
            frame_type,
            codec_id,
            avc_packet_type,
            composition_time,
            data,
        })
    }

    fn is_idle(&self) -> bool {
        self.data.is_idle()
    }

    fn requiring_bytes(&self) -> ByteCount {
        ByteCount::Unknown
    }
}

#[derive(Debug, Default)]
struct ScriptDataTagDataDecoder(RemainingBytesDecoder);
impl Decode for ScriptDataTagDataDecoder {
    type Item = ScriptDataTagData;

    fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
        track!(self.0.decode(buf, eos))
    }

    fn finish_decoding(&mut self) -> Result<Self::Item> {
        let data = track!(self.0.finish_decoding())?;
        Ok(ScriptDataTagData { data })
    }

    fn is_idle(&self) -> bool {
        self.0.is_idle()
    }

    fn requiring_bytes(&self) -> ByteCount {
        self.0.requiring_bytes()
    }
}
