use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::definitions::*;
use crate::misc::*;

pub fn create_new_file(f_info: NewFileInfo, state: BaseAddModel) -> TemplateBuildResult<(), BaseAddModel, String> {
    let new_content = f_info.new_content.clone();

    match std::fs::write(f_info.file_info.full_path, new_content) {
        Ok(_) => (),
        Err(err) => {
            return TemplateBuildResult::GeneralError(format!(
                "{} - {:?}",
                err.to_string(),
                curr_pos_err_format("create_new_file".to_string(), "add_to_template.rs".to_string(), line!().to_string()))
            )
        }
    }
    TemplateBuildResult::OkRes((), state)
}

pub fn change_file_content(f_info: ChangeFileInfo, state: BaseAddModel) -> TemplateBuildResult<(), BaseAddModel, String> {
    let mut any_match_found = false;
    let mut new_content = f_info.file_info.content.clone();

    for change_info in f_info.change_infos {
        match Regex::new(change_info.search_pattern.as_str()) {
            Ok(re) => match re.find(&f_info.file_info.content) {
                Some(found_val) => {
                    any_match_found = true;
                    new_content = new_content.replace(
                        found_val.as_str(),
                        format!("{}{}", found_val.as_str(), change_info.str_to_replace).as_str(),
                    );
                }
                None => (),
            },
            Err(err) => {
                return TemplateBuildResult::GeneralError(format!(
                    "{} - {:?}",
                    err.to_string(),
                    curr_pos_err_format("change_file_content".to_string(), "add_to_template.rs".to_string(), line!().to_string())
                ))
            }
        }
    }

    if any_match_found {
        match std::fs::write(f_info.file_info.full_path, new_content) {
            Ok(_) => (),
            Err(err) => {
                return TemplateBuildResult::GeneralError(format!(
                    "{} - {:?}",
                    err.to_string(),
                    curr_pos_err_format("change_file_content".to_string(), "add_to_template.rs".to_string(), line!().to_string())
                ))
            }
        }
    }

    TemplateBuildResult::OkRes((), state)
}

fn search_rel_path_rec(
    dir: &std::fs::DirEntry,
    rel_path: &String,
) -> std::io::Result<Option<PathBuf>> {
    let entries = std::fs::read_dir(dir.path())?;

    for entry in entries {
        let entry = entry?;
        if entry.path().is_dir() {
            if let Some(curr_dir) = entry.path().as_path().to_str() {
                if curr_dir
                    .replace("\\", "/")
                    .to_uppercase()
                    .contains(rel_path.to_uppercase().as_str())
                {
                    return Ok(Some(entry.path()));
                } else {
                    return search_rel_path_rec(&entry, rel_path)
                }
            }
        }
    }

    Ok(None)
}


fn search_from_curr_dir (
    rel_path: &String,
) -> std::io::Result<Option<PathBuf>> {
    let entries = std::fs::read_dir(".")?;

    for entry in entries {
        let entry = entry?;
        if entry.path().is_dir() {
            if let Some(curr_dir) = entry.path().as_path().to_str() {
                if curr_dir
                    .replace("\\", "/")
                    .to_uppercase()
                    .contains(rel_path.to_uppercase().as_str())
                {
                    return Ok(Some(entry.path()));
                } else {
                    return search_rel_path_rec(&entry, rel_path)
                }
            }
        }
    }

    Ok(None)
}


pub fn search_rel_paths(
    f_type: NewContentFileType,
    state: BaseAddModel,
) -> TemplateBuildResult<SearchFileInFolderInfo, BaseAddModel, String> {
    let rel_paths = f_type.clone().get_rel_folder_path(state.clone());

    let mut found_folder_opt: Option<std::path::PathBuf> = None;

    for rel_path in rel_paths.clone() {
        match search_from_curr_dir(&rel_path) {
            Ok(found_path_opt) => match found_path_opt {
                Some(found_path) => {
                    found_folder_opt = Some(found_path);
                    break;
                }
                None => (),
            },
            Err(err) => {
                return TemplateBuildResult::GeneralError(format!(
                    "{} -> {}",
                    curr_pos_err_format("search_rel_paths".to_string(), "add_to_template.rs".to_string(), line!().to_string()),
                    err.to_string()
                ))
            }
        }
    }

    let rel_paths = rel_paths.clone();
    let res = SearchFileInFolderInfo {
        rel_paths,
        found_folder: found_folder_opt,
        file_type: f_type,
        state: state.clone()
    };

    TemplateBuildResult::OkRes(res, state)
}

pub fn check_if_folder_exists(
    s_folder_info: SearchFileInFolderInfo,
    state: BaseAddModel,
) -> TemplateBuildResult<PathBuf, SearchFileInFolderInfo, String> {
    match s_folder_info.found_folder.clone() {
        Some(found_folder) => {
            return TemplateBuildResult::OkRes(found_folder, s_folder_info);
        }
        None => {
            let new_dir_to_create =
                std::path::Path::new(&state.dir_path).join(&s_folder_info.rel_paths[0]);
            match std::fs::create_dir(&new_dir_to_create) {
                Ok(_) => return TemplateBuildResult::OkRes(new_dir_to_create, s_folder_info),
                Err(err) => {
                    return TemplateBuildResult::GeneralError(format!(
                        "{} -> {}",
                        curr_pos_err_format(
                            "check_if_folder_exists".to_string(),
                            "add_to_template.rs".to_string(),
                            line!().to_string()
                        ),
                        err.to_string()
                    ))
                }
            }
        }
    }
}

pub fn find_file(folder: PathBuf, f_name: &String) -> std::io::Result<Option<BasicFileInfo>> {
    for entry in std::fs::read_dir(folder)? {
        let entry = entry?;
        if f_name == entry.file_name().to_str().unwrap() {
            return Ok(Some(BasicFileInfo {
                name: entry.file_name().to_str().unwrap().to_string(),
                full_path: entry.path().to_str().unwrap().to_string(),
            }));
        } else {
            ()
        }
    }
    Ok(None)
}

pub fn find_file_wrapper(
    folder_w_file: PathBuf,
    s_folder_info: SearchFileInFolderInfo,
    ) -> TemplateBuildResult<Option<BasicFileInfo>, SearchFileInFolderInfo, String> {
    let search_file_name = s_folder_info
        .clone()
        .file_type
        .get_file_name(s_folder_info.state.clone());
    match find_file(folder_w_file, &search_file_name) {
        Ok(found_file_opt) => TemplateBuildResult::OkRes(found_file_opt, s_folder_info),
        Err(err) => TemplateBuildResult::GeneralError(format!(
                "{} -> {}",
                curr_pos_err_format("find_file_wrapper".to_string(), "add_to_template.rs".to_string(), line!().to_string()),
                err.to_string()
                )),
    }
}

pub fn get_file_decision(
    found_file_opt: Option<BasicFileInfo>,
    s_folder_info: SearchFileInFolderInfo,
) -> TemplateBuildResult<(), BaseAddModel, String> {
    let file_action_type = s_folder_info.file_type.clone().get_file_action_type();
    let file_name = s_folder_info
        .file_type
        .clone()
        .get_file_name(s_folder_info.state.clone());
    let rel_path = s_folder_info
        .file_type
        .clone()
        .get_rel_folder_path(s_folder_info.state.clone())[0]
        .clone();
    let new_content = s_folder_info.file_type.clone().get_new_file_content();

    match (file_action_type, found_file_opt) {
        (FileActionType::AskOnFileExists, None) => {
            let full_path = s_folder_info
                .state
                .dir_path
                .join(&rel_path)
                .join(&file_name)
                .to_str()
                .unwrap()
                .to_string();

            let new_file_info = NewFileInfo {
                file_info: BasicFileInfo {
                    name: file_name,
                    full_path,
                },
                new_content,
            };

            create_new_file(new_file_info, s_folder_info.state)
        }
        (FileActionType::AskOnFileExists, Some(basic_file_info)) => {
            let new_file_info = NewFileInfo {
                file_info: basic_file_info.clone(),
                new_content,
            };

            TemplateBuildResult::FileAlreadyExists(basic_file_info, new_file_info, s_folder_info.state)
                .bind(create_new_file)
        }
        (FileActionType::AskOnFileNotExists, None) => {
            let full_path = s_folder_info
                .state
                .dir_path
                .join(&rel_path)
                .clone()
                .join(&file_name)
                .to_str()
                .unwrap()
                .to_string()
                .clone();
            let basic_file_info = BasicFileInfo {
                name: file_name.clone(),
                full_path,
            };
            let new_file_info = NewFileInfo {
                file_info: basic_file_info.clone(),
                new_content,
            };

            TemplateBuildResult::FileDoesNotExist(basic_file_info, new_file_info, s_folder_info.state)
                .bind(create_new_file)
        }
        (FileActionType::AskOnFileNotExists, Some(basic_file_info)) => {
            match std::fs::read_to_string(basic_file_info.full_path.clone()) {
                Ok(content) => {
                    let file_info = FileInfo {
                        name: basic_file_info.name,
                        full_path: basic_file_info.full_path,
                        content,
                    };

                    let change_file_info = ChangeFileInfo {
                        file_info,
                        change_infos: s_folder_info
                            .file_type
                            .get_replace_pattern_infos(s_folder_info.state.clone()),
                    };

                    change_file_content(change_file_info, s_folder_info.state)
                }
                Err(err) => TemplateBuildResult::GeneralError(format!(
                    "{} -> {}",
                    curr_pos_err_format("get_file_decision".to_string(), "add_to_template.rs".to_string(), line!().to_string()),
                    err.to_string()
                )),
            }
        }
        (FileActionType::NoAFileAction, None) => {
            let full_path = s_folder_info
                .state
                .dir_path
                .join(&rel_path)
                .join(&file_name)
                .to_str()
                .unwrap()
                .to_string();

            let new_file_info = NewFileInfo {
                file_info: BasicFileInfo {
                    name: file_name,
                    full_path,
                },
                new_content,
            };

            create_new_file(new_file_info, s_folder_info.state)
        }
        (FileActionType::NoAFileAction, Some(basic_file_info)) => {
            match std::fs::read_to_string(basic_file_info.full_path.clone()) {
                Ok(content) => {
                    let file_info = FileInfo {
                        name: basic_file_info.name,
                        full_path: basic_file_info.full_path,
                        content,
                    };

                    let change_file_info = ChangeFileInfo {
                        file_info,
                        change_infos: s_folder_info
                            .file_type
                            .get_replace_pattern_infos(s_folder_info.state.clone()),
                    };

                    change_file_content(change_file_info, s_folder_info.state)
                }
                Err(err) => TemplateBuildResult::GeneralError(format!(
                    "{} -> {}",
                    curr_pos_err_format("get_file_decision".to_string(), "add_to_template.rs".to_string(), line!().to_string()),
                    err.to_string()
                )),
            }
        }
    }
}

pub fn do_file_actions(
    f_type: NewContentFileType,
    state: BaseAddModel,
    ) -> TemplateBuildResult<(), BaseAddModel, String> {
    search_rel_paths(f_type, state)
        .bind(check_if_folder_exists)
        .bind(find_file_wrapper)
        .bind(get_file_decision)
}

pub fn do_all_file_actions(
    _: (),
    state: BaseAddModel,
) -> TemplateBuildResult<HashMap<NewContentFileType, ()>, BaseAddModel, String> {
    NewContentFileType::get_all_options()
        .bind_many(do_file_actions,state)
}

pub fn get_model(
    arg_vals: HashMap<String, ArgInfo>,
    args: Vec<String>,
) -> TemplateBuildResult<(), BaseAddModel, String> {
    let mut model = BaseAddModel::default();

    if let Some(controller_val) = arg_vals.get("-C") {
        model.controller = ArgInfo {
            name: controller_val.name.clone(),
            value: controller_val.value.clone(),
        };
    } else {
        return TemplateBuildResult::GeneralError(
            "expected value for arg '-c', found none.".to_string(),
        );
    }

    if let Some(service_val) = arg_vals.get("-S") {
        model.service = ArgInfo {
            name: service_val.name.clone(),
            value: service_val.value.clone(),
        };
    } else {
        return TemplateBuildResult::GeneralError(
            "expected value for arg '-s', found none.".to_string(),
        );
    }

    if let Some(method_val) = arg_vals.get("-M") {
        model.method = ArgInfo {
            name: method_val.name.clone(),
            value: method_val.value.clone(),
        };
    } else {
        return TemplateBuildResult::GeneralError(
            "expected value for arg '-m', found none.".to_string(),
        );
    }

    if let Some(req_type) = get_optional_arg("-rt".to_string(), args.clone()) {
        model.req_type = ArgInfo {
            name: "rt".to_string(),
            value: req_type,
        };
    } else {
        model.req_type = ArgInfo {
            name: "rt".to_string(),
            value: "GET".to_string(),
        };
    }

    if let Some(req_type) = get_optional_arg("-t".to_string(), args) {
        model.action_type = ArgInfo {
            name: "-t".to_string(),
            value: req_type,
        };
    } else {
        model.action_type = ArgInfo {
            name: "-t".to_string(),
            value: "SP".to_string(),
        };
    }

    get_curr_dir((), model)
}

pub fn get_curr_dir(_: (), state: BaseAddModel) -> TemplateBuildResult<(), BaseAddModel, String> {
    match std::env::current_dir() {
        Ok(curr_dir_ok) => {
            let mut new_state = state.clone();
            new_state.dir_path = curr_dir_ok;

            TemplateBuildResult::OkRes((),state)
        }
        Err(err) => TemplateBuildResult::GeneralError(format!(
            "{} -> {}",
            curr_pos_err_format("check_if_folder_exists".to_string(), "add_to_template.rs".to_string(), line!().to_string()),
            err.to_string()
        )),
    }
}

pub fn print_model(_: (), model: BaseAddModel) -> TemplateBuildResult<(), BaseAddModel, String> {
    print!("{:?}", model);
    TemplateBuildResult::Exit
}

pub fn get_many_args(
    arg_names: Vec<String>,
    args: Vec<String>,
) -> TemplateBuildResult<(), BaseAddModel, String> {
    arg_names
        .bind_many(check_if_arg_exists, args)
        .bind(get_model)
        .bind(print_model)
}

pub fn run_console() -> TemplateBuildResult<Actions, Vec<String>, String> {
    read_console_line()
        .bind(check_if_init_arg_exists)
        .bind(args_decide_action)
}
