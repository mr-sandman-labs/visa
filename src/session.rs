use crate::{
    error::*,
    utility::{FlushMode, MandatoryCommands, Timeout},
};
use tracing::{debug, error};
use visa_bindings::*;

#[derive(Debug)]
pub struct Session {
    value: ViSession,
}

impl Drop for Session {
    fn drop(&mut self) {
        let result = unsafe { CompletionCode::try_from(viClose(self.value)) };
        match result {
            Ok(completion_code) => debug!(
                "Session was closed with completion code: {}",
                completion_code
            ),
            Err(error) => error!("Closing session failed with code: {}", error),
        }
    }
}

impl Session {
    pub(crate) fn new(session: ViSession) -> Self {
        Self { value: session }
    }

    pub fn set_timeout(&self, timeout: Timeout) -> Result<()> {
        let completion_code = unsafe {
            CompletionCode::try_from(viSetAttribute(
                self.value,
                VI_ATTR_TMO_VALUE,
                timeout.try_into()?,
            ))?
        };
        debug!("Timeout set with completion code: {}", completion_code);
        Ok(())
    }

    pub fn write<T: AsRef<str>>(&self, command: T) -> Result<()> {
        let mut return_count: ViUInt32 = 0;
        let completion_code = unsafe {
            CompletionCode::try_from(viWrite(
                self.value,
                command.as_ref().as_ptr(),
                command.as_ref().len() as ViUInt32,
                &mut return_count,
            ))?
        };
        debug!("Write completed with code: {}", completion_code);

        if command.as_ref().len() != return_count as usize {
            return Err(Error::WriteLengthMistmatch {
                length: return_count as usize,
                expected: command.as_ref().len(),
            });
        }

        Ok(())
    }

    pub fn flush(&self, mode: FlushMode) -> Result<()> {
        let completion_code =
            unsafe { CompletionCode::try_from(viFlush(self.value, mode.bits()))? };
        debug!("Flush completed with code: {}", completion_code);
        Ok(())
    }

    pub fn read(&self) -> Result<String> {
        let mut buffer = [0u8; 4096];
        let mut output = vec![];

        loop {
            let mut return_count: ViUInt32 = 0;
            let completion_code = unsafe {
                CompletionCode::try_from(viRead(
                    self.value,
                    buffer.as_mut_ptr(),
                    buffer.len() as ViUInt32,
                    &mut return_count,
                ))?
            };
            debug!("Read completed with code: {}", completion_code);

            output.extend_from_slice(&buffer[..return_count as usize]);

            match completion_code {
                CompletionCode::Success | CompletionCode::TerminationCharacterRead => {
                    break;
                }
                CompletionCode::MaximumCount => continue,
                completion_code => return Err(Error::UnexpectedCompletionCode(completion_code)),
            }
        }

        let output = String::from_utf8(output).map_err(|_| Error::InvalidUtf8)?;

        Ok(output)
    }

    pub fn query<T: AsRef<str>>(&self, command: T) -> Result<String> {
        self.write(command)?;
        self.read()
    }
}

impl MandatoryCommands for Session {
    fn as_session(&self) -> &Session {
        self
    }
}
