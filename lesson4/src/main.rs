// This is the main function for lesson 4.

mod sum;
mod traffic;
mod area;

use traffic::Light;
use traffic::Time;

use area::Circle;
use area::Rectangle;

fn main() {

    // 1
    let green = Light::Green;
    let red = Light::Red;
    let yellow = Light::Yellow;
    println!("green light time {}", green.time());
    println!("red light time {}", red.time());
    println!("yellow light time {}", yellow.time());

    // 2
    let normal_sum: [u32; 5] = [1, 2, 3, 4, 5];
    let overflow_sum: [u32; 5] = [0xFFFFFFFF, 1, 2, 3, 4];
    println!("normal sum {:?}", sum::get_sum(&normal_sum));
    println!("stack overflow sum {:?}", sum::get_sum(&overflow_sum));

    // 3
    let square = Rectangle{
        width: 4.0,
        height: 4.0,
    };
    let circ = Circle {
        radius: 2.0,
    };
    area::print_area(&square);
    area::print_area(&circ);
}

