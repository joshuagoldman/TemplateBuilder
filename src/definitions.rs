use std::collections::HashMap;
use std::fmt::{Debug, Display};

use std::hash::Hash;
use std::io::{self, Write};
use std::path::{PathBuf};

pub struct Dummy {}

pub type DummyType = TemplateBuildResult<Dummy, Dummy, String>;

const HELP_TEXT: &str = include_str!("embedded_resources/help_info.txt");
const CONTROLLER: &str = include_str!("embedded_resources/help_info.txt");
const SERVICE_CLASS: &str = include_str!("embedded_resources/help_info.txt");
const SERVICE_INTERFACE: &str = include_str!("embedded_resources/help_info.txt");
const METHOD_CONF_CLASS: &str = include_str!("embedded_resources/help_info.txt");
const METHOD_EXEC: &str = include_str!("embedded_resources/help_info.txt");
const METHOD_RES_HANDLER: &str = include_str!("embedded_resources/help_info.txt");
const DEPENDENCY_INJ: &str = include_str!("embedded_resources/help_info.txt");
const REQUEST_MODEL: &str = include_str!("embedded_resources/help_info.txt");
const RESPONSE_MODEL: &str = include_str!("embedded_resources/help_info.txt");
const DATA_MODEL: &str = include_str!("embedded_resources/help_info.txt");

const CONTROLLER_REPLACE: &str = include_str!("embedded_resources/help_info.txt");
const SERVICE_CLASS_REPLACE: &str = include_str!("embedded_resources/help_info.txt");
const SERVICE_INTERFACE_REPLACE: &str = include_str!("embedded_resources/help_info.txt");
const METHOD_EXEC_REPLACE: &str = include_str!("embedded_resources/help_info.txt");
const METHOD_RES_HANDLER_REPLACE: &str = include_str!("embedded_resources/help_info.txt");
const DEPENDENCY_INJ_REPLACE: &str = include_str!("embedded_resources/help_info.txt");

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum NewContentFileType {
    Controller,
    ServiceClass,
    ServiceInterface,
    MethodConfClass,
    MethodExec,
    MethodResHandler,
    DependencyInj,
    RequestModel,
    ResponseModel,
    DataModel,
}

pub enum FileActionType {
    AskOnFileExists,
    AskOnFileNotExists,
    NoAFileAction
}

impl NewContentFileType {
    pub fn get_all_options() -> Vec<NewContentFileType> {
        vec![
            NewContentFileType::Controller ,
            NewContentFileType::ServiceClass ,
            NewContentFileType::ServiceInterface ,
            NewContentFileType::MethodConfClass ,
            NewContentFileType::MethodExec ,
            NewContentFileType::MethodResHandler ,
            NewContentFileType::DependencyInj ,
            NewContentFileType::RequestModel ,
            NewContentFileType::ResponseModel ,
            NewContentFileType::DataModel ,
        ]
    }
    pub fn get_file_name(self, state: BaseAddModel) -> String {
        match self {
            NewContentFileType::Controller => format!("{}.Controller.cs",state.controller.value),
            NewContentFileType::ServiceClass => format!("{}Service.cs", state.service.value),
            NewContentFileType::ServiceInterface => format!("I{}Service.cs", state.service.value),
            NewContentFileType::MethodConfClass => format!("{}Config.cs", state.method.value),
            NewContentFileType::MethodExec => format!("{}Executor:.cs", state.method.value),
            NewContentFileType::MethodResHandler => format!("{}ResultHandler.cs", state.method.value),
            NewContentFileType::DependencyInj => "RegisterServices.cs".to_string(),
            NewContentFileType::RequestModel => format!("{}RequestModel.cs", state.method.value),
            NewContentFileType::ResponseModel => format!("{}ResponseModel.cs", state.method.value),
            NewContentFileType::DataModel => format!("{}Model.cs", state.method.value),
        }
    }
    pub fn get_file_action_type(self) -> FileActionType {
        match self {
            NewContentFileType::Controller => FileActionType::AskOnFileNotExists,
            NewContentFileType::ServiceClass => FileActionType::AskOnFileExists,
            NewContentFileType::ServiceInterface => FileActionType::AskOnFileNotExists,
            NewContentFileType::MethodConfClass => FileActionType::AskOnFileExists,
            NewContentFileType::MethodExec => FileActionType::AskOnFileExists,
            NewContentFileType::MethodResHandler => FileActionType::AskOnFileExists,
            NewContentFileType::DependencyInj => FileActionType::NoAFileAction,
            NewContentFileType::RequestModel => FileActionType::AskOnFileExists,
            NewContentFileType::ResponseModel => FileActionType::AskOnFileExists,
            NewContentFileType::DataModel => FileActionType::AskOnFileExists,
        }
    }
    pub fn get_rel_folder_path(self, state: BaseAddModel) -> Vec<String> {
        let method_path_part = match state.action_type.value.to_uppercase().as_str() {
            "IF" => "If",
            _ => "StoredProcedure"
        };

        match self {
            NewContentFileType::Controller => vec!["Controller".to_string()],
            NewContentFileType::ServiceClass => vec!["Services".to_string()],
            NewContentFileType::ServiceInterface => vec!["Services/Interfaces".to_string()],
            NewContentFileType::MethodConfClass => vec![format!("{}/{}", method_path_part, state.method.value)],
            NewContentFileType::MethodExec => vec![format!("{}/{}", method_path_part, state.method.value)],
            NewContentFileType::MethodResHandler => vec![format!("{}/{}", method_path_part, state.method.value)],
            NewContentFileType::DependencyInj => vec!["".to_string()],
            NewContentFileType::RequestModel => vec!["RequestModels".to_string()],
            NewContentFileType::ResponseModel => vec!["ResponseModels".to_string()],
            NewContentFileType::DataModel => vec!["Models".to_string()],
        }
    }
    pub fn get_replace_pattern_infos(self, state: BaseAddModel) -> Vec<ChangePatternInfo> {
        let _method_path_part = match state.action_type.value.to_uppercase().as_str() {
            "IF" => "If".to_string(),
            _ => "StoredProcedure".to_string()
        };

        match self {
            NewContentFileType::Controller => {
                vec![ChangePatternInfo {
                    str_to_replace: CONTROLLER_REPLACE.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
            NewContentFileType::ServiceClass => {
                vec![ChangePatternInfo {
                    str_to_replace: SERVICE_CLASS_REPLACE.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
            NewContentFileType::ServiceInterface => {
                vec![ChangePatternInfo {
                    str_to_replace: SERVICE_INTERFACE_REPLACE.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
            NewContentFileType::MethodConfClass => {
                vec![ChangePatternInfo {
                    str_to_replace: METHOD_CONF_CLASS.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
            NewContentFileType::MethodExec => {
                vec![ChangePatternInfo {
                    str_to_replace: METHOD_EXEC_REPLACE.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
            NewContentFileType::MethodResHandler => {
                vec![ChangePatternInfo {
                    str_to_replace: METHOD_RES_HANDLER_REPLACE.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
            NewContentFileType::DependencyInj => {
                vec![ChangePatternInfo {
                    str_to_replace: DEPENDENCY_INJ_REPLACE.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
            NewContentFileType::RequestModel => {
                vec![ChangePatternInfo {
                    str_to_replace: REQUEST_MODEL.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
            NewContentFileType::ResponseModel => {
                vec![ChangePatternInfo {
                    str_to_replace: RESPONSE_MODEL.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
            NewContentFileType::DataModel => {
                vec![ChangePatternInfo {
                    str_to_replace: DATA_MODEL.clone().to_string(),
                    search_pattern: "".to_string(),
                }]
            }
        }
    }
    pub fn get_new_file_content(self) -> String {
        match self {
            NewContentFileType::Controller => CONTROLLER.clone().to_string(),
            NewContentFileType::ServiceClass => SERVICE_CLASS.clone().to_string(),
            NewContentFileType::ServiceInterface => SERVICE_INTERFACE.clone().to_string(),
            NewContentFileType::MethodConfClass => METHOD_CONF_CLASS.clone().to_string(),
            NewContentFileType::MethodExec => METHOD_EXEC.clone().to_string(),
            NewContentFileType::MethodResHandler => METHOD_RES_HANDLER.clone().to_string(),
            NewContentFileType::DependencyInj => DEPENDENCY_INJ.clone().to_string(),
            NewContentFileType::RequestModel => REQUEST_MODEL.clone().to_string(),
            NewContentFileType::ResponseModel => RESPONSE_MODEL.clone().to_string(),
            NewContentFileType::DataModel => DATA_MODEL.clone().to_string(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct SearchFileInFolderInfo {
    pub found_folder: Option<PathBuf>,
    pub rel_paths: Vec<String>,
    pub file_type: NewContentFileType,
    pub state: BaseAddModel,
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ArgInfo {
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone)]
pub struct BaseAddModel {
    pub controller: ArgInfo,
    pub service: ArgInfo,
    pub method: ArgInfo,
    pub req_type: ArgInfo,
    pub action_type: ArgInfo,
    pub dir_path: PathBuf
}

#[derive(Default, Debug, Clone)]
pub struct NewTemplateModel {
    pub new_template_path: String,
    pub name: String,
    pub source_dir: String,
}

pub struct MissingArgInfo {
    pub arg_nqme: String,
}

#[derive(Default, Debug, Clone)]
pub struct NewFileInfo {
    pub new_content: String,
    pub file_info: BasicFileInfo
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ChangePatternInfo {
    pub search_pattern: String,
    pub str_to_replace: String,
}
#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ChangeFileInfo {
    pub file_info: FileInfo,
    pub change_infos: Vec<ChangePatternInfo>,
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FileInfo {
    pub name: String,
    pub full_path: String,
    pub content: String,
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct BasicFileInfo {
    pub name: String,
    pub full_path: String,
}

pub enum TemplateBuildResult<T1, T2, E> {
    OkRes(T1, T2),
    GeneralError(E),
    MissingArg(MissingArgInfo),
    Exit,
    FileAlreadyExists(BasicFileInfo,T1,T2),
    FileDoesNotExist(BasicFileInfo,T1,T2),
    ReqFileMissing(String),
    Continue,
    InvalidArg(String),
    ShowHelp,
}

pub fn file_already_exists_action<T1, T2, E>(
    a: T1,
    b: T2,
    file_info: BasicFileInfo,
) -> TemplateBuildResult<T1, T2, E> {
    println!("File {} already exists. If you wish to override it, write 'o' followed by 'ENTER' on the keyboard,
             if you want to continue, write 'c' followed by 'ENTER' on the keyboard, otherwise click anywhere on the keyboard to exit", file_info.name);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.to_uppercase().as_str() {
            "O" => TemplateBuildResult::OkRes(a, b),
            "C" => TemplateBuildResult::Continue,
            _ => {
                println!("You chose to exit!");

                TemplateBuildResult::Exit
            }
        },
        Err(err) => {
            print!("{:?>}", err);
            TemplateBuildResult::Exit
        }
    }
}


pub fn file_does_not_exust_action<T1, T2, E>(
    a: T1,
    b: T2,
    file_info: BasicFileInfo,
) -> TemplateBuildResult<T1, T2, E> {
    println!("File {} does not  exists. If you wish to create it, write 'y' followed by 'ENTER' on the keyboard, otherwise click anywhere on the keyboard to exit", file_info.name);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.to_uppercase().as_str() {
            "Y" => TemplateBuildResult::OkRes(a, b),
            _ => {
                println!("You chose to exit!");

                TemplateBuildResult::Exit
            }
        },
        Err(err) => {
            print!("{:?>}", err);
            TemplateBuildResult::Exit
        }
    }
}

impl<T1, T2, E: Debug + Display> TemplateBuildResult<T1, T2, E> {
    pub fn bind<T3, T4, F>(self, bind_func: F) -> TemplateBuildResult<T3, T4, E>
    where
        F: Fn(T1, T2) -> TemplateBuildResult<T3, T4, E>,
    {
        match self {
            TemplateBuildResult::OkRes(ok_res, sec_param) => bind_func(ok_res, sec_param),
            TemplateBuildResult::Continue => TemplateBuildResult::Exit,
            TemplateBuildResult::GeneralError(err) => {
                print!("{:?>}", err);

                TemplateBuildResult::Exit
            }
            TemplateBuildResult::MissingArg(arg_info) => {
                print!("The argument {} is missing!", arg_info.arg_nqme);

                TemplateBuildResult::Exit
            }
            TemplateBuildResult::Exit => TemplateBuildResult::Exit,
            TemplateBuildResult::FileAlreadyExists(file_info, ok_res, sec_param) => {
                file_already_exists_action(ok_res, sec_param, file_info).bind(bind_func)
            }
            TemplateBuildResult::FileDoesNotExist(file_info, ok_res, sec_param) => {
                file_does_not_exust_action(ok_res, sec_param, file_info).bind(bind_func)
            }
            TemplateBuildResult::ReqFileMissing(file_name) => {
                print!("File {} does unfortunately not exist", file_name);

                TemplateBuildResult::Exit
            }
            TemplateBuildResult::InvalidArg(arg_val) => {
                print!("arg {} is invalid", arg_val);

                io::stdout().flush().unwrap();

                TemplateBuildResult::Exit
            }
            TemplateBuildResult::ShowHelp => {
                print!("{}", HELP_TEXT);

                io::stdout().flush().unwrap();
                TemplateBuildResult::Exit
            }
        }
    }

    pub fn id(self) -> TemplateBuildResult<T1, T2, E> {
        self.bind(id_func)
    }

    pub fn ignore(self) -> () {
        ()
    }
}

pub trait TemplateBuilderCollection<T1: Eq + Hash, T2, E> {
    fn bind_many<T3: Eq + Hash, F>(
        self,
        bind_func: F,
        sec_param: T2,
    ) -> TemplateBuildResult<HashMap<T1, T3>, T2, E>
    where
        F: Fn(T1, T2) -> TemplateBuildResult<T3, T2, E>;
}

impl<T1: Eq + Hash + Clone, T2: Clone, E> TemplateBuilderCollection<T1, T2, E> for Vec<T1> {
    fn bind_many<T3: Eq + Hash, F>(
        self,
        bind_func: F,
        sec_param: T2,
    ) -> TemplateBuildResult<HashMap<T1, T3>, T2, E>
    where
        F: Fn(T1, T2) -> TemplateBuildResult<T3, T2, E>,
    {
        let mut res_map: HashMap<T1, T3> = HashMap::new();
        for first_param in self {
            match bind_func(first_param.clone(), sec_param.clone()) {
                TemplateBuildResult::OkRes(ok_res, _) => {
                    res_map.insert(first_param, ok_res);
                }
                TemplateBuildResult::GeneralError(a) => {
                    return TemplateBuildResult::GeneralError(a)
                }
                TemplateBuildResult::MissingArg(a) => return TemplateBuildResult::MissingArg(a),
                TemplateBuildResult::Exit => return TemplateBuildResult::Exit,
                TemplateBuildResult::FileDoesNotExist(a, b, c) => {
                    let f_not_found_act_res: TemplateBuildResult<T3, T2, E> =
                        file_does_not_exust_action(b, c, a);
                    match f_not_found_act_res {
                        TemplateBuildResult::OkRes(ok_res, _) => {
                            res_map.insert(first_param, ok_res);
                        }
                        _ => return TemplateBuildResult::Exit,
                    }
                },
                TemplateBuildResult::FileAlreadyExists(a, b, c) => {
                    let f_not_found_act_res: TemplateBuildResult<T3, T2, E> =
                        file_does_not_exust_action(b, c, a);
                    match f_not_found_act_res {
                        TemplateBuildResult::OkRes(ok_res, _) => {
                            res_map.insert(first_param, ok_res);
                        },
                        TemplateBuildResult::Continue => (),
                        _ => return TemplateBuildResult::Exit,
                    }
                }
                TemplateBuildResult::ReqFileMissing(a) => {
                    return TemplateBuildResult::ReqFileMissing(a)
                }
                TemplateBuildResult::InvalidArg(a) => return TemplateBuildResult::InvalidArg(a),
                TemplateBuildResult::ShowHelp => return TemplateBuildResult::ShowHelp,
                TemplateBuildResult::Continue => return TemplateBuildResult::Continue,
            }
        }

        TemplateBuildResult::OkRes(res_map, sec_param)
    }
}

pub fn id_func<T1, T2, E>(_: T1, _: T2) -> TemplateBuildResult<T1, T2, E> {
    TemplateBuildResult::Exit
}
