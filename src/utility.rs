use crate::{error::*, session::Session};
use bitflags::bitflags;
use std::{ffi::CStr, time::Duration};
use visa_bindings::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timeout {
    Immediate,
    Custom(Duration),
    Maximum,
    Infinite,
}

impl TryFrom<Timeout> for ViUInt32 {
    type Error = Error;
    fn try_from(value: Timeout) -> Result<Self> {
        match value {
            Timeout::Immediate => Ok(0),
            Timeout::Custom(duration) => duration
                .as_millis()
                .try_into()
                .map_err(|_| Error::InvalidTimeout(duration)),
            Timeout::Maximum => Ok(0xFFFFFFFE),
            Timeout::Infinite => Ok(0xFFFFFFFF),
        }
    }
}

impl TryFrom<Timeout> for ViAttrState {
    type Error = Error;
    fn try_from(value: Timeout) -> Result<Self> {
        let value: ViUInt32 = value.try_into()?;
        Ok(value as ViAttrState)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    Exclusive,
    Shared,
    None,
}

impl From<AccessMode> for ViAccessMode {
    fn from(value: AccessMode) -> Self {
        match value {
            AccessMode::Exclusive => VI_EXCLUSIVE_LOCK,
            AccessMode::Shared => VI_SHARED_LOCK,
            AccessMode::None => VI_NO_LOCK,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FlushMode: ViUInt16 {
        const READ_BUFFER = VI_READ_BUF as _;
        const READ_BUFFER_DISCARD = VI_READ_BUF_DISCARD as _;
        const WRITE_BUFFER = VI_WRITE_BUF as _;
        const WRITE_BUFFER_DISCARD = VI_WRITE_BUF_DISCARD as _;
        const IO_INPUT_BUFFER = VI_IO_IN_BUF as _;
        const IO_INPUT_BUFFER_DISCARD = VI_IO_IN_BUF_DISCARD as _;
        const IO_OUTPUT_BUFFER = VI_IO_OUT_BUF as _;
        const IO_OUTPUT_BUFFER_DISCARD = VI_IO_OUT_BUF as _;
    }
}

pub fn stringify_buffer(buffer: &[u8]) -> Result<String> {
    let output = buffer
        .split_inclusive(|char| *char == b'\0')
        .next()
        .ok_or(Error::InvalidNullString)?;
    let output = CStr::from_bytes_with_nul(output)
        .map_err(|_| Error::InvalidNullString)?
        .to_string_lossy()
        .to_string();
    Ok(output)
}

/// IEEE 488.2 Mandatory Commands
pub trait MandatoryCommands {
    fn as_session(&self) -> &Session;

    fn clear_status(&self) -> Result<()> {
        self.as_session().write("*CLS\n")
    }

    fn standard_event_status_enable_command(
        &self,
        register: StandardEventStatusEnableRegister,
    ) -> Result<()> {
        self.as_session()
            .write(format!("*ESE {}\n", register.value()))
    }

    fn standard_event_status_enable_query(&self) -> Result<StandardEventStatusEnableRegister> {
        let response = self.as_session().query("*ESE?\n")?;
        Ok(StandardEventStatusEnableRegister::try_from(
            response.as_str(),
        )?)
    }

    fn standard_event_status_register_query(&self) -> Result<StandardEventStatusRegister> {
        let response = self.as_session().query("*ESR?\n")?;
        Ok(StandardEventStatusRegister::try_from(response.as_str())?)
    }

    fn identification_query(&self) -> Result<Identification> {
        let response = self.as_session().query("*IDN?\n")?;
        Identification::try_from(response.as_str())
    }

    fn operation_complete_command(&self) -> Result<()> {
        self.as_session().write("*OPC\n")
    }

    fn operation_complete_query(&self) -> Result<bool> {
        let response = self.as_session().query("*OPC?\n")?;
        match response.as_str() {
            "0" => Ok(false),
            "1" => Ok(true),
            response => Err(Error::OperationCompleteQueryParse(response.to_string())),
        }
    }

    fn reset_command(&self) -> Result<()> {
        self.as_session().write("*RST\n")
    }

    fn service_request_enable_command(&self, register: ServiceRequestEnable) -> Result<()> {
        self.as_session()
            .write(format!("*SRE {}\n", register.value()))
    }

    fn service_request_enable_query(&self) -> Result<ServiceRequestEnable> {
        let response = self.as_session().query("*SRE?\n")?;
        Ok(ServiceRequestEnable::try_from(response.as_str())?)
    }

    fn read_status_byte_query(&self) -> Result<StatusByteRegister> {
        let response = self.as_session().query("*STB?\n")?;
        Ok(StatusByteRegister::try_from(response.as_str())?)
    }

    fn self_test_query(&self) -> Result<bool> {
        let response = self.as_session().query("*TST?\n")?;
        match response.as_str() {
            "0" => Ok(true),
            "1" => Ok(false),
            response => Err(Error::SelfTestParse(response.to_string())),
        }
    }

    fn wait_to_continue_command(&self) -> Result<()> {
        self.as_session().write("*WAI\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identification {
    pub manufacturer: String,
    pub model: String,
    pub serial: String,
    pub firmware: String,
}

impl TryFrom<&str> for Identification {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let parts: Vec<&str> = value.trim().split(',').collect();

        if parts.len() != 4 {
            return Err(Error::IdentityParse(value.to_string()));
        }

        Ok(Self {
            manufacturer: parts[0].to_string(),
            model: parts[1].to_string(),
            serial: parts[2].to_string(),
            firmware: parts[3].to_string(),
        })
    }
}

bitflags! {
    /// Standard Event Status Register (SESR)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StandardEventStatusRegister : u8 {
        /// Set when all pending operations have completed after `*OPC`.
        const OPERATION_COMPLETE = 1 << 0;
        /// Indicates that the device requests to become controller-in-charge.
        const REQUEST_CONTROL = 1 << 1;
        /// A query was improperly formed or the response queue overflowed.
        const QUERY_ERROR = 1 << 2;
        /// Instrument-dependent error condition.
        const DEVICE_SPECIFIC_ERROR = 1 << 3;
        /// Command could not be executed due to current instrument state.
        const EXECUTION_ERROR = 1 << 4;
        /// Syntax or semantic error in a received command.
        const COMMAND_ERROR = 1 << 5;
        /// Local control or front-panel action occurred.
        const USER_REQUEST = 1 << 6;
        /// Device power-on event detected.
        const POWER_ON = 1 << 7;
        // The source may set any bits.
        const _ = !0;
    }
}

impl TryFrom<&str> for StandardEventStatusRegister {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let value = value
            .trim()
            .parse()
            .map_err(|_| Error::StandardEventStatusRegisterParse(value.to_string()))?;

        Ok(Self::from_bits_retain(value))
    }
}

bitflags! {
    /// Standard Event Status Enable Register (SESER)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StandardEventStatusEnableRegister : u8 {
        /// Set when all pending operations have completed after `*OPC`.
        const OPERATION_COMPLETE = 1 << 0;
        /// Indicates that the device requests to become controller-in-charge.
        const REQUEST_CONTROL = 1 << 1;
        /// A query was improperly formed or the response queue overflowed.
        const QUERY_ERROR = 1 << 2;
        /// Instrument-dependent error condition.
        const DEVICE_SPECIFIC_ERROR = 1 << 3;
        /// Command could not be executed due to current instrument state.
        const EXECUTION_ERROR = 1 << 4;
        /// Syntax or semantic error in a received command.
        const COMMAND_ERROR = 1 << 5;
        /// Local control or front-panel action occurred.
        const USER_REQUEST = 1 << 6;
        /// Device power-on event detected.
        const POWER_ON = 1 << 7;
        // The source may set any bits.
        const _ = !0;
    }
}

impl TryFrom<&str> for StandardEventStatusEnableRegister {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let value = value
            .trim()
            .parse()
            .map_err(|_| Error::StandardEventStatusRegisterParse(value.to_string()))?;

        Ok(Self::from_bits_retain(value))
    }
}

impl StandardEventStatusEnableRegister {
    pub fn value(&self) -> u8 {
        self.bits()
    }
}

bitflags! {
    /// Status Byte Register (STB)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusByteRegister: u8 {
        /// Bit 3 (0x08): Questionable Status Summary (QUES).
        ///
        /// Set when one or more conditions in the Questionable Status
        /// condition register group are active.
        const QUESTIONABLE_STATUS_SUMMARY = 1 << 3;
        /// Bit 4 (0x10): Message Available (MAV).
        ///
        /// Set when the instrument has one or more response messages
        /// available in the output buffer.
        const MESSAGE_AVAILABLE = 1 << 4;
        /// Bit 5 (0x20): Event Status Bit (ESB).
        ///
        /// Set when at least one enabled event exists:
        /// (ESR & ESE) != 0.
        const EVENT_STATUS_BIT = 1 << 5;
        /// Bit 6 (0x40): Request Service / Master Summary Status (RQS/MSS).
        ///
        /// Indicates that the instrument is requesting service (SRQ).
        /// This bit reflects the SRQ state and is not directly writable.
        const REQUEST_SERVICE = 1 << 6;
        /// Bit 7 (0x80): Operation Status Summary (OPER).
        ///
        /// Set when one or more conditions in the Operation Status
        /// condition register group are active.
        const OPERATION_STATUS_SUMMARY = 1 << 7;
        // The source may set any bits.
        const _ = !0;
    }
}

impl TryFrom<&str> for StatusByteRegister {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let value = value
            .trim()
            .parse()
            .map_err(|_| Error::StatusByteRegisterQueryParse(value.to_string()))?;

        Ok(Self::from_bits_retain(value))
    }
}

bitflags! {
    /// Service Request Enable Register (SRE)
    ///
    /// Determines which bits in the Status Byte Register will
    /// generate a service request (SRQ) when set.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ServiceRequestEnable: u8 {
        /// Bit 3: Enable service request on Questionable Status
        const QUESTIONABLE_STATUS = 1 << 3;
        /// Bit 4: Enable service request on Message Available
        const MESSAGE_AVAILABLE = 1 << 4;
        /// Bit 5: Enable service request on Event Status
        const EVENT_STATUS = 1 << 5;
        // Bit 6 (RQS/MSS) is read-only and cannot be enabled
        /// Bit 7: Enable service request on Operation Status
        const OPERATION_STATUS = 1 << 7;

        const _ = !0;
    }
}

impl TryFrom<&str> for ServiceRequestEnable {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let value = value
            .trim()
            .parse()
            .map_err(|_| Error::ServiceRequestEnableQueryParse(value.to_string()))?;

        Ok(Self::from_bits_retain(value))
    }
}

impl ServiceRequestEnable {
    pub fn value(&self) -> u8 {
        self.bits()
    }
}

pub trait AsScpi {
    fn as_scpi(&self) -> String;
}
