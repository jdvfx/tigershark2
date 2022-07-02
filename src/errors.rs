#[derive(Debug)]
pub struct ErrOutput {
    pub status: u32,
    pub output: String,
}

// exit nicely or panic

pub fn exit(err_output: ErrOutput) {
    //
    if err_output.status == 0 {
        println!("exit nicely");
        // use return ?
    } else {
        println!("Panic !!!");
    }
}
