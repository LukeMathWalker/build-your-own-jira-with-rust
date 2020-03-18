mod store {
    use std::collections::HashMap;

/*
pub struct TicketStore {
    /// The collection of stored tickets.
    //todo
    data: ,
}
*/

pub struct TicketStore {
    /// The collection of stored tickets.
    data: HashMap<u32, Ticket>,
}


    fn create_ticket_store() -> TicketStore
    {
        TicketStore{
            data: HashMap::new()
        }
    }

        pub enum Status {
            ToDo,
            InProgress,
            Blocked,
            Done
        }

        pub struct Ticket {
            title: String,
            description: String,
            status: Status
        }

        pub fn create_ticket(title: String, description: String, status: Status) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty!");
            }
            if title.len() > 50 {
                panic!("A title cannot be longer than 50 characters!");
            }
            if description.len() > 3000 {
                panic!("A description cannot be longer than 3000 characters!");
            }

            Ticket {
                title,
                description,
                status,
            }
        }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn lets_store_a_ticket_in_the_new_store()
        {
            let mut store = create_ticket_store();
            let title = "a_modest_title";
            let ticket = create_ticket(title.into(),
                                       "an_enlightened_description".into(),
                                       Status::InProgress);
            let ticket_id = 1;

            store.data.insert(ticket_id, ticket);


            assert_eq!(title, store.data.get(&ticket_id).unwrap().title)

        }




    }
}
