
pub trait Computable {
    fn area(&self) -> f32;
    fn display(&self) -> String;
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Computable for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn display(&self) -> String {
        format!("rectangle with width {} and height {}", self.width, self.height)
    }
}

const PI: f32 = 3.14;

pub struct Circle {
    pub radius: f32,
}

impl Computable for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * PI
    }

    fn display(&self) -> String {
        format!("circle with radius {}", self.radius)
    }
}

pub fn print_area<T: Computable>(figure : &T)
{
    println!("The figure[{}] area is {}", figure.display(), figure.area())
}