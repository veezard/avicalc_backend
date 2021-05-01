use super::*;
use crate::consts::*;
use crate::objects::*;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::result::Result;

pub fn equals_pressed(
    state: &mut Box<State>,
    string_writers: &mut Vec<RefCell<Box<dyn FnMut(String) -> ()>>>,
) {
    /*Implements shunting yard algorithm */

    let rpn_notation_maybe: Result<VecDeque<Token>, String> = entry_field_to_rpn(state);
    if let Err(err) = rpn_notation_maybe {
        string_writers[ENTRY_FIELD_WRITER].borrow_mut()(err);
        return ();
    }
    let mut rpn_notation = rpn_notation_maybe.unwrap();

    match evaluate_rpn(&mut rpn_notation) {
        Err(err) => {
            string_writers[ENTRY_FIELD_WRITER].borrow_mut()(err);
            return ();
        }
        Ok(answer) => {
            state
                .variable_values
                .set(Variable::PrevAns, state.variable_values.get(Variable::Ans));
            state.variable_values.set(Variable::Ans, answer);
            state.previous_entry_field = state.entry_field.clone();
            state.entry_field.clear();
            update_string_field(state, string_writers, ENTRY_FIELD_WRITER);
            update_string_field(state, string_writers, PREVIOUS_ENTRY_FIELD_WRITER);
            return ();
        }
    }
}
/*rpn stands for reverse polish notation*/
//This function also substitutes constants.
fn entry_field_to_rpn(state: &mut Box<State>) -> Result<VecDeque<Token>, String> {
    let mut rpn_notation: VecDeque<Token> = VecDeque::new();

    let mut operators_stack: Vec<Token> = Vec::new();
    let mut entry_field = state.entry_field.clone();

    'outer: loop {
        if entry_field.is_empty() {
            loop {
                if operators_stack.is_empty() {
                    break 'outer;
                } else {
                    rpn_notation.push_back(
                        operators_stack
                            .pop()
                            .ok_or("popping empty operator field".to_string())?,
                    );
                }
            }
        } else {
            let temp_token1: Token = entry_field
                .pop_front()
                .ok_or("popping empty entry field".to_string())?;
            match temp_token1 {
                Token::Digit(_) => {
                    entry_field.push_front(temp_token1);
                    rpn_notation.push_back(Token::Num(digits_to_float(&mut entry_field)?));
                }
                Token::Num(_) => rpn_notation.push_back(temp_token1),
                Token::Parenth(Parenth::Left) => operators_stack.push(temp_token1),
                Token::Parenth(Parenth::Right) => loop {
                    if operators_stack.is_empty() {
                        return Err("mismatched parentheses".to_string());
                    }
                    let temp_token2: Token = operators_stack
                        .pop()
                        .ok_or("popping empty operator stack".to_string())?;
                    match temp_token2 {
                        Token::Parenth(Parenth::Right) => {
                            return Err("mismatched parentheses".to_string());
                        }
                        Token::Parenth(Parenth::Left) => break,
                        op => rpn_notation.push_back(op),
                    }
                },
                Token::Variable(var) => rpn_notation.push_back(Token::Num(var.substitute(state))),
                Token::Func(_) => operators_stack.push(temp_token1),
                Token::Operator(op) => {
                    if operators_stack.is_empty() {
                        operators_stack.push(temp_token1)
                    } else {
                        let temp_token2: Token = operators_stack
                            .pop()
                            .ok_or("poppoing empty operator stack".to_string())?;
                        match temp_token2 {
                            Token::Func(_) => {
                                rpn_notation.push_back(temp_token2);
                                entry_field.push_front(temp_token1);
                            }
                            Token::Parenth(_) => {
                                operators_stack.push(temp_token2);
                                operators_stack.push(temp_token1);
                            }
                            Token::Operator(stack_operator) => {
                                if op.precedence() > stack_operator.precedence() {
                                    operators_stack.push(temp_token2);
                                    operators_stack.push(temp_token1);
                                } else if op.precedence() < stack_operator.precedence() {
                                    rpn_notation.push_back(temp_token2);
                                    entry_field.push_front(temp_token1);
                                } else {
                                    if op != stack_operator {
                                        panic!("no two operators should have same precedence");
                                    } else if op.associativity() == Associativity::Left {
                                        rpn_notation.push_back(temp_token2);
                                        entry_field.push_front(temp_token1);
                                    } else if op.associativity() == Associativity::Right {
                                        operators_stack.push(temp_token2);
                                        operators_stack.push(temp_token1);
                                    }
                                }
                            }
                            _ => panic!("operator stack has nonoperators"),
                        }
                    }
                }
            }
        }
    }
    return Ok(rpn_notation);
}
fn evaluate_rpn(rpn_notation: &mut VecDeque<Token>) -> Result<f64, String> {
    /*destroys rpn queue*/
    if rpn_notation.is_empty() {
        return Ok(0.0);
    };
    let mut num_stack: Vec<f64> = Vec::new();

    loop {
        if rpn_notation.is_empty() {
            if num_stack.len() != 1 {
                return Err("not well-formed".to_string());
            }
            return num_stack.pop().ok_or("not well-formed".to_string());
        }
        let temp_token3: Token = rpn_notation
            .pop_front()
            .ok_or("popping empty rpn stack".to_string())?;
        match temp_token3 {
            Token::Num(num) => num_stack.push(num),
            Token::Func(f) => {
                let arg = num_stack.pop().ok_or("not well-formed".to_string())?;
                num_stack.push(f.apply(arg)?);
            }
            Token::Operator(op) => {
                let arg2 = num_stack.pop().ok_or("not well-formed".to_string())?;
                let arg1 = num_stack.pop().ok_or("not well-formed".to_string())?;
                num_stack.push(op.apply(arg1, arg2)?);
            }
            _ => {
                return Err("not well-formed".to_string());
            }
        }
    }
}

fn digits_to_float(entry_field: &mut VecDeque<Token>) -> Result<f64, String> {
    //The function will read digits from two-sided queue and return the corresponding float with
    //digits popped

    let mut result: f64 = 0.0;
    let mut current_token: Token;
    let mut past_decimal_point: bool = false;
    let mut decimal_position: i32 = 0;
    loop {
        if entry_field.is_empty() {
            return Ok(result);
        }
        current_token = entry_field.pop_front().unwrap();
        match current_token {
            Token::Digit(Digit::Period) => {
                if past_decimal_point {
                    return Err("digits can't be parsed as a number".to_string());
                } else {
                    past_decimal_point = true;
                    decimal_position += 1;
                }
            }
            Token::Digit(digit) => {
                if !past_decimal_point {
                    result = result * 10. + digit.to_int().unwrap() as f64;
                } else {
                    result = result
                        + (10.0 as f64).powf(-decimal_position as f64)
                            * digit.to_int().unwrap() as f64;
                    decimal_position += 1;
                }
            }
            _ => {
                entry_field.push_front(current_token);
                return Ok(result);
            }
        }
    }
}
