use std::collections::HashMap;

use crate::misc::*;
use crate::definitions::*;


pub fn get_model(arg_vals: HashMap<String,ArgInfo>, args: Vec<String>) -> TemplateBuildResult<(), AddModel, String> {
    let mut model = AddModel::default();

    if let Some(controller_val) = arg_vals.get("-C") {
        model.controller = ArgInfo { name: controller_val.name.clone(), value: controller_val.value.clone()};
    }
    else {
        return TemplateBuildResult::GeneralError("expected value for arg '-c', found none.".to_string())
    }


    if let Some(service_val) = arg_vals.get("-S") {
        model.service =  ArgInfo { name: service_val.name.clone(), value: service_val.value.clone()};
    }
    else {
        return TemplateBuildResult::GeneralError("expected value for arg '-s', found none.".to_string())
    }


    if let Some(method_val) = arg_vals.get("-M") {
        model.method = ArgInfo { name: method_val.name.clone(), value: method_val.value.clone()};
    }
    else {
        return TemplateBuildResult::GeneralError("expected value for arg '-m', found none.".to_string())
    }

    if let Some(req_type) = get_optional_arg("-rt".to_string(), args.clone()) {
        model.req_type = ArgInfo { name: "rt".to_string(), value: req_type};
    }
    else {
        model.req_type = ArgInfo { name: "rt".to_string(), value: "GET".to_string()};
    }
    

    if let Some(req_type) = get_optional_arg("-t".to_string(), args) {
        model.action_type = ArgInfo { name: "-t".to_string(), value: req_type};
    }
    else {
        model.action_type = ArgInfo { name: "-t".to_string(), value: "SP".to_string()};
    }

    return TemplateBuildResult::OkRes((), model)
}

pub fn print_model(_: (), model: AddModel) -> TemplateBuildResult<(),AddModel,String> {
    print!("{:?}", model);
    TemplateBuildResult::Exit
}

pub fn get_many_args(arg_names: Vec<String>, args: Vec<String>) -> TemplateBuildResult<(), AddModel, String> {

    arg_names.bind_many(check_if_arg_exists, args)
        .bind(get_model)
        .bind(print_model)
}

pub fn run_console() -> TemplateBuildResult<Actions, Vec<String>, String> {
    read_console_line()
        .bind(check_if_init_arg_exists)
        .bind(args_decide_action)
}
