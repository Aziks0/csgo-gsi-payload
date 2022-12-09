use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(try_from = "String")]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl TryFrom<String> for Coordinates {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens: Vec<&str> = value.split(',').map(|token| token.trim()).collect();
        if tokens.len() != 3 {
            return Err("invalid coordinates: it should be a 3 dimensions vector");
        }

        let coordinates = tokens
            .into_iter()
            .map(|token| token.parse::<f64>())
            .collect::<Result<Vec<_>, _>>();

        match coordinates {
            Ok(coordinates) => Ok(Coordinates {
                x: coordinates[0],
                y: coordinates[1],
                z: coordinates[2],
            }),
            Err(_) => Err("parse float error"),
        }
    }
}
