use chrono::{prelude::*, Duration};

use super::utils::models::{Task, Person};
use super::utils::utils::{week_number, until_monday_08h30};

#[test]
fn task_corresponds_to_var() {
    let tasks = Task::from_var("Vaisselle".to_owned());
    let expected = Some(Task { name: "Vaisselle".to_owned() });
    assert_eq!(tasks, expected);
}

#[test]
fn tasks_correspond_to_vars() {
    let tasks = Task::from_vars(vec![
        "Vaisselle".to_owned(),
        "Sanitaires".to_owned(),
        "Sol couloir".to_owned(),
    ]);
    let expected = vec![
        Task { name: "Vaisselle".to_owned() },
        Task { name: "Sanitaires".to_owned() },
        Task { name: "Sol couloir".to_owned() },
    ];
    assert_eq!(tasks, expected);
}

#[test]
fn person_corresponds_to_var() {
    let person = Person::from_var("Michel;michel.piras@student.unamur.be".to_owned());
    let expected = Some(Person {
        name: "Michel".to_owned(),
        email_address: "michel.piras@student.unamur.be".to_owned(),
    });
    assert_eq!(person, expected);
}

#[test]
fn people_correspond_to_vars() {
    let people = Person::from_vars(vec![
        "Michel;michel.piras@student.unamur.be".to_owned(),
        "Bob;bob.luycx@gmail.com".to_owned(),
    ]);
    let expected = vec![
        Person {
            name: "Michel".to_owned(),
            email_address: "michel.piras@student.unamur.be".to_owned(),
        },
        Person {
            name: "Bob".to_owned(),
            email_address: "bob.luycx@gmail.com".to_owned(),
        },
    ];
    assert_eq!(people, expected);
}

#[test]
fn person_is_none_if_invalid() {
    let person = Person::from_var("Bob,bob.luycx@gmail.com".to_owned());
    let expected = None;
    assert_eq!(person, expected);
}

#[test]
fn people_are_ignored_if_invalid() {
    let people = Person::from_vars(vec![
        "Michel;michel.piras@student.unamur.be".to_owned(),
        "Claude <claude.dupont@skynet.be>".to_owned(),
        "Bob;bob.luycx@gmail.com".to_owned(),
    ]);
    let expected = vec![
        Person {
            name: "Michel".to_owned(),
            email_address: "michel.piras@student.unamur.be".to_owned(),
        },
        Person {
            name: "Bob".to_owned(),
            email_address: "bob.luycx@gmail.com".to_owned(),
        },
    ];
    assert_eq!(people, expected);
}

#[test]
fn week_number_works() {
    let now = DateTime::parse_from_rfc2822("Thu, 2 Jun 2022 22:52:02 GMT").unwrap().with_timezone(&Utc);
    let number = week_number(now);
    assert_eq!(number, 2735);
}

#[test]
fn week_number_works_on_mondays_before_08h30() {
    let now = DateTime::parse_from_rfc2822("Mon, 6 Jun 2022 07:26:05 GMT").unwrap().with_timezone(&Utc);
    let number = week_number(now);
    assert_eq!(number, 2735);
}

#[test]
fn week_number_works_on_mondays_at_08h30() {
    let now = DateTime::parse_from_rfc2822("Mon, 6 Jun 2022 08:30:00 GMT").unwrap().with_timezone(&Utc);
    let number = week_number(now);
    assert_eq!(number, 2736);
}

#[test]
fn week_number_works_on_mondays_after_08h30() {
    let now = DateTime::parse_from_rfc2822("Mon, 6 Jun 2022 10:45:11 GMT").unwrap().with_timezone(&Utc);
    let number = week_number(now);
    assert_eq!(number, 2736);
}

#[test]
fn until_monday_08h30_works() {
    let now = DateTime::parse_from_rfc2822("Wed, 18 May 2022 13:56:21 GMT").unwrap().with_timezone(&Utc);
    let duration = until_monday_08h30(now);
    let expected = Duration::days(4) + Duration::hours(18) + Duration::minutes(33) + Duration::seconds(39);
    assert_eq!(duration, expected);
}

#[test]
fn until_monday_08h30_works_on_mondays_before_08h30() {
    let now = DateTime::parse_from_rfc2822("Mon, 21 Feb 2022 06:06:06 GMT").unwrap().with_timezone(&Utc);
    let duration = until_monday_08h30(now);
    let expected = Duration::hours(2) + Duration::minutes(23) + Duration::seconds(54);
    assert_eq!(duration, expected);
}

#[test]
fn until_monday_08h30_works_on_mondays_at_08h30() {
    let now = DateTime::parse_from_rfc2822("Mon, 21 Feb 2022 08:30:00 GMT").unwrap().with_timezone(&Utc);
    let duration = until_monday_08h30(now);
    let expected = Duration::weeks(1);
    assert_eq!(duration, expected);
}

#[test]
fn until_monday_08h30_works_on_mondays_after_08h30() {
    let now = DateTime::parse_from_rfc2822("Mon, 21 Feb 2022 09:06:06 GMT").unwrap().with_timezone(&Utc);
    let duration = until_monday_08h30(now);
    let expected = Duration::days(6) + Duration::hours(23) + Duration::minutes(23) + Duration::seconds(54);
    assert_eq!(duration, expected);
}
