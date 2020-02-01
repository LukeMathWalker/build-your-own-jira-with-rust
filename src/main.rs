pub mod store;
pub mod models;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::store::TicketStore;
    use crate::models::{TicketDraft, Status};
    use fake::Fake;

    #[test]
    fn create_ticket_test() {
        let faker = fake::Faker;

        //arrange
        let draft = TicketDraft {
            title: faker.fake(),
            description: faker.fake(),
        };

        let mut ticket_store = TicketStore::new();

        //act
        let ticket_id = ticket_store.create(draft.clone());

        //assert
        let ticket = ticket_store.get(&ticket_id).expect("Failed to retrieve ticket.");
        assert_eq!(ticket.title, draft.title);
        assert_eq!(ticket.description, draft.description);
        assert_eq!(ticket.status, Status::ToDo);
    }
}
