use std::{
    collections::HashMap,
    path::PathBuf,
    io::{Read, Write}
};

use crate::error;

#[derive(Debug)]
pub struct Report {
    id: u64,
    date: String,
    sell: u64,
    income: u128
}

pub struct Reports {
    reports: HashMap<u64, Report>
}

impl Reports {
    pub fn new() -> Self { Self { reports: HashMap::new() } }

    pub fn add(&mut self, report: Report) {
        self.reports.insert(report.id, report);
    }

    pub fn next_id(&self) -> u64 {
        let mut ids: Vec<_> = self.reports.keys().collect();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1
        }
    }

    pub fn into_vec(mut self) -> Vec<Report> {
        let mut reps: Vec<_> = self.reports.drain().map(|kv| kv.1).collect();
        reps.sort_by_key(|rep| rep.id);
        reps
    }
}

impl Default for Reports {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_report(report: &str) -> Result<Report, error::ParseError> {
    let strings: Vec<&str> = report.split(',').collect();

    let id = match strings.first() {
        Some(id) => id.parse::<u64>()?,
        None => return Err(error::ParseError::InvalidInput("id - reports"))
    };

    let date = match strings.get(1).filter(|date| !date.is_empty()) {
        Some(date) => date.to_string(),
        None => return Err(error::ParseError::MissingField("date"))
    };

    let sell = match strings.get(2) {
        Some(sell) => sell.parse::<u64>()?,
        None => return Err(error::ParseError::InvalidInput("sell"))
    };

    let income = match strings.get(3) {
        Some(income) => income.trim().parse::<u128>()?,
        None => return Err(error::ParseError::InvalidInput("income"))
    };

    Ok(Report { id, date, sell, income })
}

fn parse_reports(reports: String, verbose: bool) -> Reports{
    let mut reps = Reports::new();

    for(i, report) in reports.split('\n').enumerate() {
        if report.is_empty() {
            continue;
        }
        match parse_report(report) {
            Ok(rep) => reps.add(rep),
            Err(err) => {
                if verbose {
                    println!("Error on {}: {} - {}\n", i+1, err, report);
                }
            }
        }
    }
    reps
}

pub fn load_reports(verbose: bool) -> std::io::Result<Reports> {
    let mut file = std::fs::File::open(PathBuf::from("bin/report.csv"))?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_reports(buffer, verbose))
}

pub fn save_reports(reports: Reports) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open(PathBuf::from("bin/report.csv"))?;

    file.write_all( b"id,date,sell,income\n")?;

    file.flush()?;

    for report in reports.into_vec().into_iter() {
        file.write_all(format!("{},{},{},{}\n", report.id, report.date, report.sell, report.income).as_bytes())?;
    }

    file.flush()?;

    Ok(())
}