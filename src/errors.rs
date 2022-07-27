use std::process;

// exit the program with:
// - a message
// - an exitcode (101:panic, 0:normal exit)

#[derive(Debug)]
pub enum Status {
    Err,
    Ok,
}
#[derive(Debug)]
pub struct CliOutput {
    pub status: Status,
    pub output: String,
}

impl CliOutput {
    pub fn new(status: &str, output: &str) -> Self {
        let status = match status {
            "err" => Status::Err,
            _ => Status::Ok,
        };
        CliOutput {
            status,
            output: output.to_owned(),
        }
    }
}

pub fn exit_or_panic(cli_output: CliOutput) {
    match cli_output.status {
        Status::Ok => {
            print!("{}", cli_output.output);
            process::exit(0);
            // exitcode=0
        }
        Status::Err => {
            print!("{}", cli_output.output);
            panic!();
            // exitcode=101
        }
    }
}
