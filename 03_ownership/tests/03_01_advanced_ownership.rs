

#[cfg(test)]
mod advanced_ownership{
    use std::fmt::{Debug, Formatter, Error};
    use std::cell::{Cell, RefCell};

    /**
        Create a full clone of a vector
     */
    #[test]
    fn test_clone(){
        let mut a = vec![1,2,3];
        let mut b = a.clone();
        a[0]=7;
        println!("a: {:?}",a);
        println!("b: {:?}",b);
        assert_eq!(a[0],7);
        assert_eq!(b[0],1);
    }


    /**
        Clone a primitive data type
     */
    #[test]
    fn test_copy_primitive(){
        let mut x = 7;
        let y = x;
        let z = x.clone();
        x = 99;
        println!("x: {}",x);
        println!("y: {}",y);
        assert_eq!(x,99);
        assert_eq!(y,7);
    }



    /**
        Struct used to show Copy semantics
        Since u32 is copy and clone, Xerox can also be copy and clone
     */
    #[derive(Copy, Clone)]
    struct Xerox{
        val: u32
    }

    /**
        Show how copy works and that a "rename" may have unintended effects
     */
    #[test]
    fn test_copy(){
        let mut foo = Xerox{ val: 0 };
        let bar = foo;
        foo.val =7;
        println!("Foo: {}", foo.val);
        println!("Bar: {}", bar.val);
        assert_eq!(foo, 7);
        assert_eq!(bar, 0);
    }


    /*
        Funktionen in std::mem
     */


    struct TreeNode{
        content: String
    }
    impl TreeNode{
        fn change(&mut self, new: String) -> String{
            /*
                // this wont work, since you can not move out of borrowed reference
                let old_value = self.content;
                self.content = new;
                old_value
             */
            std::mem::replace(&mut self.content,new)
        }
    }

    /**

    */
    #[test]
    fn test_replace(){
        println!("\ntest_replace");

        let content = "Foo".to_string();
        println!("content str:           {:p}", &content[..]);          // A
        let mut node = TreeNode{ content };
        println!("node.content str:      {:p}", &node.content[..]);     // A
        println!("node.content:          {:p}", &node.content);         // B

        let new_content = "Bar".to_string();
        println!("new_content str:       {:p}", &new_content[..]);      // C
        let y = node.change(new_content);
        println!("node.content str:      {:p}", &node.content[..]);     // C
        println!("node.content:          {:p}", &node.content);         // B
        println!("retrieved content str: {:p}", &y[..]);                // A
    }




    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    // #[derive(Clone)]
    // struct Point2D{
    //     x: i32,
    //     y: i32
    // }
    //
    // impl Debug for Point2D{
    //     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    //         f.write_str(format!("[{},{}]", self.x, self.y).as_ref())
    //     }
    // }
    //
    //
    //
    // struct CellHolder{
    //     cell: Cell<u32>
    // }
    //  impl CellHolder{
    //      fn new(content: u32) -> Self{
    //          CellHolder{
    //              cell: Cell::new(content)
    //          }
    //      }
    //  }
    //
    //
    // #[test]
    // fn test_cell(){
    //     let c = CellHolder::new(3);
    //     println!("CellHolder content: {}" , c.cell.get());
    //     c.cell.set(6);
    //     println!("CellHolder content now: {}" , c.cell.get());
    // }
    //
    // struct RefCellHolder{
    //     cell: RefCell<u32>
    // }
    // impl RefCellHolder{
    //     fn new(content: u32) -> Self{
    //         RefCellHolder{
    //             cell: RefCell::new(content)
    //         }
    //     }
    // }
    //
    // #[test]
    // fn test_ref_cell(){
    //     let c = RefCellHolder::new(3);
    //     println!("CellHolder content: {}" , RefCell::borrow(&c.cell));
    //     {
    //         let mut r = RefCell::borrow_mut(&c.cell);
    //         *r = 6;
    //     }
    //     println!("CellHolder content now: {}" , RefCell::borrow(&c.cell));
    //     *RefCell::borrow_mut(&c.cell)=9;
    //     println!("CellHolder content now: {}" , RefCell::borrow(&c.cell));
    // }
    //
    // /*
    // #[test]
    // fn test_semantics(){
    //     let mut point = Point2D{x: 2, y: 7};
    //
    //     my_borrow(&point);
    //
    //     my_borrow_mut(&mut point);
    //     println!("pt {:?}",point);
    //
    //
    //     {
    //         let mut r = &mut point;
    //         let a = my_borrow_mut_ret(r);
    //         a.x=0;
    //         println!("a {:?}",a);
    //         println!("r {:?}",r);
    //     }
    //     println!("pt {:?}",point);
    //     point.x=99;
    //     println!("pt {:?}",point);
    //
    //     let x = my_own_ret(point);
    //     println!("pt {:?}",x);
    //
    //
    //     let mut c = create_container();
    //     println!("c0 {}", c.data[0]);
    //     let mut c2 = c.clone();
    //     println!("c20 {}", c2.data[0]);
    //
    //     c.data[0]=9;
    //     println!("c0 {}", c.data[0]);
    //     println!("c20 {}", c2.data[0]);
    //
    //
    // }
    //
    // fn create_container() -> Container{
    //     let mut c = Container{data: vec![0;128].into_boxed_slice()};
    //     println!("c0 {}", c.data[0]);
    //
    //     c
    // }
    //
    //  */
    //
    // fn my_borrow(pt: &Point2D){
    //
    // }
    //
    // fn my_borrow_mut(pt: &mut Point2D){
    //
    // }
    //
    // fn my_borrow_mut_ret(pt: &mut Point2D) -> &mut Point2D{
    //     pt
    // }
    //
    // fn my_own(pt: Point2D){
    //
    // }
    //
    // fn my_own_ret(pt: Point2D) -> Point2D{
    //     Point2D{x: 2, y: 7}
    // }
    //
    //
    //
    // struct Entity{
    //     s: String
    // }
    // impl Clone for Entity {
    //     fn clone(&self) -> Entity {
    //         println!("Alarm !");
    //         Entity{s:self.s.clone()}
    //     }
    // }
    //
    // #[inline(never)]
    // fn do_stuff(e: Entity) -> Entity{
    //     println!("ptr {:p}",&e);
    //     println!("ptr {:p}",&e);
    //     e
    // }
    //
    // #[test]
    // fn test_stack_allocation(){
    //     let entity = Entity{s:"sdf".into()};
    //     println!("Ptr: {:p}",&entity);
    //     let foo = do_stuff(entity);
    //     println!("Ptr: {:p}",&foo);
    // }
    //
    //
    //
    //
    //
    // fn greedy_function(p: String){
    //
    // }
    //
    // /*
    // #[test]
    // fn test_lifetimes(){
    //     let x = "Test".to_string();
    //     let a = &x;     //--+
    //     greedy_function(x);  //  | existiert länger
    //     println!("a is {}",a);  //<-+
    // }
    //
    //  */

}