use std::ops::Add;

#[cfg(feature = "f64")]
pub(crate) type Float = f64;

#[cfg(not(feature = "f64"))]
pub(crate) type Float = f32;

macro_rules! create_quantity {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $name(pub f64);
    };
}

create_quantity!(RelativeTime);
create_quantity!(AbsoluteTime);
create_quantity!(Speed);
create_quantity!(Noise);
create_quantity!(ParticleDistanceThreshold);

// Sets up a nice relation for additive time
impl Add<RelativeTime> for AbsoluteTime {
    type Output = AbsoluteTime;
    fn add(self, rhs: RelativeTime) -> Self::Output {
        AbsoluteTime(self.0 + rhs.0)
    }
}
