#[cfg(test)]
mod test {
    use std::rc::Rc;
    use std::cell::RefCell;

    pub struct MutableNode{
        next: Option<Rc<MutableNode>>,
        value: RefCell<String>
    }

    impl MutableNode{
        fn new(value: String) -> Self{
            MutableNode{
                next: None,
                value: RefCell::new(value)
            }
        }
        fn add_next(&mut self, next: Rc<MutableNode>){
            //self.ngh.push(ngh)
            self.next=Some(next)
        }
    }

    #[test]
    fn main() {
        println!();
        let goal = MutableNode::new("World".into());
        let goal = Rc::new(goal);

        let mut m0 =  MutableNode::new("Hello".into());

        let mut m1 =  MutableNode::new("Around".into());
        let mut m2 =  MutableNode::new("the".into());

        m0.add_next(goal.clone());
        m2.add_next(goal.clone());
        m1.add_next(Rc::new(m2));


        print_acyclic_graph_mutable(&m0);
        print_acyclic_graph_mutable(&m1);

        *RefCell::borrow_mut(&goal.value) = "Clock".into();

        print_acyclic_graph_mutable(&m0);
        print_acyclic_graph_mutable(&m1);

    }

    fn print_acyclic_graph_mutable(entry: &MutableNode){
        print!("{} ",entry.value.borrow());
        match &entry.next{
            Some(next) => print_acyclic_graph_mutable(next),
            None => {print!("\n")}
        }

    }

    #[test]
    fn ref_cell_basics(){
        let c = RefCell::new(7);
        {
            let mut borrow = c.borrow_mut();
            *borrow +=1;
            // illegal: let borrow2 = c.borrow_mut();
            // illegal: let borrow3 = c.borrow();
        } // borrow goes out of scope
        println!("c: {}", c.borrow());
    }
}