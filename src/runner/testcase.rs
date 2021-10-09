use std::path::Path;

use crate::{
    problemspec::{
        generator::Generator,
        spec::{ConstraintsError, MultipleTestcaseConfig, ProblemSpec},
    },
    runner::{executor, io::write_file},
    testspec::spec::{Random, SingletaskTestSpec},
};
use thiserror::Error;

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
#[derive(Debug, Error)]
pub enum GenerateInputOutputError {
    #[error("Constraints error")]
    ConstraintsError(#[from] ConstraintsError),
    #[error("Invalid format")]
    OutputFormatError(String),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
}

pub fn generate<T>(
    base_folder: &Path,
    solution_command: Option<String>,
    seed: u64,
) -> Result<(), GenerateInputOutputError>
where
    T: ProblemSpec<T> + SingletaskTestSpec<T>,
{
    let mut random = Random::new(seed);
    let specs = T::test_cases(&mut random);

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
