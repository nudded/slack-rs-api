// Copyright 2015-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::io;
use std::error;

use hyper;
use rustc_serialize;

/// slack::Error represents errors that can happen while using the RtmClient
#[derive(Debug)]
pub enum Error {
    /// Http client error
    Http(hyper::Error),
    /// Error decoding Json
    JsonDecode(rustc_serialize::json::DecoderError),
    /// Error parsing Json
    JsonParse(rustc_serialize::json::ParserError),
    /// Error encoding Json
    JsonEncode(rustc_serialize::json::EncoderError),
    /// Slack Api Error
    Api(String),
    /// Errors that do not fit under the other types, Internal is for EG channel errors.
    Internal(String),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<rustc_serialize::json::DecoderError> for Error {
    fn from(err: rustc_serialize::json::DecoderError) -> Error {
        Error::JsonDecode(err)
    }
}

impl From<rustc_serialize::json::ParserError> for Error {
    fn from(err: rustc_serialize::json::ParserError) -> Error {
        Error::JsonParse(err)
    }
}

impl From<rustc_serialize::json::EncoderError> for Error {
    fn from(err: rustc_serialize::json::EncoderError) -> Error {
        Error::JsonEncode(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Internal(format!("{:?}", err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref e) => write!(f, "Http (hyper) Error: {:?}", e),
            Error::JsonDecode(ref e) => write!(f, "Json Decode Error: {:?}", e),
            Error::JsonParse(ref e) => write!(f, "Json Parse Error: {:?}", e),
            Error::JsonEncode(ref e) => write!(f, "Json Encode Error: {:?}", e),
            Error::Api(ref st) => write!(f, "Slack Api Error: {:?}", st),
            Error::Internal(ref st) => write!(f, "Internal Error: {:?}", st)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref e) => e.description(),
            Error::JsonDecode(ref e) => e.description(),
            Error::JsonParse(ref e) => e.description(),
            Error::JsonEncode(ref e) => e.description(),
            Error::Api(ref st) => st,
            Error::Internal(ref st) => st
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Http(ref e) => Some(e),
            Error::JsonDecode(ref e) => Some(e),
            Error::JsonParse(ref e) => Some(e),
            Error::JsonEncode(ref e) => Some(e),
            Error::Api(_) => None,
            Error::Internal(_) => None
        }
    }
}