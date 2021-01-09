// Traffic light

pub enum Light {
    Red,
    Green,
    Yellow,
}

pub trait Time {
    // Get wait time of the traffic light.
    fn time(&self) -> u8;
}


impl Time for Light {
    fn time(&self) -> u8 {
        match self {
            Light::Red => 60,
            Light::Green => 30,
            Light::Yellow=> 10,
        }
    }
}