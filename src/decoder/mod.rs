// Copyright 2017, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use std::io;
use std::fmt;
use std::error::Error;
use super::{RegKey};
use super::enums::*;
use super::winapi;

macro_rules! read_value{
    ($s:ident) => (
        match mem::replace(&mut $s.f_name, None) {
            Some(ref s) => {
                $s.key.get_value(s)
                    .map_err(DecoderError::IoError)
            },
            None => Err(DecoderError::NoFieldName)
        }
    )
}

macro_rules! parse_string{
    ($s:ident) => ({
        let s: String = try!(read_value!($s));
        s.parse().map_err(|e| DecoderError::ParseError(format!("{:?}", e)))
    })
}

macro_rules! no_impl {
    ($e:expr) => (
        Err(DecoderError::DecodeNotImplemented($e.to_owned()))
    )
}

#[cfg(feature = "serialization-rustc")] mod serialization_rustc;
#[cfg(feature = "serialization-serde")] mod serialization_serde;

#[derive(Debug)]
pub enum DecoderError{
    DecodeNotImplemented(String),
    DeserializerError(String),
    IoError(io::Error),
    ParseError(String),
    NoFieldName,
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DecoderError {
    fn description(&self) -> &str {
        match *self {
            DecoderError::DecodeNotImplemented(ref s) => s,
            DecoderError::DeserializerError(ref s) => s,
            DecoderError::IoError(ref e) => e.description(),
            DecoderError::ParseError(ref s) => s,
            DecoderError::NoFieldName => "No field name"
        }
    }
}

impl From<io::Error> for DecoderError {
    fn from(err: io::Error) -> DecoderError {
        DecoderError::IoError(err)
    }
}

pub type DecodeResult<T> = Result<T, DecoderError>;

#[derive(Debug)]
pub struct Decoder {
    key: RegKey,
    f_name: Option<String>,
}

const DECODER_SAM: winapi::DWORD = KEY_QUERY_VALUE | KEY_ENUMERATE_SUB_KEYS;

impl Decoder {
    pub fn from_key(key: &RegKey) -> DecodeResult<Decoder> {
        key.open_subkey_with_flags("", DECODER_SAM)
            .map(Decoder::new)
            .map_err(DecoderError::IoError)
    }

    fn new(key: RegKey) -> Decoder {
        Decoder{
            key: key,
            f_name: None,
        }
    }
}
