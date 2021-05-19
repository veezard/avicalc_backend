//This example runs avical in the terminal.
//See the key assignments at the bottom of this file.
extern crate avicalc_backend;
extern crate termion;

use avicalc_backend::consts::*;
use avicalc_backend::objects::*;
use avicalc_backend::*;

use std::cell::RefCell;
use std::rc::Rc;

use float_pretty_print::PrettyPrintFloat;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn float_to_pretty_string(number: f64) -> String {
    if number.is_nan() {
        "".to_string()
    } else if number.fract() < 0.0001 && number < 10000. && number.round().abs() >= 1. {
        number.round().to_string()
    } else if number.abs() < 0.0000000001 {
        "0".to_string()
    } else {
        format!("{:0.6}", PrettyPrintFloat(number))
    }
}
fn print_number_at_location(number: f64, label: &str, x: u16, y: u16) -> () {
    let mut stdout__ = stdout().into_raw_mode().unwrap();
    write!(
        stdout__,
        "{}{}{}{}{}{}{}",
        termion::cursor::Goto(x, y),
        "               ",
        termion::cursor::Goto(x, y),
        label.to_string(),
        float_to_pretty_string(number),
        termion::cursor::Goto(1, 7),
        termion::cursor::Hide,
    )
    .unwrap();
    stdout__.flush().unwrap();
}
fn main() {
    unsafe {
        //------------Initialize the function used to write on calculator screen

        fn gui_update_functions(var: Variable) -> Rc<RefCell<Box<dyn FnMut(f64) -> ()>>> {
            if var == Variable::Ans {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "ans=", 1, 3);
                })))
            } else if var == Variable::A {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "a=", 40, 1);
                })))
            } else if var == Variable::B {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "b=", 55, 1);
                })))
            } else if var == Variable::C {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "c=", 70, 1);
                })))
            } else if var == Variable::D {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "d=", 85, 1);
                })))
            } else if var == Variable::Cas {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "cas=", 40, 2);
                })))
            } else if var == Variable::Tas {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "tas=", 55, 2);
                })))
            } else if var == Variable::Altitude {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "alt=", 70, 2);
                })))
            } else if var == Variable::Altimeter {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "prst=", 85, 2);
                })))
            } else if var == Variable::Temp {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "tmp=", 40, 3);
                })))
            } else if var == Variable::DensAlt {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "dal=", 55, 3);
                })))
            } else if var == Variable::PressAlt {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "pal=", 70, 3);
                })))
            } else if var == Variable::WindDir {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "wdr=", 85, 3);
                })))
            } else if var == Variable::WindSpeed {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "wsp=", 40, 4);
                })))
            } else if var == Variable::Course {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "crs=", 55, 4);
                })))
            } else if var == Variable::HeadWind {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "hdwd=", 70, 4);
                })))
            } else if var == Variable::CrossWind {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "crwd=", 85, 4);
                })))
            } else if var == Variable::Heading {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "hdg=", 40, 5);
                })))
            } else if var == Variable::GrdSpd {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "grsp=", 55, 5);
                })))
            } else if var == Variable::DewPoint {
                Rc::new(RefCell::new(Box::new(|number: f64| {
                    print_number_at_location(number, "dwpt=", 70, 5);
                })))
            } else {
                {
                    Rc::new(RefCell::new(Box::new(|_| {})))
                }
            }
        }

        let entry_field_writer = Box::new(|entry: String| {
            let mut stdout__ = stdout().into_raw_mode().unwrap();
            write!(
                stdout__,
                "{}{}{}{}{}{}",
                termion::cursor::Goto(1, 1),
                "                                       ".to_string(),
                termion::cursor::Goto(1, 1),
                entry,
                termion::cursor::Goto(1, 7),
                termion::cursor::Hide,
            )
            .unwrap();
            stdout__.flush().unwrap();
        });
        let previous_entry_field_writer = Box::new(|entry: String| {
            let mut stdout__ = stdout().into_raw_mode().unwrap();
            write!(
                stdout__,
                "{}{}{}{}{}{}",
                termion::cursor::Goto(1, 2),
                "                                       ".to_string(),
                termion::cursor::Goto(1, 2),
                entry,
                termion::cursor::Goto(1, 7),
                termion::cursor::Hide,
            )
            .unwrap();
            stdout__.flush().unwrap();
        });
        //--------------Initialize state of the calculator
        STATE = Some(RefCell::new(Box::new(State::default(gui_update_functions))));

        STRING_DISPLAY_FUNCTIONS = Some(RefCell::new(vec![
            RefCell::new(entry_field_writer),
            RefCell::new(previous_entry_field_writer),
        ]));
    }

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 7),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
    unsafe {
        refresh_screen(&STATE, &STRING_DISPLAY_FUNCTIONS);
    }
    for key in stdin.keys() {
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, 7),
            "   ".to_string(),
            termion::cursor::Goto(1, 7),
        )
        .unwrap();
        stdout.flush().unwrap();

        unsafe {
            let key_pressed_wrapper =
                |button_number: u16| key_pressed(button_number, &STATE, &STRING_DISPLAY_FUNCTIONS);
            match key.unwrap() {
                Key::Char('q') => break,
                Key::Backspace => key_pressed_wrapper(BACKSPACE_BUTTON),
                Key::Char('\\') => key_pressed_wrapper(CLEAR_BUTTON),
                Key::Char('0') => key_pressed_wrapper(ZERO_BUTTON),
                Key::Char('1') => key_pressed_wrapper(ONE_BUTTON),
                Key::Char('2') => key_pressed_wrapper(TWO_BUTTON),
                Key::Char('3') => key_pressed_wrapper(THREE_BUTTON),
                Key::Char('4') => key_pressed_wrapper(FOUR_BUTTON),
                Key::Char('5') => key_pressed_wrapper(FIVE_BUTTON),
                Key::Char('6') => key_pressed_wrapper(SIX_BUTTON),
                Key::Char('7') => key_pressed_wrapper(SEVEN_BUTTON),
                Key::Char('8') => key_pressed_wrapper(EIGHT_BUTTON),
                Key::Char('9') => key_pressed_wrapper(NINE_BUTTON),
                Key::Char('.') => key_pressed_wrapper(PERIOD_BUTTON),
                Key::Char('+') => key_pressed_wrapper(PLUS_BUTTON),
                Key::Char('-') => key_pressed_wrapper(MINUS_BUTTON),
                Key::Char('*') => key_pressed_wrapper(MULTIPLY_BUTTON),
                Key::Char('/') => key_pressed_wrapper(DIVIDE_BUTTON),
                Key::Char('=') => key_pressed_wrapper(EQUALS_BUTTON),
                Key::Char('(') => key_pressed_wrapper(LEFT_PARENTH_BUTTON),
                Key::Char(')') => key_pressed_wrapper(RIGHT_PARENTH_BUTTON),
                Key::Char('z') => key_pressed_wrapper(SIN_BUTTON),
                Key::Char('x') => key_pressed_wrapper(COS_BUTTON),
                Key::Char('c') => key_pressed_wrapper(TAN_BUTTON),
                Key::Char('v') => key_pressed_wrapper(ARCSIN_BUTTON),
                Key::Char('b') => key_pressed_wrapper(ARCCOS_BUTTON),
                Key::Char('n') => key_pressed_wrapper(ARCTAN_BUTTON),
                Key::Char('a') => key_pressed_wrapper(ANS_BUTTON),
                Key::Char('s') => key_pressed_wrapper(A_BUTTON),
                Key::Char('d') => key_pressed_wrapper(B_BUTTON),
                Key::Char('f') => key_pressed_wrapper(C_BUTTON),
                Key::Char('g') => key_pressed_wrapper(D_BUTTON),
                Key::Char('h') => key_pressed_wrapper(TEMP_BUTTON),
                Key::Char('j') => key_pressed_wrapper(ALTITUDE_BUTTON),
                Key::Char('k') => key_pressed_wrapper(ALTIMETER_BUTTON),
                Key::Char('l') => key_pressed_wrapper(WIND_SPEED_BUTTON),
                Key::Char(';') => key_pressed_wrapper(WIND_HEADING_BUTTON),
                Key::Char(']') => key_pressed_wrapper(COURSE_BUTTON),
                Key::Char('[') => key_pressed_wrapper(CALIBRATED_AIRSPEED_BUTTON),
                Key::Char('p') => key_pressed_wrapper(PRESSURE_ALTITUDE_BUTTON),
                Key::Char('o') => key_pressed_wrapper(DENSITY_ALTITUDE_BUTTON),
                Key::Char('i') => key_pressed_wrapper(HEAD_WIND_BUTTON),
                Key::Char('u') => key_pressed_wrapper(CROSS_WIND_BUTTON),
                Key::Char('y') => key_pressed_wrapper(DEVIATION_ANGLE_BUTTON),
                Key::Char('t') => key_pressed_wrapper(TRUE_AIRSPEED_BUTTON),
                Key::Char('r') => key_pressed_wrapper(GROUND_SPEED_BUTTON),
                Key::Char('S') => key_pressed_wrapper(A_ASSIGN_BUTTON),
                Key::Char('D') => key_pressed_wrapper(B_ASSIGN_BUTTON),
                Key::Char('F') => key_pressed_wrapper(C_ASSIGN_BUTTON),
                //Key::Char('G') => key_pressed_wrapper(D_ASSIGN_BUTTON),
                Key::Char('G') => key_pressed_wrapper(DEW_POINT_ASSIGN_BUTTON),
                Key::Char('H') => key_pressed_wrapper(TEMP_ASSIGN_BUTTON),
                Key::Char('J') => key_pressed_wrapper(ALTITUDE_ASSIGN_BUTTON),
                Key::Char('K') => key_pressed_wrapper(ALTIMETER_ASSIGN_BUTTON),
                Key::Char('L') => key_pressed_wrapper(WIND_SPEED_ASSIGN_BUTTON),
                Key::Char(':') => key_pressed_wrapper(WIND_HEADING_ASSIGN_BUTTON),
                Key::Char('}') => key_pressed_wrapper(COURSE_ASSIGN_BUTTON),
                Key::Char('{') => key_pressed_wrapper(CALIBRATED_AIRSPEED_ASSIGN_BUTTON),

                //Key::Ctrl('s') => key_pressed_wrapper(A_ADD_BUTTON),
                //Key::Ctrl('d') => key_pressed_wrapper(B_ADD_BUTTON),
                //Key::Ctrl('f') => key_pressed_wrapper(C_ADD_BUTTON),
                //Key::Ctrl('g') => key_pressed_wrapper(D_ADD_BUTTON),
                //Key::Ctrl('h') => key_pressed_wrapper(TEMP_ADD_BUTTON),
                //Key::Ctrl('j') => key_pressed_wrapper(ALTITUDE_ADD_BUTTON),
                //Key::Ctrl('k') => key_pressed_wrapper(ALTIMETER_ADD_BUTTON),
                //Key::Ctrl('l') => key_pressed_wrapper(WIND_SPEED_ADD_BUTTON),
                //Key::Ctrl(';') => key_pressed_wrapper(WIND_HEADING_ADD_BUTTON),
                //Key::Ctrl(']') => key_pressed_wrapper(COURSE_ADD_BUTTON),
                //Key::Ctrl('[') => key_pressed_wrapper(CALIBRATED_AIRSPEED_ADD_BUTTON),

                //Key::Char('A') => key_pressed_wrapper(NM_TO_FEET_BUTTON),
                //Key::Char('S') => key_pressed_wrapper(FEET_TO_NM_BUTTON),
                //Key::Char('D') => key_pressed_wrapper(KNOTS_TO_FPM_BUTTON),
                //Key::Char('F') => key_pressed_wrapper(FPM_TO_KNOTS_BUTTON),
                //Key::Char('G') => key_pressed_wrapper(KPH_TO_MPM_BUTTON),
                //Key::Char('H') => key_pressed_wrapper(MPM_TO_KPH_BUTTON),
                //Key::Char('J') => key_pressed_wrapper(NM_TO_MILES_BUTTON),
                //Key::Char('K') => key_pressed_wrapper(MILES_TO_NM_BUTTON),
                //Key::Char('L') => key_pressed_wrapper(MILES_TO_KILOMETERS_BUTTON),
                //Key::Char(':') => key_pressed_wrapper(KILOMETERS_TO_MILES_BUTTON),
                //Key::Char('"') => key_pressed_wrapper(KNOTS_TO_MPH_BUTTON),
                //Key::Char('Z') => key_pressed_wrapper(MPH_TO_KNOTS_BUTTON),
                //Key::Char('X') => key_pressed_wrapper(C_TO_F_BUTTON),
                //Key::Char('C') => key_pressed_wrapper(F_TO_C_BUTTON),
                //Key::Char('V') => key_pressed_wrapper(LBS_TO_KGS_BUTTON),
                //Key::Char('B') => key_pressed_wrapper(KGS_TO_LBS_BUTTON),
                //Key::Char('N') => key_pressed_wrapper(LITERS_TO_GALLONS_BUTTON),
                //Key::Char('M') => key_pressed_wrapper(GALLONS_TO_LITTERS_BUTTON),
                //Key::Char('<') => key_pressed_wrapper(GALLONS_TO_AVIGAS_LBS_BUTTON),
                //Key::Char('>') => key_pressed_wrapper(GALLONS_TO_JET_FUEL_LBS_BUTTON),
                //Key::Char('?') => key_pressed_wrapper(INHG_TO_HECTOPASCALS_BUTTON),
                //Key::Char('}') => key_pressed_wrapper(HECTOPASCALS_TO_INHG_BUTTON),
                _ => {}
            }
        }
    }
}
