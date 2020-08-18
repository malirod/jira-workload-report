// SPDX-License-Identifier: MIT

// Copyright (C) 2020 Malinovsky Rodion (rodionmalino@gmail.com)

mod config;
mod jclient;

use config::Config;
use jclient::get_timesheets;
use xlsxwriter::{Workbook, Worksheet};

use anyhow::Result;

const COLUMNT_INDEX_KEY: u16 = 0;
const COLUMNT_INDEX_SUMMARY: u16 = 1;
const COLUMNT_INDEX_AUTHOR: u16 = 2;
const COLUMNT_INDEX_DATE: u16 = 3;
const COLUMNT_INDEX_SPENT: u16 = 4;
const COLUMNT_INDEX_LABEL: u16 = 5;
const COLUMNT_INDEX_PROJECT: u16 = 6;
const COLUMNT_INDEX_COMMON: u16 = 7;
const COLUMNT_INDEX_ARCH: u16 = 8;

const HEADER_PROJECT: &str = "Project (h)";
const HEADER_COMMON: &str = "Common (h)";
const HEADER_ARCH: &str = "Arch (h)";

fn add_heading_to_report(sheet: &mut Worksheet) -> Result<()> {
    sheet.write_string(0, COLUMNT_INDEX_KEY, "Key", None)?;
    sheet.write_string(0, COLUMNT_INDEX_SUMMARY, "Summary", None)?;
    sheet.write_string(0, COLUMNT_INDEX_AUTHOR, "Author", None)?;
    sheet.write_string(0, COLUMNT_INDEX_DATE, "Date", None)?;
    sheet.write_string(0, COLUMNT_INDEX_SPENT, "Spent (h)", None)?;
    sheet.write_string(0, COLUMNT_INDEX_LABEL, "Label", None)?;
    sheet.write_string(0, COLUMNT_INDEX_PROJECT, HEADER_PROJECT, None)?;
    sheet.write_string(0, COLUMNT_INDEX_COMMON, HEADER_COMMON, None)?;
    sheet.write_string(0, COLUMNT_INDEX_ARCH, HEADER_ARCH, None)?;
    Ok(())
}

fn calculate_label(summary: &str) -> Result<String> {
    if summary.contains("[Common]") {
        return Ok("Common".to_string());
    }
    if summary.contains("[Arch]") {
        return Ok("Arch".to_string());
    }
    if summary.contains("Overtime") {
        return Ok("Overtime".to_string());
    }
    if summary.contains("Vacation") {
        return Ok("Vacation".to_string());
    }
    if summary.contains("Sick leaves") {
        return Ok("Sick leaves".to_string());
    }
    Ok("2520".to_string())
}

fn save_report_to_excel(timesheets: &[jclient::UserTimersheet]) -> Result<()> {
    log::info!("Saving date to report");
    let workbook = Workbook::new("report.xlsx");
    for user_timesheet in timesheets.iter() {
        let mut sheet = workbook.add_worksheet(Some(&user_timesheet.user_name))?;
        add_heading_to_report(&mut sheet)?;
        let mut row_index = 1;
        for issue in user_timesheet.worklog.iter() {
            for entry in issue.entries.iter() {
                sheet.write_string(row_index, COLUMNT_INDEX_KEY, &issue.key, None)?;
                sheet.write_string(row_index, COLUMNT_INDEX_SUMMARY, &issue.summary, None)?;
                sheet.write_string(row_index, COLUMNT_INDEX_AUTHOR, &entry.author, None)?;
                sheet.write_string(
                    row_index,
                    COLUMNT_INDEX_DATE,
                    &entry.created.format("%Y-%m-%d").to_string(),
                    None,
                )?;
                let spent_hours = (entry.time_spent_secs as f64) / 3600.0;
                sheet.write_number(row_index, COLUMNT_INDEX_SPENT, spent_hours, None)?;
                sheet.write_string(
                    row_index,
                    COLUMNT_INDEX_LABEL,
                    &calculate_label(&issue.summary)?,
                    None,
                )?;
                sheet.write_formula(
                    row_index,
                    COLUMNT_INDEX_PROJECT,
                    &format!("=IF(F{0}=\"2520\",E{0},0)", row_index + 1),
                    None,
                )?;
                sheet.write_formula(
                    row_index,
                    COLUMNT_INDEX_COMMON,
                    &format!("=IF(F{0}=\"Common\",E{0},0)", row_index + 1),
                    None,
                )?;
                sheet.write_formula(
                    row_index,
                    COLUMNT_INDEX_ARCH,
                    &format!("=IF(F{0}=\"Arch\",E{0},0)", row_index + 1),
                    None,
                )?;
                row_index += 1;
            }
        }
        let last_data_row_index = row_index;
        sheet.write_string(row_index, COLUMNT_INDEX_KEY, "Total", None)?;
        let has_worklog_items = !user_timesheet.worklog.is_empty();
        if has_worklog_items {
            sheet.write_formula(
                row_index,
                COLUMNT_INDEX_SPENT,
                &format!("=SUM(E2:E{})", row_index),
                None,
            )?;
        } else {
            sheet.write_number(row_index, COLUMNT_INDEX_SPENT, 0.0, None)?;
        }
        row_index += 1;
        sheet.write_string(row_index, COLUMNT_INDEX_KEY, HEADER_PROJECT, None)?;
        if has_worklog_items {
            sheet.write_formula(
                row_index + 1,
                COLUMNT_INDEX_KEY,
                &format!("=SUM(G2:G{})", last_data_row_index),
                None,
            )?;
        } else {
            sheet.write_number(row_index + 1, COLUMNT_INDEX_KEY, 0.0, None)?;
        }

        sheet.write_string(row_index, COLUMNT_INDEX_KEY + 1, HEADER_COMMON, None)?;
        if has_worklog_items {
            sheet.write_formula(
                row_index + 1,
                COLUMNT_INDEX_KEY + 1,
                &format!("=SUM(H2:H{})", last_data_row_index),
                None,
            )?;
        } else {
            sheet.write_number(row_index + 1, COLUMNT_INDEX_KEY + 1, 0.0, None)?;
        }

        sheet.write_string(row_index, COLUMNT_INDEX_KEY + 2, HEADER_ARCH, None)?;
        if has_worklog_items {
            sheet.write_formula(
                row_index + 1,
                COLUMNT_INDEX_KEY + 2,
                &format!("=SUM(I2:I{})", last_data_row_index),
                None,
            )?;
        } else {
            sheet.write_number(row_index + 1, COLUMNT_INDEX_KEY + 2, 0.0, None)?;
        }
    }
    log::info!("Saving report");
    Ok(workbook.close()?)
}

pub async fn run() -> Result<()> {
    let config = Config::new()?;
    log::info!("Using JIRA: {}", config.credentials.server);
    let timesheets = get_timesheets(&config).await?;
    let result = save_report_to_excel(&timesheets);
    log::info!("Report completed");
    result
}
