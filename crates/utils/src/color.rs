use nannou::prelude::*;

pub const fn color(red: u8, green: u8, blue: u8) -> Rgb<u8> {
    Rgb {
        red,
        green,
        blue,
        standard: std::marker::PhantomData::<nannou::color::encoding::Srgb>,
    }
}
