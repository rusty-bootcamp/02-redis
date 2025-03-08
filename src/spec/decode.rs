use super::{CRLF_LEN, RespDecode, RespError, SimpleString, extract_simple_frame_data};
use bytes::BytesMut;

const PREFIX: &str = "+";

impl RespDecode for SimpleString {
    fn decode(buf: &mut BytesMut) -> anyhow::Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, PREFIX)?;
        // split the buffer
        let data = buf.split_to(end + CRLF_LEN);
        let simple_string = String::from_utf8_lossy(&data[PREFIX.len()..end]);
        let simple_string = SimpleString::new(simple_string);

        Ok(simple_string)
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}
