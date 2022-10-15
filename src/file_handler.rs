use std::fs;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

pub fn get_file_name(args: &Vec<String>) -> Result<String, String> {
    if args.len() >= 2 {
        let file_name = &args[1];

        if fs::metadata(file_name).is_ok() {
            return Ok(file_name.to_string());
        }

        return Err(format!("filename {} does not exists!", file_name));
    }

    Err("Missing filename argument!".to_string())
}

pub fn parse_file(file_name: &str) -> Vec<Yaml> {
    let file_contents =
        fs::read_to_string(file_name).expect("Should have been able to read the file");
    YamlLoader::load_from_str(&file_contents).unwrap()
}
