extern crate float_pretty_print;

mod assignments;
mod computation;
pub mod consts;
mod conversions;
mod enum_map;
pub mod objects;
mod variable_computations;

use crate::assignments::*;
use crate::computation::*;
use crate::consts::*;
use crate::conversions::*;
use crate::objects::*;

use std::cell::RefCell;

// These two variables hold the persistent state.
pub static mut STATE: Option<RefCell<Box<State>>> = None;
pub static mut STRING_DISPLAY_FUNCTIONS: Option<
    RefCell<Vec<RefCell<Box<dyn FnMut(String) -> ()>>>>,
> = None;

// This function gets called when a key gets pressed. The key values are specified in the consts
// module
pub fn key_pressed(
    key: u16,
    state_: &Option<RefCell<Box<State>>>,
    string_writers_: &Option<RefCell<Vec<RefCell<Box<dyn FnMut(String) -> ()>>>>>,
) -> () {
    let mut state = state_.as_ref().unwrap().borrow_mut();
    let mut string_writers = string_writers_.as_ref().unwrap().borrow_mut();
    match key {
        0..=99 => {
            token_pressed(key, &mut state, &mut string_writers);
        }
        100..=199 => assign_pressed(key, &mut state, &mut string_writers),
        200..=299 => add_to_var_pressed(key, &mut state, &mut string_writers),
        300..=399 => converstion_pressed(key, &mut state, &mut string_writers),
        1000..=1999 => match key {
            EQUALS_BUTTON => equals_pressed(&mut state, &mut string_writers),
            BACKSPACE_BUTTON => backspace_pressed(&mut state, &mut string_writers),
            CLEAR_BUTTON => clear_pressed(&mut state, &mut string_writers),
            _ => {}
        },
        _ => {
            panic! {"This math should have been exhaustive."}
        }
    }
}

pub fn entry_field_as_str(state: &mut Box<State>) -> String {
    let mut field = String::new();
    for tok in state.entry_field.iter() {
        field.push_str(&*tok.show());
    }

    return field;
}
pub fn prev_entry_field_as_str(state: &mut Box<State>) -> String {
    let mut field = String::new();
    for tok in state.previous_entry_field.iter() {
        field.push_str(&*tok.show());
    }

    return field;
}

fn token_pressed(
    key: u16,
    state: &mut Box<State>,
    string_writers: &mut Vec<RefCell<Box<dyn FnMut(String) -> ()>>>,
) {
    let token_to_add = match key {
        MINUS_BUTTON => {
            // Minus sign acts as the function negation in certain situations, e.g. as the first token
            if state.entry_field.is_empty()
                || *state.entry_field.back().unwrap() == Token::Parenth(Parenth::Left)
                || matches!(*state.entry_field.back().unwrap(), Token::Operator(_))
                || matches!(*state.entry_field.back().unwrap(), Token::Func(_))
            {
                Token::Func(Func::Negation)
            } else {
                Token::Operator(Operator::Minus)
            }
        }
        button_number => button_number_to_token(button_number),
    };

    //Add multiply between symbols if implied
    if let Some(previous_token) = state.entry_field.back() {
        if (matches!(token_to_add, Token::Func(_))
            || matches!(token_to_add, Token::Digit(_))
            || matches!(token_to_add, Token::Variable(_))
            || matches!(token_to_add, Token::Parenth(Parenth::Left)))
            && (matches!(previous_token, Token::Variable(_))
                || matches!(previous_token, Token::Parenth(Parenth::Right))
                || matches!(previous_token, Token::Digit(_)))
            && (!(matches!(token_to_add, Token::Digit(_))
                && matches!(previous_token, Token::Digit(_))))
        {
            state
                .entry_field
                .push_back(Token::Operator(Operator::Multiply));
        } else if (matches!(token_to_add, Token::Operator(_))
            && matches!(previous_token, Token::Operator(_)))
        {
            return ();
        }
    }
    //Check that an operator is placed in an appropriate place
    if let Some(previous_token) = state.entry_field.back() {
        if (matches!(token_to_add, Token::Operator(_))
            && (matches!(previous_token, Token::Operator(_))
                || matches!(previous_token, Token::Func(_))
                || matches!(previous_token, Token::Parenth(Parenth::Left))))
        {
            return;
        }
    }

    //Add ans at the beginning if line starts with an operator
    if state.entry_field.is_empty() && matches!(token_to_add, Token::Operator(_)) {
        state.entry_field.push_back(Token::Variable(Variable::Ans));
    }

    state.entry_field.push_back(token_to_add);

    update_string_field(state, string_writers, ENTRY_FIELD_WRITER);
}
fn backspace_pressed(
    state: &mut Box<State>,
    string_writers: &mut Vec<RefCell<Box<dyn FnMut(String) -> ()>>>,
) {
    if let None = state.entry_field.pop_back() {
        state.entry_field = state.previous_entry_field.clone();
        state.previous_entry_field.clear();
        state
            .variable_values
            .set(Variable::Ans, state.variable_values.get(Variable::PrevAns));
        state.variable_values.set(Variable::PrevAns, 0.0);
        update_string_field(state, string_writers, PREVIOUS_ENTRY_FIELD_WRITER);
    }
    update_string_field(state, string_writers, ENTRY_FIELD_WRITER);
}

fn clear_pressed(
    state: &mut Box<State>,
    string_writers: &mut Vec<RefCell<Box<dyn FnMut(String) -> ()>>>,
) {
    state.entry_field.clear();
    update_string_field(state, string_writers, ENTRY_FIELD_WRITER);
}

pub fn refresh_screen(
    state_: &Option<RefCell<Box<State>>>,
    string_writers_: &Option<RefCell<Vec<RefCell<Box<dyn FnMut(String) -> ()>>>>>,
) {
    let mut state = state_.as_ref().unwrap().borrow_mut();
    let mut string_writers = string_writers_.as_ref().unwrap().borrow_mut();
    for field_number in 0..string_writers.len() {
        update_string_field(&mut state, &mut string_writers, field_number);
    }
    state.refresh_variables_display();
}

pub fn update_string_field(
    state: &mut Box<State>,
    string_writers: &mut Vec<RefCell<Box<dyn FnMut(String) -> ()>>>,
    field: usize,
) {
    match field {
        ENTRY_FIELD_WRITER => {
            let mut new_entry = String::new();
            for tok in state.entry_field.iter() {
                new_entry.push_str(&*tok.show());
            }
            string_writers[ENTRY_FIELD_WRITER].borrow_mut()(new_entry);
        }
        PREVIOUS_ENTRY_FIELD_WRITER => {
            let mut new_entry = String::new();
            for tok in state.previous_entry_field.iter() {
                new_entry.push_str(&*tok.show());
            }
            string_writers[PREVIOUS_ENTRY_FIELD_WRITER].borrow_mut()(new_entry);
        }
        _ => {
            panic!("This match should be exhaustive.")
        }
    }
}
