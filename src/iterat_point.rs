use glam::{dvec2, DVec2};
use num_complex::{Complex, ComplexFloat};

pub fn iterat_point(c: DVec2, max_iterations: usize) -> (DVec2, DVec2, usize) {
    let c = Complex::new(c.x, c.y);
    let mut z = Complex::new(0., 0.);
    let mut d_z = Complex::new(0., 0.);
    let mut iterations = 0;
    while iterations < max_iterations && z.abs() < 16. {
        d_z = 2. * d_z * z + 1.; // d_z_n = (2 * d_z_n-1 * z_n-1) + 1
        z = z * z + c; // z_n = (z_n-1)^2 + c
        iterations += 1;
    }
    (dvec2(z.re, z.im), dvec2(d_z.re, d_z.im), iterations)
}

pub fn iterations_to_color(iterations: [i64; 4]) -> [u32; 4] {
    let mut colors = [0; 4];
    for i in 0..4 {
        let color_value = 128 - (iterations[i] as u32 * 16) % 128;
        colors[i] = color_value + (color_value << 8) + (color_value << 16);
    }
    colors
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PointIter {
    pub c: Complex<f64>,
    pub z: Complex<f64>,
    pub d1_z: Complex<f64>,
    pub d2_z: Complex<f64>,
    pub d3_z: Complex<f64>,
    pub d4_z: Complex<f64>,
    pub d5_z: Complex<f64>,
    pub d6_z: Complex<f64>,
    pub d7_z: Complex<f64>,
    pub current_iteration: usize,
    pub max_iterations: usize,
}
impl PointIter {
    pub fn new(c: Complex<f64>, max_iterations: usize) -> Self {
        Self {
            c,
            max_iterations,
            ..Default::default()
        }
    }
    pub fn next_iteration(&mut self) {
        let p = *self;

        self.z = p.z * p.z + self.c;
        self.d1_z = 2. * p.d1_z * p.z + 1.;
        self.d2_z = 2. * (p.z * p.d2_z + p.d1_z * p.d1_z);
        self.d3_z = 2. * (p.z * p.d3_z + 3. * p.d1_z * p.d2_z);
        self.d4_z = 2. * (p.z * p.d4_z + 3. * p.d2_z * p.d2_z + 4. * p.d3_z * p.d1_z);
        self.d5_z = 2. * (p.z * p.d5_z + 5. * p.d4_z * p.d1_z + 10. * p.d3_z * p.d2_z);
        self.d6_z = 2. * (p.z * p.d6_z + 10. * p.d3_z * p.d3_z + 6. * p.d5_z * p.d1_z + 15. * p.d4_z * p.d2_z);
        self.d7_z = 2. * (p.z * p.d7_z + 7. * p.d6_z * p.d1_z + 21. * p.d5_z * p.d2_z + 35. * p.d3_z * p.d4_z);

        self.current_iteration += 1;
    }
    pub fn is_finished(&self) -> bool {
        self.current_iteration >= self.max_iterations || self.z.abs() > 16.
    }
    pub fn iterate(&mut self)  {
        while !self.is_finished() {
            self.next_iteration();
        }
    }
    pub fn values(&self) -> [Complex<f64>; 8] {
        [self.z, self.d1_z, self.d2_z, self.d3_z, self.d4_z, self.d5_z, self.d6_z, self.d7_z]
    }
}

pub struct TaylorApproximation <const N: usize> {
    values: [Complex<f64>; N],
    position: Complex<f64>,
}
impl <const N: usize> TaylorApproximation <N> {
    pub fn new(position: Complex<f64>, values: [Complex<f64>; N]) -> Self {
        Self { position, values }
    }
    pub fn approximate_point(&self, position: Complex<f64>) -> Complex<f64> {
        let delta = position - self.position;
        let mut delta_produkt = Complex::new(1., 0.);
        let mut result = self.values[0];
        let mut factorial = 1.;
        for i in 1..N {
            delta_produkt *= delta;
            factorial *= i as f64;
            result +=  (self.values[i] / factorial) * delta_produkt;
        }
        result
    }
}
