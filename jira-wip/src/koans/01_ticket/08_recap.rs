/// We have come quite a long way now: from how to define a struct to traits and derive macros,
/// touching on tests, module system, visibility, ownership and method syntax.
/// Take a deep breath, stretch a bit, review what we have done.
///
/// Then get ready to dive in the next section!

#[derive(PartialEq, Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
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
    #[test]
    fn the_next_step_of_your_journey() {
        let i_am_ready_to_continue = __;

        assert!(i_am_ready_to_continue);
    }
}
