use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}
};
use chrono::prelude::*;
use tokio;
use axum::{
    routing::{get, post},
    Router
};

mod utils;
use utils::mail::*;
use utils::models::*;
use utils::routes::*;
use utils::utils::*;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {

    // Create a new task list, shared between all threads
    let tasks = Arc::new(Mutex::new(Task::from_vars(prefixed_vars("TASK"))));
    println!("tasks = {:?}", tasks);
    assert_ne!(tasks.lock().unwrap().len(), 0, "TASK_0 is not defined.");

    // Run web server in a separate async task, if the environment variable RUN_WEB_SERVER is set to true
    if std::env::var("RUN_WEB_SERVER").unwrap_or("false".to_string()) == "true" {
        tokio::spawn(start_server(Arc::clone(&tasks)));
    }

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

        // Rotate tasks on Monday
        let mut mutable_tasks = tasks.lock().unwrap();
        mutable_tasks.rotate_left(1);
        drop(mutable_tasks);

        // Send emails
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
    }
}

// Start the web server
async fn start_server(tasks: Arc<Mutex<Vec<Task>>>) {
    // Build our application with a route
    let app = Router::new()
    .route("/", get(index))
    .route("/rotate", post(rotate))
    .with_state(Arc::clone(&tasks))
    .fallback(fallback);

    // Run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
