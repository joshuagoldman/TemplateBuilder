use std::{path::Path, io::Cursor, collections::{HashMap}};
use crate::{definitions::*, misc::*};

use regex::Regex;
use std::fs;

pub fn change_file_content(f_info: ChangeFileInfo, state: NewTemplateModel) -> TemplateBuildResult<(), NewTemplateModel, String> {
    for change_info in f_info.clone().change_infos {
        let f_info = f_info.clone();
        match Regex::new(change_info.search_pattern.as_str()) {
            Ok(re) => {
                let mut new_content = f_info.file_info.content.clone();
                for match_val in re.find_iter(f_info.file_info.content.as_str()) {
                    let match_val_to_replace = match_val.as_str().replace("WebApiTemplate", state.name.as_str());
                    new_content = new_content.replace(match_val.as_str(), match_val_to_replace.as_str());  
                }

                match std::fs::write(f_info.file_info.full_path, new_content) {
                    Ok(_) => (),
                    Err(err) => return TemplateBuildResult::GeneralError(format!("{:?}",err.to_string()))
                }
            },
            Err(err) => return TemplateBuildResult::GeneralError(err.to_string())
        }
    }

    return TemplateBuildResult::OkRes((), state);
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>, new_name: &String) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(&entry.file_name().to_str().unwrap().replace("WebApiTemplate", new_name.as_str())),&new_name)?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(&entry.file_name().to_str().unwrap().replace("WebApiTemplate", new_name.as_str())))?;
        }
    }
    Ok(())
}

fn get_contents_all(src: impl AsRef<Path>, info_vec: &mut Vec<ChangeFileInfo>, state: NewTemplateModel) -> std::io::Result<()> {
    for entry in fs::read_dir(src.as_ref().clone())? {
        let entry = entry?;
        let file_path = entry.path();
        let file_name = entry.file_name();
        let file_type = entry.file_type()?;
        let state = state.clone();

        if file_type.is_file() {
            if file_name.clone().to_str().unwrap().ends_with(".cs") {
                let file_info = FileInfo {
                    name: file_name.to_str().unwrap().to_string(),
                    full_path: file_path.to_str().unwrap().to_string(),
                    content: std::fs::read_to_string(file_path)?
                };
                let curr_change_file_info = 
                    ChangeFileInfo {
                        change_infos: vec![
                            ChangePatternInfo { 
                                search_pattern:r"(using WebApiTemplate|namespace WebApiTemplate).*".to_string(),
                                str_to_replace: state.name.clone(),
                            }
                        ],
                        file_info
                    };
                info_vec.push(curr_change_file_info);
            }
            else if file_name.clone().to_str().unwrap().ends_with(".csproj") {
                let file_info = FileInfo {
                    name: file_name.to_str().unwrap().to_string(),
                    full_path: file_path.to_str().unwrap().to_string(),
                    content: std::fs::read_to_string(file_path)?
                };
                let curr_change_file_info = 
                    ChangeFileInfo {
                        change_infos: vec![
                            ChangePatternInfo { 
                                search_pattern:r"(using WebApiTemplate|namespace WebApiTemplate).*".to_string(),
                                str_to_replace: state.name.clone(),
                            }
                        ],
                        file_info
                    };
                info_vec.push(curr_change_file_info);
            }
            else if file_name.clone().to_str().unwrap().ends_with(".sln") {
                let file_info = FileInfo {
                    name: file_name.to_str().unwrap().to_string(),
                    full_path: file_path.to_str().unwrap().to_string(),
                    content: std::fs::read_to_string(file_path)?
                };
                let curr_change_file_info = 
                    ChangeFileInfo {
                        change_infos: vec![
                            ChangePatternInfo { 
                                search_pattern:r"Project\(.*".to_string(),
                                str_to_replace: state.name.clone(),
                            }
                        ],
                        file_info
                    };
                info_vec.push(curr_change_file_info);
            }
        }
        else {
            get_contents_all(src.as_ref().join(&file_name.clone()), info_vec, state)?;
        }
    }
    Ok(())
}

pub fn change_file_contents(_:(), state: NewTemplateModel) -> TemplateBuildResult<HashMap<ChangeFileInfo,()>, NewTemplateModel, String> {
    let template_dir = Path::new(&state.new_template_path.clone().as_str()).join(&state.name.clone().as_str());
    match copy_dir_all(&Path::new(&state.source_dir.clone().replace("WebApiTemplate.zip", "temp")), &template_dir, &state.name.clone()) {
        Ok(_) => (),
        Err(err) => return TemplateBuildResult::GeneralError(err.to_string())
    }

    let mut change_file_infos: Vec<ChangeFileInfo> = Vec::new();
    match get_contents_all(template_dir, &mut change_file_infos, state.clone()) {
        Ok(_) => (),
        Err(err) => return TemplateBuildResult::GeneralError(err.to_string())
    }

    change_file_infos.bind_many(change_file_content, state)
}

pub fn get_template_zip_path(_:(), state:NewTemplateModel) -> TemplateBuildResult<(), NewTemplateModel, String> {
    match std::env::var("TEMPLATE_PATH") {
        Ok(path) => {
        
            let new_state = NewTemplateModel { source_dir: path,
                                                name: state.name.clone(),
                                                new_template_path: state.new_template_path.clone() };
            TemplateBuildResult::OkRes((),new_state)
        },
        Err(err) => TemplateBuildResult::GeneralError(err.to_string())
    }
}


pub fn validate_template_path(_:(), state:NewTemplateModel) -> TemplateBuildResult<(), NewTemplateModel, String> {
    if std::fs::metadata(&state.source_dir).is_ok() {
        TemplateBuildResult::OkRes((),state)
    } else {
        TemplateBuildResult::GeneralError(format!("File {} does not exist", state.source_dir))
    }
}

pub fn get_new_template_name_arg(arg_info:ArgInfo, args: Vec<String>) -> TemplateBuildResult<Vec<String>, NewTemplateModel, String> {
    let mut state = NewTemplateModel::default();
    state.name = arg_info.value;

    TemplateBuildResult::OkRes(args, state)
}

pub fn create_new_dir(_:(), state: NewTemplateModel) -> TemplateBuildResult<(),NewTemplateModel,String> {
    let final_unzip_path = format!("{}/{}", state.new_template_path, state.name);
    match std::fs::create_dir_all(&final_unzip_path) {
        Ok(_) => TemplateBuildResult::OkRes((), state),
        Err(err) => TemplateBuildResult::GeneralError(err.to_string())
    }
}

pub fn validate_template_dest_path(path: String, state:NewTemplateModel) -> TemplateBuildResult<(), NewTemplateModel, String> {
    let dir = Path::new(&path);
    if dir.is_dir() {
        let new_state = NewTemplateModel { source_dir: state.source_dir.clone(),
                                            name: state.name.clone(),
                                            new_template_path: path };
        TemplateBuildResult::OkRes((),new_state)
    } else {
        TemplateBuildResult::GeneralError(format!("Folder {} does not exist", path))
    }
}

pub fn get_curr_dir(state: NewTemplateModel) -> TemplateBuildResult<String, NewTemplateModel, String> {
    match std::env::current_dir() {
        Ok(curr_dir) => {
            match curr_dir.to_str() {
                
                Some(curr_dir_str) => TemplateBuildResult::OkRes(curr_dir_str.to_string(), state),
                _ => TemplateBuildResult::GeneralError("Unable to get current directory".to_string()),
            }
        },
        Err(err) => TemplateBuildResult::GeneralError(err.to_string()),
    }
}

pub fn get_template_destination_path(args: Vec<String>, state: NewTemplateModel) -> TemplateBuildResult<(), NewTemplateModel, String> {
    if let Some(dest_path) = get_optional_arg("-p".to_string(), args) {
       TemplateBuildResult::OkRes(dest_path, state)
           .bind(validate_template_dest_path)
    }
    else {
        get_curr_dir(state)
            .bind(validate_template_dest_path)
    }
}

pub fn get_template_source_path(_:(), state: NewTemplateModel) -> TemplateBuildResult<(), NewTemplateModel, String> {
    get_template_zip_path((), state)
        .bind(validate_template_path)
}
pub fn unzip_template(_:(), state: NewTemplateModel) -> TemplateBuildResult<(), NewTemplateModel, String> {
    let final_dest_folder_string = format!("{}", state.source_dir.clone().replace("WebApiTemplate.zip", "temp"));
    let final_dest_folder = std::path::Path::new(&final_dest_folder_string);
    let new_dest_folder_string = format!("{}/{}", state.new_template_path.clone(), state.name.clone());
    let new_dest_folder = std::path::Path::new(&new_dest_folder_string);
    if final_dest_folder.is_dir() {
        match fs::remove_dir_all(final_dest_folder) {
            Ok(_) => (),
            Err(err) => return TemplateBuildResult::GeneralError(err.to_string())
        }
    }
    if new_dest_folder.is_dir() {
        match fs::remove_dir_all(new_dest_folder) {
            Ok(_) => (),
            Err(err) => return TemplateBuildResult::GeneralError(err.to_string())
        }
    }
    match std::fs::read(state.source_dir.clone()) {
        Ok(zip_file_bytes) => {
            match zip_extract::extract(Cursor::new(zip_file_bytes), final_dest_folder, true) {
                Ok(_) => TemplateBuildResult::OkRes((), state),
                Err(err) => TemplateBuildResult::GeneralError(err.to_string())
            }
        },
        Err(err) => TemplateBuildResult::GeneralError(err.to_string())
    }
}

pub fn get_new_template_model(args: Vec<String>) -> TemplateBuildResult<HashMap<ChangeFileInfo,()>, NewTemplateModel, String> {
    check_if_arg_exists("-n".to_string(), args)
        .bind(get_new_template_name_arg)
        .bind(get_template_destination_path)
        .bind(get_template_source_path)
        .bind(unzip_template)
        .bind(change_file_contents)
}
