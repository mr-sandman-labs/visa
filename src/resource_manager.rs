use crate::{
    error::*,
    session::Session,
    utility::{AccessMode, Timeout, stringify_buffer},
};
use std::ffi::CString;
use tracing::{debug, error};
use visa_bindings::*;

#[derive(Debug)]
pub struct ResourceManager {
    value: ViSession,
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        let result = unsafe { CompletionCode::try_from(viClose(self.value)) };
        match result {
            Ok(completion_code) => debug!(
                "Default Resource Manager was closed with completion code: {}",
                completion_code
            ),
            Err(error) => error!("Default Resource Manager failed during closing: {}", error),
        }
    }
}

impl ResourceManager {
    pub fn new() -> Result<Self> {
        let mut session: ViSession = 0;
        let completion_code = unsafe { CompletionCode::try_from(viOpenDefaultRM(&mut session))? };
        debug!(
            "Default Resource Manager initialized with completion code: {}",
            completion_code
        );
        Ok(Self { value: session })
    }

    pub fn open_session<T: AsRef<str>>(
        &self,
        resource_name: T,
        access_mode: AccessMode,
        timeout: Timeout,
    ) -> Result<Session> {
        let mut session: ViSession = 0;
        let resource_name =
            CString::new(resource_name.as_ref()).map_err(|_| Error::InvalidResourceName)?;
        let completion_code = unsafe {
            CompletionCode::try_from(viOpen(
                self.value,
                resource_name.as_ptr(),
                access_mode.into(),
                timeout.try_into()?,
                &mut session,
            ))?
        };
        debug!(
            "Resource {:?} opened with completion code: {}",
            resource_name, completion_code
        );

        Ok(Session::new(session))
    }

    /// Finds all instruments that match the expression.
    ///
    /// | Special Characters and Operators | Meaning                                                                                                                                                                                                                                                |
    /// | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    /// | ?                                | Matches any one character.                                                                                                                                                                                                                             |
    /// | \                                | Makes the character that follows it an ordinary character instead of special character. For example, when a question mark follows a backslash (\?), it matches the ? character instead of any one character.                                           |
    /// | \[list\]                         | Matches any one character from the enclosed list. You can use a hyphen to match a range of characters.                                                                                                                                                 |
    /// | [^list]                          | Matches any character not in the enclosed list. You can use a hyphen to match a range of characters.                                                                                                                                                   |
    /// | *                                | Matches 0 or more occurrences of the preceding character or expression.                                                                                                                                                                                |
    /// | +                                | Matches 1 or more occurrences of the preceding character or expression.                                                                                                                                                                                |
    /// | Exp\|exp                         | Matches either the preceding or following expression. The or operator \| matches the entire expression that precedes or follows it and not just the character that precedes or follows it. For example, VXI\|GPIB means (VXI)\|(GPIB), not VX(I\|G)PIB.|
    /// | (exp)                            | Grouping characters or expressions.                                                                                                                                                                                                                    |
    pub fn find_resources<T: AsRef<str>>(&self, expression: T) -> Result<Vec<String>> {
        let mut list: ViFindList = 0;
        let mut count: ViUInt32 = 0;
        let mut description = [0u8; VI_FIND_BUFLEN as _];

        let expression = CString::new(expression.as_ref()).map_err(|_| Error::InvalidNullString)?;

        let completion_code = unsafe {
            CompletionCode::try_from(viFindRsrc(
                self.value,
                expression.as_ptr() as _,
                &mut list,
                &mut count,
                description.as_mut_ptr() as _,
            ))?
        };
        debug!("Found resources with completion code: {}", completion_code);

        let mut resources = vec![];

        if count < 1 {
            return Ok(resources);
        }

        let resource = stringify_buffer(&description)?;
        resources.push(resource);

        for _ in 1..count {
            let completion_code = unsafe {
                CompletionCode::try_from(viFindNext(list, description.as_mut_ptr() as _))?
            };
            debug!(
                "Found next resource with completion code: {}",
                completion_code
            );
            let resource = stringify_buffer(&description)?;
            resources.push(resource);
        }

        Ok(resources)
    }
}
