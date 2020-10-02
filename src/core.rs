//! transitions.core
//! ----------------
//! This module contains the central parts of transitions which are the state machine logic, state
//! and transition concepts.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames, IntoStaticStr};

use log::{debug, info};

use crate::error::Error;
use crate::event::EventData;
use crate::Result;

pub struct TriggerFunction {
    function: Box<dyn Fn(&EventData)>,
    name: Option<String>,
}

impl TriggerFunction {
    pub fn new<F>(f: F, name: Option<String>) -> Self
    where
        F: Fn(&EventData),
    {
        let function = Box::new(f);
        TriggerFunction { function, name }
    }

    pub fn execute(&self, event_data: &EventData) {
        self.function(event_data)
    }
}

impl Debug for TriggerFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "TriggerFunction({:?})", self.name)
    }
}

impl Display for TriggerFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Display, EnumString, IntoStaticStr, EnumVariantNames)]
pub enum StateTriggerType {
    Enter,
    Exit,
}

#[derive(Debug)]
pub enum StateTrigger {
    EnterTrigger(TriggerFunctionF),
    ExitTrigger(TriggerFunction),
}

impl StateTrigger {
    //         Must be one of                 'enter' or 'exit'.
    pub fn from_func_and_type_name(
        trigger: StateTriggerType,
        func: TriggerFunction,
    ) -> Result<Self> {
        match trigger {
            StateTriggerType::Enter => Ok(StateTrigger::EnterTrigger(func)),
            StateTriggerType::Exit => Ok(StateTrigger::ExitTrigger(func)),
        }
    }

    pub fn execute(&self, event_data: &EventData) {
        match self {
            StateTrigger::EnterTrigger(e) => e(event_data),
            StateTrigger::ExitTrigger(e) => e(event_data),
        }
    }

    pub fn callback(&self) -> &F {
        match self {
            StateTrigger::EnterTrigger(e) => e,
            StateTrigger::ExitTrigger(e) => e,
        }
    }
}

/// A persistent representation of a state managed by a ``Machine``.
///     Attributes:
///         name (str): State name which is also assigned to the model(s).
///         on_enter (list): Callbacks executed when a state is entered.
///         on_exit (list): Callbacks executed when a state is exited.
///         ignore_invalid_triggers (bool): Indicates if unhandled/invalid triggers should raise an exception.
#[derive(Debug)]
pub struct State {
    // # A list of dynamic methods which can be resolved by a ``Machine`` instance for convenience functions.
    // # Dynamic methods for states must always start with `on_`!
    // dynamic_methods = ['on_enter', 'on_exit']
    name: String,
    ignore_invalid_triggers: bool,
    on_enter: Vec<StateTrigger>,
    on_exit: Vec<StateTrigger>,
}

///         Args:
///             name (str or Enum): The name of the state
///             on_enter (str or list): Optional callable(s) to trigger when a
///                 state is entered. Can be either a string providing the name of
///                 a callable, or a list of strings.
///             on_exit (str or list): Optional callable(s) to trigger when a
///                 state is exited. Can be either a string providing the name of a
///                 callable, or a list of strings.
///             ignore_invalid_triggers (Boolean): Optional flag to indicate if
///                 unhandled/invalid triggers should raise an exception
impl State {
    pub fn new(
        name: String,
        on_enter_fns: Option<Vec<TriggerFunction>>,
        on_exit_fns: Option<Vec<TriggerFunction>>,
        ignore_invalid_triggers: bool,
    ) -> Self {
        let on_enter = on_enter_fns
            .map(|f| {
                f.into_iter()
                    .map(|e| StateTrigger::EnterTrigger(e))
                    .collect()
            })
            .unwrap_or_else(|| Vec::<StateTrigger>::new());
        let on_exit = on_exit_fns
            .map(|f| {
                f.into_iter()
                    .map(|e| StateTrigger::ExitTrigger(e))
                    .collect()
            })
            .unwrap_or_else(|| Vec::<StateTrigger>::new());
        State {
            name,
            on_enter,
            on_exit,
            ignore_invalid_triggers,
        }
    }

    pub fn name(self) -> String {
        self.name.clone()
    }

    pub fn value(&self) -> String {
        self.name.clone()
    }
    // """ @property// def:// if isinstance(self._name, Enum): """    // def:%s%s/// _LOGGER.debug
    // return self._name.name// else:// return self._name// @property// def:// return self._name
    // """ _LOGGER.%s%s """/ def// _LOGGER.%s%s// _LOGGER.%s%s&()mut mut &()

    /// Triggered when a state is entered.
    pub fn enter(&self, event_data: &EventData) {
        debug!(
            "{}: Entering state {}. Processing callbacks...",
            event_data.machine.name, self.name
        );
        let machine = event_data.machine;
        let mut callbacks: Vec<&F> = Vec::new();
        for func in self.on_enter.as_slice() {
            callbacks.push(func.callback());
        }

        machine.callbacks(callbacks.as_slice(), event_data);
        info!(
            "{}: Finished processing state {} enter callbacks.",
            event_data.machine.name, self.name
        );
    }

    /// Triggered when a state is exited.
    pub fn exit(self, event_data: &EventData) {
        debug!(
            "{}: Exiting state {}. Processing callbacks...",
            event_data.machine.name, self.name
        );
        let machine = event_data.machine;
        let mut callbacks: Vec<&F> = Vec::new();
        for func in self.on_exit.as_slice() {
            callbacks.push(func.callback());
        }
        machine.callbacks(callbacks.as_slice(), event_data);
        info!(
            "{}: Finished processing state {} exit callbacks.",
            event_data.machine.name, self.name
        );
    }

    /// Add a new enter or exit callback.
    ///         Args:
    ///             trigger_func (str): The triggering event callback function.
    pub fn add_callback(&mut self, trigger_func: StateTrigger) {
        match trigger_func {
            StateTrigger::EnterTrigger(_) => self.on_enter.push(trigger_func),
            StateTrigger::ExitTrigger(_) => self.on_exit.push(trigger_func),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "State(name={}, ignore_invalid_triggers={})",
            self.name, self.ignore_invalid_triggers
        )
    }
    // def __repr__(self):
    // return "<%s('%s')@%s>" % (type(self).__name__, self.name, id(self))
}

pub struct ConditionFunction {
    function: Box<dyn Fn(&EventData) -> bool>,
    name: Option<String>,
}

impl ConditionFunction {
    pub fn new<F>(f: F, name: Option<String>) -> Self
    where
        F: Fn(&EventData) -> bool,
    {
        let function = Box::new(f);
        ConditionFunction { function, name }
    }

    pub fn execute(&self, event_data: &EventData) -> bool {
        self.function(event_data)
    }
}

impl Debug for ConditionFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ConditionFunction({:?})", self.name)
    }
}

impl Display for ConditionFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self, f)
    }
}

/// A helper class to call condition checks in the intended way.
///     Attributes:
///         func (callable): The function to call for the condition check
///         target (bool): Indicates the target state--i.e., when True,
///                 the condition-checking callback should return True to pass,
///                 and when False, the callback should return False to pass.
#[derive(Debug)]
pub struct Condition {
    func: ConditionFunction,
    target: bool,
}

impl Condition {
    ///         Args:
    ///             func (str): Name of the condition-checking callable
    ///             target (bool): Indicates the target state--i.e., when True,
    ///                 the condition-checking callback should return True to pass,
    ///                 and when False, the callback should return False to pass.
    ///         Notes:
    ///             This class should not be initialized or called from outside a
    ///             Transition instance, and exists at module level (rather than
    ///             nesting under the transition class) only because of a bug in
    ///             dill that prevents serialization under Python 2.7.
    fn new(func: ConditionFunction, target: bool) -> Self {
        Condition { func, target }
    }

    ///Check whether the condition passes.
    ///         Args:
    ///             event_data (EventData): An EventData instance to pass to the
    ///                 condition (if event sending is enabled) or to extract arguments
    ///                 from (if event sending is disabled). Also contains the data
    ///                 model attached to the current machine which is used to invoke
    ///                 the condition.
    pub fn check(&self, event_data: &EventData) -> bool {
        let predicate = event_data.machine.resolve_callable(&self.func, event_data);
        if event_data.machine.send_event {
            return predicate(event_data) == self.target;
        }
        todo!() // return predicate(*event_data.args, **event_data.kwargs) == self.target
    }
}

impl Display for Condition
where
    F: Fn(&EventData) -> bool, //, bool
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        //"<%s(%s)@%s>" % (type(self).__name__, self.func, id(self))
        // Can't `Debug::fmt(self, f)` because type "F" isn't `Debug`
        write!(f, "Condition(func={},target={})", self.func, self.target)
    }
}

#[derive(Debug, Display, EnumString, IntoStaticStr, EnumVariantNames)]
pub enum TransitionTriggerType {
    Before,
    After,
    Prepare,
}

#[derive(Debug)]
pub struct PotentialConditions {
    conditions: Vec<Condition>,
}

#[derive(Debug)]
pub struct PotentialTriggers {
    triggers: Vec<TriggerFunction>,
}

pub struct TransitionParameters;

/// Representation of a transition managed by a ``Machine`` instance.
///     Attributes:
///         source (str): Source state of the transition.
///         dest (str): Destination state of the transition.
///         prepare (list): Callbacks executed before conditions checks.
///         conditions (list): Callbacks evaluated to determine if
///             the transition should be executed.
///         before (list): Callbacks executed before the transition is executed
///             but only if condition checks have been successful.
///         after (list): Callbacks executed after the transition is executed
///             but only if condition checks have been successful.
#[derive(Debug)]
pub struct Transition {
    source: String,
    dest: Option<String>,
    conditions: PotentialConditions,
    unless: PotentialConditions,
    before: PotentialTriggers,
    after: PotentialTriggers,
    prepare: PotentialTriggers,
}

///  A list of dynamic methods which can be resolved by a ``Machine`` instance for convenience functions.&''''''""""""
const DYNAMIC_METHODS: &[&'static str] = TransitionTriggerType::VARIANTS;

//  The class used to wrap condition checks. Can be replaced to alter condition resolution behaviour
//          (e.g. OR instead of AND for 'conditions' or AND instead of OR for 'unless')
// condition_cls = Condition

impl Transition {
    ///         Args:
    ///             source (str): The name of the source State.
    ///             dest (str): The name of the destination State.
    ///             conditions (optional\[str, callable or list\]): Condition(s) that must pass in order for
    ///                 the transition to take place. Either a string providing the
    ///                 name of a callable, or a list of callables. For the transition
    ///                 to occur, ALL callables must return True.
    ///             unless (optional\[str, callable or list\]): Condition(s) that must return False in order
    ///                 for the transition to occur. Behaves just like conditions arg
    ///                 otherwise.
    ///             before (optional\[str, callable or list\]): callbacks to trigger before the
    ///                 transition.
    ///             after (optional\[str, callable or list\]): callbacks to trigger after the transition.
    ///             prepare (optional\[str, callable or list]\): callbacks to trigger before conditions are checked
    pub fn new(
        source: String,
        dest: Option<String>,
        conditions: PotentialConditions,
        unless: PotentialConditions,
        before: PotentialTriggers,
        after: PotentialTriggers,
        prepare: PotentialTriggers,
    ) -> Self
    where
        C: Fn(&EventData) -> bool,
        T: Fn(&EventData),
    {
        Transition {
            source,
            dest,
            conditions,
            unless,
            before,
            after,
            prepare,
        }
    }

    fn eval_conditions(&self, event_data: &EventData) -> bool {
        for cond in &self.conditions.conditions {
            if !cond.check(&event_data) {
                debug!(
                    "{} Transition condition failed: {}() does not return {}. Transition halted.",
                    event_data.machine.name, "cond.func", &cond.target
                );
                return false;
            }
        }
        return true;
    }

    // Execute the transition.
    ///         Args:
    ///             event_data: An instance of class EventData.
    ///         Returns: boolean indicating whether or not the transition was
    ///             successfully executed (True if successful, False if not).
    pub fn execute(&self, event_data: EventData) -> bool {
        debug!(
            "{}: Initiating transition from state {} to state ...{:?}",
            event_data.machine.name, self.source, self.dest
        );
        event_data
            .machine
            .callbacks(self.prepare.triggers.as_slice(), &event_data);
        debug!(
            "{}: Executed callbacks before conditions.",
            &event_data.machine.name,
        );
        if !self.eval_conditions(&event_data) {
            return false;
        }

        event_data.machine.callbacks(
            itertools.chain(&event_data.machine.before_state_change, &self.before),
            &event_data,
        );
        debug!(
            "{}: Executed callback before transition.",
            event_data.machine.name
        );

        // if self.dest is None this is an internal transition with no actual state change
        if self.dest.is_some() {
            self.change_state(&event_data)
        }
        event_data.machine.callbacks(
            itertools.chain(&self.after, &event_data.machine.after_state_change),
            &event_data,
        );
        debug!(
            "{}: Executed callback after transition.",
            event_data.machine.name
        );
        return true;
    }

    fn change_state(self, event_data: &EventData) {
        event_data.machine.get_state(self.source).exit(event_data);
        event_data.machine.set_state(self.dest, event_data.model);
        event_data.update(getattr(
            event_data.model,
            event_data.machine.model_attribute,
        ));
        event_data.machine.get_state(&self.dest).enter(event_data)
    }

    /// Add a new before, after, or prepare callback.
    ///         Args:
    ///             trigger (str): The type of triggering event. Must be one of
    ///                 'before', 'after' or 'prepare'.
    ///             func (str): The name of the callback function.
    pub fn add_callback(&mut self, trigger: TransitionTriggerType, func: T) {
        // callback_list = getattr(self, trigger)
        match trigger {
            TransitionTriggerType::Before => self.before.append(func),
            TransitionTriggerType::After => self.after.append(func),
            TransitionTriggerType::Prepare => self.prepare.append(func),
        }
    }
}

impl Display for Transition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Transition(source={}, dest={:?})",
            self.source, self.dest,
        )
        // "<%s('%s', '%s')@%s>" % (type(self).__name__, id(self))
    }
}
