use super::*;
use crate::computation::*;
use crate::consts::*;
use crate::objects::*;

use std::cell::RefCell;

pub fn converstion_pressed(
    key: u16,
    state: &mut Box<State>,
    string_writers: &mut Vec<RefCell<Box<dyn FnMut(String) -> ()>>>,
) -> () {
    let value_to_convert: f64;
    if !state.entry_field.is_empty() {
        let result = compute_entry_field(state);
        match result {
            Err(err) => {
                string_writers[ENTRY_FIELD_WRITER].borrow_mut()(err);
                return ();
            }
            Ok(answer) => {
                state.entry_field.clear();
                update_string_field(state, string_writers, ENTRY_FIELD_WRITER);
                value_to_convert = answer;
            }
        }
    } else {
        value_to_convert = state.variable_values.get(Variable::Ans);
    }
    let units = button_number_to_pair_of_units(key);
    let original_answer = value_to_convert;
    let converted_answer = original_answer.convert(units.0, units.1);
    state.variable_values.set(Variable::Ans, converted_answer);
}
