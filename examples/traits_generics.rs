trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Generic function using trait bound
fn print_area<A: Area>(shape: &A) {
    println!("Area: {}", shape.area());
}

fn main() {
    let c = Circle { radius: 2.0 };
    let r = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    print_area(&c);
    print_area(&r);
}
