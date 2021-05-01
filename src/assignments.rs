use crate::computation::*;
use crate::consts::*;
use crate::objects::*;
use crate::variable_computations::*;

use std::cell::RefCell;
pub fn assign_pressed(
    key: u16,
    state: &mut Box<State>,
    string_writers: &mut Vec<RefCell<Box<dyn FnMut(String) -> ()>>>,
) -> () {
    if !state.entry_field.is_empty() {
        equals_pressed(state, string_writers)
    }
    let value_to_assign = state.variable_values.get(Variable::Ans);
    match key {
        button_number => {
            let variable = button_number_to_token(button_number).to_inner_var().expect("A non-variable token is attempted to be assigned to. Probably a mistake in button_number_to_token function");
            state.variable_values.set(variable, value_to_assign);
            update_unassignable_quantities(state);
        }
    }
}

pub fn add_to_var_pressed(
    key: u16,
    state: &mut Box<State>,
    string_writers: &mut Vec<RefCell<Box<dyn FnMut(String) -> ()>>>,
) -> () {
    if !state.entry_field.is_empty() {
        equals_pressed(state, string_writers)
    }
    let value_to_add = state.variable_values.get(Variable::Ans);
    match key {
        button_number => {
            let variable = button_number_to_token(button_number).to_inner_var().expect("A non-variable token is attempted to be assigned to. Probably a mistake in button_number_to_token function");
            state
                .variable_values
                .set(variable, value_to_add + state.variable_values.get(variable));
            update_unassignable_quantities(state);
        }
    }
}
