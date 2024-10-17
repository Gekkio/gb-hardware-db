use std::str;

fn parse_hash<const N: usize>(text: &str) -> Option<[u8; N]> {
    if text.len() != N * 2 {
        return None;
    }
    let bytes = text.as_bytes().chunks(2).map(|chunk| {
        let string = str::from_utf8(chunk).ok()?;
        u8::from_str_radix(string, 16).ok()
    });
    let mut result = [0; N];
    for (byte, parsed) in result.iter_mut().zip(bytes) {
        *byte = parsed?;
    }
    Some(result)
}

macro_rules! impl_hash {
    (pub struct $name:ident([u8; $n:expr]), $algo:expr) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub struct $name([u8; $n]);

        impl $name {
            pub fn parse(text: &str) -> Result<$name, crate::ParseError> {
                let bytes =
                    parse_hash(text).ok_or(crate::ParseError(concat!("invalid ", $algo)))?;
                Ok(Self(bytes))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                for byte in &self.0 {
                    write!(f, "{:02x}", byte)?;
                }
                Ok(())
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(&format!("{}", self))
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(
                        &self,
                        formatter: &mut ::std::fmt::Formatter,
                    ) -> ::std::fmt::Result {
                        formatter.write_str(concat!("hex-formatted ", $algo))
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        $name::parse(v).map_err(serde::de::Error::custom)
                    }
                }
                deserializer.deserialize_str(Visitor)
            }
        }
    };
}

impl_hash!(pub struct Crc32([u8; 4]), "CRC-32");
impl_hash!(pub struct Md5([u8; 16]), "MD5");
impl_hash!(pub struct Sha1([u8; 20]), "SHA-1");
impl_hash!(pub struct Sha256([u8; 32]), "SHA-256");
