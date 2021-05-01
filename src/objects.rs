use std::collections::VecDeque;
use std::f64::consts::PI;

use crate::enum_map::*;
use std::f64::NAN;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::cell::RefCell;
use std::rc::Rc;

//Persistent state of the calculator
pub struct State {
    pub entry_field: Box<VecDeque<Token>>,
    pub previous_entry_field: Box<VecDeque<Token>>,
    pub variable_values: EnumMapOfVars,
}

impl State {
    pub fn default<N>(gui_updaters: N) -> State
    //creates default values for variables but you still need to provide functions for displaying them
    where
        N: Fn(Variable) -> Rc<RefCell<Box<dyn FnMut(f64) -> ()>>>,
    {
        return State {
            entry_field: Box::new(VecDeque::new()),
            previous_entry_field: Box::new(VecDeque::new()),
            variable_values: EnumMapOfVars::new(
                |var| match var {
                    Variable::Altimeter => 29.9212,
                    Variable::Temp => 15.,
                    Variable::WindDir => 0.,
                    Variable::WindSpeed => 0.,
                    Variable::Altitude => 0.,
                    Variable::Ias => 0.,
                    Variable::Tas => 0.,
                    Variable::GrdSpd => 0.,
                    Variable::DensAlt => 0.,
                    Variable::PressAlt => 0.,
                    Variable::DevAngl => 0.,
                    Variable::HeadWind => 0.,
                    Variable::CrossWind => 0.,
                    Variable::Heading => 0.,
                    _ => NAN,
                },
                gui_updaters,
            ),
        };
    }

    // New state with prescribed variable values
    pub fn new<N, M>(gui_updaters: N, variable_values: M) -> State
    where
        N: Fn(Variable) -> Rc<RefCell<Box<dyn FnMut(f64) -> ()>>>,
        M: Fn(Variable) -> f64,
    {
        return State {
            entry_field: Box::new(VecDeque::new()),
            previous_entry_field: Box::new(VecDeque::new()),
            variable_values: EnumMapOfVars::new(variable_values, gui_updaters),
        };
    }

    pub fn refresh_variables_display(&self) -> () {
        for var in Variable::iter() {
            self.variable_values
                .gui_update_functions
                .get(var)
                .borrow_mut()(self.variable_values.values_of_variables.get(var));
        }
    }

    pub fn to_enumMap(&self) -> EnumMap<Variable, f64> {
        self.variable_values.values_of_variables.clone()
    }
    pub fn from_enum_map(&mut self, values: EnumMap<Variable, f64>) -> () {
        self.variable_values.values_of_variables = values;
        self.refresh_variables_display();
    }
}

// EnumMap of type Variable which also carries the callback functions to execute when values get
// set.
pub struct EnumMapOfVars {
    values_of_variables: EnumMap<Variable, f64>,
    gui_update_functions: EnumMap<Variable, Rc<RefCell<Box<dyn FnMut(f64) -> ()>>>>,
}

impl EnumMapOfVars {
    pub fn new<M, N>(values: M, functions: N) -> EnumMapOfVars
    where
        M: Fn(Variable) -> f64,
        N: Fn(Variable) -> Rc<RefCell<Box<dyn FnMut(f64) -> ()>>>,
    {
        let var_values = EnumMapOfVars {
            values_of_variables: EnumMap::new(values),
            gui_update_functions: EnumMap::new(functions),
        };
        return var_values;
    }
    pub fn get(&self, key: Variable) -> f64 {
        return self.values_of_variables.get(key);
    }
    pub fn set(&mut self, key: Variable, value: f64) -> () {
        self.values_of_variables.set(key, value);
        self.gui_update_functions.get(key).borrow_mut()(value);
    }
}
pub trait Show {
    fn show(&self) -> String;
}

//The entry field of the calculator is a sequence of tokens which can be digits, operators,
//functions, parentheses, variables,...
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    Operator(Operator),
    Digit(Digit),
    Parenth(Parenth),
    Variable(Variable),
    Func(Func),
    Num(f64),
}
impl Token {
    pub fn to_inner_var(&self) -> Option<Variable> {
        //Returns none if token is not a Variable
        if let Token::Variable(var) = *self {
            return Some(var);
        } else {
            return None;
        }
    }
}
impl Eq for Token {}

impl Show for Token {
    fn show(&self) -> String {
        match *self {
            Token::Operator(x) => x.show(),
            Token::Digit(x) => x.show(),
            Token::Parenth(x) => x.show(),
            Token::Variable(x) => x.show(),
            Token::Func(x) => x.show(),
            Token::Num(x) => x.to_string(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Func {
    Negation,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
}
impl Show for Func {
    fn show(&self) -> String {
        match *self {
            Func::Negation => "-".to_string(),
            Func::Sin => "sin\u{200a}".to_string(),
            Func::Cos => "cos\u{200a}".to_string(),
            Func::Tan => "tan\u{200a}".to_string(),
            Func::Arcsin => "asin\u{200a}".to_string(),
            Func::Arccos => "acos\u{200a}".to_string(),
            Func::Arctan => "atan\u{200a}".to_string(),
        }
    }
}

impl Func {
    pub fn apply(&self, x: f64) -> Result<f64, String> {
        match *self {
            Func::Negation => Ok(-x),
            Func::Sin => Ok(((x / 180.) * PI).sin()),
            Func::Cos => Ok(((x / 180.) * PI).cos()),
            Func::Tan => Ok(((x / 180.) * PI).tan()),
            Func::Arcsin => (x.asin() * (180. / PI)).to_result("argument out of range".to_string()),
            Func::Arccos => (x.acos() * (180. / PI)).to_result("argument out of range".to_string()),
            Func::Arctan => (x.atan() * (180. / PI)).to_result("argument out of range".to_string()),
        }
    }
}

// This is an interface for a type that itself implements Result-like behavior, e.g. f64 can be
// NAN
pub trait Resultable {
    type Item;
    fn to_result(&self, error: String) -> Result<Self::Item, String>;
}

impl Resultable for f64 {
    type Item = f64;
    fn to_result(&self, error: String) -> Result<f64, String> {
        if self.is_nan() {
            return Err(error);
        } else {
            return Ok(*self);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
pub enum Variable {
    Ans,
    PrevAns,
    A,
    B,
    C,
    D,
    Ias,
    Tas,
    Altitude,
    Altimeter,
    Temp,
    DensAlt,
    PressAlt,
    WindDir,
    WindSpeed,
    Heading,
    HeadWind,
    CrossWind,
    DevAngl,
    GrdSpd,
}

impl Show for Variable {
    fn show(&self) -> String {
        match *self {
            Variable::Ans => "ans".to_string(),
            Variable::PrevAns => "".to_string(),
            Variable::A => "a".to_string(),
            Variable::B => "b".to_string(),
            Variable::C => "c".to_string(),
            Variable::D => "d".to_string(),
            Variable::Ias => "ias".to_string(),
            Variable::Tas => "tas".to_string(),
            Variable::Altitude => "alt".to_string(),
            Variable::Altimeter => "prst".to_string(),
            Variable::Temp => "tmp".to_string(),
            Variable::DensAlt => "dal".to_string(),
            Variable::PressAlt => "pal".to_string(),
            Variable::WindDir => "wdr".to_string(),
            Variable::WindSpeed => "wsp".to_string(),
            Variable::Heading => "hdg".to_string(),
            Variable::HeadWind => "hdwd".to_string(),
            Variable::CrossWind => "crwd".to_string(),
            Variable::DevAngl => "dang".to_string(),
            Variable::GrdSpd => "grsp".to_string(),
        }
    }
}

impl Variable {
    pub fn substitute(&self, state: &mut State) -> f64 {
        return state.variable_values.get(*self);
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}
impl Show for Operator {
    fn show(&self) -> String {
        match *self {
            Operator::Plus => "+".to_string(),
            Operator::Minus => "-".to_string(),
            Operator::Multiply => "*".to_string(),
            Operator::Divide => "/".to_string(),
        }
    }
}

impl Operator {
    pub fn apply(&self, a: f64, b: f64) -> Result<f64, String> {
        match *self {
            Operator::Plus => Ok(a + b),
            Operator::Minus => Ok(a - b),
            Operator::Multiply => Ok(a * b),
            Operator::Divide => {
                if b.abs() > 0.0000001 {
                    Ok(a / b)
                } else {
                    Err("Can't divide by 0".to_string())
                }
            }
        }
    }
    pub fn precedence(&self) -> u8 {
        match *self {
            Operator::Plus => 1,
            Operator::Minus => 2,
            Operator::Multiply => 3,
            Operator::Divide => 4,
        }
    }
    pub fn associativity(&self) -> Associativity {
        match *self {
            Operator::Plus => Associativity::Left,
            Operator::Minus => Associativity::Left,
            Operator::Multiply => Associativity::Left,
            Operator::Divide => Associativity::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Digit {
    //My digits include period so that a sequence of digits could become a float
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Period,
}

impl Show for Digit {
    fn show(&self) -> String {
        match *self {
            Digit::Zero => "0".to_string(),
            Digit::One => "1".to_string(),
            Digit::Two => "2".to_string(),
            Digit::Three => "3".to_string(),
            Digit::Four => "4".to_string(),
            Digit::Five => "5".to_string(),
            Digit::Six => "6".to_string(),
            Digit::Seven => "7".to_string(),
            Digit::Eight => "8".to_string(),
            Digit::Nine => "9".to_string(),
            Digit::Period => ".".to_string(),
        }
    }
}
impl Digit {
    pub fn to_int(&self) -> Result<u8, String> {
        match *self {
            Digit::Zero => Ok(0),
            Digit::One => Ok(1),
            Digit::Two => Ok(2),
            Digit::Three => Ok(3),
            Digit::Four => Ok(4),
            Digit::Five => Ok(5),
            Digit::Six => Ok(6),
            Digit::Seven => Ok(7),
            Digit::Eight => Ok(8),
            Digit::Nine => Ok(9),
            Digit::Period => Err("trying to convert period to int".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Parenth {
    Left,
    Right,
}

impl Show for Parenth {
    fn show(&self) -> String {
        match *self {
            Parenth::Right => ")".to_string(),
            Parenth::Left => "(".to_string(),
        }
    }
}

pub enum Unit {
    NauticalMile,
    Mile,
    Kilometer,
    Foot,
    Meter,
    Knot,
    FeetPerMinute,
    MilesPerHour,
    KilometersPerHour,
    MetersPerMinute,
    Liter,
    Gallon,
    Kilogram,
    Pound,
    Celcius,
    Kelvin,
    Fahrenheit,
    AvigasPound,
    JetfuelPound,
    InHg,
    Kilopascal,
    Hectopascal,
    Pascal,
}

impl Unit {
    fn to_basic_unit(self, x: f64) -> f64 {
        match self {
            Unit::NauticalMile => x * 1852.,
            Unit::Mile => x * 1609.34,
            Unit::Kilometer => x * 1000.,
            Unit::Foot => x * 0.3048,
            Unit::Meter => x,
            Unit::Knot => x * 1.852,
            Unit::FeetPerMinute => x * 0.3048 * 60. / 1000.,
            Unit::MilesPerHour => x * 1.60934,
            Unit::KilometersPerHour => x,
            Unit::MetersPerMinute => x * 1000. / 60.,
            Unit::Liter => x / 3.78541,
            Unit::Gallon => x,
            Unit::Kilogram => x,
            Unit::Pound => x * 0.453592,
            Unit::Celcius => x,
            Unit::Kelvin => x - 273.15,
            Unit::Fahrenheit => (x - 32.) * 5. / 9.,
            Unit::AvigasPound => x / 6.01,
            Unit::JetfuelPound => x / 6.71,
            Unit::InHg => x * 33.8639,
            Unit::Kilopascal => x * 10.,
            Unit::Hectopascal => x,
            Unit::Pascal => x / 100.,
        }
    }
    fn from_basic_unit(self, x: f64) -> f64 {
        match self {
            Unit::NauticalMile => x / 1852.,
            Unit::Mile => x / 1609.34,
            Unit::Kilometer => x / 1000.,
            Unit::Foot => x / 0.3048,
            Unit::Meter => x,
            Unit::Knot => x / 1.852,
            Unit::FeetPerMinute => x * 1000. / (0.3048 * 60.),
            Unit::MilesPerHour => x / 1.60934,
            Unit::KilometersPerHour => x,
            Unit::MetersPerMinute => x * 60. / 1000.,
            Unit::Liter => x * 3.78541,
            Unit::Gallon => x,
            Unit::Kilogram => x,
            Unit::Pound => x / 0.453592,
            Unit::Celcius => x,
            Unit::Kelvin => x + 273.15,
            Unit::Fahrenheit => (x * 9. / 5.) + 32.,
            Unit::AvigasPound => x * 6.01,
            Unit::JetfuelPound => x * 6.71,
            Unit::InHg => x / 33.8639,
            Unit::Kilopascal => x / 10.,
            Unit::Hectopascal => x,
            Unit::Pascal => x * 100.,
        }
    }
}
pub trait Convertable {
    fn convert(self, from: Unit, to: Unit) -> Self;
}
impl Convertable for f64 {
    fn convert(self, from: Unit, to: Unit) -> f64 {
        to.from_basic_unit(from.to_basic_unit(self))
    }
}

#[cfg(test)]
#[test]
fn test_conversion() {
    assert!(((15.0 as f64).convert(Unit::Liter, Unit::Gallon) - 3.96258).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Gallon, Unit::Liter) - 56.7812).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Knot, Unit::MilesPerHour) - 17.2617).abs() < 0.0001);
    assert!(
        ((15.0 as f64).convert(Unit::MilesPerHour, Unit::KilometersPerHour) - 24.1402).abs()
            < 0.0001
    );
    assert!(((15.0 as f64).convert(Unit::InHg, Unit::Hectopascal) - 507.96).abs() < 0.01);
    assert!(((15.0 as f64).convert(Unit::InHg, Unit::Kilopascal) - 50.796).abs() < 0.01);
    assert!(((15.0 as f64).convert(Unit::Kelvin, Unit::Celcius) - (-258.15)).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Celcius, Unit::Kelvin) - (288.15)).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Celcius, Unit::Fahrenheit) - 59.).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Foot, Unit::Mile) - 0.00284091).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Foot, Unit::NauticalMile) - 0.00246868).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Foot, Unit::Kilometer) - 0.004572).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Gallon, Unit::JetfuelPound) - 100.65).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Gallon, Unit::AvigasPound) - 90.15).abs() < 0.0001);
    assert!(((15.0 as f64).convert(Unit::Knot, Unit::FeetPerMinute) - 1519.03).abs() < 0.01);
    assert!(((15.0 as f64).convert(Unit::Kilogram, Unit::Pound) - 33.0693).abs() < 0.0001);
}
