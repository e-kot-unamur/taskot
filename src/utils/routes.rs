use std::{
    str,
    sync::{Arc, Mutex},
};
use axum::{
    response::{Redirect, Html},
    extract::{State, RawForm},
};
use toml::from_str;

use super::models::{Task, Person, HtmlPage};
use super::utils::prefixed_vars;

// Route to get the tasks of every person
pub async fn index(State(tasks): State<Arc<Mutex<Vec<Task>>>>) -> Html<String> {

    // People list
    let people = Person::from_vars(prefixed_vars("PERSON"));

    // Read the HTML page from the Patterns.toml file
    let path = std::env::current_dir().unwrap();
    let entries = std::fs::read_dir(path.as_path()).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
    let content = std::fs::read_to_string("./src/Patterns.toml").unwrap();
    let html_page = from_str::<HtmlPage>(content.as_str()).unwrap();

    // Build the HTML page
    let mut printing = String::new();
    printing.push_str(&html_page.head);
    printing.push_str(&html_page.title);
    for (person, task) in people.iter().zip(tasks.lock().unwrap().iter()) {
        printing.push_str(&html_page.ul_content.replace("{person}", person.name.as_str()).replace("{task}", task.name.as_str()));
    }
    printing.push_str(&html_page.form_start);
    printing.push_str(&html_page.form_n_rotations.replace("{rotations_max}", people.len().to_string().as_str()));
    printing.push_str(&html_page.form_key);
    printing.push_str(&html_page.form_submit);

    Html(printing)
}

// Route to rotate the tasks of every person
pub async fn rotate(State(tasks): State<Arc<Mutex<Vec<Task>>>>, RawForm(form): RawForm) -> Redirect {

    // Get the number of rotations and the key from the form
    let string_received = str::from_utf8(&form).unwrap();
    let split = string_received.split("&").collect::<Vec<&str>>();
    let n_rotations = split[0].split("=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
    let key = split[1].split("=").collect::<Vec<&str>>()[1];

    // Rotate the tasks if the key is correct
    let expected_key = std::env::var("ROTATE_KEY").unwrap_or("".to_string());
    if expected_key != "".to_string() && key == expected_key {
        let mut mutate_tasks = tasks.lock().unwrap();
        mutate_tasks.rotate_left(n_rotations);
    }
    
    Redirect::to("/")
}

// Route te redirect to the index route
pub async fn fallback() -> Redirect {
    Redirect::to("/")
}
