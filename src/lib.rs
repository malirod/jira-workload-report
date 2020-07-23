// SPDX-License-Identifier: MIT

// Copyright (C) 2020 Malinovsky Rodion (rodionmalino@gmail.com)

mod config;
mod jclient;

use config::Config;
use jclient::get_timesheets;

use anyhow::Result;

pub async fn run() -> Result<()> {
    let config = Config::new()?;
    log::info!("Using JIRA: {}", config.credentials.server);
    let timesheets = get_timesheets(&config).await?;
    for user_timesheet in timesheets.iter() {
        for issue in user_timesheet.worklog.iter() {
            for entry in issue.entries.iter() {
                log::info!(
                    "Key: {}; Summary: {};  User: {}; Date: {}; Spent(h): {:.2}",
                    issue.key,
                    issue.summary,
                    entry.author,
                    entry.created.format("%Y-%m-%d"),
                    (entry.time_spent_secs as f64) / 3600.0
                );
            }
        }
    }
    Ok(())
}
