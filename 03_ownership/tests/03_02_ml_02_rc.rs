
#[cfg(test)]
mod test {
    use std::rc::Rc;

    pub struct Node{
        next: Option<Rc<Node>>,
        value: String
    }

    impl Node{
        fn new(value: String) -> Self{
            Node{ next: None, value}
        }
        fn set_next(&mut self, next: Rc<Node>){
            self.next=Some(next)
        }
    }

    fn print_acyclic_graph(entry: &Node){
        print!("{} ",entry.value);
        match &entry.next{
            Some(next) => print_acyclic_graph(next),
            None => {print!("\n")}
        }
    }



    #[test]
    fn manual_creation(){
        println!();

        let world = Node::new("World".into());
        let world = Rc::new(world);

        let mut hello =  Node::new("Hello".into());
        hello.set_next(Rc::clone(&world));

        let mut the =  Node::new("the".into());
        the.set_next(Rc::clone(&world));
        let the = Rc::new(the);

        let mut around =  Node::new("Around".into());
        around.set_next(Rc::clone(&the));

        print_acyclic_graph(&hello);
        print_acyclic_graph(&around);
    }



    #[test]
    fn creation_with_helper_function(){
        println!();

        let mut hello =  Node::new("Hello".into());
        let mut around =  Node::new("Around".into());
        let mut the =  Node::new("the".into());

        append_world(&mut hello, &mut the);

        let the = Rc::new(the);
        around.set_next(Rc::clone(&the));

        print_acyclic_graph(&hello);
        print_acyclic_graph(&around);
    }

    fn append_world(hello: &mut Node, the: &mut Node) {
        let world = Node::new("World".into());
        let world = Rc::new(world);

        hello.set_next(Rc::clone(&world));
        the.set_next(Rc::clone(&world));
    }

}
