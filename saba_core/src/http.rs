#![not_std]

extern crate alloc;

use crate::error::Error;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<Header>,
    body: String,
}

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Result<Self, Error> {
        // pending...
    }
}
