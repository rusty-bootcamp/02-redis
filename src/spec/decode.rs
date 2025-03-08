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

#[cfg(test)]
mod tests {
    use bytes::BufMut;

    use super::*;

    #[test]
    fn test_simple_string_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::from("+OK\r\n");
        let simple_string = SimpleString::decode(&mut buf)?;
        assert_eq!(simple_string, SimpleString::new("OK".to_string()));

        buf.extend_from_slice(b"+hello\r");
        let simple_string = SimpleString::decode(&mut buf);
        assert_eq!(simple_string.unwrap_err(), RespError::NotComplete);

        buf.put_u8(b'\n');
        let simple_string = SimpleString::decode(&mut buf)?;
        assert_eq!(simple_string, SimpleString::new("hello".to_string()));

        Ok(())
    }
}
