#[derive(PartialEq, Debug)]
///Ip Enum
///
/// #variants
///
/// ClassA : includes a single String.
/// ClassB : includes a single String.
/// ClassC : includes a single String.
/// ClassD : includes a single String.
/// Error : includes a single String.
pub enum Ip {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    Error(String),
}
/// check_ip_address function checks in which Ip class the following IP address belongs.
///
/// #Arguments
///
/// adder(tuple) : have Ip address in it.
///
/// #Return
///
/// Returns Ip(Enum) Type.
pub fn check_ip_address(adder: (i32, i32, i32, i32)) -> Ip {
    let mut result: String = String::new();
    match adder {
        (a, b, c, d) if a > -1 && a < 128 => {
            result.push_str(&a.to_string());
            result.push_str(&".".to_string());
            result.push_str(&b.to_string());
            result.push_str(&".".to_string());
            result.push_str(&c.to_string());
            result.push_str(&".".to_string());
            result.push_str(&d.to_string());
            Ip::ClassA(result)
        }
        (a, b, c, d) if a > 127 && a < 191 => {
            result.push_str(&a.to_string());
            result.push_str(&".".to_string());
            result.push_str(&b.to_string());
            result.push_str(&".".to_string());
            result.push_str(&c.to_string());
            result.push_str(&".".to_string());
            result.push_str(&d.to_string());
            Ip::ClassB(result)
        }
        (a, b, c, d) if a > 191 && a < 223 => {
            result.push_str(&a.to_string());
            result.push_str(&".".to_string());
            result.push_str(&b.to_string());
            result.push_str(&".".to_string());
            result.push_str(&c.to_string());
            result.push_str(&".".to_string());
            result.push_str(&d.to_string());
            Ip::ClassC(result)
        }
        (a, b, c, d) if a > 223 && a < 239 => {
            result.push_str(&a.to_string());
            result.push_str(&".".to_string());
            result.push_str(&b.to_string());
            result.push_str(&".".to_string());
            result.push_str(&c.to_string());
            result.push_str(&".".to_string());
            result.push_str(&d.to_string());
            Ip::ClassD(result)
        }
        _ => Ip::Error(String::from("Invalid Ip Address")),
    }
}
