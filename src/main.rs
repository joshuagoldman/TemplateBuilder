use dotenv::dotenv;
use template_build::*;

fn main() {
    dotenv().ok();
    run_console().ignore();
}

