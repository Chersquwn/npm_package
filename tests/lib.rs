use std::{env::current_dir, vec};

use npm_package::{get_package_info, is_package_exists, Options, PackageInfo, PackageJSON};

#[test]
fn it_should_get_package_info() {
    let pkg_info = get_package_info("consola", Options::default());

    assert_eq!(pkg_info, Some(PackageInfo {
        name: String::from("consola"),
        version: String::from("3.2.3"),
        root_path: current_dir().unwrap().join("node_modules/consola"),
        package_json_path: current_dir().unwrap().join("node_modules/consola/package.json"),
        package_entry: current_dir().unwrap().join("node_modules/consola/dist/index.mjs"),
        package_json: PackageJSON { 
            name: Some(String::from("consola")), 
            version: Some(String::from("3.2.3")), 
            description: Some(String::from("Elegant Console Wrapper")), 
            homepage: None, 
            keywords: Some(vec![String::from("console"), String::from("logger"), String::from("reporter"), String::from("elegant"), String::from("cli"), String::from("universal"), String::from("unified"), String::from("prompt"), String::from("clack"), String::from("format"), String::from("error"), String::from("stacktrace")]), 
            license: Some(String::from("MIT")), 
            private: None, 
            author: None, 
            files: Some(vec![String::from("dist"), String::from("lib"), String::from("*.d.ts")]), 
            r#type: Some(String::from("module")), 
            main: Some(String::from("./lib/index.cjs")), 
            module: Some(String::from("./dist/index.mjs")), 
            exports: None, 
            types: Some(String::from("./dist/index.d.ts")), 
            browser: Some(String::from("./dist/browser.mjs")), 
            bin: None, 
            scripts: Some(serde_json::from_str(r#"{ "build": "unbuild", "lint:fix": "eslint . --fix && prettier -w src examples test", "lint": "eslint . && prettier -c src examples test", "test": "pnpm lint && pnpm vitest run --coverage", "release": "pnpm test && pnpm build && changelogen --release --push && npm publish", "dev": "vitest" }"#).unwrap()), 
            dependencies: None, 
            dev_dependencies: Some(serde_json::from_str(r#"{ "typescript": "^5.1.6", "changelogen": "^0.5.3", "defu": "^6.1.2", "sentencer": "^0.2.1", "eslint-config-unjs": "^0.2.1", "lodash": "^4.17.21", "@vitest/coverage-v8": "^0.32.2", "prettier": "^3.0.0", "unbuild": "^1.2.1", "is-unicode-supported": "^1.3.0", "jiti": "^1.18.2", "@types/node": "^20.3.3", "eslint": "^8.44.0", "@clack/core": "^0.3.2", "vitest": "^0.32.2", "sisteransi": "^1.0.5", "std-env": "^3.3.3", "string-width": "^6.1.0" }"#).unwrap()), 
            peer_dependencies: None, 
            peer_dependencies_meta: None, 
            optional_dependencies: None, 
            engines: Some(serde_json::from_str(r#"{"node": "^14.18.0 || >=16.10.0"}"#).unwrap()) 
        }
    }));
}

#[test]
fn it_should_package_exists() {
    assert!(is_package_exists("magic-string", &Options::default()));
    assert!(!is_package_exists("abc", &Options::default()));
}