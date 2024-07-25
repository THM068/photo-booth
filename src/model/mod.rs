use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub surname: String,
    pub email: String,
}

impl Contact {
    pub fn new(name: String, surname: String, email: String) -> Contact {
        Contact {
            name,
            surname,
            email,
        }
    }

    pub fn setName(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
}
