
#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}
pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;
        
       
        (d.seconds as f64) / (EARTH_YEAR_SECONDS * Self::orbital_period())
    }
    fn orbital_period() -> f64;
}

macro_rules! planet {
    ($planet_name:ident, $period:expr) => {
        pub struct $planet_name;
        impl Planet for $planet_name {
            fn orbital_period() -> f64 {
                $period
            }
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
