use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;
use crate::identifier::Identifiers;
use crate::matcher::SourceMatcherEnum;
use crate::processor::SourceProcessor;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) identifiers: Identifiers,
    pub(crate) rules: HashMap<String, ConfigRule>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ConfigRule {
    pub(crate) matcher: Box<SourceMatcherEnum>,
    pub(crate) processors: Vec<SourceProcessor>,
}

pub(crate) trait ReadFromFile {
    fn load_from_json(file_path: &Path) -> Config;
}

impl ReadFromFile for Config {
    fn load_from_json(file_path: &Path) -> Config {
        let mut file = File::open(file_path).expect("open config file failed");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("read config file failed");
        let data: Value = serde_json::from_str(file_content.as_str()).expect("parse config file failed");
        println!("{:?}", data);
        // 你可以使用下标操作符来访问json对象的Launcher字段
        // 你可以使用clone方法来复制一个值类型，或者使用take方法来转移所有权
        let launcher = data["Launcher"].clone();
        // 你可以使用serde_json::from_value函数来把它转换成Config类型
        let config: Config = serde_json::from_value(launcher).expect("convert to Config failed");

        config


    }
}