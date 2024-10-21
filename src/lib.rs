use std::{
    collections::HashMap,
    env::current_dir,
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use serde::{Deserialize, Serialize};
use validate_npm_package_name::validate;

#[derive(Default)]
pub struct Options<'a> {
    pub cwd: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum BinType {
    String(String),
    HashMap(HashMap<String, String>),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ExportValue {
    String(String),
    HashMap(HashMap<String, String>),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct PackageJSON {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<String>,
    pub private: Option<bool>,
    pub author: Option<String>,
    pub files: Option<Vec<String>>,
    pub r#type: Option<String>,
    pub main: Option<String>,
    pub module: Option<String>,
    #[serde(flatten)]
    pub exports: Option<HashMap<String, ExportValue>>,
    pub types: Option<String>,
    pub browser: Option<String>,
    #[serde(flatten)]
    pub bin: Option<BinType>,
    pub scripts: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependenciesMeta")]
    pub peer_dependencies_meta: Option<HashMap<String, String>>,
    #[serde(rename = "optionalDependencies")]
    pub optional_dependencies: Option<HashMap<String, String>>,
    pub engines: Option<HashMap<String, String>>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub root_path: PathBuf,
    pub package_json_path: PathBuf,
    pub package_entry: PathBuf,
    pub package_json: PackageJSON,
}

static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| current_dir().unwrap());

pub fn get_package_info(name: &str, options: Options) -> Option<PackageInfo> {
    let validate_result = validate(&name.to_string());

    if !validate_result.valid_for_new_packages && !validate_result.valid_for_old_packages {
        return None;
    }

    let package_json_path = get_package_json_path(name, &options);

    package_json_path.as_ref()?;

    let package_json = get_package_json(package_json_path.as_ref().unwrap().as_path());

    let package_entry = get_package_entry(package_json_path.as_ref().unwrap().as_path());

    if package_json.is_none() || package_entry.is_none() {
        return None;
    }

    Some(PackageInfo {
        name: name.to_string(),
        version: package_json.clone().unwrap().version?,
        root_path: package_json_path
            .as_ref()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf(),
        package_entry: package_entry.unwrap(),
        package_json_path: package_json_path.unwrap(),
        package_json: package_json.unwrap(),
    })
}

pub fn get_package_json_path(name: &str, options: &Options) -> Option<PathBuf> {
    let id = format!("node_modules/{}/package.json", name);
    let pkg_json_path = resolve(&id, options);

    match pkg_json_path {
        Ok(pkg_json_path) => Some(pkg_json_path),
        Err(_) => None,
    }
}

pub fn is_package_exists(name: &str, options: &Options) -> bool {
    let pkg_json_path = get_package_json_path(name, options);

    pkg_json_path.is_some()
}

fn get_package_json(path: &Path) -> Option<PackageJSON> {
    let json = fs::read_to_string(path);

    match json {
        Ok(json) => Some(serde_json::from_str(&json).unwrap()),
        Err(_) => None,
    }
}

fn get_package_entry(path: &Path) -> Option<PathBuf> {
    let pkg_json = get_package_json(path);

    if let Some(pkg_json) = pkg_json {
        let root = path.parent().unwrap();

        if pkg_json.r#type.is_some_and(|t| t == "module") && pkg_json.module.is_some() {
            Some(root.join(pkg_json.module.unwrap()))
        } else if pkg_json
            .exports
            .as_ref()
            .is_some_and(|exports| exports.contains_key("."))
        {
            let exports = pkg_json.exports.unwrap();
            let root_entry = exports.get(".").unwrap();

            match root_entry {
                ExportValue::String(root_entry) => Some(root.join(root_entry)),
                ExportValue::HashMap(root_entry) => {
                    if root_entry.get("import").is_some() {
                        Some(root.join(root_entry.get("import").unwrap()))
                    } else {
                        Some(root.join(root_entry.get("require").unwrap()))
                    }
                }
            }
        } else {
            Some(root.join(pkg_json.main.as_ref().unwrap()))
        }
    } else {
        None
    }
}

fn resolve(name: &str, options: &Options) -> Result<PathBuf, String> {
    let cwd = match options.cwd {
        Some(cwd) => Path::new(cwd),
        None => CURRENT_DIR.as_path(),
    };
    let id = cwd.join(name);

    if id.try_exists().unwrap() {
        Ok(id)
    } else {
        Err(format!("Cannot find module {} from {:?}", name, id))
    }
}
