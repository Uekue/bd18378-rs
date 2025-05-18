use strum::FromRepr;

/// The `WriteRegister` enum represents various writeable registers
/// of the ROHM BD18378 LED Driver IC, along with their corresponding hexadecimal addresses.
#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u8)]
pub enum WriteRegister {
    
    // LED current calibration registers
    ChannelCalibration00 = 0x48,
    ChannelCalibration01 = 0x49,
    ChannelCalibration02 = 0x4A,
    ChannelCalibration03 = 0x4B,
    ChannelCalibration04 = 0x4C,
    ChannelCalibration05 = 0x4D,
    ChannelCalibration06 = 0x4E,
    ChannelCalibration07 = 0x4F,
    ChannelCalibration08 = 0x50,
    ChannelCalibration09 = 0x51,
    ChannelCalibration10 = 0x52,
    ChannelCalibration11 = 0x53,
    
    // LED enable registers
    ChannelEnable00To05 = 0x56,
    ChannelEnable06To11 = 0x57,
    
    // IC reset register
    StatusReset = 0x6B,
    SoftwareReset = 0x6C,
    
    // Reserved registers used during IC initialization
    Reserved79 = 0x79,
    Reserved7A = 0x7A,
    Reserved7B = 0x7B,
    ReservedB5 = 0xB5,
    ReservedB6 = 0xB6,
    ReservedB7 = 0xB7,
    ReservedB8 = 0xB8,
    ReservedB9 = 0xB9,
}

impl TryFrom<u8> for WriteRegister {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let reg = WriteRegister::from_repr(value);
        match reg {
            Some(reg) => Ok(reg),
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ReadRegister {
    Status = 0xA8,
}
