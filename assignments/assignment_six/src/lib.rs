mod check_coordinates;
mod check_ip_address_class;
mod test;

pub fn main_func() -> bool {
    let points: (i32, i32) = (2, -2);
    check_coordinates::check_coordinates_quadrant(points);

    let add: (i32, i32, i32, i32) = (192, 0, 1, 1);
    check_ip_address_class::check_ip_address(add);

    true
}
