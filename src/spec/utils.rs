use bytes::{Buf, BytesMut};

use super::RespError;

pub const CRLF: &[u8] = b"\r\n";
pub const CRLF_LEN: usize = CRLF.len();

// find nth CRLF in the buffer
/// 查找缓冲区中的第nth个CRLF序列
fn find_crlf(buf: &[u8], nth: usize) -> Option<usize> {
    buf.windows(2)
        .enumerate()
        .filter(|(i, window)| *i < buf.len() - 1 && window[0] == b'\r' && window[1] == b'\n')
        .map(|(i, _)| i)
        .enumerate()
        .filter(|(count, _)| *count + 1 == nth)
        .map(|(_, i)| i)
        .next()
}

pub fn extract_simple_frame_data(buf: &[u8], prefix: &str) -> Result<usize, RespError> {
    if buf.len() < 3 {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "expect: SimpleString({}), got: {:?}",
            prefix, buf,
        )));
    }

    let end = find_crlf(buf, 1).ok_or(RespError::NotComplete)?;

    Ok(end)
}

pub fn extract_fixed_data(
    buf: &mut BytesMut,
    expect: &str,
    expect_type: &str,
) -> Result<(), RespError> {
    if buf.len() < expect.len() {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(expect.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "expect: {}, got: {:?}",
            expect_type, buf,
        )));
    }

    buf.advance(expect.len());

    Ok(())
}
