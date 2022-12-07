use serde::{Deserialize, Serialize};

struct Hexcode(String);

impl TryFrom<&str> for Hexcode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().nth(0) == Some('#') && value.len() == 7 {
            Ok(Self(value.to_owned()))
        } else {
            Err("Not a valid hex string")
        }
    }
}

impl ToString for Hexcode {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Colour {
    Blue,
    Red,
    Green,
    White,
    Black,
}

impl ToString for Colour {
    fn to_string(&self) -> String {
        match self {
            Colour::Blue => Hexcode::try_from("#0000ff").unwrap().to_string(),
            Colour::Red => Hexcode::try_from("#ff0000").unwrap().to_string(),
            Colour::Green => Hexcode::try_from("#00ff00").unwrap().to_string(),
            Colour::White => Hexcode::try_from("#ffffff").unwrap().to_string(),
            Colour::Black => Hexcode::try_from("#000000").unwrap().to_string(),
        }
    }
}
