use crate::opts::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let iter = headers.iter().zip(record.iter());
        // let json_value = match format  {
        //     OutputFormat::Json => iter.collect::<serde_json::Value>(),
        //     OutputFormat::Yaml => iter.collect::<serde_yaml::Value>(),
        //     OutputFormat::Toml => iter.collect::<toml::Value>(),

        // };
        let json_value = iter.collect::<serde_json::Value>();
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        // OutputFormat::Toml => toml::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
