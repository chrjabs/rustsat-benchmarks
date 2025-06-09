use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod cardenc;
mod openwbo;
mod pbenc;
mod randomweights;

#[derive(Subcommand)]
enum Benchmark {
    CardEncoding(#[arg(flatten)] cardenc::Opts),
    PbEncoding(#[arg(flatten)] pbenc::Opts),
    RandomWeights(#[arg(flatten)] randomweights::Opts),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    globals: Globals,
    #[command(subcommand)]
    benchmark: Benchmark,
}

#[derive(Args)]
struct Globals {
    /// Evaluate benchmark output rather than running the benchmark
    #[arg(short, long, global = true)]
    evaluate: bool,
    /// The random seed for generating input data
    #[arg(short, long, global = true, default_value_t = 42)]
    seed: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.globals.evaluate {
        match cli.benchmark {
            Benchmark::CardEncoding(opts) => cardenc::eval(cli.globals.seed, opts),
            Benchmark::PbEncoding(opts) => pbenc::eval(cli.globals.seed, opts),
            Benchmark::RandomWeights(_) => {
                anyhow::bail!("cannot evaluate random weights benchmark")
            }
        }
    } else {
        match cli.benchmark {
            Benchmark::CardEncoding(opts) => cardenc::exec(opts),
            Benchmark::PbEncoding(opts) => pbenc::exec(opts),
            Benchmark::RandomWeights(opts) => {
                for w in randomweights::get(cli.globals.seed, opts) {
                    print!("{w} ");
                }
                println!();
                Ok(())
            }
        }
    }
}
