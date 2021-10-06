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
}

fn check_constraints<T>(spec: &T, testcase_id: usize) -> Result<(), ConstraintsError>
where
    T: ProblemSpec,
{
    if let Err(errors) = spec.constraints() {
        println!("  {}: FAILED", testcase_id);
        println!("    Reasons:");
        //TODO: print the variable values
        for error in &errors.messages {
            println!("    * Expected: {}", error);
        }
        Err(errors)
    } else {
        println!("  {}: OK", testcase_id);
        Ok(())
    }
}

fn write_file(content: &str, path: &PathBuf) {
    let mut input_file = File::create(path).unwrap();
    input_file.write_all(content.as_bytes()).unwrap();
}

pub fn generate_inputs_outputs<T>(
    base_folder: String,
    solution_command: Option<String>,
    seed: usize,
) -> Result<(), GenerateInputOutputError>
where
    T: ProblemSpec + TestSpec<T>,
{
    let specs = T::test_cases();
    println!("Generating test cases...");
    println!("[ OFFICIAL TEST CASES ]");

    let base_folder = Path::new(&base_folder);
    if base_folder.exists() {
        remove_dir_all(&base_folder).expect("Failed to remove folder");
    }
    create_dir(&base_folder).expect("Failed to create output folder");

    match T::multiple_test_case_config() {
        Some(multi_test_config) => {
            let mut inputs = String::new();
            inputs.push_str(format!("{}\n", specs.len()).as_str());
            for (i, spec) in specs.iter().enumerate() {
                check_constraints(spec, i + 1)?;
                let input = spec.input_format().generate();
                inputs.push_str(&input);
                if i != specs.len() - 1 {
                    inputs.push_str("\n");
                }
            }
            //TODO check multiple_test_case_config

            let input_path = base_folder.join(format!("{}.in", 1));
            write_file(&inputs, &input_path);

            if let Some(solution_command) = &solution_command {
                let output = executor::execute(&solution_command, &inputs);

                let output_path = base_folder.join(format!("{}.out", 1));
                write_file(&output, &output_path);
            }
            Ok(())
        }
        None => {
            for (i, spec) in specs.iter().enumerate() {
                check_constraints(spec, i + 1)?;

                let input = spec.input_format().generate();
                let input_path = base_folder.join(format!("{}.in", i + 1));
                write_file(&input, &input_path);

                if let Some(solution_command) = &solution_command {
                    let output = executor::execute(&solution_command, &input);

                    let output_path = base_folder.join(format!("{}.out", i + 1));
                    write_file(&output, &output_path);
                }
            }
            Ok(())
        }
    }
}
