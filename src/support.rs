//!

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct KeySet {}

impl Display for KeySet {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        unimplemented!()
    }
}

// def listify(obj):
// """Wraps a passed object into a list in case it has not been a list, tuple before.
//     Returns an empty list in case ``obj`` is None.
//     Args:
//         obj: instance to be converted into a list.
//     Returns:
//         list: May also return a tuple in case ``obj`` has been a tuple before.
//     """
// if obj is None:
// return []
//
// return obj if isinstance(obj, (list, tuple, EnumMeta)) else [obj]

// def _prep_ordered_arg(desired_length, arguments=None):
// """Ensure list of arguments passed to add_ordered_transitions has the proper length.
//     Expands the given arguments and apply same condition, callback
//     to all transitions if only one has been given.
//     Args:
//         desired_length (int): The size of the resulting list
//         arguments (optional[str, reference or list]): Parameters to be expanded.
//     Returns:
//         list: Parameter sets with the desired length.
//     """
// arguments = listify(arguments) if arguments is not None else [None]
// if len(arguments) != desired_length and len(arguments) != 1:
// raise ValueError("Argument length must be either 1 or the same length as "
// "the number of transitions.")
// if len(arguments) == 1:
// return arguments * desired_length
// return arguments
