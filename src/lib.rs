use chrono::{Duration, Utc};
use csv::Reader;
use dotenv::dotenv;
use schedule_flows::schedule_cron_job;
use sendgrid_flows::{send_email, Email};
use slack_flows::send_message_to_channel;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    //time_to_invoke is a string of 3 numbers separated by spaces, representing minute, hour, and day
    //* is the spaceholder for non-specified numbers
    let mut time_to_invoke = env::var("time_to_invoke").unwrap_or("* 12 *".to_string());
    time_to_invoke.push_str(" * *");

    schedule_cron_job(time_to_invoke, String::from("cron_job_evoked"), |body| {
        callback(body)
    })
    .await;
}

async fn callback(_payload: Vec<u8>) {
    let sendgrid_token_name =
        env::var("sendgrid_token_name").unwrap_or("jaykchen@gmail.com".to_string());

    let file_path = Path::new("path_to_your_csv_file.csv");
    let file = File::open(&file_path).expect("Could not open csv file");
    let mut reader = csv::Reader::from_reader(file);

    for result in reader.records() {
        let record = result.expect("a CSV record");
        let username = &record[0];
        let email = &record[1];

        let content = format!(
            r#"
        Hi {}, <br/>
        Welcome to the {} community, thank you for your contribution!"#,
            username, "your_project"
        );
        let email_obj = Email {
            to: vec![email.to_string()],
            subject: String::from("Thank you for contributing to this repository"),
            content: content,
        };
        send_email(&sendgrid_token_name, &email_obj).expect("failed to send email");
    }
}
