use std::path::Path;

use crate::{
    problemspec::{
        generator::Generator,
        spec::{ConstraintsError, MultitaskProblemSpec, ProblemSpec},
    },
    runner::{executor, io::write_file},
    testspec::spec::{MultitaskTestSpec, SingletaskTestSpec},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateSampleTestCaseError {
    #[error("Constraints error")]
    ConstraintsError(#[from] ConstraintsError),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Sample output mismatch")]
    SampleOutputMismatch(String, String),
}
//TODO: refactor, extract common logics
//TODO: unit test
fn _generate<T>(
    base_folder: &Path,
    specs: Vec<T>,
    solution_command: Option<&str>,
) -> Result<(), GenerateSampleTestCaseError>
where
    T: ProblemSpec<T>,
{
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

            if let Some(solution_command) = &solution_command {
                let observed_output = executor::execute(&solution_command, &inputs);
                //TODO: zip only takes the lower len, check the remaining
                for (expected_output, output) in outputs
                    .split("\n")
                    .into_iter()
                    .zip(observed_output.split("\n").into_iter())
                {
                    if expected_output != output {
                        return Err(GenerateSampleTestCaseError::SampleOutputMismatch(
                            expected_output.to_string(),
                            output.to_string(),
                        ));
                    }
                }
            }

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

                if let Some(solution_command) = &solution_command {
                    let observed_output = executor::execute(&solution_command, &input);
                    //TODO: zip only takes the lower len, check the remaining
                    for (expected_output, output) in output
                        .split("\n")
                        .into_iter()
                        .zip(observed_output.split("\n").into_iter())
                    {
                        if expected_output != output {
                            return Err(GenerateSampleTestCaseError::SampleOutputMismatch(
                                expected_output.to_string(),
                                output.to_string(),
                            ));
                        }
                    }
                }

                write_file(&output, &output_path)?;
            }
            Ok(())
        }
    }
}

pub fn generate<T>(
    base_folder: &Path,
    solution_command: Option<&str>,
) -> Result<(), GenerateSampleTestCaseError>
where
    T: ProblemSpec<T> + SingletaskTestSpec<T>,
{
    let specs = T::sample_test_cases();
    _generate(base_folder, specs, solution_command)
}

pub fn generate_multitask<T>(
    base_folder: &Path,
    solution_command: Option<&str>,
) -> Result<(), GenerateSampleTestCaseError>
where
    T: ProblemSpec<T> + MultitaskProblemSpec<T> + MultitaskTestSpec<T>,
{
    let specs = T::sample_test_cases();
    _generate(base_folder, specs, solution_command)
}
