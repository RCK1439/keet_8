use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args()
        .collect();

    if let Err(e) = keet_8::run(args) {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    
    ExitCode::SUCCESS
}
