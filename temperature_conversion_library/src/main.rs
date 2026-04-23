use std::fmt::{Display, Error, Formatter};

fn main() {
    let boiling = Celsius(100.0);
    let f: Fahrenheit = Celsius(100.0).into();
    let k: Kelvin = Celsius(100.0).into();
    println!("{boiling} = {f} = {k}");

    match Kelvin::try_from(-10.0) {
        Ok(k) => println!("{k}"),
        Err(e) => println!("Error: {e}"),
    }
}

struct Celsius(f64);
struct Fahrenheit(f64);
struct Kelvin(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Celsius> for Kelvin {
    fn from(c: Celsius) -> Self {
        Kelvin(c.0 + 273.15)
    }
}

impl TryFrom<f64> for Kelvin {
    type Error = String;
    fn try_from(f: f64) -> Result<Self, Self::Error> {
        if f < 0.0 {
            Err("below absolute zero".to_string())?
        }
        Ok(Kelvin(f))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:.2}°C", self.0)
    }
}

impl Display for Fahrenheit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:.2}°F", self.0)
    }
}
impl Display for Kelvin {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:.2}K", self.0)
    }
}
