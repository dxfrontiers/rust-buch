#[cfg(test)]
mod tests{
    use crate::tests::ticket_system::{Ticket, Envelope};

    mod ticket_system {
        use std::cell::Cell;
        use std::rc::Rc;

        pub struct Envelope{
            pub ticket: Rc<Ticket>
        }

        impl Envelope{
            pub fn order_ticket() -> Self{
                let ticket = Ticket::new(7);
                Envelope{ ticket: Rc::new(ticket) }
            }
        }

        pub struct Ticket {
            pub id: u32,
            used: Cell<bool>
        }

        impl Ticket{
            fn new(id: u32) -> Self {
                Ticket{ id, used: Cell::new(false) }
            }

            pub fn is_spent(&self) -> bool {
                self.used.get()
            }

            pub fn spend(&self) -> Result<(),()>{
                match self.used.replace(true){
                    false => Ok(()),
                    true => Err(())
                }
            }
        }
    }


    #[test]
    fn test_ticket(){
        println!();
        let Envelope{ ticket } = Envelope::order_ticket();
        //ticket.id=9090;
        println!("Ticket_id: {}",ticket.id);
        println!("Ticket_spent: {}",ticket.is_spent());
        assert_eq!(ticket.is_spent(),false);
        ticket.spend().expect("Ticket has been used before");
        println!("Ticket_spent: {}",ticket.is_spent());
        assert!(ticket.is_spent());
        assert!(ticket.spend().is_err());
    }
}