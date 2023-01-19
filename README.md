# Rust Hash Table Benchmarks

This repo benchmarks the throughput of Rust cryptographic hash implementations.
The results are in [BenchResults.ipynb](BenchResults.ipynb).

There is some refinement left to do in order to make it more rigorous (e.g.
logging the exact Rust and crate versions used for each run), but it provides a
starting point to understand the applied performance.

## Contributing Benchmarks

The benchmarks can be run with:

    $ cargo run --release -- -o "results/MySystem.tsv"

For contributing results to the repository, please make "MySystem" a short string that
is adequate for labeling charts and will identify the system sufficiently for readers
to understand the results.  In the pull request description, explain what the system is.

## Updating Results

The `environment.yml` file defines a Conda environment with the requirements for running
the notebook.

    $ conda env create -n hashbench -f environment.yml
    $ conda activate hashbench
    $ jupytext -s --execute BenchResults.md
