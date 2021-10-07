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
        if !output.starts_with(output_prefix) {
            return Err(GenerateInputOutputError::OutputFormatError(format!(
                "Output prefix is not correct: {}",
                output_prefix
            )));
        }
    }
    Ok(())
}

pub fn generate_inputs_outputs<T>(
    base_folder: String,
    solution_command: Option<String>,
    seed: u64,
) -> Result<(), GenerateInputOutputError>
where
    T: ProblemSpec + TestSpec<T>,
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
