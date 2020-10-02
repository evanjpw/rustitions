# rustitions
A Rust State Machine Library Modeled After PyTransitions

This is a dynamic state machine library for Rust, modeled after
the **[pytransitions/transitions](https://github.com/pytransitions/transitions)**
Python state machine package.

There are many fine state machine crates for Rust, typically able
to do compile time validation and consistency checking, and usually
leaning heavily on the macro system. These are awesome! You probably
would prefer to use one of these:
* [state_machine_future](https://crates.io/crates/state_machine_future)
* [machine](https://crates.io/crates/machine)
* [macro-machines](https://crates.io/crates/macro-machines)
* [smlang](https://crates.io/crates/smlang)
* [mode](https://crates.io/crates/mode)
* [state_machine](https://crates.io/crates/state_machine)
* [rust-fsm](https://crates.io/crates/rust-fsm)
  and [rust-fsm-dsl](https://crates.io/crates/rust-fsm-dsl)

or one of the many more choices.

So, why this crate?

Several reasons:
* Maybe you need a state machine defined dynamically at runtime.
* Maybe you need a state machine that can directly use the same
 state machine definition that 
 [PyTransitions](https://github.com/pytransitions/transitions)
  uses.
* Maybe you need a state machine that someone who is _not_ a Rust
 programmer can define.

Or, any of a variety of reasons.
