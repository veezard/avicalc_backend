use crate::objects::*;
use std::f64::consts::PI;

pub fn update_unassignable_quantities(state: &mut Box<State>) {
    let altimeter = state
        .variable_values
        .get(Variable::Altimeter)
        .convert(Unit::InHg, Unit::Pascal);
    let altitude = state
        .variable_values
        .get(Variable::Altitude)
        .convert(Unit::Foot, Unit::Meter);
    let temperature = state
        .variable_values
        .get(Variable::Temp)
        .convert(Unit::Celcius, Unit::Kelvin);
    let dew_point = state.variable_values.get(Variable::DewPoint); // The dew point is in Celcius

    let pressure = pressure_from_altimeter_and_altitude(altimeter, altitude);
    let pressure_altitude = altitude_from_std_pressure(pressure);

    let water_vapor_pressure = water_vapor_pressure_from_dew_point(dew_point);
    let dry_air_pressure = pressure - water_vapor_pressure;

    let density =
        density_from_pressure_and_temperature(dry_air_pressure, temperature, M_MOLAR_MASS_DRY_AIR)
            + density_from_pressure_and_temperature(
                water_vapor_pressure,
                temperature,
                M_MOLAR_MASS_WATER_VAPOR,
            );
    let density_altitude = altitude_from_std_density(density);

    let cas = state.variable_values.get(Variable::Cas);
    let tas = from_density_and_cas_to_tas(density, cas);

    let wind_direction = state.variable_values.get(Variable::WindDir);
    let wind_speed = state.variable_values.get(Variable::WindSpeed);
    let course = state.variable_values.get(Variable::Course);
    let head_wind = (PI * (course - wind_direction) / 180.).cos() * wind_speed;
    let cross_wind = (PI * (wind_direction - course) / 180.).sin() * wind_speed;

    let deviation_angle = 180. * (cross_wind / tas).asin() / PI;
    let heading = (course + deviation_angle + 360.) % 360.;

    let ground_speed = tas * (PI * deviation_angle / 180.).cos() - head_wind;

    state.variable_values.set(
        Variable::PressAlt,
        pressure_altitude.convert(Unit::Meter, Unit::Foot),
    );
    state.variable_values.set(
        Variable::DensAlt,
        density_altitude.convert(Unit::Meter, Unit::Foot),
    );
    state.variable_values.set(Variable::Tas, tas);
    state.variable_values.set(Variable::CrossWind, cross_wind);
    state.variable_values.set(Variable::HeadWind, head_wind);
    state.variable_values.set(Variable::Heading, heading);
    state.variable_values.set(Variable::GrdSpd, ground_speed);
}

// Let P_0,T_0, r_0 be the pressure, temperature, and dry air density at sea level (in pascals and kelvin). Then
// T(h) = T_0 - h L
// P(h) = P_0 (1-Lh/T_0)^(gM/RL)
// r(h) = (P_0/T_0)(M/R) (1-Lh/T_0)^(gM/RL -1)
// r(h) = r_0 (1-Lh/T_0)^(gM/RL -1)
// P= r T (R/M)

fn altitude_from_std_pressure(pressure: f64) -> f64 {
    (1. - (pressure / STD_P_0).powf(1. / GM_BY_RL)) * STD_T_0 / L_TEMPERATURE_LAPSE
}

fn altitude_from_std_density(density: f64) -> f64 {
    (1. - (density / STD_R_0).powf(1. / (GM_BY_RL - 1.))) * STD_T_0 / L_TEMPERATURE_LAPSE
}
// The altimeter setting takes into account the error of altimeter indication. The altimeter shifts
// the indicated altitude by the would be pressure altitude at height 0
// h =~ ((AS/P_0)^{RL/gM} - (P/P_0)^{RL/gM}) (T_0/L) // AS is the altimeter setting
// The altimeter setting is such that the indicated altitude is correct at the airfield altitude.
// P = P_0 (  (AS/P_0)^{RL/gM)- h (L/T_0))^{gM/RL}

fn pressure_from_altimeter_and_altitude(altimeter: f64, altitude: f64) -> f64 {
    STD_P_0
        * ((altimeter / STD_P_0).powf(RL_BY_GM) - altitude * L_TEMPERATURE_LAPSE / STD_T_0)
            .powf(GM_BY_RL)
}

fn density_from_pressure_and_temperature(pressure: f64, temperature: f64, molar_mass: f64) -> f64 {
    pressure * molar_mass / (temperature * R_IDEAL_GAS_CONSTANT)
}

fn water_vapor_pressure_from_dew_point(dew_point: f64) -> f64 {
    //Arden Buck equation
    //dew_point should be in celcius

    (0.61121 * ((18.678 - dew_point / 234.5) * (dew_point / (257.14 + dew_point))).exp())
        .convert(Unit::Kilopascal, Unit::Pascal)
}

fn from_density_and_cas_to_tas(density: f64, cas: f64) -> f64 {
    cas * (STD_R_0 / density).sqrt()
}

const STD_P_0: f64 = 101324.8126; //kg / m s^2 = pascals
const STD_T_0: f64 = 288.15; //K
const STD_R_0: f64 = 1.225; // kg/ m^3

const L_TEMPERATURE_LAPSE: f64 = 0.0065; // K/m
const M_MOLAR_MASS_DRY_AIR: f64 = 0.0289652; //kg/mol
const M_MOLAR_MASS_WATER_VAPOR: f64 = 0.01801528; //kg/mol
const G_ACCELERATION_GRAVITY: f64 = 9.80665; // m/s^2
const R_IDEAL_GAS_CONSTANT: f64 = 8.31446; // kg m^2 / s^2 mol K
const GM_BY_RL: f64 =
    G_ACCELERATION_GRAVITY * M_MOLAR_MASS_DRY_AIR / (R_IDEAL_GAS_CONSTANT * L_TEMPERATURE_LAPSE);
const RL_BY_GM: f64 = 1. / GM_BY_RL;

#[cfg(test)]
mod test_var_computations {
    //Tests should be ran in one thread sequentially.
    use super::*;
    use crate::*;
    use std::rc::Rc;

    fn compare(a: f64, b: f64, preccision: f64) -> bool {
        (a - b).abs() < preccision
    }
    unsafe fn initiate_state() {
        fn gui_update_functions(_: Variable) -> Rc<RefCell<Box<dyn FnMut(f64) -> ()>>> {
            Rc::new(RefCell::new(Box::new(|_: f64| {})))
        }
        let entry_field_writer = Box::new(|_: String| {});
        let previous_entry_field_writer = Box::new(|_: String| {});
        //--------------Initialize state of the calculator
        STATE = Some(RefCell::new(Box::new(State::default(gui_update_functions))));

        STRING_DISPLAY_FUNCTIONS = Some(RefCell::new(vec![
            RefCell::new(entry_field_writer),
            RefCell::new(previous_entry_field_writer),
        ]));
    }

    #[test]
    fn test_pressure_altitude() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            state.variable_values.set(Variable::Altimeter, 29.);
            state.variable_values.set(Variable::Altitude, 8000.);
            update_unassignable_quantities(&mut state);
            assert!(compare(
                state.variable_values.get(Variable::PressAlt),
                8862.,
                10.
            ));
        }
    }
    #[test]
    fn test_density_altitude() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            state.variable_values.set(Variable::Altimeter, 28.);
            state.variable_values.set(Variable::Altitude, 15000.);
            state.variable_values.set(Variable::Temp, 5.);
            state.variable_values.set(Variable::DewPoint, 25.);
            update_unassignable_quantities(&mut state);
            assert!(compare(
                state.variable_values.get(Variable::DensAlt),
                20125.,
                10.
            ));
        }
    }
    #[test]
    fn test_tas() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            state.variable_values.set(Variable::Altimeter, 28.5);
            state.variable_values.set(Variable::Altitude, 11000.);
            state.variable_values.set(Variable::Temp, 0.);
            state.variable_values.set(Variable::Cas, 65.);
            update_unassignable_quantities(&mut state);
            assert!(compare(state.variable_values.get(Variable::Tas), 80., 1.));
        }
    }
    #[test]
    fn test_cross_wind() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            update_unassignable_quantities(&mut state);
            state.variable_values.set(Variable::WindDir, 100.);
            state.variable_values.set(Variable::WindSpeed, 24.);
            state.variable_values.set(Variable::Course, 210.);
            update_unassignable_quantities(&mut state);
            assert!(compare(
                state.variable_values.get(Variable::CrossWind),
                -22.6,
                0.1
            ));
        }
    }
    #[test]
    fn test_head_wind() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            state.variable_values.set(Variable::WindDir, 100.);
            state.variable_values.set(Variable::WindSpeed, 24.);
            state.variable_values.set(Variable::Course, 210.);
            update_unassignable_quantities(&mut state);
            assert!(compare(
                state.variable_values.get(Variable::HeadWind),
                -8.2,
                0.1
            ));
        }
    }
    #[test]
    fn test_ground_speed() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            update_unassignable_quantities(&mut state);
            //assert!(compare(state.variable_values.get(Variable::GrdSpd) , 0. , 0.001));
        }
    }
    #[test]
    fn test_heading() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            state.variable_values.set(Variable::WindDir, 100.);
            state.variable_values.set(Variable::WindSpeed, 24.);
            state.variable_values.set(Variable::Course, 210.);
            state.variable_values.set(Variable::Altimeter, 28.5);
            state.variable_values.set(Variable::Altitude, 11000.);
            state.variable_values.set(Variable::Temp, 0.);
            state.variable_values.set(Variable::Cas, 65.);
            update_unassignable_quantities(&mut state);
            assert!(compare(
                state.variable_values.get(Variable::Heading),
                210. - 16.40,
                0.1
            ));
        }
    }
}
