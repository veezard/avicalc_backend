use super::*;
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
    let value_to_assign: f64;
    if !state.entry_field.is_empty() {
        let result = compute_entry_field(state);
        match result {
            Err(err) => {
                string_writers[ENTRY_FIELD_WRITER].borrow_mut()(err);
                return ();
            }
            Ok(answer) => {
                state.entry_field.clear();
                state
                    .variable_values
                    .set(Variable::PrevAns, state.variable_values.get(Variable::Ans));
                state.variable_values.set(Variable::Ans, answer);
                update_string_field(state, string_writers, ENTRY_FIELD_WRITER);
                value_to_assign = answer;
            }
        }
    } else {
        value_to_assign = state.variable_values.get(Variable::Ans);
    }
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
    let value_to_add: f64;
    if !state.entry_field.is_empty() {
        let result = compute_entry_field(state);
        match result {
            Err(err) => {
                string_writers[ENTRY_FIELD_WRITER].borrow_mut()(err);
                return ();
            }
            Ok(answer) => {
                state.entry_field.clear();
                state
                    .variable_values
                    .set(Variable::PrevAns, state.variable_values.get(Variable::Ans));
                state.variable_values.set(Variable::Ans, answer);
                update_string_field(state, string_writers, ENTRY_FIELD_WRITER);
                value_to_add = answer;
            }
        }
    } else {
        value_to_add = state.variable_values.get(Variable::Ans);
    }
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
