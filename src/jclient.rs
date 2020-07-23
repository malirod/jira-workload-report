// SPDX-License-Identifier: MIT

// Copyright (C) 2020 Malinovsky Rodion (rodionmalino@gmail.com)

use crate::config::Config;
use crate::config::Credentials;

use anyhow::Result;
use chrono::prelude::*;
use chrono::serde::ts_milliseconds;
use chrono::{NaiveDate, Utc, Weekday};
use futures::future;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserTimersheet {
    pub worklog: Vec<Worklog>,
}

#[derive(Deserialize, Debug)]
pub struct Worklog {
    pub key: String,
    pub summary: String,
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "timeSpent")]
    pub time_spent_secs: i64,
    pub author: String,
    #[serde(with = "ts_milliseconds")]
    pub created: DateTime<Utc>,
}

async fn get_timesheet(
    client: &Client,
    credentials: &Credentials,
    user_name: &str,
    start: &NaiveDate,
    end: &NaiveDate,
) -> Result<UserTimersheet> {
    let issues_response = client
        .get(&format!(
            "{}/rest/timesheet-gadget/1.0/raw-timesheet.json?targetUser={}&startDate={}&endDate={}",
            credentials.server,
            user_name,
            start.format("%Y-%m-%d"),
            end.format("%Y-%m-%d")
        ))
        .basic_auth(&credentials.username, Some(&credentials.password))
        .send()
        .await?;
    let user_timesheet: UserTimersheet = issues_response.json().await?;
    Ok(user_timesheet)
}

pub async fn get_timesheets(config: &Config) -> Result<Vec<UserTimersheet>> {
    let now = Utc::now();
    let period_start = NaiveDate::from_isoywd(now.year(), config.options.weeks, Weekday::Mon);
    let period_end = NaiveDate::from_isoywd(now.year(), config.options.weeks, Weekday::Sun);
    log::info!("Period: from {} to {}", period_start, period_end);

    let jclient = Client::new();
    let mut result: Vec<UserTimersheet> = Vec::new();

    let users_ref = &config.options.users;
    let timesheets = future::join_all(users_ref.iter().map(|user_name| {
        let jclient_ref = &jclient;
        let credentials_ref = &config.credentials;
        let period_start_ref = &period_start;
        let period_end_ref = &period_end;
        async move {
            get_timesheet(
                &jclient_ref,
                &credentials_ref,
                &user_name,
                &period_start_ref,
                &period_end_ref,
            )
            .await
        }
    }))
    .await;

    for timesheet in timesheets {
        match timesheet {
            Ok(timesheet) => result.push(timesheet),
            Err(error) => log::error!("Failed to get workload. Error: {}", error),
        }
    }

    Ok(result)
}
