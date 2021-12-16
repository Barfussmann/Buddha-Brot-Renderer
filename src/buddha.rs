use core::simd::*;

struct Buddha {
    z_x: f64x4,
    z_y: f64x4,
    z_squared_x: f64x4,
    z_squared_y: f64x4,
    c_x: f64x4,
    c_y: f64x4,
}

#[allow(unused_variables)]
impl Buddha {
    fn iterate(&mut self) {
        self.z_y = f64x4::splat(2.) * self.z_x * self.z_y + self.c_y;
        self.z_x = self.z_squared_x - self.z_squared_y + self.c_x;
        self.z_squared_x = self.z_x * self.z_x;
        self.z_squared_y = self.z_y * self.z_y;
        let abs = self.z_squared_x + self.z_squared_y;
        let inside = abs.lanes_le(f64x4::splat(4.));

        let dummy = f64x4::splat(0.);
        let lower_bound = dummy;
        let upper_bound = dummy;
        let left_bound = dummy;
        let right_bound = dummy;
        let x_screen = dummy;
        let y_screen = dummy;
        let width = i64x4::splat(0);

        let x_inside = self.z_x.lanes_ge(right_bound) & self.z_x.lanes_le(left_bound);
        let y_inside = self.z_y.lanes_ge(lower_bound) & self.z_y.lanes_le(upper_bound);
        let both_inside = x_inside & y_inside;
        let index = unsafe { self.z_x.to_int_unchecked() + self.z_y.to_int_unchecked() * width };
        let inside_index = index & both_inside.to_int();
        // inside_index.
        // let test: u64x8 = self.z_x.try_into().unwrap();
    }
}
