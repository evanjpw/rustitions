//!

use crate::core::{ConditionFunction, State, StateTrigger, TransitionParameters, TriggerFunction};
use crate::event::EventData;
use indexmap::IndexMap;
use log::info;

pub(crate) type Model = Machine;

pub fn getattr(model: &Model, attribute: &str) -> State {
    todo!()
}

/// Machine manages states, transitions and ;
/// models. In case it is initialized without a specific model
///     (or specifically no model), it will also act as a model itself. Machine takes also care of decorating
///     models with conveniences functions related to added transitions and states during runtime.
///     Attributes:
///         states (OrderedDict): Collection of all registered states.
///         events (dict): Collection of transitions ordered by trigger/event.
///         models (list): List of models attached to the machine.
///         initial (str): Name of the initial state for new models.
///         prepare_event (list): Callbacks executed when an event is triggered.
///         before_state_change (list): Callbacks executed after condition checks but before transition is conducted.
///             Callbacks will be executed BEFORE the custom callbacks assigned to the transition.
///         after_state_change (list): Callbacks executed after the transition has been conducted.
///             Callbacks will be executed AFTER the custom callbacks assigned to the transition.
///         finalize_event (list): Callbacks will be executed after all transitions callbacks have been executed.
///             Callbacks mentioned here will also be called if a transition or condition check raised an error.
///         queued (bool): Whether transitions in callbacks should be executed immediately (False) or sequentially.
///         send_event (bool): When True, any arguments passed to trigger methods will be wrapped in an EventData
///             object, allowing indirect and encapsulated access to data. When False, all positional and keyword
///             arguments will be passed directly to all callback methods.
///         auto_transitions (bool):  When True (default), every state will automatically have an associated
///             to_{state}() convenience trigger in the base model.
///         ignore_invalid_triggers (bool): When True, any calls to trigger methods that are not valid for the
///             present state (e.g., calling an a_to_b() trigger when the current state is c) will be silently
///             ignored rather than raising an invalid transition exception.
///         name (str): Name of the ``Machine`` instance mainly used for easier log message distinction.
#[derive(Debug)]
pub struct Machine {
    // states: IndexMap<String, State>,
    // initial: String,
    models: Vec<Model>,
    //events
    // prepare_event////
    pub before_state_change: Vec<TriggerFunction>,
    pub after_state_change: Vec<TriggerFunction>,
    // finalize_event
    queued: bool,
    pub(crate) send_event: bool,
    auto_transitions: bool,
    ignore_invalid_triggers: bool,
    pub name: String,
    pub(crate) model_attribute: String,
}

///   separates callback type from state/transition name
const SEPARATOR: char = '_';
///    will be expanded to ALL states
const WILDCARD_ALL: char = '*';
///   will be expanded to source state
const WILDCARD_SAME: char = '=';
// state_cls = State
// transition_cls = Transition
// event_cls = Event

impl Machine {
    ///         Args:
    ///             model (object or list): The object(s) whose states we want to manage. If 'self',
    ///                 the current Machine instance will be used the model (i.e., all
    ///                 triggering events will be attached to the Machine itself). Note that an empty list
    ///                 is treated like no model.
    ///             states (list or Enum): A list or enumeration of valid states. Each list element can be either a
    ///                 string, an enum member or a State instance. If string or enum member, a new generic State
    ///                 instance will be created that is named according to the string or enum member's name.
    ///             initial (str, Enum or State): The initial state of the passed model[s].
    ///             transitions (list): An optional list of transitions. Each element
    ///                 is a dictionary of named arguments to be passed onto the
    ///                 Transition initializer.
    ///             send_event (boolean): When True, any arguments passed to trigger
    ///                 methods will be wrapped in an EventData object, allowing
    ///                 indirect and encapsulated access to data. When False, all
    ///                 positional and keyword arguments will be passed directly to all
    ///                 callback methods.
    ///             auto_transitions (boolean): When True (default), every state will
    ///                 automatically have an associated to_{state}() convenience
    ///                 trigger in the base model.
    ///             ordered_transitions (boolean): Convenience argument that calls
    ///                 add_ordered_transitions() at the end of initialization if set
    ///                 to True.
    ///             ignore_invalid_triggers: when True, any calls to trigger methods
    ///                 that are not valid for the present state (e.g., calling an
    ///                 a_to_b() trigger when the current state is c) will be silently
    ///                 ignored rather than raising an invalid transition exception.
    ///             before_state_change: A callable called on every change state before
    ///                 the transition happened. It receives the very same args as normal
    ///                 callbacks.
    ///             after_state_change: A callable called on every change state after
    ///                 the transition happened. It receives the very same args as normal
    ///                 callbacks.
    ///             name: If a name is set, it will be used as a prefix for logger output
    ///             queued (boolean): When True, processes transitions sequentially. A trigger
    ///                 executed in a state callback function will be queued and executed later.
    ///                 Due to the nature of the queued processing, all transitions will
    ///                 _always_ return True since conditional checks cannot be conducted at queueing time.
    ///             prepare_event: A callable called on for before possible transitions will be processed.
    ///                 It receives the very same args as normal callbacks.
    ///             finalize_event: A callable called on for each triggered event after transitions have been processed.
    ///                 This is also called when a transition raises an exception.
    ///             **kwargs additional arguments passed to next class in MRO. This can be ignored in most cases.
    /// model_attribute='state'???
    pub fn new(
        model: Option<Model>,                                  /*=self*/
        states: &[State],                                      //=None=None
        initial: Option<State>,                                /*='initial'*/
        transitions: &[TransitionParameters],                  /*=None*/
        send_event: bool,                                      /*=False*/
        auto_transitions: bool,                                /*=True*/
        ordered_transitions: bool,                             /*=False*/
        ignore_invalid_triggers: bool,                         /*=None*/
        possible_before_state_change: Option<TriggerFunction>, /*=None*/
        possible_after_state_change: Option<TriggerFunction>,  /*=None*/
        name: Option<String>,                                  /*=None*/
        queued: bool,                                          /*=False*/
        prepare_event: TriggerFunction,                        /*=None*/
        finalize_event: TriggerFunction,
        model_attribute: Option<String>, /*='state'*/ /*,kwargs*/
    ) -> Self {
        // # calling super in case `Machine` is used as a mix in
        // # all keyword arguments should be consumed by now if this is not the case
        // try:
        // super(Machine, self).__init__(**kwargs)
        // except TypeError as err:
        // raise ValueError('Passing arguments {0} caused an inheritance error: {1}'.format(kwargs.keys(), err))

        // # initialize protected attributes first

        // self._transition_queue = deque()
        // self._initial = None

        // self.states = OrderedDict()
        // lf
        // let initial
        // let states
        let models: Vec<Model> = Vec::new();
        let name = name.map(|n| n + ": ").unwrap_or_else(|| String::from(""));

        let model_attribute = model_attribute.unwrap_or_else(|| String::from("state"));

        // self.models = []

        // if states is not None:
        // self.add_states(states)
        //
        // if initial is not None:
        // self.initial = initial

        // if transitions is not None:
        // self.add_transitions(transitions)

        // if ordered_transitions:
        // self.add_ordered_transitions()

        // if model:
        // self.add_model(model)
        // self.events = {}
        // self.prepare_event = prepare_event
        let mut before_state_change: Vec<TriggerFunction> = Vec::new();
        if let Some(f) = possible_before_state_change {
            before_state_change.push(f);
        }
        let mut after_state_change: Vec<TriggerFunction> = Vec::new();
        if let Some(f) = possible_after_state_change {
            after_state_change.push(f);
        }
        // self.finalize_event = finalize_event
        // self._prepare_event = []
        // self._finalize_event = []
        Machine {
            send_event,
            queued,
            auto_transitions,
            ignore_invalid_triggers,
            name,
            models,
            before_state_change,
            after_state_change,
            model_attribute,
        }
    }

    // def add_model(self, model, initial=None):
    // """ Register a model with the state machine, initializing triggers and callbacks. """
    // models = listify(model)
    //
    // if initial is None:
    // if self.initial is None:
    // raise ValueError("No initial state configured for machine, must specify when adding model.")
    // else:
    // initial = self.initial
    //
    // for mod in models:
    // mod = self if mod == 'self' else mod
    // if mod not in self.models:
    // self._checked_assignment(mod, 'trigger', partial(self._get_trigger, mod))
    //
    // for trigger in self.events:
    // self._add_trigger_to_model(trigger, mod)
    //
    // for state in self.states.values():
    // self._add_model_to_state(state, mod)
    //
    // self.set_state(initial, model=mod)
    // self.models.append(mod)
    //
    // def remove_model(self, model):
    // """ Remove a model from the state machine. The model will still contain all previously added triggers
    //         and callbacks, but will not receive updates when states or transitions are added to the Machine. """
    // models = listify(model)
    //
    // for mod in models:
    // self.models.remove(mod)
    //
    // @classmethod
    // def _create_transition(cls, *args, **kwargs):
    // return cls.transition_cls(*args, **kwargs)
    //
    // @classmethod
    // def _create_event(cls, *args, **kwargs):
    // return cls.event_cls(*args, **kwargs)
    //
    // @classmethod
    // def _create_state(cls, *args, **kwargs):
    // return cls.state_cls(*args, **kwargs)
    //
    // @property
    // def initial(self):
    // """ Return the initial state. """
    // return self._initial
    //
    // @initial.setter
    // def initial(self, value):
    // if isinstance(value, State):
    // if value.name not in self.states:
    // self.add_state(value)
    // else:
    // _ = self._has_state(value, raise_error=True)
    // self._initial = value.name
    // else:
    // state_name = value.name if isinstance(value, Enum) else value
    // if state_name not in self.states:
    // self.add_state(state_name)
    // self._initial = state_name
    //
    // @property
    // def has_queue(self):
    // """ Return boolean indicating if machine has queue or not """
    // return self._queued
    //
    // @property
    // def model(self):
    // """ List of models attached to the machine. For backwards compatibility, the property will
    //         return the model instance itself instead of the underlying list  if there is only one attached
    //         to the machine.
    //         """
    // if len(self.models) == 1:
    // return self.models[0]
    // return self.models
    //
    // @property
    // def before_state_change(self):
    // """Callbacks executed after condition checks but before transition is conducted.
    //         Callbacks will be executed BEFORE the custom callbacks assigned to the transition."""
    // return self._before_state_change
    //
    // # this should make sure that _before_state_change is always a list
    // @before_state_change.setter
    // def before_state_change(self, value):
    // self._before_state_change = listify(value)
    //
    // @property
    // def after_state_change(self):
    // """Callbacks executed after the transition has been conducted.
    //         Callbacks will be executed AFTER the custom callbacks assigned to the transition."""
    // return self._after_state_change
    //
    // # this should make sure that _after_state_change is always a list
    // @after_state_change.setter
    // def after_state_change(self, value):
    // self._after_state_change = listify(value)
    //
    // @property
    // def prepare_event(self):
    // """Callbacks executed when an event is triggered."""
    // return self._prepare_event
    //
    // # this should make sure that prepare_event is always a list
    // @prepare_event.setter
    // def prepare_event(self, value):
    // self._prepare_event = listify(value)
    //
    // @property
    // def finalize_event(self):
    // """Callbacks will be executed after all transitions callbacks have been executed.
    //         Callbacks mentioned here will also be called if a transition or condition check raised an error."""
    // return self._finalize_event
    //
    // # this should make sure that finalize_event is always a list
    // @finalize_event.setter
    // def finalize_event(self, value):
    // self._finalize_event = listify(value)

    /// Return the State instance with the passed name.
    pub fn get_state(&self, state: &State) -> &State {
        // if isinstance(state, Enum):
        // state = state.name
        // if state not in self.states:
        // raise ValueError("State '%s ' is not a registered state." % state)
        // return self.states[state]
        todo!()
    }

    // # In theory this function could be static. This however causes some issues related to inheritance and
    // # pickling down the chain.
    // Check whether the current state matches the named state. This function is not called directly
    ///             but assigned as partials to model instances (e.g. is_A -> partial(_is_state, 'A', model)).
    ///         Args:
    ///             state (str): name of the checked state
    ///             model: model to be checked
    ///         Returns:
    ///             bool: Whether the model's current state is state.
    pub fn is_state(&self, state: &State, model: &Model) {
        // return getattr(model, self.model_attribute) == state
        todo!()
    }

    pub fn get_model_state(&self, model: &Model) {
        // return self.get_state(getattr(model, self.model_attribute))
        todo!()
    }

    ///             Set the current state.
    ///         Args:
    ///             state (str or Enum or State): value of state to be set
    ///             model (optional[object]): targeted model; if not set, all models will be set to 'state'
    pub fn set_state(&mut self, state: &State, model: Option<&Model> /*=None*/) {
        // if not isinstance(state, State):
        // state = self.get_state(state)
        // models = self.models if model is None else listify(model)
        //
        // for mod in models:
        // setattr(mod, self.model_attribute, state.value)
        todo!()
    }

    /// Alias for add_states.
    pub fn add_state(
        &mut self,
        states: Vec<&State>,
        on_enter: Vec<TriggerFunction>,
        on_exit: Vec<TriggerFunction>,
        ignore_invalid_triggers: bool,
        //**kwargs,
    ) {
        self.add_states(
            states,
            on_enter,
            on_exit,
            ignore_invalid_triggers,
            // **kwargs
        )
    }

    /// Add new state(s).
    ///         Args:
    ///             states (list, str, dict, Enum or State): a list, a State instance, the
    ///                 name of a new state, an enumeration (member) or a dict with keywords to pass on to the
    ///                 State initializer. If a list, each element can be a string, State or enumeration member.
    ///             on_enter (str or list): callbacks to trigger when the state is
    ///                 entered. Only valid if first argument is string.
    ///             on_exit (str or list): callbacks to trigger when the state is
    ///                 exited. Only valid if first argument is string.
    ///             ignore_invalid_triggers: when True, any calls to trigger methods
    ///                 that are not valid for the present state (e.g., calling an
    ///                 a_to_b() trigger when the current state is c) will be silently
    ///                 ignored rather than raising an invalid transition exception.
    ///                 Note that this argument takes precedence over the same
    ///                 argument defined at the Machine level, and is in turn
    ///                 overridden by any ignore_invalid_triggers explicitly
    ///                 passed in an individual state's initialization arguments.
    ///             **kwargs additional keyword arguments used by state mixins.
    pub fn add_states(
        &mut self,
        states: Vec<&State>,
        on_enter: Vec<TriggerFunction>, /*=None*/
        on_exit: Vec<TriggerFunction>,  /*=None*/
        ignore_invalid_triggers: bool,  /*=None*/
                                        //**kwargs,
    ) {

        // ignore = ignore_invalid_triggers
        // if ignore is None:
        // ignore = self.ignore_invalid_triggers

        // states = listify(states)

        // for state in states:
        // if isinstance(state, (string_types, Enum)):
        // state = self._create_state(
        // state, on_enter=on_enter, on_exit=on_exit,
        // ignore_invalid_triggers=ignore, **kwargs)
        // elif isinstance(state, dict):
        // if 'ignore_invalid_triggers' not in state:
        // state['ignore_invalid_triggers'] = ignore
        // state = self._create_state(**state)
        // self.states[state.name] = state
        // for model in self.models:
        // self._add_model_to_state(state, model)
        // if self.auto_transitions:
        // for a_state in self.states.keys():
        // # add all states as sources to auto transitions 'to_<state>' with dest <state>
        // if a_state == state.name:
        // if self.model_attribute == 'state':
        // method_name = 'to_%s' % a_state
        // else:
        // method_name = 'to_%s_%s' % (self.model_attribute, a_state)
        // self.add_transition('to_%s' % a_state, self.WILDCARD_ALL, a_state,
        // prepare=partial(_warning_wrapper_to, 'to_%s' % a_state))
        // self.add_transition(method_name, self.WILDCARD_ALL, a_state)

        // # add auto transition with source <state> to <a_state>
        // else:
        // if self.model_attribute == 'state':
        // method_name = 'to_%s' % a_state
        // else:
        // method_name = 'to_%s_%s' % (self.model_attribute, a_state)
        // self.add_transition('to_%s' % a_state, state.name, a_state,
        // prepare=partial(_warning_wrapper_to, 'to_%s' % a_state))
        // self.add_transition(method_name, state.name, a_state)
    }

    // def _add_model_to_state(self, state, model):
    // # Add convenience function 'is_<state_name>' (e.g. 'is_A') to the model.
    // # When model_attribute has been customized, add 'is_<model_attribute>_<state_name>' instead
    // # to potentially support multiple states on one model (e.g. 'is_custom_state_A' and 'is_my_state_B').
    //
    // func = partial(self.is_state, state.value, model)
    // if self.model_attribute == 'state':
    // method_name = 'is_%s' % state.name
    // else:
    // method_name = 'is_%s_%s' % (self.model_attribute, state.name)
    // self._checked_assignment(model, 'is_%s' % state.name, partial(_warning_wrapper_is, method_name, func))
    // self._checked_assignment(model, method_name, func)
    //
    // # Add dynamic method callbacks (enter/exit) if there are existing bound methods in the model
    // # except if they are already mentioned in 'on_enter/exit' of the defined state
    // for callback in self.state_cls.dynamic_methods:
    // method = "{0}_{1}".format(callback, state.name)
    // if hasattr(model, method) and inspect.ismethod(getattr(model, method)) and \
    // method not in getattr(state, callback):
    // state.add_callback(callback[3:], method)
    //
    // def _checked_assignment(self, model, name, func):
    // if hasattr(model, name):
    // _LOGGER.warning("%sModel already contains an attribute '%s'. Skip binding.", self.name, name)
    // else:
    // setattr(model, name, func)
    //
    // def _add_trigger_to_model(self, trigger, model):
    // self._checked_assignment(model, trigger, partial(self.events[trigger].trigger, model))
    //
    // def _get_trigger(self, model, trigger_name, *args, **kwargs):
    // """Convenience function added to the model to trigger events by name.
    //         Args:
    //             model (object): Model with assigned event trigger.
    //             machine (Machine): The machine containing the evaluated events.
    //             trigger_name (str): Name of the trigger to be called.
    //             *args: Variable length argument list which is passed to the triggered event.
    //             **kwargs: Arbitrary keyword arguments which is passed to the triggered event.
    //         Returns:
    //             bool: True if a transitions has been conducted or the trigger event has been queued.
    //         """
    // try:
    // event = self.events[trigger_name]
    // except KeyError:
    // state = self.get_model_state(model)
    // ignore = state.ignore_invalid_triggers if state.ignore_invalid_triggers is not None \
    // else self.ignore_invalid_triggers
    // if not ignore:
    // raise AttributeError("Do not know event named '%s'." % trigger_name)
    // return False
    // return event.trigger(model, *args, **kwargs)
    //
    // def get_triggers(self, *args):
    // """ Collects all triggers FROM certain states.
    //         Args:
    //             *args: Tuple of source states.
    //         Returns:
    //             list of transition/trigger names.
    //         """
    // states = set(args)
    // return [t for (t, ev) in self.events.items() if any(state in ev.transitions for state in states)]
    //
    // def add_transition(self, trigger, source, dest, conditions=None,
    // unless=None, before=None, after=None, prepare=None, **kwargs):
    // """ Create a new Transition instance and add it to the internal list.
    //         Args:
    //             trigger (str): The name of the method that will trigger the
    //                 transition. This will be attached to the currently specified
    //                 model (e.g., passing trigger='advance' will create a new
    //                 advance() method in the model that triggers the transition.)
    //             source(str or list): The name of the source state--i.e., the state we
    //                 are transitioning away from. This can be a single state, a
    //                 list of states or an asterisk for all states.
    //             dest (str): The name of the destination State--i.e., the state
    //                 we are transitioning into. This can be a single state or an
    //                 equal sign to specify that the transition should be reflexive
    //                 so that the destination will be the same as the source for
    //                 every given source. If dest is None, this transition will be
    //                 an internal transition (exit/enter callbacks won't be processed).
    //             conditions (str or list): Condition(s) that must pass in order
    //                 for the transition to take place. Either a list providing the
    //                 name of a callable, or a list of callables. For the transition
    //                 to occur, ALL callables must return True.
    //             unless (str or list): Condition(s) that must return False in order
    //                 for the transition to occur. Behaves just like conditions arg
    //                 otherwise.
    //             before (str or list): Callables to call before the transition.
    //             after (str or list): Callables to call after the transition.
    //             prepare (str or list): Callables to call when the trigger is activated
    //             **kwargs: Additional arguments which can be passed to the created transition.
    //                 This is useful if you plan to extend Machine.Transition and require more parameters.
    //         """
    // if trigger == self.model_attribute:
    // raise ValueError("Trigger name cannot be same as model attribute name.")
    // if trigger not in self.events:
    // self.events[trigger] = self._create_event(trigger, self)
    // for model in self.models:
    // self._add_trigger_to_model(trigger, model)
    //
    // if source == self.WILDCARD_ALL:
    // source = list(self.states.keys())
    // else:
    // # states are checked lazily which means we will only raise exceptions when the passed state
    // # is a State object because of potential confusion (see issue #155 for more details)
    // source = [s.name if isinstance(s, State) and self._has_state(s, raise_error=True) or hasattr(s, 'name') else
    // s for s in listify(source)]
    //
    // for state in source:
    // if dest == self.WILDCARD_SAME:
    // _dest = state
    // elif dest is not None:
    // if isinstance(dest, State):
    // _ = self._has_state(dest, raise_error=True)
    // _dest = dest.name if hasattr(dest, 'name') else dest
    // else:
    // _dest = None
    // _trans = self._create_transition(state, _dest, conditions, unless, before,
    // after, prepare, **kwargs)
    // self.events[trigger].add_transition(_trans)
    //
    // def add_transitions(self, transitions):
    // """ Add several transitions.
    //         Args:
    //             transitions (list): A list of transitions.
    //         """
    // for trans in listify(transitions):
    // if isinstance(trans, list):
    // self.add_transition(*trans)
    // else:
    // self.add_transition(**trans)
    //
    // def add_ordered_transitions(self, states=None, trigger='next_state',
    // loop=True, loop_includes_initial=True,
    // conditions=None, unless=None, before=None,
    // after=None, prepare=None, **kwargs):
    // """ Add a set of transitions that move linearly from state to state.
    //         Args:
    //             states (list): A list of state names defining the order of the
    //                 transitions. E.g., ['A', 'B', 'C'] will generate transitions
    //                 for A --> B, B --> C, and C --> A (if loop is True). If states
    //                 is None, all states in the current instance will be used.
    //             trigger (str): The name of the trigger method that advances to
    //                 the next state in the sequence.
    //             loop (boolean): Whether or not to add a transition from the last
    //                 state to the first state.
    //             loop_includes_initial (boolean): If no initial state was defined in
    //                 the machine, setting this to True will cause the _initial state
    //                 placeholder to be included in the added transitions. This argument
    //                 has no effect if the states argument is passed without the
    //                 initial state included.
    //             conditions (str or list): Condition(s) that must pass in order
    //                 for the transition to take place. Either a list providing the
    //                 name of a callable, or a list of callables. For the transition
    //                 to occur, ALL callables must return True.
    //             unless (str or list): Condition(s) that must return False in order
    //                 for the transition to occur. Behaves just like conditions arg
    //                 otherwise.
    //             before (str or list): Callables to call before the transition.
    //             after (str or list): Callables to call after the transition.
    //             prepare (str or list): Callables to call when the trigger is activated
    //             **kwargs: Additional arguments which can be passed to the created transition.
    //                 This is useful if you plan to extend Machine.Transition and require more parameters.
    //         """
    // if states is None:
    // states = list(self.states.keys())  # need to listify for Python3
    // len_transitions = len(states)
    // if len_transitions < 2:
    // raise ValueError("Can't create ordered transitions on a Machine "
    // "with fewer than 2 states.")
    // if not loop:
    // len_transitions -= 1
    // # ensure all args are the proper length
    // conditions = _prep_ordered_arg(len_transitions, conditions)
    // unless = _prep_ordered_arg(len_transitions, unless)
    // before = _prep_ordered_arg(len_transitions, before)
    // after = _prep_ordered_arg(len_transitions, after)
    // prepare = _prep_ordered_arg(len_transitions, prepare)
    // # reorder list so that the initial state is actually the first one
    // try:
    // idx = states.index(self._initial)
    // states = states[idx:] + states[:idx]
    // first_in_loop = states[0 if loop_includes_initial else 1]
    // except ValueError:
    // # since initial is not part of states it shouldn't be part of the loop either
    // first_in_loop = states[0]
    //
    // for i in range(0, len(states) - 1):
    // self.add_transition(trigger, states[i], states[i + 1],
    // conditions=conditions[i],
    // unless=unless[i],
    // before=before[i],
    // after=after[i],
    // prepare=prepare[i],
    // **kwargs)
    // if loop:
    // self.add_transition(trigger, states[-1],
    // # omit initial if not loop_includes_initial
    // first_in_loop,
    // conditions=conditions[-1],
    // unless=unless[-1],
    // before=before[-1],
    // after=after[-1],
    // prepare=prepare[-1],
    // **kwargs)
    //
    // def get_transitions(self, trigger="", source="*", dest="*"):
    // """ Return the transitions from the Machine.
    //         Args:
    //             trigger (str): Trigger name of the transition.
    //             source (str): Limits list to transitions from a certain state.
    //             dest (str): Limits list to transitions to a certain state.
    //         """
    // if trigger:
    // try:
    // events = (self.events[trigger], )
    // except KeyError:
    // return []
    // else:
    // events = self.events.values()
    // transitions = []
    // for event in events:
    // transitions.extend(
    // itertools.chain.from_iterable(event.transitions.values()))
    // return [transition
    // for transition in transitions
    // if (transition.source, transition.dest) == (
    // source if source != "*" else transition.source,
    // dest if dest != "*" else transition.dest)]
    //
    // def remove_transition(self, trigger, source="*", dest="*"):
    // """ Removes a transition from the Machine and all models.
    //         Args:
    //             trigger (str): Trigger name of the transition.
    //             source (str): Limits removal to transitions from a certain state.
    //             dest (str): Limits removal to transitions to a certain state.
    //         """
    // source = listify(source) if source != "*" else source
    // dest = listify(dest) if dest != "*" else dest
    // # outer comprehension, keeps events if inner comprehension returns lists with length > 0
    // tmp = {key: value for key, value in
    // {k: [t for t in v
    // # keep entries if source should not be filtered; same for dest.
    // if (source != "*" and t.source not in source) or (dest != "*" and t.dest not in dest)]
    // # }.items() takes the result of the inner comprehension and uses it
    // # for the outer comprehension (see first line of comment)
    // for k, v in self.events[trigger].transitions.items()}.items()
    // if len(value) > 0}
    // # convert dict back to defaultdict in case tmp is not empty
    // if tmp:
    // self.events[trigger].transitions = defaultdict(list, **tmp)
    // # if no transition is left remove the trigger from the machine and all models
    // else:
    // for model in self.models:
    // delattr(model, trigger)
    // del self.events[trigger]
    //
    // def dispatch(self, trigger, *args, **kwargs):
    // """ Trigger an event on all models assigned to the machine.
    //         Args:
    //             trigger (str): Event name
    //             *args (list): List of arguments passed to the event trigger
    //             **kwargs (dict): Dictionary of keyword arguments passed to the event trigger
    //         Returns:
    //             bool The truth value of all triggers combined with AND
    //          """""""" "
    // return all([getattr(model, trigger)(*args, **kwargs) for model in self.models])
    /// Triggers a list of callbacks
    pub(crate) fn callbacks(&self, funcs: &[&TriggerFunction], event_data: &EventData) {
        // for func in funcs:
        // self.callback(func, event_data)
        // _LOGGER.info("%sExecuted callback '%s'", self.name, func)
    }

    // def callback(self, func, event_data):
    // """ Trigger a callback function with passed event_data parameters. In case func is a string,
    //             the callable will be resolved from the passed model in event_data. This function is not intended to
    //             be called directly but through state and transition callback definitions.
    //         Args:
    //             func (str or callable): The callback function.
    //                 1. First, if the func is callable, just call it
    //                 2. Second, we try to import string assuming it is a path to a func
    //                 3. Fallback to a model attribute
    //             event_data (EventData): An EventData instance to pass to the
    //                 callback (if event sending is enabled) or to extract arguments
    //                 from (if event sending is disabled).
    //         """
    //
    // func = self.resolve_callable(func, event_data)
    // if self.send_event:
    // func(event_data)
    // else:
    // func(*event_data.args, **event_data.kwargs)

    // def _has_state(self, state, raise_error=False):
    // found = state in self.states.values()
    // if not found and raise_error:
    // msg = 'State %s has not been added to the machine' % (state.name if hasattr(state, 'name') else state)
    // raise ValueError(msg)
    // return found

    // def _process(self, trigger):
    //
    // # default processing
    // if not self.has_queue:
    // if not self._transition_queue:
    // # if trigger raises an Error, it has to be handled by the Machine.process caller
    // return trigger()
    // else:
    // raise MachineError("Attempt to process events synchronously while transition queue is not empty!")
    //
    // # process queued events
    // self._transition_queue.append(trigger)
    // # another entry in the queue implies a running transition; skip immediate execution
    // if len(self._transition_queue) > 1:
    // return True
    //
    // # execute as long as transition queue is not empty
    // while self._transition_queue:
    // try:
    // self._transition_queue[0]()
    // self._transition_queue.popleft()
    // except Exception:
    // # if a transition raises an exception, clear queue and delegate exception handling
    // self._transition_queue.clear()
    // raise
    // return True
    //
    // @classmethod
    // def _identify_callback(cls, name):
    // # Does the prefix match a known callback?
    // for callback in itertools.chain(cls.state_cls.dynamic_methods, cls.transition_cls.dynamic_methods):
    // if name.startswith(callback):
    // callback_type = callback
    // break
    // else:
    // return None, None
    //
    // # Extract the target by cutting the string after the type and separator
    // target = name[len(callback_type) + len(cls.SEPARATOR):]
    //
    // # Make sure there is actually a target to avoid index error and enforce _ as a separator
    // if target == '' or name[len(callback_type)] != cls.SEPARATOR:
    // return None, None
    //
    // return callback_type, target
    //
    // def __getattr__(self, name):
    // # Machine.__dict__ does not contain double underscore variables.
    // # Class variables will be mangled.
    // if name.startswith('__'):
    // raise AttributeError("'{}' does not exist on <Machine@{}>"
    // .format(name, id(self)))
    //
    // # Could be a callback
    // callback_type, target = self._identify_callback(name)
    //
    // if callback_type is not None:
    // if callback_type in self.transition_cls.dynamic_methods:
    // if target not in self.events:
    // raise AttributeError("event '{}' is not registered on <Machine@{}>"
    // .format(target, id(self)))
    // return partial(self.events[target].add_callback, callback_type)
    //
    // elif callback_type in self.state_cls.dynamic_methods:
    // state = self.get_state(target)
    // return partial(state.add_callback, callback_type[3:])
    //
    // try:
    // return self.__getattribute__(name)
    // except AttributeError:
    // # Nothing matched
    // raise AttributeError("'{}' does not exist on <Machine@{}>".format(name, id(self)))
    // "'{0}' instead.".format(meth_name), DeprecationWarning)
}
