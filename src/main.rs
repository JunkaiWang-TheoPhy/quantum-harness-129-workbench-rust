use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use ed_workbench_rs::coupled_cluster::{CcConfig, solve_cc};
use ed_workbench_rs::davidson::{DavidsonConfig, lowest_eigenpair};
use ed_workbench_rs::dense_fci::ground_state_energy;
use ed_workbench_rs::determinant::DeterminantBasis;
use ed_workbench_rs::direct_fci::DirectFciOperator;
use ed_workbench_rs::fcidump::Fcidump;
use ed_workbench_rs::mbpt::solve_mbpt;
use ed_workbench_rs::operator::LinearOperator;
use ed_workbench_rs::optimizer::BfgsConfig;
use ed_workbench_rs::problem::ElectronicProblem;
use ed_workbench_rs::reference::{Reference, sha256_hex};
use ed_workbench_rs::truncated_ci::solve_ci;
use ed_workbench_rs::unitary_cc::UnitaryCcModel;

#[derive(Debug, Parser)]
#[command(name = "ed-workbench-rs")]
#[command(about = "Rust Level 0 ED/FCI workbench for Quantum Harness #129")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Inspect {
        fcidump: PathBuf,
    },
    DenseFci {
        fcidump: PathBuf,
    },
    Davidson {
        fcidump: PathBuf,
        #[arg(long, default_value_t = 1e-8)]
        residual_tolerance: f64,
        #[arg(long, default_value_t = 100)]
        max_iterations: usize,
        #[arg(long, default_value_t = 24)]
        max_subspace: usize,
    },
    Cc {
        fcidump: PathBuf,
        reference: PathBuf,
        #[arg(long)]
        rank: usize,
        #[arg(long, default_value_t = 1e-8)]
        residual_tolerance: f64,
        #[arg(long, default_value_t = 100)]
        max_iterations: usize,
    },
    Ci {
        fcidump: PathBuf,
        #[arg(long)]
        rank: usize,
        #[arg(long, default_value_t = 1e-9)]
        residual_tolerance: f64,
    },
    Mbpt {
        fcidump: PathBuf,
        reference: PathBuf,
        #[arg(long)]
        order: usize,
    },
    Ucc {
        fcidump: PathBuf,
        #[arg(long)]
        rank: usize,
        #[arg(long, default_value_t = 1e-7)]
        gradient_tolerance: f64,
        #[arg(long, default_value_t = 100)]
        max_iterations: usize,
    },
    Verify {
        fcidump: PathBuf,
        reference: PathBuf,
        #[arg(long, default_value_t = 1e-10)]
        tolerance: f64,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Inspect { fcidump } => {
            let bytes = fs::read(&fcidump)?;
            let dump = Fcidump::parse(std::str::from_utf8(&bytes)?)?;
            let basis = DeterminantBasis::new(dump.norb, dump.nelec, dump.ms2)?;
            println!("NORB: {}", dump.norb);
            println!("NELEC: {}", dump.nelec);
            println!("MS2: {}", dump.ms2);
            println!("ECORE: {:.16}", dump.ecore);
            println!("one-electron records: {}", dump.one_body_record_count());
            println!("two-electron records: {}", dump.two_body_record_count());
            println!("alpha strings: {}", basis.alpha_strings.len());
            println!("beta strings: {}", basis.beta_strings.len());
            println!("determinants: {}", basis.len());
        }
        Command::DenseFci { fcidump } => {
            let bytes = fs::read(&fcidump)?;
            let dump = Fcidump::parse(std::str::from_utf8(&bytes)?)?;
            println!("{:.15}", ground_state_energy(&dump)?);
        }
        Command::Davidson {
            fcidump,
            residual_tolerance,
            max_iterations,
            max_subspace,
        } => {
            let bytes = fs::read(&fcidump)?;
            let dump = Fcidump::parse(std::str::from_utf8(&bytes)?)?;
            let problem = ElectronicProblem::from_fcidump(&dump)?;
            let operator = DirectFciOperator::new(problem)?;
            let mut initial = vec![0.0; operator.dimension()];
            let reference_index = operator
                .diagonal()
                .iter()
                .enumerate()
                .min_by(|(_, left), (_, right)| left.total_cmp(right))
                .map(|(index, _)| index)
                .ok_or("empty determinant basis")?;
            initial[reference_index] = 1.0;
            let result = lowest_eigenpair(
                &operator,
                &initial,
                &DavidsonConfig {
                    residual_tolerance,
                    energy_tolerance: residual_tolerance * 0.01,
                    max_iterations,
                    max_subspace,
                },
            )?;
            println!("energy: {:.15}", result.energy);
            println!("residual norm: {:.3e}", result.residual_norm);
            println!("iterations: {}", result.iterations);
            println!("converged: {}", result.converged);
            if !result.converged {
                return Err("Davidson did not converge".into());
            }
        }
        Command::Cc {
            fcidump,
            reference,
            rank,
            residual_tolerance,
            max_iterations,
        } => {
            let bytes = fs::read(&fcidump)?;
            let dump = Fcidump::parse(std::str::from_utf8(&bytes)?)?;
            let reference = Reference::load(&reference)?;
            let operator = DirectFciOperator::new(ElectronicProblem::from_fcidump(&dump)?)?;
            let result = solve_cc(
                &operator,
                rank,
                &reference.active_orbital_energies,
                &CcConfig {
                    residual_tolerance,
                    energy_tolerance: residual_tolerance * 0.01,
                    max_iterations,
                    ..Default::default()
                },
            )?;
            for item in &result.iterations {
                println!(
                    "iter {:3}  E={:.15}  dE={:.3e}  |R|={:.3e}",
                    item.iteration, item.energy, item.energy_change, item.residual_norm
                );
            }
            println!("CC({rank}) energy: {:.15}", result.energy);
            println!("residual norm: {:.3e}", result.residual_norm);
            println!("converged: {}", result.converged);
            if !result.converged {
                return Err(format!("CC({rank}) did not converge").into());
            }
        }
        Command::Ci {
            fcidump,
            rank,
            residual_tolerance,
        } => {
            let bytes = fs::read(&fcidump)?;
            let dump = Fcidump::parse(std::str::from_utf8(&bytes)?)?;
            let operator = DirectFciOperator::new(ElectronicProblem::from_fcidump(&dump)?)?;
            let result = solve_ci(
                &operator,
                rank,
                &DavidsonConfig {
                    residual_tolerance,
                    energy_tolerance: residual_tolerance * 0.01,
                    max_iterations: 100,
                    max_subspace: 24,
                },
            )?;
            println!("CI({rank}) energy: {:.15}", result.energy);
            println!("residual norm: {:.3e}", result.residual_norm);
            println!("iterations: {}", result.iterations);
            println!("converged: {}", result.converged);
            if !result.converged {
                return Err(format!("CI({rank}) did not converge").into());
            }
        }
        Command::Mbpt {
            fcidump,
            reference,
            order,
        } => {
            let bytes = fs::read(&fcidump)?;
            let dump = Fcidump::parse(std::str::from_utf8(&bytes)?)?;
            let reference = Reference::load(&reference)?;
            let operator = DirectFciOperator::new(ElectronicProblem::from_fcidump(&dump)?)?;
            let result = solve_mbpt(&operator, &reference.active_orbital_energies, order)?;
            println!("reference energy: {:.15}", result.reference_energy);
            for index in 0..order {
                println!(
                    "order {:2}: correction={:.15e}  total={:.15}",
                    index + 1,
                    result.corrections[index],
                    result.partial_sums[index]
                );
            }
        }
        Command::Ucc {
            fcidump,
            rank,
            gradient_tolerance,
            max_iterations,
        } => {
            let bytes = fs::read(&fcidump)?;
            let dump = Fcidump::parse(std::str::from_utf8(&bytes)?)?;
            let operator = DirectFciOperator::new(ElectronicProblem::from_fcidump(&dump)?)?;
            let model = UnitaryCcModel::new(&operator, rank)?;
            let result = model.optimize(&BfgsConfig {
                gradient_tolerance,
                max_iterations,
                finite_difference_step: 1e-5,
            });
            println!("UCC({rank}) energy: {:.15}", result.value);
            println!("gradient norm: {:.3e}", result.gradient_norm);
            println!("iterations: {}", result.iterations);
            println!("parameters: {}", result.parameters.len());
            println!("converged: {}", result.converged);
            if !result.converged {
                return Err(format!("UCC({rank}) did not converge").into());
            }
        }
        Command::Verify {
            fcidump,
            reference,
            tolerance,
        } => {
            if !tolerance.is_finite() || tolerance <= 0.0 {
                return Err("tolerance must be finite and positive".into());
            }
            let bytes = fs::read(&fcidump)?;
            let actual_checksum = sha256_hex(&bytes);
            let reference = Reference::load(&reference)?;
            if actual_checksum != reference.fcidump_sha256 {
                return Err(format!(
                    "FCIDUMP checksum mismatch: expected {}, got {}",
                    reference.fcidump_sha256, actual_checksum
                )
                .into());
            }
            let dump = Fcidump::parse(std::str::from_utf8(&bytes)?)?;
            let rust_energy = ground_state_energy(&dump)?;
            let error = (rust_energy - reference.fci_energy).abs();
            println!("system: {}", reference.system);
            println!("Rust dense FCI: {:.15}", rust_energy);
            println!("PySCF FCI:      {:.15}", reference.fci_energy);
            println!("absolute error: {:.3e}", error);
            println!("tolerance:      {:.3e}", tolerance);
            if error > tolerance {
                return Err(format!("verification failed: {error:e} > {tolerance:e}").into());
            }
            println!("verification: PASS");
        }
    }
    Ok(())
}
