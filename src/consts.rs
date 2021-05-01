use crate::objects::*;

//----------------String display function numbers: correspond to elements in the vector STRING_DISPLAY_FUNCTIONS
pub const ENTRY_FIELD_WRITER: usize = 0;
pub const PREVIOUS_ENTRY_FIELD_WRITER: usize = 1;

// ----------------Button numbers-----------------
// Separately handled buttons >=1000
pub const EQUALS_BUTTON: u16 = 1000;
pub const BACKSPACE_BUTTON: u16 = 1001;
pub const CLEAR_BUTTON: u16 = 1002;

// Add symbol to entry field buttons 0-99
pub const ZERO_BUTTON: u16 = 0;
pub const ONE_BUTTON: u16 = 1;
pub const TWO_BUTTON: u16 = 2;
pub const THREE_BUTTON: u16 = 3;
pub const FOUR_BUTTON: u16 = 4;
pub const FIVE_BUTTON: u16 = 5;
pub const SIX_BUTTON: u16 = 6;
pub const SEVEN_BUTTON: u16 = 7;
pub const EIGHT_BUTTON: u16 = 8;
pub const NINE_BUTTON: u16 = 9;
pub const PERIOD_BUTTON: u16 = 10;

pub const PLUS_BUTTON: u16 = 11;
pub const MINUS_BUTTON: u16 = 12;
pub const MULTIPLY_BUTTON: u16 = 13;
pub const DIVIDE_BUTTON: u16 = 14;

pub const LEFT_PARENTH_BUTTON: u16 = 15;
pub const RIGHT_PARENTH_BUTTON: u16 = 16;

pub const SIN_BUTTON: u16 = 21;
pub const COS_BUTTON: u16 = 22;
pub const TAN_BUTTON: u16 = 23;
pub const ARCSIN_BUTTON: u16 = 24;
pub const ARCCOS_BUTTON: u16 = 25;
pub const ARCTAN_BUTTON: u16 = 26;

pub const ANS_BUTTON: u16 = 30;

pub const A_BUTTON: u16 = 41;
pub const B_BUTTON: u16 = 42;
pub const C_BUTTON: u16 = 43;
pub const D_BUTTON: u16 = 44;

pub const TEMP_BUTTON: u16 = 51;
pub const ALTITUDE_BUTTON: u16 = 52;
pub const ALTIMETER_BUTTON: u16 = 53;
pub const WIND_SPEED_BUTTON: u16 = 54;
pub const WIND_HEADING_BUTTON: u16 = 55;
pub const HEADING_BUTTON: u16 = 56;
pub const INDICATED_AIRSPEED_BUTTON: u16 = 57;

// unassignable variables
pub const PRESSURE_ALTITUDE_BUTTON: u16 = 61;
pub const DENSITY_ALTITUDE_BUTTON: u16 = 62;
pub const HEAD_WIND_BUTTON: u16 = 63;
pub const CROSS_WIND_BUTTON: u16 = 64;
pub const DEVIATION_ANGLE_BUTTON: u16 = 65;
pub const TRUE_AIRSPEED_BUTTON: u16 = 66;
pub const GROUND_SPEED_BUTTON: u16 = 67;

// Assign buttons 100-199
pub const A_ASSIGN_BUTTON: u16 = 101;
pub const B_ASSIGN_BUTTON: u16 = 102;
pub const C_ASSIGN_BUTTON: u16 = 103;
pub const D_ASSIGN_BUTTON: u16 = 104;

pub const TEMP_ASSIGN_BUTTON: u16 = 111;
pub const ALTITUDE_ASSIGN_BUTTON: u16 = 112;
pub const ALTIMETER_ASSIGN_BUTTON: u16 = 113;
pub const WIND_SPEED_ASSIGN_BUTTON: u16 = 114;
pub const WIND_HEADING_ASSIGN_BUTTON: u16 = 115;
pub const HEADING_ASSIGN_BUTTON: u16 = 116;
pub const INDICATED_AIRSPEED_ASSIGN_BUTTON: u16 = 117;
//Add buttons 200-299
pub const A_ADD_BUTTON: u16 = 201;
pub const B_ADD_BUTTON: u16 = 202;
pub const C_ADD_BUTTON: u16 = 203;
pub const D_ADD_BUTTON: u16 = 204;

pub const TEMP_ADD_BUTTON: u16 = 211;
pub const ALTITUDE_ADD_BUTTON: u16 = 212;
pub const ALTIMETER_ADD_BUTTON: u16 = 213;
pub const WIND_SPEED_ADD_BUTTON: u16 = 214;
pub const WIND_HEADING_ADD_BUTTON: u16 = 215;
pub const HEADING_ADD_BUTTON: u16 = 216;
pub const INDICATED_AIRSPEED_ADD_BUTTON: u16 = 217;

// conversion buttons 300-399
pub const NM_TO_FEET_BUTTON: u16 = 301;
pub const FEET_TO_NM_BUTTON: u16 = 302;
pub const KNOTS_TO_FPM_BUTTON: u16 = 303;
pub const FPM_TO_KNOTS_BUTTON: u16 = 304;
pub const KPH_TO_MPM_BUTTON: u16 = 305;
pub const MPM_TO_KPH_BUTTON: u16 = 306;
pub const NM_TO_MILES_BUTTON: u16 = 307;
pub const MILES_TO_NM_BUTTON: u16 = 308;
pub const MILES_TO_KILOMETERS_BUTTON: u16 = 309;
pub const KILOMETERS_TO_MILES_BUTTON: u16 = 310;
pub const KNOTS_TO_MPH_BUTTON: u16 = 311;
pub const MPH_TO_KNOTS_BUTTON: u16 = 312;
pub const C_TO_F_BUTTON: u16 = 313;
pub const F_TO_C_BUTTON: u16 = 314;
pub const LBS_TO_KGS_BUTTON: u16 = 315;
pub const KGS_TO_LBS_BUTTON: u16 = 316;
pub const LITERS_TO_GALLONS_BUTTON: u16 = 317;
pub const GALLONS_TO_LITTERS_BUTTON: u16 = 318;
pub const GALLONS_TO_AVIGAS_LBS_BUTTON: u16 = 319;
pub const GALLONS_TO_JET_FUEL_LBS_BUTTON: u16 = 320;
pub const INHG_TO_HECTOPASCALS_BUTTON: u16 = 321;
pub const HECTOPASCALS_TO_INHG_BUTTON: u16 = 322;
pub const FEET_TO_METERS: u16 = 323;
pub const METERS_TO_FEET: u16 = 324;

pub fn button_number_to_token(button: u16) -> Token {
    //This function should clean up the code quite a bit
    match button {
        ZERO_BUTTON => Token::Digit(Digit::Zero),
        ONE_BUTTON => Token::Digit(Digit::One),
        TWO_BUTTON => Token::Digit(Digit::Two),
        THREE_BUTTON => Token::Digit(Digit::Three),
        FOUR_BUTTON => Token::Digit(Digit::Four),
        FIVE_BUTTON => Token::Digit(Digit::Five),
        SIX_BUTTON => Token::Digit(Digit::Six),
        SEVEN_BUTTON => Token::Digit(Digit::Seven),
        EIGHT_BUTTON => Token::Digit(Digit::Eight),
        NINE_BUTTON => Token::Digit(Digit::Nine),
        PERIOD_BUTTON => Token::Digit(Digit::Period),
        PLUS_BUTTON => Token::Operator(Operator::Plus),
        MINUS_BUTTON => Token::Operator(Operator::Minus),
        MULTIPLY_BUTTON => Token::Operator(Operator::Multiply),
        DIVIDE_BUTTON => Token::Operator(Operator::Divide),
        LEFT_PARENTH_BUTTON => Token::Parenth(Parenth::Left),
        RIGHT_PARENTH_BUTTON => Token::Parenth(Parenth::Right),
        SIN_BUTTON => Token::Func(Func::Sin),
        COS_BUTTON => Token::Func(Func::Cos),
        TAN_BUTTON => Token::Func(Func::Tan),
        ARCSIN_BUTTON => Token::Func(Func::Arcsin),
        ARCCOS_BUTTON => Token::Func(Func::Arccos),
        ARCTAN_BUTTON => Token::Func(Func::Arctan),
        ANS_BUTTON => Token::Variable(Variable::Ans),
        A_BUTTON => Token::Variable(Variable::A),
        B_BUTTON => Token::Variable(Variable::B),
        C_BUTTON => Token::Variable(Variable::C),
        D_BUTTON => Token::Variable(Variable::D),
        TEMP_BUTTON => Token::Variable(Variable::Temp),
        ALTITUDE_BUTTON => Token::Variable(Variable::Altitude),
        ALTIMETER_BUTTON => Token::Variable(Variable::Altimeter),
        WIND_SPEED_BUTTON => Token::Variable(Variable::WindSpeed),
        WIND_HEADING_BUTTON => Token::Variable(Variable::WindDir),
        HEADING_BUTTON => Token::Variable(Variable::Heading),
        INDICATED_AIRSPEED_BUTTON => Token::Variable(Variable::Ias),
        PRESSURE_ALTITUDE_BUTTON => Token::Variable(Variable::PressAlt),
        DENSITY_ALTITUDE_BUTTON => Token::Variable(Variable::DensAlt),
        HEAD_WIND_BUTTON => Token::Variable(Variable::HeadWind),
        CROSS_WIND_BUTTON => Token::Variable(Variable::CrossWind),
        DEVIATION_ANGLE_BUTTON => Token::Variable(Variable::DevAngl),
        TRUE_AIRSPEED_BUTTON => Token::Variable(Variable::Tas),
        GROUND_SPEED_BUTTON => Token::Variable(Variable::GrdSpd),
        A_ASSIGN_BUTTON => Token::Variable(Variable::A),
        B_ASSIGN_BUTTON => Token::Variable(Variable::B),
        C_ASSIGN_BUTTON => Token::Variable(Variable::C),
        D_ASSIGN_BUTTON => Token::Variable(Variable::D),
        TEMP_ASSIGN_BUTTON => Token::Variable(Variable::Temp),
        ALTITUDE_ASSIGN_BUTTON => Token::Variable(Variable::Altitude),
        ALTIMETER_ASSIGN_BUTTON => Token::Variable(Variable::Altimeter),
        WIND_SPEED_ASSIGN_BUTTON => Token::Variable(Variable::WindSpeed),
        WIND_HEADING_ASSIGN_BUTTON => Token::Variable(Variable::WindDir),
        HEADING_ASSIGN_BUTTON => Token::Variable(Variable::Heading),
        INDICATED_AIRSPEED_ASSIGN_BUTTON => Token::Variable(Variable::Ias),
        A_ADD_BUTTON => Token::Variable(Variable::A),
        B_ADD_BUTTON => Token::Variable(Variable::B),
        C_ADD_BUTTON => Token::Variable(Variable::C),
        D_ADD_BUTTON => Token::Variable(Variable::D),
        TEMP_ADD_BUTTON => Token::Variable(Variable::Temp),
        ALTITUDE_ADD_BUTTON => Token::Variable(Variable::Altitude),
        ALTIMETER_ADD_BUTTON => Token::Variable(Variable::Altimeter),
        WIND_SPEED_ADD_BUTTON => Token::Variable(Variable::WindSpeed),
        WIND_HEADING_ADD_BUTTON => Token::Variable(Variable::WindDir),
        HEADING_ADD_BUTTON => Token::Variable(Variable::Heading),
        INDICATED_AIRSPEED_ADD_BUTTON => Token::Variable(Variable::Ias),

        _ => {
            panic!("This match should be exhaustive.")
        }
    }
}

pub fn button_number_to_pair_of_units(button: u16) -> (Unit, Unit) {
    match button {
        NM_TO_FEET_BUTTON => (Unit::NauticalMile, Unit::Foot),
        FEET_TO_NM_BUTTON => (Unit::Foot, Unit::NauticalMile),
        KNOTS_TO_FPM_BUTTON => (Unit::Knot, Unit::FeetPerMinute),
        FPM_TO_KNOTS_BUTTON => (Unit::FeetPerMinute, Unit::Knot),
        KPH_TO_MPM_BUTTON => (Unit::KilometersPerHour, Unit::MetersPerMinute),
        MPM_TO_KPH_BUTTON => (Unit::MetersPerMinute, Unit::KilometersPerHour),
        NM_TO_MILES_BUTTON => (Unit::NauticalMile, Unit::Mile),
        MILES_TO_NM_BUTTON => (Unit::Mile, Unit::NauticalMile),
        MILES_TO_KILOMETERS_BUTTON => (Unit::Mile, Unit::Kilometer),
        KILOMETERS_TO_MILES_BUTTON => (Unit::Kilometer, Unit::Mile),
        KNOTS_TO_MPH_BUTTON => (Unit::Knot, Unit::MilesPerHour),
        MPH_TO_KNOTS_BUTTON => (Unit::MilesPerHour, Unit::Knot),
        C_TO_F_BUTTON => (Unit::Celcius, Unit::Fahrenheit),
        F_TO_C_BUTTON => (Unit::Fahrenheit, Unit::Celcius),
        LBS_TO_KGS_BUTTON => (Unit::Pound, Unit::Kilogram),
        KGS_TO_LBS_BUTTON => (Unit::Kilogram, Unit::Pound),
        LITERS_TO_GALLONS_BUTTON => (Unit::Liter, Unit::Gallon),
        GALLONS_TO_LITTERS_BUTTON => (Unit::Gallon, Unit::Liter),
        GALLONS_TO_AVIGAS_LBS_BUTTON => (Unit::Gallon, Unit::AvigasPound),
        GALLONS_TO_JET_FUEL_LBS_BUTTON => (Unit::Gallon, Unit::JetfuelPound),
        INHG_TO_HECTOPASCALS_BUTTON => (Unit::InHg, Unit::Hectopascal),
        HECTOPASCALS_TO_INHG_BUTTON => (Unit::Hectopascal, Unit::InHg),
        _ => {
            panic!("This match should be exhaustive");
            (Unit::Celcius, Unit::Celcius) //To please the almighty compiler
        }
    }
}
