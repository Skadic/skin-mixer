use std::collections::{HashMap};

use crate::{elem_type::{ElemType, get_file_base64}, globals::SKIN_DIR};


pub fn skin_elements_by_type(elem_type: ElemType) -> std::io::Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    let file_name = ElemType::get_file_name(elem_type);

    for skin_dir in std::fs::read_dir(SKIN_DIR)? {
        let skin_dir = skin_dir?;
        if skin_dir.metadata().unwrap().is_dir() {
            let skin_name = skin_dir.file_name().to_string_lossy().into_owned();
            let img_as_base64 = get_file_base64(&skin_name, file_name);

            if let Some(data) = img_as_base64 {
                map.insert(skin_name, data);
            }
        }
    }

    Ok(map)
}
