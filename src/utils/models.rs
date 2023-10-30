use serde::Deserialize;
use json;
use std::{fs::File, io::{Write, Read}, path::Path};

use super::utils::prefixed_vars;

/// This struct represents a task that can be assigned to a person.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    pub name: String,
}

impl Task {
    pub fn from_file() -> Vec<Self> {
        if !(Path::new("/taskot/persistent/tasks.json").exists()) {
            File::create("/taskot/persistent/tasks.json").unwrap();
        }

        let mut file = File::open("/taskot/persistent/tasks.json").unwrap();
        if file.metadata().unwrap().len() == 0 {
            return vec![]
        }
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let vars: Vec<String> = json::parse(&contents).unwrap().members().map(|s| s.to_string()).collect();
        Self::from_vars(vars)
    }

    pub fn from_vars(vars: Vec<String>) -> Vec<Self> {
        vars.into_iter()
            .filter_map(Self::from_var)
            .collect()
    }

    pub fn from_var(name: String) -> Option<Self> {
        Some(Self { name })
    }

    pub fn save_tasks(vars: Vec<Self>) {
        let extracted_vars: Vec<String> = vars.into_iter().map(|t| t.name).collect();
        let json = json::stringify(extracted_vars);
        let mut file = File::create("/taskot/persistent/tasks.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn get() -> Vec<Self> {
        let file_tasks = Self::from_file();
        let env_tasks = Self::from_vars(prefixed_vars("TASK"));
        let env_tasks_str = env_tasks.iter().map(|t| t.name.clone()).collect::<Vec<String>>();
        let tasks = if file_tasks.len() != 0 && file_tasks.len() == env_tasks.len() 
        && file_tasks.iter().all(|a| env_tasks_str.contains(&a.name)) {
            file_tasks
        } else {
            env_tasks
        };
        tasks
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
