use std::io::stdout;
use std::{path::PathBuf, io::Write};
use std::fs::File;

use clap::Parser;
use anyhow::Result;
use crypto_hashes::digest::Digest;
use happylog::clap::LogOpts;
use friendly::{bytes, duration};
use log::*;

mod bencher;
use bencher::HashBencher;

const DEFAULT_MEM_SIZE: usize = 256 * 1024 * 1024;
const DEFAULT_ITERS: usize = 20;
const MIB_MULT: f64 = 1024.0 * 1024.0;

/// Benchmark Rust cryptographic hash functions.
#[derive(Debug, Parser)]
#[command(name="hashbench")]
struct HashBench {
  #[command(flatten)]
  logging: LogOpts,

  /// Specify an output file.
  #[arg(long="output", short='o')]
  output: Option<PathBuf>,

  /// Specify the size of memory buffer to (repeatedly) hash.
  #[arg(long="mem-size")]
  mem_size: Option<usize>,

  /// Number of iterations to hash.
  #[arg(long="iters", short='n')]
  iters: Option<usize>,
}

fn bench_and_print<W: Write, H: Digest, F: Fn() -> H>(out: &mut W, bench: &HashBencher, name: &str, new: F) -> Result<f64> {
  info!("benchmarking {}", name);
  let hash = new();
  let dur = bench.measure_hash(hash);
  let rate = bench.duration_to_rate(&dur);
  info!("{}: hashed {} in {} ({})", name, bytes(bench.total_bytes()), duration(dur), bytes(rate).suffix("B/s"));
  writeln!(out, "{}\t{}", name, rate / MIB_MULT)?;
  Ok(rate)
}

fn main() -> Result<()> {
  let args = HashBench::parse();
  args.logging.init()?;

  let size = args.mem_size.unwrap_or(DEFAULT_MEM_SIZE);
  let iters = args.iters.unwrap_or(DEFAULT_ITERS);

  let bench = HashBencher::new(size, iters);
  info!("initializing benchmark of {} in {} iters", bytes(bench.total_bytes()), iters);

  let mut out: Box<dyn Write> = if let Some(file) = &args.output {
    Box::new(File::create(file)?)
  } else {
    Box::new(stdout())
  };

  bench_and_print(&mut out, &bench, "MD5", crypto_hashes::md5::Md5::new)?;
  bench_and_print(&mut out, &bench, "SHA-1", crypto_hashes::sha1::Sha1::new)?;
  bench_and_print(&mut out, &bench, "SHA-256", crypto_hashes::sha2::Sha256::new)?;
  bench_and_print(&mut out, &bench, "SHA-512", crypto_hashes::sha2::Sha512::new)?;
  bench_and_print(&mut out, &bench, "SHA3-256", crypto_hashes::sha3::Sha3_256::new)?;
  bench_and_print(&mut out, &bench, "SHA3-512", crypto_hashes::sha3::Sha3_512::new)?;

  Ok(())
}
