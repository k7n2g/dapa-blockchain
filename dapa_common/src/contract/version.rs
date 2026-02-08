use std::{fmt, str::FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::serializer::*;


#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum ContractVersion {
    #[default]
    V0,
    V1,
}

impl ContractVersion {
    #[inline(always)]
    pub const fn variants() -> [ContractVersion; 2] {
        [
            ContractVersion::V0,
            ContractVersion::V1,
        ]
    }
}

impl FromStr for ContractVersion {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v0" | "0" => Ok(ContractVersion::V0),
            "v1" | "1" => Ok(ContractVersion::V1),
            _ => Err("Invalid contract version"),
        }
    }
}


impl fmt::Display for ContractVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractVersion::V0 => write!(f, "v0"),
            ContractVersion::V1 => write!(f, "v1"),
        }
    }
}

impl Serializer for ContractVersion {
    fn write(&self, writer: &mut Writer) {
        writer.write_u8(*self as u8);
    }

    fn read(reader: &mut Reader) -> Result<Self, ReaderError> {
        match reader.read_u8()? {
            0 => Ok(ContractVersion::V0),
            1 => Ok(ContractVersion::V1),
            _ => Err(ReaderError::InvalidValue),
        }
    }

    fn size(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        assert!(ContractVersion::V0 < ContractVersion::V1);
    }
}