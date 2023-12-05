// use clap::Parser;

// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     path: std::path::PathBuf,
// }


// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args = Cli::parse();

//     // let result = std::fs::read_to_string("test.txt");
//     // let content = match result {
//     //     Ok(content) => { content }
//     //     // Err(error) => { panic!("Can't deal with {}, just exit here", error); } // exit the program
//     //     Err(error) => { return Err(error.into()); }
//     // };

//     // let content = std::fs::read_to_string("test.txt").unwrap(); // error handling shortcut

//     // let content = std::fs::read_to_string("test.txt")?; // error handling shortcut -- can use this generally for all error types since it converts them if we put Box<dyn std::error:Error> type as return type

//     // custom error


//     println!("File content: {}", content);

//     let content = std::fs::read_to_string(&args.path).expect("could not read file");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line)
//         }
//     }
//     Ok(())
// }

// implementing with a custom error -- this way it does not keep the error stack

// #[derive(Debug)]
// struct CustomError(String);

// fn main() -> Result<(), CustomError> {
//     let path = "test.txt";
//     let content = std::fs::read_to_string(path)
//         .map_err(|err| CustomError(format!("Error reading `{}`, {}", path, err)))?;

//     println!("File content: {}", content);

//     Ok(())
// }

// implementing with anyhow

// use anyhow::{Context, Result};

// fn main() -> Result<()> {
//     let path = "test.txt";
//     let content = std::fs::read_to_string(path)
//         .with_context(|| format!("Error reading `{}`", path))?;

//     println!("File content: {}", content);

//     Ok(())
// }


// full implementation
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

// fn main() -> Result<()> {
//     let args = Cli::parse();

//     let content = std::fs::read_to_string(&args.path)
//         .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

//     let pb = indicatif::ProgressBar::new(10000000);
//     for _i in 0..10000000 {
//         pb.inc(1);
//     }
//     pb.finish_with_message("done");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line)
//         }
//     }

//     Ok(())
// }


// using a logger

use log::{info, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented");
 
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}