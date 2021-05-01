use crate::objects::*;
use std::f64::consts::PI;

pub fn update_unassignable_quantities(state: &mut Box<State>) {
    let p_0 = state
        .variable_values
        .get(Variable::Altimeter)
        .convert(Unit::InHg, Unit::Pascal);
    let altitude = state
        .variable_values
        .get(Variable::Altitude)
        .convert(Unit::Foot, Unit::Meter);
    let temp = state
        .variable_values
        .get(Variable::Temp)
        .convert(Unit::Celcius, Unit::Kelvin);
    let t_0 = temp + altitude * L_TEMPERATURE_LAPSE;
    let pressure_assuming_std_temp = pressure_function(p_0, STD_T_0, altitude);
    let pressure_altitude = std_pressure_to_altitude(pressure_assuming_std_temp);

    let density = density_function(p_0, t_0, altitude);
    let density_altitude = std_density_to_altitude(density);

    let ias = state.variable_values.get(Variable::Ias);
    let tas = from_density_and_ias_to_tas(density, ias);

    let wind_direction = state.variable_values.get(Variable::WindDir);
    let wind_speed = state.variable_values.get(Variable::WindSpeed);
    let heading = state.variable_values.get(Variable::Heading);
    let head_wind = (PI * (heading - wind_direction) / 180.).cos() * wind_speed;
    let cross_wind = (PI * (wind_direction - heading) / 180.).sin() * wind_speed;

    let deviation_angle = 180. * (cross_wind / tas).asin() / PI;

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
    state
        .variable_values
        .set(Variable::DevAngl, deviation_angle);
    state.variable_values.set(Variable::GrdSpd, ground_speed);
}

// Let P_0,T_0 be the pressure and temperature at sea level (in pascals and kelvin). Then
// T(h) = T_0 - h L
// P(h) = P_0 (1-Lh/T_0)^(gM/RL)
// r(h) = (P_0/T_0)(M/R) (1-Lh/T_0)^(gM/RL -1)
// r(h) = R_0 (1-Lh/T_0)^(gM/RL -1)

fn pressure_function(p_0: f64, t_0: f64, h: f64) -> f64 {
    p_0 * (1. - (L_TEMPERATURE_LAPSE * h) / t_0).powf(GM_BY_RL)
}
fn density_function(p_0: f64, t_0: f64, h: f64) -> f64 {
    (p_0 / t_0) * M_BY_R * (1. - (L_TEMPERATURE_LAPSE * h) / t_0).powf(GM_BY_RL - 1.)
}

fn std_pressure_to_altitude(pressure: f64) -> f64 {
    (1. - (pressure / STD_P_0).powf(1. / GM_BY_RL)) * STD_T_0 / L_TEMPERATURE_LAPSE
}

fn std_density_to_altitude(density: f64) -> f64 {
    (1. - (density / STD_R_0).powf(1. / (GM_BY_RL - 1.))) * STD_T_0 / L_TEMPERATURE_LAPSE
}

fn from_density_and_ias_to_tas(density: f64, ias: f64) -> f64 {
    ias * (STD_R_0 / density).sqrt()
}

const STD_P_0_IN_INHG: f64 = 29.9212; //inHg
const STD_P_0: f64 = 101324.8126; //kg / m s^2 = pascals
const STD_T_0: f64 = 288.15; //K
const STD_R_0: f64 = 1.225; // kg/ m^3

const L_TEMPERATURE_LAPSE: f64 = 0.0065; // K/m
const M_MOLAR_MASS: f64 = 0.0289652; //kg/mol
const G_ACCELERATION_GRAVITY: f64 = 9.80665; // m/s^2
const R_IDEAL_GAS_CONSTANT: f64 = 8.31446; // kg m^2 / s^2 mol K
const GM_BY_RL: f64 =
    G_ACCELERATION_GRAVITY * M_MOLAR_MASS / (R_IDEAL_GAS_CONSTANT * L_TEMPERATURE_LAPSE);
const M_BY_R: f64 = M_MOLAR_MASS / R_IDEAL_GAS_CONSTANT; // s^2 K / m^2

#[cfg(test)]
mod test_var_computations {
    //Tests should be ran in one thread sequentially.
    use super::*;
    use crate::*;
    use std::rc::Rc;

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
            state.variable_values.set(Variable::Altimeter, 31.);
            state.variable_values.set(Variable::Altitude, 15000.);
            update_unassignable_quantities(&mut state);
            assert!(state.variable_values.get(Variable::PressAlt) - 14017. < 200.);
        }
    }
    #[test]
    fn test_density_altitude() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            state.variable_values.set(Variable::Altimeter, 31.);
            state.variable_values.set(Variable::Altitude, 15000.);
            state.variable_values.set(Variable::Temp, 5.);
            update_unassignable_quantities(&mut state);
            assert!(state.variable_values.get(Variable::DensAlt) - 16038. < 500.);
        }
    }
    #[test]
    fn test_tas() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            update_unassignable_quantities(&mut state);
            //assert!(state.variable_values.get(Variable::Tas) - 0. < 0.001);
        }
    }
    #[test]
    fn test_cross_wind() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            update_unassignable_quantities(&mut state);
            //assert!(state.variable_values.get(Variable::CrossWind) - 0. < 0.001);
        }
    }
    #[test]
    fn test_head_wind() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            update_unassignable_quantities(&mut state);
            //assert!(state.variable_values.get(Variable::HeadWind) - 0. < 0.001);
        }
    }
    #[test]
    fn test_ground_speed() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            update_unassignable_quantities(&mut state);
            //assert!(state.variable_values.get(Variable::GrdSpd) - 0. < 0.001);
        }
    }
    #[test]
    fn test_deviation_angle() {
        unsafe {
            initiate_state();
            let mut state = &mut STATE.as_ref().unwrap().borrow_mut();
            update_unassignable_quantities(&mut state);
            //assert!(state.variable_values.get(Variable::DevAngl) - 0. < 0.001);
        }
    }
}
