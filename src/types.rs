#[cfg(feature = "f64")]
pub(crate) type Float = f64;

#[cfg(not(feature = "f64"))]
pub(crate) type Float = f32;

macro_rules! create_quantity {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug)]
        pub(crate) struct $name(pub(crate) f64);
    };
}

create_quantity!(RelativeTime);
create_quantity!(AbsoluteTime);
create_quantity!(Speed);
create_quantity!(Noise);
