#[derive(PartialEq, Debug)]
///Quadrants Enum.
///
/// #variants.
///
/// First : includes an anonymous struct inside it.
/// Second : includes an anonymous struct inside it.
/// Third : includes an anonymous struct inside it.
/// Fourth : includes an anonymous struct inside it.
/// Origin : includes an anonymous struct inside it.
/// Axis : includes an anonymous struct inside it.

pub enum Quadrants {
    First { abscissa: i32, ordinate: i32 },
    Second { abscissa: i32, ordinate: i32 },
    Third { abscissa: i32, ordinate: i32 },
    Fourth { abscissa: i32, ordinate: i32 },
    Origin { abscissa: i32, ordinate: i32 },
    Axis { abscissa: i32, ordinate: i32 },
}
/// check_coordinates_quadrant function checks on which quadrants the points lies.
///
/// #Arguments
///
/// pts(tuple) : pts have coordinate(x,y) value in it.
///
/// #Return
///
/// Returns Quadrants(Enum) type
pub fn check_coordinates_quadrant(pts: (i32, i32)) -> Quadrants {
    match pts {
        (x, y) if x > 0 && y > 0 => Quadrants::First {
            abscissa: x,
            ordinate: y,
        },
        (x, y) if x < 0 && y > 0 => Quadrants::Second {
            abscissa: x,
            ordinate: y,
        },
        (x, y) if x < 0 && y < 0 => Quadrants::Third {
            abscissa: x,
            ordinate: y,
        },
        (x, y) if x > 0 && y < 0 => Quadrants::Fourth {
            abscissa: x,
            ordinate: y,
        },
        (x, y) if x == 0 && y == 0 => Quadrants::Origin {
            abscissa: x,
            ordinate: y,
        },
        _ => Quadrants::Axis {
            abscissa: pts.0,
            ordinate: pts.1,
        },
    }
}
