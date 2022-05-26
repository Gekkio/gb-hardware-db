// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use std::{fmt, str};

use crate::ParseError;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Sha256([u8; 32]);

impl Sha256 {
    pub fn parse(text: &str) -> Result<Sha256, ParseError> {
        if text.len() != 64 {
            return Err(ParseError("invalid SHA256"));
        }
        let bytes = text.as_bytes().chunks(2).map(|chunk| {
            let string = str::from_utf8(chunk).ok()?;
            u8::from_str_radix(string, 16).ok()
        });
        let mut result = Sha256([0; 32]);
        for (byte, parsed) in result.0.iter_mut().zip(bytes) {
            *byte = parsed.ok_or(ParseError("invalid SHA256"))?;
        }
        Ok(result)
    }
}

impl fmt::Display for Sha256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Serialize for Sha256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for Sha256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Sha256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("hex-formatted SHA256")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Sha256::parse(v).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
