// #[cfg(test)]
// mod tests {
//     // We want to import every public function from the outer / root scope of the module
//     use super::*;
//
//     fn string_move(s: String) {
//         println!("{}", s);
//     }
//
//     fn string_borrow(s: &String) {
//         println!("{}", s);
//     }
//
//     #[test]
//     fn test_move() {
//         let s = String::from("String");
//         let &amper = *s;
//
//         string_move(amper);
//     }
//
//     #[test]
//     fn test_borrow() {
//         let s = String::from("String");
//         let y = &s;
//         string_borrow(y);
//         println!("{}", s);
//     }
// }
