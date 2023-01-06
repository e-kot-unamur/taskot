use serde::Deserialize;

/// This struct represents a task that can be assigned to a person.
#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub name: String,
}

impl Task {
    pub fn from_vars(vars: Vec<String>) -> Vec<Self> {
        vars.into_iter()
            .filter_map(Self::from_var)
            .collect()
    }

    pub fn from_var(name: String) -> Option<Self> {
        Some(Self { name })
    }
}

/// This struct represents a person to whom a task can be assigned.
#[derive(Debug, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub email_address: String,
}

impl Person {
    pub fn from_vars(vars: Vec<String>) -> Vec<Self> {
        vars.into_iter()
            .filter_map(Self::from_var)
            .collect()
    }

    pub fn from_var(var: String) -> Option<Self> {
        var.split_once(';')
            .map(|(name, email_address)| Person {
                name: name.to_owned(),
                email_address: email_address.to_owned(),
            })
    }
}

// This struct represents the HTML page that is sent on the / route
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct HtmlPage {
    pub head: String,
    pub title: String,
    pub ul_content: String,
    pub form_start: String,
    pub form_n_rotations: String,
    pub form_key: String,
    pub form_submit: String,
}
