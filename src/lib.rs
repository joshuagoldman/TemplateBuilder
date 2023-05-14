pub mod misc;
pub mod definitions;
pub mod add_to_template;
pub mod new_template;
use definitions::*;
use misc::*;

pub fn run_console() -> TemplateBuildResult<Actions, Vec<String>, String> {
    read_console_line()
        .bind(check_if_init_arg_exists)
        .bind(args_decide_action)
}

