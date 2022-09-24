// Copyright (c) 2022 voidfield101
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt;

use byteorder::{ReadBytesExt, LittleEndian};

use crate::error::ParsingError::GenericParsingError;

use super::RiffParsable;

/**
 * Common structure of a header in a Riff file consisting of name and length of the following data.
 */
pub struct RiffHeader {
    name: [u8;4],
    length: u32
}

impl fmt::Debug for RiffHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RiffHeader").field("name", &self.get_name().unwrap_or("Unknown".to_string())).field("length", &self.length).finish()
    }
}

impl RiffParsable for RiffHeader{
    fn parse(input: &mut dyn std::io::Read, size: u32, offset: u64, parent: &str) -> Result<Self, Box<dyn std::error::Error>> 
    {
        let mut name: [u8;4] = [0;4];
        input.read_exact(&mut name)?;

        let length = input.read_u32::<LittleEndian>()?;
        return Ok(Self { name: name, length: length })
    }

    fn size(&self) -> u32 {
        8
    }
}

impl RiffHeader {

    /**
     * True for containers which will hold other chunk headers.
     * Containers are also directly followed by the header id which is not stored in the header struct.
     */
    pub fn is_container(&self) -> bool {
        self.name == "RIFF".as_bytes() || self.name == "LIST".as_bytes()
    }

    /**
     * Get the byte array for the header name (size fixed to 4)
     */
    pub fn get_name_bytes(&self) -> &[u8;4] {
        &self.name
    }

    /**
     * Gets the string for the header name (conversion may fail)
     */
    pub fn get_name(&self) -> Option<String> {
        let res = String::from_utf8(self.name.to_vec());
        if let Ok(name) = res {
            Some(name)
        }
        else {
            None
        }
    }

    /**
     * Gets the length of the data following the header
     */
    pub fn get_length(&self) -> u32 {
        self.length
    }

}