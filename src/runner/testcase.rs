use std::path::Path;

use crate::{
    problemspec::{
        generator::Generator,
        spec::{ConstraintsError, MultipleTestcaseConfig, MultitaskProblemSpec, ProblemSpec},
    },
    runner::{executor, io::write_file},
    testspec::{
        random::Random,
        spec::{MultitaskTestSpec, SingletaskTestSpec},
    },
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

fn _generate<T>(
    base_folder: &Path,
    specs: &Vec<T>,
    multi_test_config: Option<&MultipleTestcaseConfig>,
    solution_command: Option<&str>,
    name_prefix: Option<&str>,
    subtask_constraints: Option<fn(&T) -> Result<(), ConstraintsError>>,
) -> Result<(), GenerateInputOutputError>
where
    T: ProblemSpec<T>,
{
    match multi_test_config {
        Some(multi_test_config) => {
            let mut inputs = String::new();
            let t = specs.len();
            inputs.push_str(format!("{}\n", t).as_str());
            for (i, spec) in specs.iter().enumerate() {
                println!("Testcase #{}...", i + 1);
                if let Some(subtask_constraints) = subtask_constraints {
                    subtask_constraints(&spec)?;
                }
                spec.constraints()?;

                let input = spec.input_format().generate();
                inputs.push_str(&input);
                if i != specs.len() - 1 {
                    inputs.push_str("\n");
                }
            }
            let constraints = multi_test_config.constraints;
            constraints(t)?;

            let file_name = if let Some(name_prefix) = name_prefix {
                format!("{}", name_prefix)
            } else {
                format!("{}", 1)
            };

            let input_path = base_folder.join(format!("{}.in", file_name));
            write_file(&inputs, &input_path)?;

            if let Some(solution_command) = &solution_command {
                let output = executor::execute(&solution_command, &inputs);

                check_output(&multi_test_config, &output)?;

                let output_path = base_folder.join(format!("{}.out", file_name));
                write_file(&output, &output_path)?;
            }
            Ok(())
        }
        None => {
            for (i, spec) in specs.iter().enumerate() {
                println!("Testcase #{}...", i + 1);
                if let Some(subtask_constraints) = subtask_constraints {
                    subtask_constraints(&spec)?;
                }
                spec.constraints()?;

                let file_name = if let Some(name_prefix) = name_prefix {
                    format!("{}_{}", name_prefix, i + 1)
                } else {
                    format!("{}", i + 1)
                };
                let input = spec.input_format().generate();
                let input_path = base_folder.join(format!("{}.in", file_name));
                write_file(&input, &input_path)?;

                if let Some(solution_command) = &solution_command {
                    let output = executor::execute(&solution_command, &input);

                    let output_path = base_folder.join(format!("{}.out", file_name));
                    write_file(&output, &output_path)?;
                }
            }
            Ok(())
        }
    }
}

pub fn generate<T>(
    base_folder: &Path,
    solution_command: Option<&str>,
    seed: u64,
) -> Result<(), GenerateInputOutputError>
where
    T: ProblemSpec<T> + SingletaskTestSpec<T>,
{
    let mut random = Random::new(seed);
    let specs = T::test_cases(&mut random);
    let multi_test_config = T::multiple_test_case_config();
    _generate(
        base_folder,
        &specs,
        multi_test_config.as_ref(),
        solution_command,
        None,
        None,
    )
}

pub fn generate_multitask<T>(
    base_folder: &Path,
    solution_command: Option<&str>,
    seed: u64,
) -> Result<(), GenerateInputOutputError>
where
    T: ProblemSpec<T> + MultitaskTestSpec<T> + MultitaskProblemSpec<T>,
{
    let mut random = Random::new(seed);
    let configs = [T::subtask_1(), T::subtask_2(), T::subtask_3()];
    let specs = [
        T::test_cases_subtask_1(&mut random),
        T::test_cases_subtask_2(&mut random),
        T::test_cases_subtask_3(&mut random),
    ];
    let multi_test_config = T::multiple_test_case_config();

    for (i, (spec, config)) in specs.iter().zip(configs.iter()).enumerate() {
        match (spec, config) {
            (Some(spec), Some(config)) => {
                let subtask_constraints = config.constraints;
                println!("Subtask #{}...", i + 1);

                _generate(
                    base_folder,
                    spec,
                    multi_test_config.as_ref(),
                    solution_command,
                    Some(i.to_string().as_str()),
                    Some(subtask_constraints),
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}
