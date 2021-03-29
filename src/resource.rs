use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use std::fmt;

pub struct System {
    planets: Vec<Planet>,
    system_anomolies: Vec<SystemAnomoly>,
}

pub enum PlanetType {
    Standard,
}

pub struct Planet {
    planet_type: PlanetType,
}

pub enum SystemAnomoly {}

#[derive(Debug, Clone, Copy)]
pub enum Resource {
    Ore,
    Crystal,
    Energy,
    Science,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct ResourceHarvester {
    count: u32,
    ore_cost: u32,
    crystal_cost: u32,
    energy_cost: u32,
    science_cost: u32,
}

pub struct ResourceQuantity {
    resource: Resource,
    price: u32,
}

pub struct ResourceFactory {
    inputs: Vec<ResourceQuantity>,
    outputs: Vec<ResourceQuantity>,
}

impl System {
    fn new(planets: Vec<Planet>, system_anomolies: Vec<SystemAnomoly>) -> Self {
        System {
            planets: planets,
            system_anomolies: system_anomolies,
        }
    }
}

impl Planet {
    fn new(planet_type: PlanetType) -> Self {
        Planet {
            planet_type: planet_type,
        }
    }
}

pub fn home_system(seed: u64) -> System {
    let mut planets = Vec::new();
    planets.push(Planet::new(PlanetType::Standard));

    let system_anomolies = Vec::new();

    return System::new(planets, system_anomolies);
}

// pub fn generate_universe(seed: u64) -> Universe {
//     let mut rng = StdRng::seed_from_u64(seed);

//     let roll = rng.gen_range(5, 15);
//     let mut galaxies = Vec::with_capacity(roll);

//     for n in 1..=roll {
//         let g = Galaxy::new(seed);
//         galaxies.push(g);
//     }

//     let u = Universe::new(seed, galaxies);

//     return u;
// }
