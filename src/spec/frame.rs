use anyhow::Result;
use bytes::BytesMut;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

use super::RespError;

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    const PREFIX: &'static str;
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

#[derive(Debug)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(NullBulkString),
    Array(Array),
    Null(Null),
    NullArray(NullArray),
    Boolean(bool),
    Double(f64),
    Map(Map),
    Set(Set),
}

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleString(pub String);
#[derive(Debug, PartialEq, Eq)]
pub struct SimpleError(pub String);
#[derive(Debug, PartialEq, Eq)]
pub struct Null;
#[derive(Debug, PartialEq, Eq)]
pub struct NullArray;
#[derive(Debug, PartialEq, Eq)]
pub struct NullBulkString;
#[derive(Debug, PartialEq, Eq)]
pub struct BulkString(pub Vec<u8>);
#[derive(Debug)]
pub struct Array(pub Vec<RespFrame>);
#[derive(Debug)]
pub struct Map(pub HashMap<String, RespFrame>);
#[derive(Debug)]
pub struct Set(pub HashSet<RespFrame>);

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Array {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Map {
    type Target = HashMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Set {
    type Target = HashSet<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
