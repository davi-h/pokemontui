use std::process::Command;
use std::collections::HashMap;

fn main() {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1"])
        .output()
        .expect("failed to run cargo metadata");

    let json = String::from_utf8(output.stdout).unwrap();
    let metadata: serde_json::Value = serde_json::from_str(&json).unwrap();

    let packages = metadata["packages"].as_array().unwrap();

    let mut deps: HashMap<String, Vec<String>> = HashMap::new();

    for pkg in packages {
        let name = pkg["name"].as_str().unwrap().to_string();

        let mut list = Vec::new();
        for dep in pkg["dependencies"].as_array().unwrap() {
            list.push(dep["name"].as_str().unwrap().to_string());
        }

        deps.insert(name, list);
    }

    // rules
    let rules: HashMap<&str, Vec<&str>> = HashMap::from([
        ("domain", vec![]),
        ("contracts", vec!["domain"]),
        ("application", vec!["domain", "contracts"]),
        ("engine", vec!["domain", "contracts"]),
        ("infrastructure", vec!["contracts"]),
        ("interface", vec!["application"]),
        ("app", vec!["application", "interface", "infrastructure", "engine"]),
    ]);

    let mut failed = false;

    for (crate_name, crate_deps) in &deps {
        if let Some(allowed) = rules.get(crate_name.as_str()) {
            for dep in crate_deps {
                if !allowed.contains(&dep.as_str()) && dep != "std" {
                    println!("❌ {} depends on forbidden crate {}", crate_name, dep);
                    failed = true;
                }
            }
        }
    }

    if failed {
        std::process::exit(1);
    } else {
        println!("✅ Architecture valid");
    }
}