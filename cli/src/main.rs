use reginae_solver::{Board, Solution, Solver};
use std::{
    env,
    io::{self, Read},
};
use tracing_subscriber::filter::EnvFilter;

fn main() -> io::Result<()> {
    let mut libraries = Vec::new();
    let mut solver = Solver::default();

    // load dynamic libraries
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if &arg != "-l" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unknown argument {arg}"),
            ));
        }

        let value = args.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "a value must be provided to a library argument".to_string(),
            )
        })?;

        let mut parts = value.split(':');

        let path = parts.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "the path of the library cannot be empty".to_string(),
            )
        })?;

        let function = parts.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "the function name cannot be empty".to_string(),
            )
        })?;

        let weight = parts
            .next()
            .map(|p| p.parse::<f64>())
            .transpose()
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("failed parsing the weight: {e}"),
                )
            })?
            .unwrap_or(0.0);

        let lib = unsafe {
            libloading::Library::new(path).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("error while reading the library: {e}"),
                )
            })?
        };

        let function: libloading::Symbol<fn(&Board, usize) -> f64> = unsafe {
            lib.get(function.as_bytes()).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("error while finding the function symbol name: {e}"),
                )
            })?
        };

        solver.with_evaluator(*function, weight);

        // avoid dropping the library so the function pointer will be valid until execution
        libraries.push(lib);
    }

    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;
    input.retain(|c| c.is_ascii_digit() || c == ',');
    let mut inputs = input.split(',');

    let width = inputs
        .next()
        .expect("no width provided")
        .parse::<usize>()
        .expect("invalid width provided");
    let queens = inputs
        .map(|i| i.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let filter = match env::var_os("RUST_LOG") {
        Some(_) => EnvFilter::try_from_default_env().expect("Invalid `RUST_LOG` provided"),
        None => EnvFilter::new("info"),
    };

    tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(io::stderr)
        .with_env_filter(filter)
        .with_ansi(true)
        .with_level(true)
        .with_line_number(true)
        .init();

    let mut board = Board::new(width);
    queens.into_iter().for_each(|q| {
        board.toggle(q);
    });

    let Solution {
        board,
        success,
        jumps,
    } = solver.solve(board);

    println!(
        "{success} with {jumps} jumps: {:?}",
        board.sorted_queens().collect::<Vec<_>>().as_slice()
    );

    Ok(())
}
