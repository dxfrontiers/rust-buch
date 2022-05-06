#[cfg(test)]
mod test {
    use std::cell::Cell;
    use std::borrow::Borrow;

    #[test]
    fn cell_basics(){
        let c = Cell::new(7u8);
        let c0: &Cell<u8> = &c;
        let c1 = &c;
        let c2 = &c;
        Cell::set(c0,0);
        Cell::set(c1,1);
        Cell::set(c2,2);
        println!("c: {}", c.get());
        println!("c0: {}", c0.get());
        println!("c1: {}", c1.get());
        println!("c2: {}", c2.get());
    }
}
