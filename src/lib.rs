//!

mod core;
mod error;
mod event;
mod machine;
mod support;

// _LOGGER = logging.getLogger(__name__)
// _LOGGER.addHandler(logging.NullHandler())

// warnings.filterwarnings(action='default', message=r".*transitions version.*", category=DeprecationWarning)
pub type Result<T> = std::result::Result<T, error::Error>;
