use chrono::{prelude::*, Duration};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

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

    fn from_var(name: String) -> Option<Self> {
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

    fn from_var(var: String) -> Option<Self> {
        var.split_once(';')
            .map(|(name, email_address)| Person {
                name: name.to_owned(),
                email_address: email_address.to_owned(),
            })
    }
}

/// Fetches numbered environment variables with a given prefix.
///
/// # Example
///
/// ```
/// std::env::set_var("THING_0", "hi");
/// std::env::set_var("THING_1", "hello");
/// let vars = taskot::prefixed_vars("THING");
/// assert_eq!(vars, ["hi", "hello"]);
/// ```
pub fn prefixed_vars(prefix: &str) -> Vec<String> {
    let mut vars = vec![];
    let mut index = 0;
    loop {
        let name = format!("{}_{}", prefix, index);
        index += 1;
        if let Ok(var) = std::env::var(name) {
            vars.push(var);
        } else {
            return vars;
        }
    }
}

/// Returns the number of whole weeks elapsed since Mon, 28 Dec 1969 08:30:00.
pub fn week_number<Tz: TimeZone>(now: DateTime<Tz>) -> i64 {
    // Epoch time starts actually starts on a Thursday.
    // Therefore we add 3 days to our timestamp.
    (now + Duration::days(3) - Duration::hours(8) - Duration::minutes(30)).timestamp()
        / Duration::weeks(1).num_seconds()
}

/// Returns the remaining duration until next Monday at 08:30:00.
pub fn until_monday_08h30<Tz: TimeZone>(now: DateTime<Tz>) -> Duration {
    let today_08h30 = now.date().and_hms(8, 30, 0);

    let days_to_wait = if today_08h30.weekday() == Weekday::Mon {
        if now <= today_08h30 {
            Duration::zero()
        } else {
            Duration::weeks(1)
        }
    } else {
        Duration::days(7 - today_08h30.weekday().num_days_from_monday() as i64)
    };

    let monday_08h30 = today_08h30 + days_to_wait;
    monday_08h30 - now
}

/// Sends an email to an SMTPS server on port 465.
///
/// # Parameters
///
/// * `host` is the SMTP server name.
/// * `username` is the sender's username used for authentication.
/// * `password` is the sender's password used for authentication.
/// * `from` is the source address.
/// * `to` is the destination address.
/// * `subject` is the email's subject.
/// * `body` is the email's content.
pub fn send_email(
    host: &str,
    username: &str,
    password: &str,
    from: &str,
    to: &str,
    subject: String,
    body: String,
) -> Result<<SmtpTransport as Transport>::Ok, <SmtpTransport as Transport>::Error> {
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN) // Allows UTF-8.
        .body(body)
        .unwrap();

    let credentials = Credentials::new(username.to_owned(), password.to_owned());

    let mailer = SmtpTransport::relay(host)
        .unwrap()
        .credentials(credentials)
        .build();

    mailer.send(&email)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let expected = Duration::zero();
        assert_eq!(duration, expected);
    }

    #[test]
    fn until_monday_08h30_works_on_mondays_after_08h30() {
        let now = DateTime::parse_from_rfc2822("Mon, 21 Feb 2022 09:06:06 GMT").unwrap().with_timezone(&Utc);
        let duration = until_monday_08h30(now);
        let expected = Duration::days(6) + Duration::hours(23) + Duration::minutes(23) + Duration::seconds(54);
        assert_eq!(duration, expected);
    }
}
