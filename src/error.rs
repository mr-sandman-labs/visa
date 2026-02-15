use visa_bindings::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown system error (miscellaneous error).")]
    System,
    #[error("The given session or object reference is invalid.")]
    InvalidObject,
    #[error(
        "Specified type of lock cannot be obtained or specified operation cannot be performed, because the resource is locked."
    )]
    ResourceLocked,
    #[error("Invalid expression specified for search.")]
    InvalidExpression,
    #[error(
        "Insufficient location information or the device or resource is not present in the system."
    )]
    ResourceNotFound,
    #[error("Invalid resource reference specified. Parsing error.")]
    InvalidResourceName,
    #[error("Invalid access mode.")]
    InvalidAccessMode,
    #[error("Timeout expired before operation completed.")]
    Timeout,
    #[error(
        "Unable to deallocate the previously allocated data structures corresponding to this session or object reference."
    )]
    ClosingFailed,
    #[error("Specified degree is invalid.")]
    InvalidDegree,
    #[error("Specified job identifier is invalid.")]
    InvalidJobId,
    #[error(
        "The specified attribute is not defined or supported by the referenced session, event, or find list."
    )]
    AttributeNotSupported,
    #[error(
        "The specified state of the attribute is not valid, or is not supported as defined by the session, event, or find list."
    )]
    AttributeStateNotSupported,
    #[error("The specified attribute is Read Only.")]
    AttibuteReadOnly,
    #[error("The specified type of lock is not supported by this resource.")]
    InvalidLockType,
    #[error("The access key to the resource associated with this session is invalid.")]
    InvalidAccessKey,
    #[error("Specified event type is not supported by the resource.")]
    InvalidEvent,
    #[error("Invalid mechanism specified.")]
    InvalidMechanism,
    #[error("A handler is not currently installed for the specified event.")]
    HandlerNotInstalled,
    #[error("The given handler reference is invalid.")]
    InvalidHandlerReference,
    #[error("Specified event context is invalid.")]
    InvalidContext,
    #[error(
        "The event queue for the specified type has overflowed (usually due to previous events not having been closed)."
    )]
    QueueOverflow,
    #[error(
        "The session must be enabled for events of the specified type in order to receive them."
    )]
    NotEnabled,
    #[error("The operation was aborted.")]
    Abort,
    #[error("Violation of raw write protocol occurred during transfer.")]
    RawWriteProtocolViolation,
    #[error("Violation of raw write protocol occurred during transfer.")]
    RawReadProtocolViolation,
    #[error("Device reported an output protocol error during transfer.")]
    OutputProtocolViolation,
    #[error("Device reported an input protocol error during transfer.")]
    InputProtocolViolation,
    #[error("Bus error occurred during transfer.")]
    Bus,
    #[error(
        "Unable to queue the asynchronous operation because there is already an operation in progress."
    )]
    InProgress,
    #[error(
        "Unable to start operation because setup is invalid (due to attributes being set to an inconsistent state)."
    )]
    InvalidSetup,
    #[error(
        "Unable to queue asynchronous operation (usually due to the I/O completion event not being enabled or insufficient space in the session's queue)."
    )]
    Queue,
    #[error("Insufficient system resources to perform necessary memory allocation.")]
    Allocation,
    #[error("Invalid buffer mask specified.")]
    InvalidMask,
    #[error("Could not perform operation because of I/O error.")]
    Io,
    #[error("A format specifier in the format string is invalid.")]
    InvalidFormat,
    #[error("A format specifier in the format string is not supported.")]
    FormatNotSupported,
    #[error("The specified trigger line is currently in use.")]
    TriggerLineInUse,
    #[error("The specified mode is not supported by this VISA implementation.")]
    ModeNotSupported,
    #[error("Service request has not been received for the session.")]
    ServiceRequestNotReceived,
    #[error("Invalid address space specified.")]
    InvalidAddressSpace,
    #[error("Invalid offset specified.")]
    InvalidOffset,
    #[error("Invalid source or destination width specified.")]
    InvalidWidth,
    #[error("Specified offset is not accessible from this hardware.")]
    OffsetNotAccessible,
    #[error("Cannot support source and destination widths that are different.")]
    VariableWidthNotSupported,
    #[error("The specified session is not currently mapped.")]
    SessionNotMapped,
    #[error("A previous response is still pending, causing a multiple query error.")]
    ResponsePending,
    #[error("No Listeners condition is detected (both NRFD and NDAC are deasserted).")]
    NoListeners,
    #[error(
        "The interface associated with this session is not currently the controller in charge."
    )]
    NotControllerInCharge,
    #[error("The interface associated with this session is not the system controller.")]
    NotSystemController,
    #[error("The given session or object reference does not support this operation.")]
    OperationNotSupported,
    #[error("An interrupt is still pending from a previous call.")]
    InterruptPending,
    #[error("A parity error occurred during transfer.")]
    AsrlParity,
    #[error("A framing error occurred during transfer.")]
    AsrlFraming,
    #[error(
        "An overrun error occurred during transfer. A character was not read from the hardware before the next character arrived."
    )]
    AsrlOverrun,
    #[error("The path from trigSrc to trigDest is not currently mapped.")]
    TriggerNotMapped,
    #[error("The specified offset is not properly aligned for the access width of the operation.")]
    OffsetNotAligned,
    #[error("A specified user buffer is not valid or cannot be accessed for the required size.")]
    UserBuffer,
    #[error("The resource is valid, but VISA cannot currently access it.")]
    ResourceBusy,
    #[error("Specified width is not supported by this hardware.")]
    WidthNotSupported,
    #[error("The value of some parameter—which parameter is not known—is invalid.")]
    InvalidParameter,
    #[error("The protocol specified is invalid.")]
    InvalidProtocol,
    #[error("Invalid size of window specified.")]
    InvalidSize,
    #[error("The specified session currently contains a mapped window.")]
    WindowMapped,
    #[error("The given operation is not implemented.")]
    OperationNotImplemented,
    #[error("Invalid length specified.")]
    InvalidLength,
    #[error("The specified mode is invalid.")]
    InvalidMode,
    #[error("The current session did not have any lock on the resource.")]
    SessionNotLocked,
    #[error("The device does not export any memory.")]
    MemoryNotShared,
    #[error("A code library required by VISA could not be located or loaded.")]
    LibraryNotFound,
    #[error(
        "The interface cannot generate an interrupt on the requested level or with the requested statusID value."
    )]
    InterruptNotSupported,
    #[error("The value specified by the line parameter is invalid.")]
    InvalidLine,
    #[error(
        "An error occurred while trying to open the specified file. Possible reasons include an invalid path or lack of access rights."
    )]
    FileAccess,
    #[error("An error occurred while performing I/O on the specified file.")]
    FileIo,
    #[error(
        "One of the specified lines (trigSrc or trigDest) is not supported by this VISA implementation, or the combination of lines is not a valid mapping."
    )]
    LineNotSupported,
    #[error("The specified mechanism is not supported for the given event type.")]
    MechanismNotSupported,
    #[error("The interface type is valid but the specified interface number is not configured.")]
    InterfaceNumberNotConfigured,
    #[error("The connection for the given session has been lost.")]
    ConnectionLost,
    #[error("The remote machine does not exist or is not accepting any connections.")]
    MachineNotAvailable,
    #[error(
        "Access to the resource or remote machine is denied. This is due to lack of sufficient privileges for the current user or machine."
    )]
    NoPermission,
    #[error("Invalid error code supplied from VISA: {0}.")]
    InvalidErrorCode(i32),
    #[error("Invalid completion code supplied from VISA: {0}.")]
    InvalidCompletionCode(u32),
    #[error("Invalid timeout value: {}.", &0)]
    InvalidTimeout(std::time::Duration),
    #[error("Write command wrote {length} bytes instead of {expected}.")]
    WriteLengthMistmatch { length: usize, expected: usize },
    #[error("The buffer contains invalid UTF-8 characters.")]
    InvalidUtf8,
    #[error("The string is not null terminated.")]
    InvalidNullString,
    #[error("Invalid identity, the parsed string had more than 4 fields: {0}.")]
    IdentityParse(String),
    #[error("Invalid Standard Event Status Register (SESR) response: {0}.")]
    StandardEventStatusRegisterParse(String),
    #[error("Invalid Standard Event Status Enable Register (SESER) response : {0}.")]
    StandardEventStatusEnableRegisterParse(String),
    #[error("Invalid Operation Complete (*OPC?) response: {0}.")]
    OperationCompleteQueryParse(String),
    #[error("Invalid Status Byte Register (*STB?) response: {0}.")]
    StatusByteRegisterQueryParse(String),
    #[error("Invalid Self Test (*TST?) response: {0}.")]
    SelfTestParse(String),
    #[error("Invalid Service Request Enable (*SRE?) response: {0}.")]
    ServiceRequestEnableQueryParse(String),
    #[error("Unexpected completion code: {0}")]
    UnexpectedCompletionCode(CompletionCode),
}

#[derive(Debug)]
pub enum CompletionCode {
    Success,
    EventEnabled,
    EventDisabled,
    QueueEmpty,
    TerminationCharacterRead,
    MaximumCount,
    DeviceNotPresent,
    TrigPathMapped,
    QueueNotEmpty,
    DoNotInvokeHandler,
    NestedSharedLock,
    NestedExclusiveLock,
    AsynchronousOperationHandledSynchronously,
    QueueOverflow,
    ConfigurationNotLoaded,
    NullObject,
    AttributeStateNotSupported,
    UnknownStatus,
    BufferNotSupported,
    ExtendedFunctionNotImplemented,
}

impl std::fmt::Display for CompletionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "Operation completed successfully."),
            Self::EventEnabled => write!(
                f,
                "Specified event is already enabled for at least one of the specified mechanisms."
            ),
            Self::EventDisabled => write!(
                f,
                "Specified event is already disabled for at least one of the specified mechanisms."
            ),
            Self::QueueEmpty => write!(
                f,
                "Operation completed successfully, but queue was already empty."
            ),
            Self::TerminationCharacterRead => {
                write!(f, "The specified termination character was read.")
            }
            Self::MaximumCount => {
                write!(f, "The number of bytes read is equal to the input count.")
            }
            Self::DeviceNotPresent => write!(
                f,
                "Session opened successfully, but the device at the specified address is not responding."
            ),
            Self::TrigPathMapped => {
                write!(f, "The path from trigSrc to trigDest is already mapped.")
            }
            Self::QueueNotEmpty => write!(
                f,
                "Wait terminated successfully on receipt of an event notification. There is still at least one more event occurrence of the requested type(s) available for this session."
            ),
            Self::DoNotInvokeHandler => write!(
                f,
                "Event handled successfully. Do not invoke any other handlers on this session for this event."
            ),
            Self::NestedSharedLock => write!(
                f,
                "Operation completed successfully, and this session has nested shared locks."
            ),
            Self::NestedExclusiveLock => write!(
                f,
                "Operation completed successfully, and this session has nested exclusive locks."
            ),
            Self::AsynchronousOperationHandledSynchronously => write!(
                f,
                "Asynchronous operation request was actually performed synchronously."
            ),
            Self::QueueOverflow => write!(
                f,
                "The event returned is valid. One or more events that occurred have not been raised because there was no room available on the queue at the time of their occurrence. This could happen because VI_ATTR_MAX_QUEUE_LENGTH is not set to a large enough value for your application and/or events are coming in faster than you are servicing them."
            ),
            Self::ConfigurationNotLoaded => write!(
                f,
                "The specified configuration either does not exist or could not be loaded; using VISA-specified defaults."
            ),
            Self::NullObject => write!(f, "The specified object reference is uninitialized."),
            Self::AttributeStateNotSupported => write!(
                f,
                "Although the specified state of the attribute is valid, it is not supported by this resource implementation."
            ),
            Self::UnknownStatus => write!(
                f,
                "The status code passed to the operation could not be interpreted."
            ),
            Self::BufferNotSupported => write!(f, "The specified buffer is not supported."),
            Self::ExtendedFunctionNotImplemented => write!(
                f,
                "The operation succeeded, but a lower level driver did not implement the extended functionality."
            ),
        }
    }
}

impl TryFrom<ViStatus> for CompletionCode {
    type Error = Error;

    fn try_from(value: ViStatus) -> std::result::Result<Self, Self::Error> {
        match value {
            VI_ERROR_SYSTEM_ERROR => Err(Self::Error::System),
            VI_ERROR_INV_OBJECT => Err(Self::Error::InvalidObject),
            VI_ERROR_RSRC_LOCKED => Err(Self::Error::ResourceLocked),
            VI_ERROR_INV_EXPR => Err(Self::Error::InvalidExpression),
            VI_ERROR_RSRC_NFOUND => Err(Self::Error::ResourceNotFound),
            VI_ERROR_INV_RSRC_NAME => Err(Self::Error::InvalidResourceName),
            VI_ERROR_INV_ACC_MODE => Err(Self::Error::InvalidAccessMode),
            VI_ERROR_TMO => Err(Self::Error::Timeout),
            VI_ERROR_CLOSING_FAILED => Err(Self::Error::ClosingFailed),
            VI_ERROR_INV_DEGREE => Err(Self::Error::InvalidDegree),
            VI_ERROR_INV_JOB_ID => Err(Self::Error::InvalidJobId),
            VI_ERROR_NSUP_ATTR => Err(Self::Error::AttributeNotSupported),
            VI_ERROR_NSUP_ATTR_STATE => Err(Self::Error::AttributeStateNotSupported),
            VI_ERROR_ATTR_READONLY => Err(Self::Error::AttibuteReadOnly),
            VI_ERROR_INV_LOCK_TYPE => Err(Self::Error::InvalidLockType),
            VI_ERROR_INV_ACCESS_KEY => Err(Self::Error::InvalidAccessKey),
            VI_ERROR_INV_EVENT => Err(Self::Error::InvalidEvent),
            VI_ERROR_INV_MECH => Err(Self::Error::InvalidMechanism),
            VI_ERROR_HNDLR_NINSTALLED => Err(Self::Error::HandlerNotInstalled),
            VI_ERROR_INV_HNDLR_REF => Err(Self::Error::InvalidHandlerReference),
            VI_ERROR_INV_CONTEXT => Err(Self::Error::InvalidContext),
            VI_ERROR_QUEUE_OVERFLOW => Err(Self::Error::QueueOverflow),
            VI_ERROR_NENABLED => Err(Self::Error::NotEnabled),
            VI_ERROR_ABORT => Err(Self::Error::Abort),
            VI_ERROR_RAW_WR_PROT_VIOL => Err(Self::Error::RawWriteProtocolViolation),
            VI_ERROR_RAW_RD_PROT_VIOL => Err(Self::Error::RawReadProtocolViolation),
            VI_ERROR_OUTP_PROT_VIOL => Err(Self::Error::OutputProtocolViolation),
            VI_ERROR_INP_PROT_VIOL => Err(Self::Error::InputProtocolViolation),
            VI_ERROR_BERR => Err(Self::Error::Bus),
            VI_ERROR_IN_PROGRESS => Err(Self::Error::InProgress),
            VI_ERROR_INV_SETUP => Err(Self::Error::InvalidSetup),
            VI_ERROR_QUEUE_ERROR => Err(Self::Error::Queue),
            VI_ERROR_ALLOC => Err(Self::Error::Allocation),
            VI_ERROR_INV_MASK => Err(Self::Error::InvalidMask),
            VI_ERROR_IO => Err(Self::Error::Io),
            VI_ERROR_INV_FMT => Err(Self::Error::InvalidFormat),
            VI_ERROR_NSUP_FMT => Err(Self::Error::FormatNotSupported),
            VI_ERROR_LINE_IN_USE => Err(Self::Error::TriggerLineInUse),
            VI_ERROR_NSUP_MODE => Err(Self::Error::ModeNotSupported),
            VI_ERROR_SRQ_NOCCURRED => Err(Self::Error::ServiceRequestNotReceived),
            VI_ERROR_INV_SPACE => Err(Self::Error::InvalidAddressSpace),
            VI_ERROR_INV_OFFSET => Err(Self::Error::InvalidOffset),
            VI_ERROR_INV_WIDTH => Err(Self::Error::InvalidWidth),
            VI_ERROR_NSUP_OFFSET => Err(Self::Error::OffsetNotAccessible),
            VI_ERROR_NSUP_VAR_WIDTH => Err(Self::Error::VariableWidthNotSupported),
            VI_ERROR_WINDOW_NMAPPED => Err(Self::Error::SessionNotMapped),
            VI_ERROR_RESP_PENDING => Err(Self::Error::ResponsePending),
            VI_ERROR_NLISTENERS => Err(Self::Error::NoListeners),
            VI_ERROR_NCIC => Err(Self::Error::NotControllerInCharge),
            VI_ERROR_NSYS_CNTLR => Err(Self::Error::NotSystemController),
            VI_ERROR_NSUP_OPER => Err(Self::Error::OperationNotSupported),
            VI_ERROR_INTR_PENDING => Err(Self::Error::InterruptPending),
            VI_ERROR_ASRL_PARITY => Err(Self::Error::AsrlParity),
            VI_ERROR_ASRL_FRAMING => Err(Self::Error::AsrlFraming),
            VI_ERROR_ASRL_OVERRUN => Err(Self::Error::AsrlOverrun),
            VI_ERROR_TRIG_NMAPPED => Err(Self::Error::TriggerNotMapped),
            VI_ERROR_NSUP_ALIGN_OFFSET => Err(Self::Error::OffsetNotAligned),
            VI_ERROR_USER_BUF => Err(Self::Error::UserBuffer),
            VI_ERROR_RSRC_BUSY => Err(Self::Error::ResourceBusy),
            VI_ERROR_NSUP_WIDTH => Err(Self::Error::WidthNotSupported),
            VI_ERROR_INV_PARAMETER => Err(Self::Error::InvalidParameter),
            VI_ERROR_INV_PROT => Err(Self::Error::InvalidProtocol),
            VI_ERROR_INV_SIZE => Err(Self::Error::InvalidSize),
            VI_ERROR_WINDOW_MAPPED => Err(Self::Error::WindowMapped),
            VI_ERROR_NIMPL_OPER => Err(Self::Error::OperationNotImplemented),
            VI_ERROR_INV_LENGTH => Err(Self::Error::InvalidLength),
            VI_ERROR_INV_MODE => Err(Self::Error::InvalidMode),
            VI_ERROR_SESN_NLOCKED => Err(Self::Error::SessionNotLocked),
            VI_ERROR_MEM_NSHARED => Err(Self::Error::MemoryNotShared),
            VI_ERROR_LIBRARY_NFOUND => Err(Self::Error::LibraryNotFound),
            VI_ERROR_NSUP_INTR => Err(Self::Error::InterruptNotSupported),
            VI_ERROR_INV_LINE => Err(Self::Error::InvalidLine),
            VI_ERROR_FILE_ACCESS => Err(Self::Error::FileAccess),
            VI_ERROR_FILE_IO => Err(Self::Error::FileIo),
            VI_ERROR_NSUP_LINE => Err(Self::Error::LineNotSupported),
            VI_ERROR_NSUP_MECH => Err(Self::Error::MechanismNotSupported),
            VI_ERROR_INTF_NUM_NCONFIG => Err(Self::Error::InterfaceNumberNotConfigured),
            VI_ERROR_CONN_LOST => Err(Self::Error::ConnectionLost),
            VI_ERROR_MACHINE_NAVAIL => Err(Self::Error::MachineNotAvailable),
            VI_ERROR_NPERMISSION => Err(Self::Error::NoPermission),
            other => {
                if other < 0 {
                    return Err(Self::Error::InvalidErrorCode(other));
                }
                match other as u32 {
                    VI_SUCCESS => Ok(Self::Success),
                    VI_SUCCESS_EVENT_EN => Ok(Self::EventEnabled),
                    VI_SUCCESS_EVENT_DIS => Ok(Self::EventDisabled),
                    VI_SUCCESS_QUEUE_EMPTY => Ok(Self::QueueEmpty),
                    VI_SUCCESS_TERM_CHAR => Ok(Self::TerminationCharacterRead),
                    VI_SUCCESS_MAX_CNT => Ok(Self::MaximumCount),
                    VI_SUCCESS_DEV_NPRESENT => Ok(Self::DeviceNotPresent),
                    VI_SUCCESS_TRIG_MAPPED => Ok(Self::TrigPathMapped),
                    VI_SUCCESS_QUEUE_NEMPTY => Ok(Self::QueueNotEmpty),
                    VI_SUCCESS_NCHAIN => Ok(Self::DoNotInvokeHandler),
                    VI_SUCCESS_NESTED_SHARED => Ok(Self::NestedSharedLock),
                    VI_SUCCESS_NESTED_EXCLUSIVE => Ok(Self::NestedExclusiveLock),
                    VI_SUCCESS_SYNC => Ok(Self::AsynchronousOperationHandledSynchronously),
                    VI_WARN_QUEUE_OVERFLOW => Ok(Self::QueueOverflow),
                    VI_WARN_CONFIG_NLOADED => Ok(Self::ConfigurationNotLoaded),
                    VI_WARN_NULL_OBJECT => Ok(Self::NullObject),
                    VI_WARN_NSUP_ATTR_STATE => Ok(Self::AttributeStateNotSupported),
                    VI_WARN_UNKNOWN_STATUS => Ok(Self::UnknownStatus),
                    VI_WARN_NSUP_BUF => Ok(Self::BufferNotSupported),
                    VI_WARN_EXT_FUNC_NIMPL => Ok(Self::ExtendedFunctionNotImplemented),
                    other => Err(Error::InvalidCompletionCode(other)),
                }
            }
        }
    }
}
