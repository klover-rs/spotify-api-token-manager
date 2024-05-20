use std::fs;


pub fn stm_data_folder() -> String {
    let mut stm_data_folder = String::new();

    if let Some(home_dir) = dirs::home_dir() {
        let folder_path = home_dir.join("stm_data");

        if !folder_path.exists() {
            if let Err(err) = fs::create_dir(&folder_path) {
                eprintln!("failed the create folder: {}", err);
            }
        }
        let path_string = folder_path
            .into_os_string()
            .into_string()
            .unwrap_or_else(|os_string| os_string.to_string_lossy().into_owned());

        stm_data_folder.push_str(&path_string);

    } else {
        println!("unable to get the document directory");
    }

    

    stm_data_folder
}

