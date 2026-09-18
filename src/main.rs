                     // [VERSION - 2 -- PROJECTILE CALCULATOR]

fn flight_time(u:f64 , angle:f64 ) -> f64 {
   let radians = (angle * std::f64::consts::PI) / 180.0;

   let formula_time = (2.0 * u *radians.sin()) / 9.81;

   println!("The total time for flight is {:.2} seconds." , formula_time);
   formula_time
}




fn maximum_height(u:f64 , angle:f64 ) -> f64 {
   let radian = angle * std::f64::consts::PI / 180.0;

   let formula_height = (u*radian.sin())*(u*radian.sin())/(2.0 * 9.81);

   println!("The maximum height is {:.2} metre" , formula_height);
   formula_height
}



fn horizontal_range(u:f64 , angle:f64) -> f64 {
   let radian = 2.0*(angle * std::f64::consts::PI / 180.0);

   let formula_horizontal = (u*u * radian.sin()) / 9.81;

   println!("The horizontal range is {:.2} metre" ,formula_horizontal );
   formula_horizontal
}

fn main() {
   flight_time(20.0 , 45.0);
   maximum_height(20.0 ,45.0);
   horizontal_range(20.0,45.0);
}














