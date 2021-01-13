use lazy_static::lazy_static;
use ElemType::*;
use std::collections::HashMap;
use crate::globals::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ElemType {
    Hitcircle,
    Hitcircleoverlay,
    Cursor,
    Cursortrail
}

lazy_static! {
    static ref ELEM_TYPES: HashMap<&'static str, ElemType> = {
        let mut m = HashMap::new();
        m.insert("hitcircle", Hitcircle);
        m.insert("hitcircleoverlay", Hitcircleoverlay);
        m.insert("cursor", Cursor);
        m.insert("cursortrail", Cursortrail);

        m
    };
}

lazy_static! {
    static ref FILE_NAME: HashMap<ElemType, &'static str> = {
        let mut m = HashMap::new();
        m.insert(Hitcircle, "hitcircle.png");
        m.insert(Hitcircleoverlay, "hitcircleoverlay.png");
        m.insert(Cursor, "cursor.png");
        m.insert(Cursortrail, "cursortrail.png");

        m
    };
}

impl ElemType {
    pub fn get_file_name(self) -> &'static str {
        FILE_NAME[&self]
    }
    
    pub fn from_name(name: &str) -> Option<ElemType> {
        ELEM_TYPES.get(name).copied()
    }
}

pub fn get_file_base64(skin_name: &str, file: &str) -> Option<String> {
    let mut path = std::path::PathBuf::from(SKIN_DIR);
    path.push(skin_name);
    path.push(file);

    if path.exists() {
        let v = std::fs::read(path).expect("Error reading");
        return Some(base64::encode(v));
    }
    None
}

