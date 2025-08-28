use core::f64;
use std::ops::Add;

#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(not(feature = "f64"))]
pub type Float = f32;

#[cfg(feature = "f64")]
pub(crate) const PI: f64 = std::f64::consts::PI;

#[cfg(not(feature = "f64"))]
pub(crate) const PI: f32 = std::f32::consts::PI;

macro_rules! create_quantity {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $name(pub Float);
    };
}

create_quantity!(RelativeTime);
create_quantity!(AbsoluteTime);
create_quantity!(Speed);
create_quantity!(Noise);
create_quantity!(ParticleDistanceThreshold);
create_quantity!(DomainBoundaryLength);

// Sets up a nice relation for additive time
impl Add<RelativeTime> for AbsoluteTime {
    type Output = AbsoluteTime;
    fn add(self, rhs: RelativeTime) -> Self::Output {
        AbsoluteTime(self.0 + rhs.0)
    }
}
