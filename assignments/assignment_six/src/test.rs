#[cfg(test)]
pub mod tests {
    use crate::check_coordinates::{check_coordinates_quadrant, Quadrants};

    #[test]
    fn check_for_fourth_quadrant() {
        let points: (i32, i32) = (2, -2);
        assert_eq!(
            check_coordinates_quadrant(points),
            Quadrants::Fourth {
                abscissa: 2,
                ordinate: -2
            }
        );
    }

    #[test]
    fn check_for_first_quadrant() {
        let points: (i32, i32) = (2, 2);
        assert_eq!(
            check_coordinates_quadrant(points),
            Quadrants::First {
                abscissa: 2,
                ordinate: 2
            }
        );
    }

    #[test]
    fn check_for_second_quadrant() {
        use crate::check_coordinates::{check_coordinates_quadrant, Quadrants};
        let points: (i32, i32) = (-2, 2);
        assert_eq!(
            check_coordinates_quadrant(points),
            Quadrants::Second {
                abscissa: -2,
                ordinate: 2
            }
        );
    }

    #[test]
    fn check_for_third_quadrant() {
        use crate::check_coordinates::{check_coordinates_quadrant, Quadrants};
        let points: (i32, i32) = (-2, -2);
        assert_eq!(
            check_coordinates_quadrant(points),
            Quadrants::Third {
                abscissa: -2,
                ordinate: -2
            }
        );
    }

    #[test]
    fn check_for_origin() {
        use crate::check_coordinates::{check_coordinates_quadrant, Quadrants};
        let points: (i32, i32) = (0, 0);
        assert_eq!(
            check_coordinates_quadrant(points),
            Quadrants::Origin {
                abscissa: 0,
                ordinate: 0
            }
        );
    }

    #[test]
    fn point_on_axis() {
        use crate::check_coordinates::{check_coordinates_quadrant, Quadrants};
        let points: (i32, i32) = (0, 1);
        assert_eq!(
            check_coordinates_quadrant(points),
            Quadrants::Axis {
                abscissa: 0,
                ordinate: 1
            }
        );
    }

    #[test]
    fn check_address_class_a() {
        use crate::check_ip_address_class::{check_ip_address, Ip};
        let add: (i32, i32, i32, i32) = (0, 0, 1, 1);
        let ip: String = "0.0.1.1".to_string();
        assert_eq!(check_ip_address(add), Ip::ClassA(ip));
    }

    #[test]
    fn check_address_class_b() {
        use crate::check_ip_address_class::{check_ip_address, Ip};
        let add: (i32, i32, i32, i32) = (128, 0, 1, 1);
        let ip: String = "128.0.1.1".to_string();
        assert_eq!(check_ip_address(add), Ip::ClassB(ip));
    }

    #[test]
    fn check_address_class_c() {
        use crate::check_ip_address_class::{check_ip_address, Ip};
        let add: (i32, i32, i32, i32) = (192, 0, 1, 1);
        let ip: String = "192.0.1.1".to_string();
        assert_eq!(check_ip_address(add), Ip::ClassC(ip));
    }

    #[test]
    fn check_address_class_d() {
        use crate::check_ip_address_class::{check_ip_address, Ip};
        let add: (i32, i32, i32, i32) = (224, 0, 1, 1);
        let ip: String = "224.0.1.1".to_string();
        assert_eq!(check_ip_address(add), Ip::ClassD(ip));
    }

    #[test]
    fn check_if_invalid() {
        use crate::check_ip_address_class::{check_ip_address, Ip};
        let add: (i32, i32, i32, i32) = (-1, 0, 1, 1);
        assert_eq!(
            check_ip_address(add),
            Ip::ClassE("Belong to class E".to_string())
        );
    }

    #[test]
    fn check_func_test() {
        use crate::main_func;
        assert!(main_func());
    }
}
