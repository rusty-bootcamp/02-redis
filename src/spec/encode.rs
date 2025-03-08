use super::{
    Array, BulkString, Map, Null, NullBulkString, RespEncode, RespFrame, Set, SimpleError,
    SimpleString,
};

impl RespEncode for RespFrame {
    fn encode(self) -> Vec<u8> {
        todo!()
    }
}

// integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self.is_negative() { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

// simple string: "+OK\r\n"
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

// simple error: "-Error message\r\n"
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

// bulk string: "$<length>\r\n<value>\r\n"
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// null bulk string: "$-1\r\n"
impl RespEncode for NullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

// null: "_\r\n"
impl RespEncode for Null {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

// array: "*<length>\r\n<element>\r\n"
impl RespEncode for Array {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("*{}\r\n", self.len()).into_bytes());
        for item in self.0 {
            buf.extend_from_slice(&item.encode());
        }
        buf
    }
}

// boolean: "#<f|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { "t" } else { "f" }).into_bytes()
    }
}

// double: "[<+|->]<integral>[.<fractional>[<E|e>[sign]<exponent>]\r\n"
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(64);
        buf.extend_from_slice(&format!(",{:+e}\r\n", self).into_bytes());
        buf
    }
}

// map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
impl RespEncode for Map {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (key, value) in self.0 {
            let encoded_key = SimpleString::new(key).encode();
            buf.extend_from_slice(&encoded_key);
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

// set: "~<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for Set {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());
        for item in self.0 {
            buf.extend_from_slice(&item.encode());
        }
        buf
    }
}
