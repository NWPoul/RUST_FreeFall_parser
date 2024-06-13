use std::f64;



pub fn prompt_to_exit(msg: &str) {
    println!("{}\nPress 'enter' to exit...\n", {msg});
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}


pub fn abs_max(f_prev: f64, f_new: f64) -> f64 {
    f_prev.abs().max(f_new.abs())
}

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    fn dot_product(a: &Vector, b: &Vector) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    fn angle_between(a: &Vector, b: &Vector) -> f64 {
        let cos_theta = Self::dot_product(a, b) / (a.magnitude() * b.magnitude());
        cos_theta.acos()
    }
}
