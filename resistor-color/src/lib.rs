use enum_iterator::{all, Sequence};
use int_enum::{IntEnum, IntEnumError};

#[repr(u8)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value().into()
}

pub fn value_to_color_string(value: u32) -> String {
    let _color  = || -> Result<ResistorColor, IntEnumError<ResistorColor>> {
        ResistorColor::from_int(value.try_into().unwrap())
    };

    if let Err(_err) = _color() {
        return String::from("value out of range")
    }

    format!("{:?}", _color().unwrap())
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
