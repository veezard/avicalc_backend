use crate::computation::*;
use crate::consts::*;
use crate::objects::*;

use std::cell::RefCell;

pub fn converstion_pressed(
    key: u16,
    state: &mut Box<State>,
    string_writers: &mut Vec<RefCell<Box<dyn FnMut(String) -> ()>>>,
) -> () {
    if !state.entry_field.is_empty() {
        equals_pressed(state, string_writers)
    }
    let units = button_number_to_pair_of_units(key);
    let original_answer = state.variable_values.get(Variable::Ans);
    let converted_answer = original_answer.convert(units.0, units.1);
    state.variable_values.set(Variable::Ans, converted_answer);
}
