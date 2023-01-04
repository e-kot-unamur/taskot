use taskot::*;
use chrono::prelude::*;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}
};
use tokio;
use axum::{
    routing::get,
    response::{Redirect, Html},
    Router,
    extract::State
};

#[tokio::main]
async fn main() {

    // Create a new task list, shared between all threads
    let tasks = Arc::new(Mutex::new(Task::from_vars(prefixed_vars("TASK"))));
    println!("tasks = {:?}", tasks);
    assert_ne!(tasks.lock().unwrap().len(), 0, "TASK_0 is not defined.");

    // Run web server in a separate async task
    tokio::spawn(start_server(Arc::clone(&tasks))); 

    // Email settings
    let email_host = std::env::var("EMAIL_HOST").expect("EMAIL_HOST is not defined.");
    let email_username = std::env::var("EMAIL_HOST_USERNAME").expect("EMAIL_HOST_USERNAME is not defined.");
    let email_password = std::env::var("EMAIL_HOST_PASSWORD").expect("EMAIL_HOST_PASSWORD is not defined.");
    let email_from = std::env::var("EMAIL_FROM").expect("EMAIL_FROM is not defined.");
    println!("Email host = {:?}, username = {:?}, password = ***, from = {:?}.",email_host, email_username, email_from);

    // People list
    let people = Person::from_vars(prefixed_vars("PERSON"));
    println!("people = {:?}", people);
    assert_ne!(people.len(), 0, "PERSON_0 is not defined.");

    assert_eq!(tasks.lock().unwrap().len(), people.len(), "There must be the same amount of people and tasks.");

    // Main loop
    let n_rotations = week_number(Local::now()) % tasks.lock().unwrap().len() as i64;

    let mut mutable_tasks = tasks.lock().unwrap();
    mutable_tasks.rotate_left(n_rotations as usize);
    drop(mutable_tasks);

    loop {
        println!("Waiting until next Monday at 08:30.");
        let wait_duration = until_monday_08h30(Local::now());
        std::thread::sleep(wait_duration.to_std().unwrap());

        for (person, task) in people.iter().zip(tasks.lock().unwrap().iter()) {
            println!("Sending email to {} <{}> (task = {:?}).", person.name, person.email_address, task.name);
            let result = send_email(
                &email_host,
                &email_username,
                &email_password,
                &email_from,
                &format!("{} <{}>", person.name, person.email_address),
                generate_email_subject(person, task),
                generate_email_body(person, task),
            );

            if let Err(e) = result {
                eprintln!("Failed: {}.", e);
            }
        }

        let mut mutable_tasks = tasks.lock().unwrap();
        mutable_tasks.rotate_left(1);
        drop(mutable_tasks);
    }
}

/// Writes the email's subject according to the person and its task.
fn generate_email_subject(_person: &Person, task: &Task) -> String {
    format!("Ta tâche de cette semaine ({})", task.name)
}

/// Writes the email's body according to the person and its task.
fn generate_email_body(person: &Person, task: &Task) -> String {
    format!(
        "Bonjour {},\n\
        \n\
        Cette semaine, ta tâche est \"{}\".\n\
        \n\
        Cordialement,\n\
        TasKot v0.1.0",
        person.name, task.name,
    )
}

async fn start_server(tasks: Arc<Mutex<Vec<Task>>>) {
    // Build our application with a route
    let app = Router::new()
    .route("/", get(index))
    .route("/rotate", get(rotate))
    .with_state(Arc::clone(&tasks));

    // Run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// Route to get the tasks of every person (on the web server started in the main function)
async fn index(State(tasks): State<Arc<Mutex<Vec<Task>>>>) -> Html<String> {

    // People list
    let people = Person::from_vars(prefixed_vars("PERSON"));
    assert_ne!(people.len(), 0, "PERSON_0 is not defined.");

    // String with tasks and people (to be printed on the web page, in HTML)
    let mut printing = String::new();
    printing.push_str("<h1>Tâches de cette semaine :</h1>\n<ul>\n");
    for (person, task) in people.iter().zip(tasks.lock().unwrap().iter()) {
        printing.push_str(format!("<li>{}: {}</li>\n", person.name, task.name).to_owned().as_str());
    }
    printing.push_str("</ul>\n<br>\n<form action='http://taskot.e-kot.be/rotate'><input type='submit' value='Tourner la roue' /></form>");

    Html(printing)
}

// Route to rotate the tasks of every person (on the web server started in the main function)
async fn rotate(State(tasks): State<Arc<Mutex<Vec<Task>>>>) -> Redirect {

    let mut mutate_tasks = tasks.lock().unwrap();
    mutate_tasks.rotate_left(1);
    
    Redirect::to("/")
}