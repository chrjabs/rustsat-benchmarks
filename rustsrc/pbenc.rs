use std::io;

use anyhow::Result;
use rand::{Rng, SeedableRng};
use rustsat::{
    encodings::pb::{BinaryAdder, BoundUpper, DynamicPolyWatchdog, GeneralizedTotalizer},
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
    bound: usize,
    /// The weights of the input literals
    weights: Vec<usize>,
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum Encoding {
    /// The default generalized totalizer encoding implementation in RustSAT
    Gte,
    /// The binary adder encoding
    BinaryAdder,
    /// The dynamic polynomial watchdog encoding
    Dpw,
    /// The Open-WBO GTE encoding
    OpenWboGte,
    /// The Open-WBO binary adder encoding
    OpenWboAdder,
}

pub fn exec(opts: Opts) -> Result<()> {
    let Some(encoding) = opts.encoding else {
        anyhow::bail!("an encoding type must be specified for running the benchmark");
    };
    match encoding {
        Encoding::Gte => exec_generic::<GeneralizedTotalizer>(&opts.weights, opts.bound),
        Encoding::BinaryAdder => exec_generic::<BinaryAdder>(&opts.weights, opts.bound),
        Encoding::Dpw => exec_generic::<DynamicPolyWatchdog>(&opts.weights, opts.bound),
        Encoding::OpenWboGte => crate::openwbo::gte(&opts.weights, opts.bound),
        Encoding::OpenWboAdder => crate::openwbo::adder(&opts.weights, opts.bound),
    }
}

fn exec_generic<Enc: BoundUpper + FromIterator<(Lit, usize)>>(
    weights: &[usize],
    bound: usize,
) -> Result<()> {
    let n_inputs = u32::try_from(weights.len())?;
    let mut enc =
        Enc::from_iter((0..n_inputs).map(|idx| (Lit::new(idx, false), weights[idx as usize])));
    let mut vm = BasicVarManager::from_next_free(Var::new(n_inputs));
    let mut cnf = Cnf::new();
    enc.encode_ub(bound..=bound, &mut cnf, &mut vm)?;
    for a in enc.enforce_ub(bound)? {
        cnf.add_unit(a);
    }
    cnf.write_dimacs(&mut std::io::stdout(), vm.n_used())?;
    Ok(())
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
    let n_inputs = u32::try_from(opts.weights.len())?;

    // Test 10 random assignments
    for _ in 0..5 {
        let mut assumps = Vec::with_capacity(opts.weights.len());
        let mut sum = 0;
        for idx in 0..n_inputs {
            if rng.random_bool(0.5) {
                sum += opts.weights[idx as usize];
                assumps.push(Lit::new(idx, false));
                print!("1");
            } else {
                assumps.push(Lit::new(idx, true));
                print!("0");
            }
        }
        println!();
        let res = solver.solve_assumps(&assumps)?;
        println!("# sum={sum}; res={res:?}");
        if sum <= opts.bound && res == SolverResult::Sat
            || sum > opts.bound && res == SolverResult::Unsat
        {
            println!("PASSED")
        } else {
            println!("FAILED")
        }
    }

    Ok(())
}
