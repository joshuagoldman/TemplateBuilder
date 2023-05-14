use std::{hash::Hash};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::io::{Write, self};

pub struct Dummy {
}

pub type DummyType = TemplateBuildResult<Dummy,Dummy,String>;

const HELP_TEXT:&str = include_str!("embedded_resources/help_info.txt");


#[derive(Default, Debug, Hash, PartialEq, Eq)]
pub struct ArgInfo {
    pub name: String,
    pub value: String
}

#[derive(Default, Debug)]
pub struct AddModel {
    pub controller: ArgInfo,
    pub service:  ArgInfo,
    pub method: ArgInfo,
    pub req_type: ArgInfo,
    pub action_type: ArgInfo
}

#[derive(Default, Debug, Clone)]
pub struct NewTemplateModel {
    pub new_template_path: String,
    pub name: String,
    pub source_dir: String
}

pub struct MissingArgInfo {
    pub arg_nqme: String,
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ChangeFileInfo {
    pub file_info: FileInfo,
    pub search_pattern: String,
    pub str_to_replace: String
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FileInfo {
    pub name:String,
    pub full_path: String,
    pub content: String
}

pub struct NewFile {
    pub name: String,
    pub full_path: String
}

pub enum TemplateBuildResult<T1,T2,E> {
    OkRes(T1,T2),
    GeneralError(E),
    MissingArg(MissingArgInfo),
    Exit,
    FileNotExist(FileInfo,T1,T2),
    ReqFileMissing(String),
    InvalidArg(String),
    ShowHelp
}

pub fn file_not_found_action<T1,T2,E>(a: T1, b:T2, file_info: FileInfo) -> TemplateBuildResult<T1,T2,E> {
    println!("File {} does not exist. If you wish to create it, write 'y' followed by 'ENTER' on the keyboard, otherwise click anywhere on the keyboard to exit", file_info.name);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.to_uppercase().as_str() {
                "Y" => {
                    TemplateBuildResult::OkRes(a,b)
                }
                _ => {
                    println!("You chose to exit!");

                    TemplateBuildResult::Exit
                }


            }
        },
        Err(err) => {
            print!("{:?>}", err);
            TemplateBuildResult::Exit 
        }
    }
}

impl<T1,T2,E: Debug + Display>TemplateBuildResult<T1,T2,E> {
    pub fn bind<T3,T4,F>(self, bind_func: F) -> TemplateBuildResult<T3,T4,E>
    where 
        F: Fn(T1,T2) -> TemplateBuildResult<T3,T4,E> , 
    {
        match self {
            TemplateBuildResult::OkRes(ok_res, sec_param) => bind_func(ok_res, sec_param),
            TemplateBuildResult::GeneralError(err) => {
                print!("{:?>}", err);

                TemplateBuildResult::Exit
                
            },
            TemplateBuildResult::MissingArg(arg_info) => {
                print!("The argument {} is missing!", arg_info.arg_nqme);

                TemplateBuildResult::Exit
            },
            TemplateBuildResult::Exit => TemplateBuildResult::Exit,
            TemplateBuildResult::FileNotExist(file_info, ok_res, sec_param) => {
                file_not_found_action(ok_res, sec_param, file_info)
                    .bind(bind_func)

            },
            TemplateBuildResult::ReqFileMissing(file_name) => {

                print!("File {} does unfortunately not exist", file_name );

                TemplateBuildResult::Exit
            },
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

    pub fn id(self) -> TemplateBuildResult<T1,T2,E> {
       self.bind(id_func)
    }


    pub fn ignore(self) -> () {
        ()
    }
}

pub trait TemplateBuilderCollection<T1: Eq + Hash,T2,E> {
    fn bind_many<T3: Eq + Hash,F>(self, bind_func: F, sec_param: T2) -> TemplateBuildResult<HashMap<T1,T3>,T2,E>
    where
        F: Fn(T1,T2) -> TemplateBuildResult<T3,T2,E>;
}

impl<T1: Eq + Hash + Clone,T2: Clone,E> TemplateBuilderCollection<T1,T2,E> for Vec<T1> {
    fn bind_many<T3: Eq + Hash,F>(self, bind_func: F, sec_param: T2) -> TemplateBuildResult<HashMap<T1,T3>,T2,E>
    where
        F: Fn(T1,T2) -> TemplateBuildResult<T3,T2,E>,
    {
        let mut res_map: HashMap<T1,T3> = HashMap::new();
        for first_param in self {
            match bind_func(first_param.clone(), sec_param.clone()) {
                TemplateBuildResult::OkRes(ok_res,_) => {
                    res_map.insert(first_param, ok_res);
                }
                TemplateBuildResult::GeneralError(a) => return TemplateBuildResult::GeneralError(a),
                TemplateBuildResult::MissingArg(a) => return TemplateBuildResult::MissingArg(a),
                TemplateBuildResult::Exit => return TemplateBuildResult::Exit,
                TemplateBuildResult::FileNotExist(a, b, c) => {

                    let f_not_found_act_res: TemplateBuildResult<T3, T2, E> = file_not_found_action(b,c,a);
                    match f_not_found_act_res {
                        TemplateBuildResult::OkRes(ok_res,_ ) => {
                            res_map.insert(first_param, ok_res);
                        },
                        _ => return TemplateBuildResult::Exit,
                    }


                },
                TemplateBuildResult::ReqFileMissing(a) => return TemplateBuildResult::ReqFileMissing(a) ,
                TemplateBuildResult::InvalidArg(a) => return TemplateBuildResult::InvalidArg(a),
                TemplateBuildResult::ShowHelp => return TemplateBuildResult::ShowHelp,
            } 
        }

        TemplateBuildResult::OkRes(res_map, sec_param)
    }
}

pub fn id_func<T1,T2,E>(_:T1, _: T2) -> TemplateBuildResult<T1,T2,E> {
    TemplateBuildResult::Exit
}
