//!

use crate::support::KeySet;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// MachineError is used for issues related to state transitions and current states.
    /// For instance, it is raised for invalid transitions or machine configuration issues.
    #[error("MachineError: {0}")]
    MachineError(String),
    #[error("ArgumentsError: Argument length must be either 1 or the same length as the number of transitions.")]
    ArgumentsError,
    #[error("InheritanceError: Passing arguments {0} caused an inheritance error: {1}")]
    InheritanceError(KeySet, String),
    #[error("InitialStateError: No initial state configured for machine, must specify when adding model.")]
    InitialStateError,
    #[error("RegisteredStateError: State '{0}' is not a registered state.")]
    RegisteredStateError(String),
    #[error("UnknownEventError: Do not know event named '{0}'.")]
    UnknownEventError(String),
    #[error("TriggerNameError: Trigger name cannot be same as model attribute name.")]
    TriggerNameError(String),
    #[error("InsufficientStatesError: Can't create ordered transitions on a Machine with fewer than 2 states.")]
    InsufficientStatesError,
    #[error("CallableError: Callable with name '{0}' could neither be retrieved from the passed model nor imported from a module.")]
    CallableError(String),
    #[error("UnknownStateError: State {0} has not been added to the machine")]
    UnknownStateError(String),
    #[error("'{0}' does not exist on <Machine@{1}>")]
    MachineAttributeError(String, String),
    #[error("event '{0}' is not registered on <Machine@{1}>")]
    MachineEventAttributeError(String, String),
    // #[error("{0}")]
}
