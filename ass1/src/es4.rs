use physical_constants::SPEED_OF_LIGHT_IN_VACUUM;

pub fn e_equals_mc_squared(mass: f32) -> f64 {
    mass as f64 * SPEED_OF_LIGHT_IN_VACUUM * SPEED_OF_LIGHT_IN_VACUUM
}

pub fn es4() {
    println!(
        "With a mass of 80_000 g the energy is {}!",
        e_equals_mc_squared(80_000.0)
    );
}
