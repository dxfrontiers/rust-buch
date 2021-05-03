
#[cfg(test)]
mod advanced_ownership{

    // "broken" by performance gains: the nerwer rustc doe sno longer allocate on the heap if not needed
    // /*
    //     A container saving its data on the heap
    //  */
    // struct Container{
    //     data: Box<[u8]>,
    // }
    // /*
    //     A container saving its data on the stack
    //  */
    // struct Point3D{
    //     data: [u8;3]
    // }
    //
    // /*
    // Creates a heap allocated container and returns it along with its original address as an integer
    //  */
    // fn create_container() -> (u64,Container) {
    //     let data: Box<[u8]> = Box::new([0, 0, 7]);
    //     println!("c_ptr \t{:p}", data);
    //     println!("c_ptr \t{:p}", &data);
    //
    //     let mut c = Container{data};
    //     println!("c_ptr \t{:p}", c.data);
    //     let addr:u64 = &c.data[0] as *const u8 as u64;
    //     (addr,c)
    // }
    //
    //
    // /*
    // Creates a stack allocated point and returns it along with its original address as an integer
    //  */
    // fn create_point() -> (u64, Point3D) {
    //     let data = [0,8,15];
    //     let mut p = Point3D{ data };
    //     println!("p_ptr \t{:p}", &p.data);
    //     let addr:u64 = &p.data[0] as *const u8 as u64;
    //     (addr,p)
    // }
    //
    //
    // /*
    //     Make sure that the address of the stack allocated data structure is different after it has been returned
    //     Also make sure that the address of the heap allocated data structure is the same after it is returned
    //  */
    // #[test]
    // fn test_stack_vs_heap(){
    //     // create a new line for pretty output in tests
    //     println!();
    //
    //     let (addr_original,p0) = create_point();
    //     println!("p0_ptr \t{:p}", &p0.data[0]);
    //
    //     // Get the actual address of the data and make sure it is different from the original one
    //     // obtained from within the create function
    //     let addr_observed = &p0.data[0] as *const u8 as u64;
    //     assert_ne!(addr_original,addr_observed);
    //
    //
    //     let  (addr_original,c0) = create_container();
    //     println!("c0_ptr \t{:p}", &c0.data[0]);
    //
    //     let addr_observed = &c0.data[0] as *const u8 as u64;
    //     assert_eq!(addr_original,addr_observed);
    //
    //
    // }
    //
    //
    // impl Clone for Container{
    //     fn clone(&self) -> Self {
    //         println!("Cloning");
    //         Container{
    //             data: self.data.clone()
    //         }
    //     }
    // }
    //
    // #[test]
    // fn test_clone(){
    //     let (_,mut c0) = create_container();
    //     println!("c0 \t{}", c0.data[0]);
    //     println!("c0_ptr \t{:p}", &c0.data[0]);
    //     let addr0 = &c0.data[0] as *const u8 as u64;
    //
    //     let mut c1 = c0.clone();
    //     println!("c1 \t{}", c1.data[0]);
    //     println!("c0_ptr \t{:p}", &c1.data[0]);
    //     let addr1 = &c1.data[0] as *const u8 as u64;
    //
    //     // Since we cloned the container, the first element of its data has to have a different address than the original one
    //     assert_ne!(addr0,addr1);
    //
    //     c0.data[0]=7;
    //
    //     // the changed variable has to be changed but the original must be unaffected
    //     assert_eq!(c0.data[0], 7);
    //     assert_eq!(c1.data[0], 0);
    // }



    /*
         Self referencing structs
         A chain of nodes, containing a reference to the same type
     */

    enum BoxChain{
        End(u32),
        More(u32,Box<BoxChain>)
    }

    impl BoxChain{
        fn aggregate(&self) -> u32 {
            match self{
                BoxChain::More(val,more) => val + more.aggregate(),
                BoxChain::End(val) => *val
            }
        }
    }

    #[test]
    fn test_box_chain() {
        let a = Box::new(BoxChain::End(7));
        let b = Box::new(BoxChain::More(6, a));
        let chain = BoxChain::More(5, b);

        let result = chain.aggregate();
        println!("test_box_chain result: {}",result);
        assert_eq!(result,5+6+7)
    }

}
