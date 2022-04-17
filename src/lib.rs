#![no_std]
#![allow(non_snake_case)]

use fugit::{Duration, ExtU64};
use modular_bitfield::*;
use modular_bitfield::specifiers::*;
use one_wire_bus::{self, Address, OneWire, OneWireError, OneWireResult};

#[derive(BitfieldSpecifier)]
#[bits = 2]
#[derive(Debug)]
pub enum ConversionResolution {
    NineBit   = 0b00,
    TenBit    = 0b01,
    ElevenBit = 0b10,
    TwelveBit = 0b11,
}

impl ConversionResolution {
    pub fn resolution_time<const NOM: u32, const DENOM: u32>(&self) -> Duration<u64, NOM, DENOM> {
        match self {
            ConversionResolution::TwelveBit => 750.millis(),
            ConversionResolution::ElevenBit => 750.millis() / 2,
            ConversionResolution::TenBit    => 750.millis() / 4,
            ConversionResolution::NineBit   => 750.millis() / 8,
        }
    }
}

#[bitfield]
#[derive(Debug)]
pub struct Configuration {
    #[skip]
    __: B1,
    #[bits = 2]
    pub conversion_resolution: ConversionResolution,
    #[skip]
    __: B5,
}

#[derive(Debug)]
pub struct ScratchPad {
    pub temperature: f32,
    pub alarm_tH_or_general_purpose_byte_1: u8,
    pub alarm_tL_or_general_purpose_byte_2: u8,
    pub configuration: Configuration,
    pub reserved_1: u8,
    pub reserved_2: u8,
    pub reserved_3: u8,
    pub crc: u8,
}

impl ScratchPad {
    pub fn is_valid(&self) -> bool {
        // TODO calculate the CRC using the first 7 bytes of the scratchpad
        unimplemented!()
    }
}

pub const FAMILY_CODE: u8 = 0x28;

pub struct DS18B20 {
    address: Address,
}

impl DS18B20 {
    pub fn new<E>(address: Address) -> OneWireResult<DS18B20, E> {
        if address.family_code() == FAMILY_CODE {
            Ok(DS18B20 { address })
        } else {
            Err(OneWireError::FamilyCodeMismatch)
        }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    // Convert T 44h
    pub fn convertT<T, E>(&self, onewire: &mut OneWire<T>, delay: &mut impl DelayUs<u16>) -> OneWireResult<(), E>
    where
        T: InputPin<Error = E>,
        T: OutputPin<Error = E>,
    {
        onewire.send_command(commands::CONVERTT, Some(&self.address), delay)?;
        Ok(())
    }

    // Write scratchpad 4Eh
    // Read scratchpad BEh
    // Copy scratchpad 48h
    // Recall E^2 B8h
    // Read power supply B4h
}

// Search ROM F0h
// Read ROM 33h
// Match ROM 55h
// Skip ROM CCh
// Alarm search ECh

