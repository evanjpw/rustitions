//!

use crate::core::{State, Transition, TransitionTriggerType, TriggerFunction};
use crate::error::Error;
use crate::machine::{Machine, Model};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Collection of relevant data related to the ongoing transition attempt.
///     Attributes:
///         state (State): The State from which the Event was triggered.
///         event (Event): The triggering Event.
///         machine (Machine): The current Machine instance.
///         model (object): The model/object the machine is bound to.
///         args (list): Optional positional arguments from trigger method
///             to store internally for possible later use.
///         kwargs (dict): Optional keyword arguments from trigger method
///             to store internally for possible later use.
///         transition (Transition): Currently active transition. Will be assigned during triggering.
///         error (Error): In case a triggered event causes an Error, it is assigned here and passed on.
///         result (bool): True in case a transition has been successful, False otherwise.
pub struct EventData<'a, 'b: 'a> {
    state: &'a State,
    event: &'a Event<'b>,
    pub(crate) machine: &'a mut Machine,
    pub(crate) model: &'a Model,
    //*args
    //**kwargs_
    transition: Option<&'a Transition>,
    error: Option<Error>,
    result: bool,
}

impl<'a, 'b> EventData<'a, 'b> {
    ///         Args:
    ///             state (State): The State from which the Event was triggered.
    ///             event (Event): The triggering Event.
    ///             machine (Machine): The current Machine instance.
    ///             model (object): The model/object the machine is bound to.
    ///             args (tuple): Optional positional arguments from trigger method
    ///                 to store internally for possible later use.
    ///             kwargs (dict): Optional keyword arguments from trigger method
    ///                 to store internally for possible later use.
    pub fn new(state: &'a State, event: &'b Event, machine: &'a mut Machine, model: &'a Model) -> Self {
        //, args, kwargs
        // self.args = args
        // self.kwargs = kwargs
        EventData {
            state,
            event,
            machine,
            model,
            error: None,
            result: false,
            transition: None,
        }
    }

    /// Updates the EventData object with the passed state.
    ///         Attributes:
    ///             state (State, str or Enum): The state object, enum member or string to assign to EventData.
    pub fn update(&mut self, state: State) {
        // if not isinstance(state, State):
        // self.state = self.machine.get_state(state)
        todo!()
    }
}

impl Display for EventData<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        unimplemented!()
    }
    // def __repr__(self):
    // return "<%s('%s', %s)@%s>" % (type(self).__name__, self.state,
    // getattr(self, 'transition'), id(self))_
}

#[derive(Debug)]
/// A collection of transitions assigned to the same trigger
pub struct Event<'a> {
    name: String,
    machine: &'a Machine,
    //transitions
}

impl<'a> Event<'a> {
    ///         Args:
    ///             name (str): The name of the event, which is also the name of the
    ///                 triggering callable (e.g., 'advance' implies an advance()
    ///                 method).
    ///             machine (Machine): The current Machine instance.
    pub fn new(name: String, machine: &'a Machine) -> Self {
        Event { name, machine }
        // self.transitions = defaultdict(list)
    }

    /// Add a transition to the list of potential transitions.
    ///         Args:
    ///             transition (Transition): The Transition instance to add to the
    ///                 list.
    pub fn add_transition(self, transition: Transition) {
        // self.transitions[transition.source].append(transition)
    }

    /// Serially execute all transitions that match the current state,
    ///         halting as soon as one successfully completes.
    ///         Args:
    ///             args and kwargs: Optional positional or named arguments that will
    ///                 be passed onto the EventData object, enabling arbitrary state
    ///                 information to be passed on to downstream triggered functions.
    ///         Returns: boolean indicating whether or not a transition was
    ///             successfully executed (True if successful, False if not).
    pub fn trigger(&self, model: &Model) { // *args, **kwargs,
                                           // func = partial(self._trigger, model, *args, **kwargs)
                                           // # pylint: disable=protected-access
                                           // # noinspection PyProtectedMember
                                           // # Machine._process should not be called somewhere else. That's why it should not be exposed
                                           // # to Machine users.
                                           // return self.machine._process(func)
    }

    /// Internal trigger function called by the ``Machine`` instance. This should not
    ///         be called directly but via the public method ``Machine.trigger``.
    pub fn machine_trigger(&self, model: Model) {
        // *args, **kwargs
        // state = self.machine.get_model_state(model)
        // if state.name not in self.transitions:
        // msg = "%sCan't trigger event %s from state %s!" % (self.machine.name, self.name,
        // state.name)
        // ignore = state.ignore_invalid_triggers if state.ignore_invalid_triggers is not None \
        // else self.machine.ignore_invalid_triggers
        // if ignore:
        // _LOGGER.warning(msg)
        // return False
        // else:
        // raise MachineError(msg)
        // event_data = EventData(state, self, self.machine, model, args=args, kwargs=kwargs)
        // return self._process(event_data)
    }

    pub fn _process(&self, event_data: &EventData) {
        // self.machine.callbacks(self.machine.prepare_event, event_data)
        // _LOGGER.debug("%sExecuted machine preparation callbacks before conditions.", self.machine.name)

        // try:
        // for trans in self.transitions[event_data.state.name]:
        // event_data.transition = trans
        // if trans.execute(event_data):
        // event_data.result = True
        // break
        // except Exception as err:
        // event_data.error = err
        // raise
        // finally:
        // self.machine.callbacks(self.machine.finalize_event, event_data)
        // _LOGGER.debug("%sExecuted machine finalize callbacks", self.machine.name)

        // return event_data.result
    }

    // Add a new before or after callback to all available transitions.
    ///         Args:
    ///             trigger (str): The type of triggering event. Must be one of
    ///                 'before', 'after' or 'prepare'.
    ///             func (str): The name of the callback function.
    pub fn add_callback(self, trigger: TransitionTriggerType, func: TriggerFunction) {
        // for trans in itertools.chain(*self.transitions.values()):
        // trans.add_callback(trigger, func)
    }
}

impl Display for Event<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self, f)
    }
    // def __repr__(self):
    // return "<%s('%s')@%s>" % (type(self).__name__, self.name, id(self))
}
