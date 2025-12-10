use crate::cli::{Args, GraphQCommand};
use crate::cmds::{ConsoleError, benchmark_query, execute_query, handle_console_cmd};
use crate::domain::QueryResults;
use crate::repository::get_db_client;
use crate::utils::get_pager;
use crate::view::{ConsoleConfig, get_results};
use anyhow::Context;
use chrono::Utc;
use clap::Parser;
use etcetera::{BaseStrategy, HomeDirError};
use std::io::Read;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("couldn't determine your home directory: {0}")]
    XdgError(#[from] HomeDirError),
    #[error(transparent)]
    ConsoleCmdError(#[from] ConsoleError),
    #[error(transparent)]
    Uncategorised(#[from] anyhow::Error),
}

impl AppError {
    pub fn follow_up(&self) -> Option<String> {
        match self {
            AppError::XdgError(_) => None,
            AppError::ConsoleCmdError(_) => None,
            AppError::Uncategorised(_) => None,
        }
    }

    pub fn is_unexpected(&self) -> bool {
        match self {
            AppError::XdgError(_) => true,
            AppError::ConsoleCmdError(_) => false,
            AppError::Uncategorised(_) => false,
        }
    }
}

pub async fn run() -> Result<(), AppError> {
    let xdg = etcetera::choose_base_strategy()?;
    crate::logging::setup(&xdg)?;
    let args = Args::parse();

    if args.debug {
        print!("DEBUG INFO\n{args}");
        return Ok(());
    }

    match args.command {
        GraphQCommand::Console {
            page_results,
            write_results,
            results_directory,
            results_format,
        } => {
            let console_config = ConsoleConfig {
                page_results,
                write_results,
                results_directory,
                results_format,
                history_file_path: xdg.data_dir().join("grf").join("history.txt"),
            };

            handle_console_cmd(console_config).await?;
        }
        GraphQCommand::Query {
            query,
            page_results,
            benchmark,
            bench_num_runs,
            bench_num_warmup_runs,
            print_query,
            write_results,
            results_directory,
            results_format,
        } => {
            if benchmark && write_results {
                return Err(AppError::Uncategorised(anyhow::anyhow!(
                    "cannot benchmark and write results at the same time"
                )));
            }

            let pager = if page_results {
                Some(get_pager()?)
            } else {
                None
            };

            let db_client = get_db_client().await?;

            let query = if query.as_str() == "-" {
                let mut buffer = String::new();
                std::io::stdin()
                    .read_to_string(&mut buffer)
                    .context("couldn't read query from stdin")?;
                buffer.trim().to_string()
            } else {
                query
            };

            if print_query {
                println!(
                    r#"---
{query}
---
"#
                );
            }

            if benchmark {
                benchmark_query(&db_client, &query, bench_num_runs, bench_num_warmup_runs).await?;
            } else {
                let results = execute_query(&db_client, &query).await?;
                let results = match results {
                    QueryResults::Empty => {
                        println!("No results");
                        return Ok(());
                    }
                    QueryResults::NonEmpty(res) => res,
                };

                if write_results {
                    let results_file_path = crate::service::write_results(
                        &results,
                        &results_directory,
                        &results_format,
                        Utc::now(),
                    )
                    .context("couldn't write results")?;
                    println!("Wrote results to {}", results_file_path.to_string_lossy());

                    if let Some(pager) = pager {
                        crate::service::page_results(&results_file_path, &pager)?;
                    }
                } else if let Some(pager) = pager {
                    let temp_results_directory = tempfile::tempdir()
                        .context("couldn't create temporary directory for paging results")?;
                    let results_file_path = crate::service::write_results(
                        &results,
                        &temp_results_directory,
                        &results_format,
                        Utc::now(),
                    )
                    .context("couldn't write results to temporary location")?;

                    crate::service::page_results(&results_file_path, &pager)?;
                } else {
                    let results_str = get_results(&results);
                    println!("{}", results_str);
                }
            }
        }
    }

    Ok(())
}
