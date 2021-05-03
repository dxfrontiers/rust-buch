

#[cfg(test)]
mod test {

    /*
        Showcases usage of Box and Lifetimes to build an acyclic graph
        This implementation has some problems, some of which are fixed in the next sample
     */

    pub struct Node<'a>{
        next: Option<&'a Box<Node<'a>>>,
        value: String
    }

    impl<'a> Node<'a>{
        fn new(value: String) -> Self{
            Node{ next: None, value}
        }
        fn set_next(&mut self, next: &'a Box<Node<'a>>){
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
    fn main(){
        println!();

        let mut hello =  Node::new("Hello".into());
        let mut around =  Node::new("Around".into());
        let mut the =  Box::new(Node::new("the".into()));
        let world = Box::new( Node::new("World".into()));

        hello.set_next(&world);
        the.set_next(&world);


        //append_world(&mut hello, &mut the);


        around.set_next(&the);

        print_acyclic_graph(&hello);
        print_acyclic_graph(&around);


    }

    /*
    fn append_world(hello: &mut Node, the: &mut Box<Node>) {
        let world = Node::new("World".into());
        let world = Box::new(world);

        hello.set_next(&world);
        the.set_next(&world);
    }

     */
}
