//! The core work is now complete: we have implemented the functionality we wanted to have in
//! our JIRA clone.
//!
//! Nonetheless, we still can't probe our system interactively: there is no user interface.
//! That will be the focus of the next (and last) section.
//!
//! Take your time to review what you did - you have come a long way!
use super::id_generation::TicketId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketStore {
    data: HashMap<TicketId, Ticket>,
    current_id: TicketId,
}

impl TicketStore {
    pub fn new() -> TicketStore {
        TicketStore {
            data: HashMap::new(),
            current_id: 0,
        }
    }

    pub fn save(&mut self, draft: TicketDraft) -> TicketId {
        let id = self.generate_id();
        let timestamp = Utc::now();
        let ticket = Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
            created_at: timestamp.clone(),
            updated_at: timestamp,
        };
        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    pub fn list(&self) -> Vec<&Ticket> {
        self.data.values().collect()
    }

    pub fn update(&mut self, id: &TicketId, patch: TicketPatch) -> Option<&Ticket> {
        if let Some(ticket) = self.data.get_mut(id) {
            if let Some(title) = patch.title {
                ticket.title = title
            }
            if let Some(description) = patch.description {
                ticket.description = description
            }
            if let Some(status) = patch.status {
                ticket.status = status
            }

            ticket.updated_at = Utc::now();

            Some(ticket)
        } else {
            None
        }
    }

    pub fn delete(&mut self, id: &TicketId) -> Option<DeletedTicket> {
        self.data.remove(id).map(|ticket| DeletedTicket {
            ticket,
            deleted_at: Utc::now(),
        })
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn new(title: String) -> Result<Self, ValidationError> {
        if title.is_empty() {
            return Err(ValidationError("Title cannot be empty!".to_string()));
        }
        if title.len() > 50 {
            return Err(ValidationError(
                "A title cannot be longer than 50 characters!".to_string(),
            ));
        }
        Ok(Self(title))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TicketDescription(String);

impl TicketDescription {
    pub fn new(description: String) -> Result<Self, ValidationError> {
        if description.len() > 3000 {
            Err(ValidationError(
                "A description cannot be longer than 3000 characters!".to_string(),
            ))
        } else {
            Ok(Self(description))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketPatch {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeletedTicket {
    ticket: Ticket,
    deleted_at: DateTime<Utc>,
}

impl DeletedTicket {
    pub fn ticket(&self) -> &Ticket {
        &self.ticket
    }
    pub fn deleted_at(&self) -> &DateTime<Utc> {
        &self.deleted_at
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ValidationError(String);

impl Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    id: TicketId,
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Ticket {
    pub fn title(&self) -> &TicketTitle {
        &self.title
    }
    pub fn description(&self) -> &TicketDescription {
        &self.description
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn id(&self) -> &TicketId {
        &self.id
    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_next_step_of_your_journey() {
        let i_am_ready_to_continue = true;

        assert!(i_am_ready_to_continue);
    }
}
