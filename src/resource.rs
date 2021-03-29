use itertools::Itertools;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal, Standard, WeightedIndex};
use std::fmt;

pub struct System {
    planets: Planets,
    system_anomolies: Vec<SystemAnomoly>,
}

#[derive(Debug, Clone, Copy)]
pub enum PlanetType {
    Standard,
    Gia,
    Barren,
}

#[derive(Debug, Clone, Copy)]
pub enum PlanetSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Giant,
}

struct Planets(pub Vec<Planet>);

pub struct Planet {
    planet_type: PlanetType,
    planet_size: PlanetSize,
    anomolies: PlanetAnomolies,
}

pub enum SystemAnomoly {}

struct PlanetAnomolies(pub Vec<PlanetAnomoly>);

// nick: I don't know rust well enough to know if this is typical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanetAnomoly {
    MineralRich,
    CrystalRich,
    EnergyRich,
    TribalCiv,
    AdvancedCiv,
    NuclearWasteLand,
}

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

impl fmt::Display for PlanetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for PlanetSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for PlanetAnomoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Planet(planet_type={}, planet_size={}, anomolies=[{}])",
            self.planet_type, self.planet_size, self.anomolies
        )
    }
}

impl fmt::Display for Planets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, planet| {
            result.and_then(|_| write!(f, "{}", planet))
        })
    }
}

impl fmt::Display for PlanetAnomolies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, planet_anomoly| {
            result.and_then(|_| write!(f, "{}", planet_anomoly))
        })
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "System(planets=[{}])", self.planets)
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

pub fn home_system() -> System {
    let planets = Planets(vec![Planet {
        planet_type: PlanetType::Standard,
        planet_size: PlanetSize::Large,
        anomolies: PlanetAnomolies(Vec::new()),
    }]);
    let system_anomolies = Vec::new();

    return System {
        planets: planets,
        system_anomolies: system_anomolies,
    };
}

impl Distribution<PlanetType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PlanetType {
        let weights = [5, 1, 1];
        let dist = WeightedIndex::new(&weights).unwrap();
        match dist.sample(rng) {
            0 => PlanetType::Standard,
            1 => PlanetType::Gia,
            _ => PlanetType::Barren,
        }
    }
}

impl Distribution<PlanetSize> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PlanetSize {
        let weights = [10, 20, 50, 20, 10, 1];
        let dist = WeightedIndex::new(&weights).unwrap();
        match dist.sample(rng) {
            0 => PlanetSize::ExtraSmall,
            1 => PlanetSize::Small,
            2 => PlanetSize::Medium,
            3 => PlanetSize::Large,
            4 => PlanetSize::ExtraLarge,
            _ => PlanetSize::Giant,
        }
    }
}

impl Distribution<PlanetAnomoly> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PlanetAnomoly {
        let weights = [10, 10, 10, 10, 1, 1];
        let dist = WeightedIndex::new(&weights).unwrap();
        match dist.sample(rng) {
            0 => PlanetAnomoly::MineralRich,
            1 => PlanetAnomoly::CrystalRich,
            2 => PlanetAnomoly::EnergyRich,
            3 => PlanetAnomoly::TribalCiv,
            4 => PlanetAnomoly::AdvancedCiv,
            _ => PlanetAnomoly::NuclearWasteLand,
        }
    }
}

impl Distribution<System> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> System {
        let planet_count = rng.gen_range(5, 15);
        let planets = ComplexPlanet.sample_iter(rng).take(planet_count).collect();
        System {
            planets: Planets(planets),
            system_anomolies: Vec::new(),
        }
    }
}

pub struct ComplexPlanet;

impl Distribution<Planet> for ComplexPlanet {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Planet {
        let planet_type: PlanetType = rng.gen();
        let planet_size: PlanetSize = rng.gen();

        // nick: A planet will have either 0 or 1 anomoly twice as often as it has 2 and very rarely will have 3.
        let anomoly_count_weights = [500, 500, 250, 1];
        let anomoly_count_dist = WeightedIndex::new(&anomoly_count_weights).unwrap();
        let anomoly_count = anomoly_count_dist.sample(rng);

        let mut anomolies: Vec<PlanetAnomoly> =
            Standard.sample_iter(rng).take(anomoly_count).collect();
        // Anomolies should be unique.
        // TODO: Remove anomolies that conflict with each other.
        anomolies = anomolies.into_iter().unique().collect();

        Planet {
            planet_type: planet_type,
            planet_size: planet_size,
            anomolies: PlanetAnomolies(anomolies),
        }
    }
}
