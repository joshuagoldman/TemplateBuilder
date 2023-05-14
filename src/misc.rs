use std::io;
use std::io::Write;

use crate::definitions::*;
use crate::add_to_template::*;
use crate::new_template::*;

pub fn get_optional_arg(arg_name: String, args: Vec<String>) -> Option<String> {
   match check_if_arg_exists(arg_name, args) {
       TemplateBuildResult::OkRes(arg_val,_) => Some(arg_val.value),
       _ => None
   }
}

pub fn args_decide_action( first_arg: String, args: Vec<String>) -> TemplateBuildResult<Actions,Vec<String>,String> {
    match first_arg.to_uppercase().as_str() {
        "NEW" => {
            get_new_template_model(args)
                .id()
                .ignore();

            run_console()
        },
        "ADD" => {
            let arg_names: Vec<String> = vec!["-C","-S", "-M"]
                .into_iter().map(|x| x.to_string()).collect();

            get_many_args(arg_names, args)
                .id()
                .ignore();

            run_console()
        },
        "-H" => {
            
            let help_res: DummyType = TemplateBuildResult::ShowHelp;

            help_res
                .id()
                .ignore();

            run_console()
        } 
        "EXIT" => TemplateBuildResult::Exit,
        _ => {
            let invalid_arg_res: DummyType = TemplateBuildResult::InvalidArg(first_arg);

            invalid_arg_res
                .id()
                .ignore();

            run_console()
        }
    }
}


pub fn check_if_arg_val_exists(pos:usize, args:Vec<String>) -> TemplateBuildResult<ArgInfo,Vec<String>,String> {
    let arg_val_pos = pos + 1;
    let arg_len_min_one = args.len() - 1;
    match arg_len_min_one < arg_val_pos {
        false => {
            TemplateBuildResult::OkRes(ArgInfo { name: args[pos].clone(), value: args[pos + 1].clone()}, args.clone())
        },
        true => {
            TemplateBuildResult::GeneralError(format!("arg value for {} does not exist", args[pos]))
        }
    }
}

pub fn check_if_arg_exists(arg_name: String, args:Vec<String>) -> TemplateBuildResult<ArgInfo,Vec<String>,String> {
    match args.iter()
              .enumerate()
              .find(|key_val| key_val.1.to_uppercase().replace(" ", "").to_string() == arg_name.to_uppercase()) {
                Some(key_val) => check_if_arg_val_exists(key_val.0,args),
                None => {
                    TemplateBuildResult::MissingArg(MissingArgInfo { arg_nqme: arg_name })
                } 
            }

}

pub fn check_if_init_arg_exists(arg_pos: usize, args:Vec<String>) -> TemplateBuildResult<String,Vec<String>,String> {
    match args.len() -1 >= arg_pos {
        true => TemplateBuildResult::OkRes(args[arg_pos].clone(), args.clone()),
        false => TemplateBuildResult::GeneralError("No initial arg exists".to_string()),
    }
}


pub fn read_console_line() -> TemplateBuildResult<usize,Vec<String>,String> {
    println!();
    println!();
    println!("Welcome to Template Builder Console! Enter command '-h' followed by 'ENTER' for help.");
    print!("templatebuilder>>  ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let args: Vec<String> = input
                .trim()
                .split(" ") 
                .filter(|a| a.replace(" ", "") != "")
                .map(|a| a.replace(" ", "").replace("\n", "").trim().to_string())
                .collect();
            
            TemplateBuildResult::OkRes(0, args)
        },
        Err(e) => TemplateBuildResult::GeneralError(e.to_string()),
    }
}

pub enum Actions {
    New,
    Add,
    Help
}

