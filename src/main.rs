use chrono::prelude::*;
use taskot::*;

use tokio;
#[macro_use] extern crate rocket;
use rocket::config::Config;

#[rocket::main]
async fn main() {

    // Run web server in a separate async task
    tokio::spawn(async {

        // Define the web server's configuration
        let figment = rocket::Config::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("log_level", "off"))
            .merge(("secret_key", std::env::var("SECRET_KEY").expect("SECRET_KEY is not defined.")));
        let config = Config::from(figment);

        // Launch the web server
        let _rocket = rocket::custom(&config)
        .mount("/", routes![tasks])
        .launch()
        .await;
    }); 

    // Email settings
    let email_host = std::env::var("EMAIL_HOST").expect("EMAIL_HOST is not defined.");
    let email_username = std::env::var("EMAIL_HOST_USERNAME").expect("EMAIL_HOST_USERNAME is not defined.");
    let email_password = std::env::var("EMAIL_HOST_PASSWORD").expect("EMAIL_HOST_PASSWORD is not defined.");
    let email_from = std::env::var("EMAIL_FROM").expect("EMAIL_FROM is not defined.");
    println!("Email host = {:?}, username = {:?}, password = ***, from = {:?}.",email_host, email_username, email_from);

    // Tasks and people
    let mut tasks = Task::from_vars(prefixed_vars("TASK"));
    println!("tasks = {:?}", tasks);
    assert_ne!(tasks.len(), 0, "TASK_0 is not defined.");

    let people = Person::from_vars(prefixed_vars("PERSON"));
    println!("people = {:?}", people);
    assert_ne!(people.len(), 0, "PERSON_0 is not defined.");

    assert_eq!(tasks.len(), people.len(), "There must be the same amount of people and tasks.");

    // Main loop
    let n_rotations = week_number(Local::now()) % tasks.len() as i64;
    tasks.rotate_left(n_rotations as usize);

    loop {
        println!("Waiting until next Monday at 08:30.");
        let wait_duration = until_monday_08h30(Local::now());
        std::thread::sleep(wait_duration.to_std().unwrap());

        for (person, task) in people.iter().zip(&tasks) {
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

        tasks.rotate_left(1);
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

// Route to get the tasks of every person (on the web server started in the main function)
#[get("/tasks")]
fn tasks() -> String {
    // Tasks and people
    let tasks = Task::from_vars(prefixed_vars("TASK"));
    assert_ne!(tasks.len(), 0, "TASK_0 is not defined.");

    let people = Person::from_vars(prefixed_vars("PERSON"));
    assert_ne!(people.len(), 0, "PERSON_0 is not defined.");

    // String with tasks and people
    let mut printing = String::new();
    for (person, task) in people.iter().zip(&tasks) {
        printing.push_str(format!("{}: {}\n", person.name, task.name).to_owned().as_str());
    }

    printing
}