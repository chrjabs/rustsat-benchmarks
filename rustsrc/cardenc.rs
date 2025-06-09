use std::io;

use anyhow::Result;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rustsat::{
    encodings::card::{BoundUpper, Totalizer},
    instances::{BasicVarManager, Cnf, ManageVars, SatInstance},
    solvers::{Solve, SolveIncremental, SolverResult},
    types::{Lit, Var},
};
use rustsat_cadical::CaDiCaL;

#[derive(clap::Args)]
pub struct Opts {
    /// The cardinality encoding to use
    #[arg(short = 'E', long)]
    encoding: Option<Encoding>,
    /// The cardinality bound to encode on the input literals
    #[arg(short, long, default_value_t = 150)]
    bound: u32,
    /// The number of inputs to generate the cardinality encoding for
    #[arg(short, long, default_value_t = 300)]
    n_inputs: u32,
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum Encoding {
    /// The default totalizer encoding implementation in RustSAT
    Totalizer,
    /// The Open-WBO totalizer encoding
    OpenWboTotalizer,
}

pub fn exec(opts: Opts) -> Result<()> {
    let Some(encoding) = opts.encoding else {
        anyhow::bail!("an encoding type must be specified for running the benchmark");
    };
    match encoding {
        Encoding::Totalizer => exec_generic::<Totalizer>(opts.n_inputs, opts.bound as usize),
        Encoding::OpenWboTotalizer => crate::openwbo::totalizer(opts.n_inputs, opts.bound as usize),
    }
}

fn exec_generic<Enc: BoundUpper + FromIterator<Lit>>(n_inputs: u32, bound: usize) -> Result<()> {
    let mut enc = Enc::from_iter((0..n_inputs).map(|idx| Lit::new(idx, false)));
    let mut vm = BasicVarManager::from_next_free(Var::new(n_inputs));
    let mut cnf = Cnf::new();
    enc.encode_ub(bound..=bound, &mut cnf, &mut vm)?;
    for a in enc.enforce_ub(bound)? {
        cnf.add_unit(a);
    }
    cnf.write_dimacs(&mut std::io::stdout(), vm.n_used())?;
    Ok(())
}

fn test_n_true(
    n_true: u32,
    truth: &mut [bool],
    solver: &mut CaDiCaL,
    rng: &mut rand_chacha::ChaCha12Rng,
) -> Result<SolverResult> {
    for t in truth.iter_mut().take(n_true as usize) {
        *t = true;
    }
    truth.shuffle(rng);
    let mut assumps = Vec::with_capacity(truth.len());
    for (i, t) in truth.iter().copied().enumerate() {
        assumps.push(Lit::new(i as u32, !t));
        print!("{}", Into::<u8>::into(t));
    }
    println!();
    solver.solve_assumps(&assumps)
}

/// Reads the encoding produced by the benchmark from stdin, counts the number of clauses and
/// variables and verifies the encoding correctness
pub fn eval(seed: u64, opts: Opts) -> Result<()> {
    if opts.encoding.is_some() {
        anyhow::bail!("do not specify an encoding type when evaluating output");
    }
    let (cnf, vm) =
        SatInstance::<BasicVarManager>::from_dimacs(&mut io::BufReader::new(io::stdin()))?
            .into_cnf();
    let n_vars = vm.n_used();
    let n_clauses = cnf.len();
    println!("VARS={n_vars}");
    println!("CLAUSES={n_clauses}");
    println!("# testing encoding");

    let mut solver = CaDiCaL::default();
    solver.add_cnf(cnf)?;

    let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(seed);

    let mut truth = Vec::with_capacity(opts.n_inputs as usize);

    // Randomly test 5 SAT and 5 UNSAT assignments to the inputs
    for _ in 0..5 {
        let val = rng.random_range(0..=opts.bound);
        truth.clear();
        truth.resize(opts.n_inputs as usize, false);
        if test_n_true(val, &mut truth, &mut solver, &mut rng)? == SolverResult::Sat {
            println!("PASSED");
        } else {
            println!("FAILED");
        }
    }
    for _ in 0..5 {
        let val = rng.random_range(opts.bound + 1..=opts.n_inputs);
        truth.clear();
        truth.resize(opts.n_inputs as usize, false);
        if test_n_true(val, &mut truth, &mut solver, &mut rng)? == SolverResult::Unsat {
            println!("PASSED");
        } else {
            println!("FAILED");
        }
    }

    Ok(())
}
