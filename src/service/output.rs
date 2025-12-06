use crate::domain::OutputFormat;
use anyhow::Context;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn write_results<P>(
    results: &[Value],
    results_directory: P,
    format: &OutputFormat,
    reference_time: DateTime<Utc>,
) -> anyhow::Result<PathBuf>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(&results_directory).with_context(|| {
        format!(
            "failed to create results directory: {}",
            results_directory.as_ref().to_string_lossy()
        )
    })?;

    let file_name = reference_time.format("%b-%d-%H-%M-%S");
    let output_file_path = match format {
        OutputFormat::Csv => todo!(),
        OutputFormat::Json => results_directory
            .as_ref()
            .join(format!("{}.json", file_name)),
    };

    let file = File::create(&output_file_path).with_context(|| {
        format!(
            "couldn't open output file: {}",
            output_file_path.to_string_lossy()
        )
    })?;

    match format {
        OutputFormat::Csv => todo!(),
        OutputFormat::Json => write_json(results, file)?,
    }

    Ok(output_file_path)
}

fn write_json<W>(results: &[Value], mut writer: W) -> anyhow::Result<()>
where
    W: Write,
{
    let json_string =
        serde_json::to_string_pretty(results).context("couldn't serialize results to JSON")?;
    writer
        .write_all(json_string.as_bytes())
        .context("couldn't write bytes to file")?;

    Ok(())
}
