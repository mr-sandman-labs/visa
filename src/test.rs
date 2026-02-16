use crate::test::instrument::Instrument;
use crate::{resource_manager::ResourceManager, utility::MandatoryCommands};
use color_eyre::{Result, eyre::eyre};
use tracing::{info, level_filters::LevelFilter};

mod instrument {
    use crate::{
        resource_manager::ResourceManager,
        session::Session,
        utility::{AccessMode, MandatoryCommands, Timeout},
    };
    use thiserror::Error;

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error(transparent)]
        Visa(#[from] crate::error::Error),
        #[error("Instrument not found")]
        InstrumentNotFound,
    }

    const MANUFACTURER: &str = "";
    const MODEL: &str = "";

    pub struct Instrument {
        session: Session,
    }

    impl Instrument {
        pub fn from_serial<T: AsRef<str>>(
            resource_manager: &ResourceManager,
            serial: T,
        ) -> Result<Self> {
            let resource = resource_manager.find_resource_by_identification(
                "*?INSTR",
                MANUFACTURER,
                MODEL,
                serial.as_ref(),
            )?;
            let session = resource_manager.open_session(
                resource,
                AccessMode::Exclusive,
                Timeout::Immediate,
            )?;
            Ok(Self { session })
        }

        pub fn from_resource<T: AsRef<str>>(
            resource_manager: &ResourceManager,
            resource: T,
        ) -> Result<Self> {
            let session = resource_manager.open_session(
                resource,
                AccessMode::Exclusive,
                Timeout::Immediate,
            )?;

            let identification = session.identification_query()?;

            if identification.manufacturer == MANUFACTURER && identification.model == MODEL {
                return Ok(Instrument { session });
            }

            Err(Error::InstrumentNotFound)
        }
    }

    impl MandatoryCommands for Instrument {
        fn as_session(&self) -> &Session {
            &self.session
        }
    }
}

#[test]
fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();

    // Create Resource Manager
    let resource_manager = ResourceManager::new()?;

    // Find all instrument resources
    let resources = resource_manager.find_resources("?*INSTR")?;

    info!("Resources:\n{:#?}", resources);
    if resources.is_empty() {
        return Err(eyre!(
            "At least one valid instrument resource is required for testing"
        ));
    }

    let instrument = Instrument::from_serial(&resource_manager, "1234")?;

    // Query Identification
    let identification = instrument.identification_query()?;
    info!("Identification:\n{:#?}", identification);

    // Query Standard Event Status Enable
    let standard_event_status_enable = instrument.standard_event_status_enable_query()?;
    info!(
        "Standard Event Status Enable:\n{:#?}",
        standard_event_status_enable
    );

    // Query Standard Event Status Register
    let standard_event_status_register = instrument.standard_event_status_register_query()?;
    info!(
        "Standard Event Status Register:\n{:#?}",
        standard_event_status_register
    );

    // Query Operation Complete
    let operation_complete = instrument.operation_complete_query()?;
    info!("Operation Complete:\n{}", operation_complete);

    // Query Service Request Enable
    let service_enable = instrument.service_request_enable_query()?;
    info!("Service Request Enable:\n{:#?}", service_enable);

    // Query Status Byte Register
    let status_byte_register = instrument.read_status_byte_query()?;
    info!("Status Byte Register:\n{:#?}", status_byte_register);

    // Query Self Test
    let self_test = instrument.self_test_query()?;
    info!("Self Test:\n{}", self_test);

    Ok(())
}
