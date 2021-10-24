use super::problemspec::spec::*;
use super::runner::*;
use super::testspec::spec::*;
use clap::{AppSettings, Parser};

#[derive(Parser)]
#[clap(
    version = "0.1",
    author = "Pahlevi Fikri Auliya <pahlevi.fikri.auliya@gmail.com>"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Generate(GenerateCommand),
    Grade(GradeCommand),
}

#[derive(Parser)]
struct GenerateCommand {
    #[clap(short, long, default_value = "tc")]
    output: String,

    #[clap(short, long)]
    solution: Option<String>,

    #[clap(long, default_value = "0")]
    seed: u64,
}

#[derive(Parser)]
struct GradeCommand {
    #[clap(short, long, default_value = "tc")]
    output: String,

    #[clap(short, long, default_value = "./solution")]
    solution: String,
}

pub fn run<T>()
where
    T: SingletaskTestSpec<T> + ProblemSpec<T>,
{
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Generate(g) => {
            match run_singletask::<T>(&g.output, g.solution.as_ref().map(String::as_str), g.seed) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        SubCommand::Grade(_) => {
            todo!()
        }
    }
}
pub fn run_multi<T>()
where
    T: MultitaskTestSpec<T> + ProblemSpec<T> + MultitaskProblemSpec<T>,
{
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Generate(g) => {
            match run_multitask::<T>(&g.output, g.solution.as_ref().map(String::as_str), g.seed) {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        SubCommand::Grade(_) => {
            todo!()
        }
    }
}

// pub fn run_multi<T>()
// where
//     T: MultitaskProblemSpec<T> + MultitaskTestSpec<T>,
// {
//     let opts: Opts = Opts::parse();

//     match opts.subcmd {
//         SubCommand::Generate(g) => {
//             println!("[ SAMPLE TEST CASES ]");
//             match generate_sample_test_cases::<T>(&"samples") {
//                 Ok(_) => {}
//                 Err(err) => {
//                     println!("  ❌");
//                     match err {
//                         GenerateSampleTestCaseError::ConstraintsError(errors) => {
//                             for error in &errors.messages {
//                                 println!("    * Expected: {}", error);
//                             }
//                         }
//                         GenerateSampleTestCaseError::IOError(error) => {
//                             println!("    * IO error: {}", error);
//                         }
//                     }
//                 }
//             }

//             println!();
//             println!("[ OFFICIAL TEST CASES ]");
//             match generate_inputs_outputs::<T>(&g.output, g.solution, g.seed) {
//                 Ok(_) => {}
//                 Err(err) => {
//                     println!("  ❌");
//                     match err {
//                         GenerateInputOutputError::ConstraintsError(errors) => {
//                             for error in &errors.messages {
//                                 println!("    * Expected: {}", error);
//                             }
//                         }
//                         GenerateInputOutputError::OutputFormatError(error) => {
//                             println!("    * Formatting error: {}", error);
//                         }
//                         GenerateInputOutputError::IOError(error) => {
//                             println!("    * IO error: {}", error);
//                         }
//                     }
//                 }
//             }
//         }
//         SubCommand::Grade(_) => {
//             todo!()
//         }
//     }
// }
