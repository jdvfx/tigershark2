use std::process;

// exit the program with:
// - a message
// - an exitcode (101:panic, 0:normal exit)

pub enum Status {
    Err,
    Ok,
}

pub struct CliOutput {
    pub status: Status,
    pub output: String,
}

// let panic = CliOutput {
//     status: Status::Err,
//     output: "panic".to_owned(),
// };
// let exit = CliOutput {
//     status: Status::Ok,
//     output: "all good".to_owned(),
// };
// exit_or_panic(panic);

pub fn exit_or_panic(cli_output: CliOutput) {
    //
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
