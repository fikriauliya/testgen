use thiserror::Error;

mod executor;

use std::fs::{create_dir, remove_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::problemspec::generator::Generator;

use super::problemspec::spec::*;
use super::testspec::spec::*;

#[derive(Debug, Error)]
pub enum GenerateInputOutputError {
    #[error("Constraints error")]
    ConstraintsError(#[from] ConstraintsError),
    #[error("Invalid format")]
    OutputFormatError(String),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum GenerateSampleTestCaseError {
    #[error("Constraints error")]
    ConstraintsError(#[from] ConstraintsError),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
}

fn write_file(content: &str, path: &PathBuf) -> Result<(), std::io::Error> {
    let mut input_file = File::create(path)?;
    input_file.write_all(content.as_bytes())?;
    Ok(())
}

fn prepare_folder<'a>(base_folder: &'a str) -> Result<&'a Path, std::io::Error> {
    let base_folder = Path::new(base_folder);
    if base_folder.exists() {
        remove_dir_all(&base_folder)?;
    }
    create_dir(&base_folder)?;
    Ok(base_folder)
}

fn check_output(
    multi_test_config: &MultipleTestcaseConfig,
    output: &str,
) -> Result<(), GenerateInputOutputError> {
    if let Some(output_prefix) = &multi_test_config.output_prefix {
        let lines = output.split("\n");
        for (i, line) in lines.enumerate() {
            let output_prefix = output_prefix.replace("{}", &(i + 1).to_string());
            if !line.is_empty() && !line.starts_with(&output_prefix) {
                return Err(GenerateInputOutputError::OutputFormatError(format!(
                    "Output prefix is not correct: expected {}, found: {}",
                    output_prefix, line
                )));
            }
        }
    }
    Ok(())
}

pub fn generate_sample_test_cases<T>(base_folder: &str) -> Result<(), GenerateSampleTestCaseError>
where
    T: ProblemSpec<T> + SingletaskTestSpec<T>,
{
    let specs = T::sample_test_cases();
    let base_folder = prepare_folder(&base_folder)?;
    match T::multiple_test_case_config() {
        Some(multi_test_config) => {
            let mut inputs = String::new();
            let mut outputs = String::new();

            let t = specs.len();
            inputs.push_str(format!("{}\n", t).as_str());
            for (i, spec) in specs.iter().enumerate() {
                println!("Sample case #{}...", i + 1);
                spec.constraints()?;

                let input = spec.input_format().generate();
                inputs.push_str(&input);
                if i != specs.len() - 1 {
                    inputs.push_str("\n");
                }

                if let Some(output_prefix) = &multi_test_config.output_prefix {
                    outputs.push_str(&output_prefix.replace("{}", &(i + 1).to_string()));
                };
                let output = spec.output_format().generate();
                outputs.push_str(&output);
                if i != specs.len() - 1 {
                    outputs.push_str("\n");
                }
            }
            let constraints = multi_test_config.constraints;
            constraints(t)?;

            let input_path = base_folder.join(format!("sample_{}.in", 1));
            write_file(&inputs, &input_path)?;

            let output_path = base_folder.join(format!("sample_{}.out", 1));
            write_file(&outputs, &output_path)?;
            Ok(())
        }
        None => {
            for (i, spec) in specs.iter().enumerate() {
                println!("Sample case #{}...", i + 1);
                spec.constraints()?;

                let input = spec.input_format().generate();
                let input_path = base_folder.join(format!("sample_{}.in", i + 1));
                write_file(&input, &input_path)?;

                let output = spec.output_format().generate();
                let output_path = base_folder.join(format!("sample_{}.out", i + 1));
                write_file(&output, &output_path)?;
            }
            Ok(())
        }
    }
}

pub fn generate_inputs_outputs<T>(
    base_folder: &str,
    solution_command: Option<String>,
    seed: u64,
) -> Result<(), GenerateInputOutputError>
where
    T: ProblemSpec<T> + SingletaskTestSpec<T>,
{
    let mut random = Random::new(seed);
    let specs = T::test_cases(&mut random);
    let base_folder = prepare_folder(&base_folder)?;

    match T::multiple_test_case_config() {
        Some(multi_test_config) => {
            let mut inputs = String::new();
            let t = specs.len();
            inputs.push_str(format!("{}\n", t).as_str());
            for (i, spec) in specs.iter().enumerate() {
                println!("Testcase #{}...", i + 1);
                spec.constraints()?;

                let input = spec.input_format().generate();
                inputs.push_str(&input);
                if i != specs.len() - 1 {
                    inputs.push_str("\n");
                }
            }
            let constraints = multi_test_config.constraints;
            constraints(t)?;

            let input_path = base_folder.join(format!("{}.in", 1));
            write_file(&inputs, &input_path)?;

            if let Some(solution_command) = &solution_command {
                let output = executor::execute(&solution_command, &inputs);

                check_output(&multi_test_config, &output)?;

                let output_path = base_folder.join(format!("{}.out", 1));
                write_file(&output, &output_path)?;
            }
            Ok(())
        }
        None => {
            for (i, spec) in specs.iter().enumerate() {
                println!("Testcase #{}...", i + 1);
                spec.constraints()?;

                let input = spec.input_format().generate();
                let input_path = base_folder.join(format!("{}.in", i + 1));
                write_file(&input, &input_path)?;

                if let Some(solution_command) = &solution_command {
                    let output = executor::execute(&solution_command, &input);

                    let output_path = base_folder.join(format!("{}.out", i + 1));
                    write_file(&output, &output_path)?;
                }
            }
            Ok(())
        }
    }
}
