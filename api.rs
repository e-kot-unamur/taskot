#[macro_use] extern crate rocket;
mod src;
use src::{prefixed_vars, Person, Task};

#[get("/")]
async fn index() -> &'static str {
    "Hello Wold."
}

#[get("/tasks")]
fn tasks() -> String {
    // Tasks and people
    let tasks = Task::from_vars(prefixed_vars("TASK"));
    assert_ne!(tasks.len(), 0, "TASK_0 is not defined.");

    let people = Person::from_vars(prefixed_vars("PERSON"));
    assert_ne!(people.len(), 0, "PERSON_0 is not defined.");

    let mut printing = String::new();
    // Create a string with the tasks of every person
    for (person, task) in people.iter().zip(&tasks) {
        printing.push_str(format!("{}: {}\n", person.name, task.name).to_owned().as_str());
    }

    printing
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, tasks])
}