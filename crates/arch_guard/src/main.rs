use std::{collections::HashMap, process::Command};

fn main() {
    println!("🔎 Running architecture guards...\n");

    let metadata = cargo_metadata();
    let workspace = workspace_members(&metadata);
    let deps = extract_workspace_deps(&metadata, &workspace);
    let rules = layer_rules();

    let mut failed = false;

    for (krate, dependencies) in &deps {
        for dep in dependencies {
            // regra 1 — violação de camada
            if violates_layer(krate, dep, &rules) {
                violation("layer violation", krate, dep);
                failed = true;
            }

            // regra 2 — dependência proibida
            if forbidden_dependency(krate, dep) {
                violation("forbidden dependency", krate, dep);
                failed = true;
            }

            // regra 3 — domínio deve ser puro
            if domain_must_be_pure(krate, dep) {
                violation("domain purity violation", krate, dep);
                failed = true;
            }
        }
    }

    if failed {
        println!("\n❌ Architecture violations detected.");
        std::process::exit(1);
    } else {
        println!("✅ Architecture OK");
    }
}

//
// ─────────────────────────────────────────────
// METADATA
// ─────────────────────────────────────────────
//

fn cargo_metadata() -> serde_json::Value {
    let out = Command::new("cargo")
        .args(["metadata", "--format-version=1"])
        .output()
        .expect("failed to run cargo metadata");

    serde_json::from_slice(&out.stdout).expect("invalid cargo metadata JSON")
}

fn workspace_members(meta: &serde_json::Value) -> Vec<String> {
    let mut members = Vec::new();

    if let Some(pkgs) = meta["packages"].as_array() {
        for pkg in pkgs {
            if pkg["source"].is_null() {
                members.push(pkg["name"].as_str().unwrap().to_string());
            }
        }
    }

    members
}

fn extract_workspace_deps(
    meta: &serde_json::Value,
    workspace: &[String],
) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for pkg in meta["packages"].as_array().unwrap() {
        let name = pkg["name"].as_str().unwrap().to_string();

        if !workspace.iter().any(|w| w == &name) {
            continue;
        }

        let deps = pkg["dependencies"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["name"].as_str().unwrap().to_string())
            .filter(|dep| workspace.iter().any(|w| w == dep))
            .collect();

        map.insert(name, deps);
    }

    map
}

//
// ─────────────────────────────────────────────
// LAYER RULES
// ─────────────────────────────────────────────
//

fn layer_rules() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("domain", vec![]),
        ("contracts", vec![]),
        ("application", vec!["domain", "contracts", "engine"]),
        ("engine", vec!["domain", "contracts"]),
        ("infrastructure", vec!["contracts"]),
        ("interface", vec!["application", "contracts"]),
        (
            "app",
            vec![
                "application",
                "interface",
                "infrastructure",
                "engine",
                "contracts",
                "domain",
                "shared",
            ],
        ),
    ])
}

fn violates_layer(
    krate: &str,
    dep: &str,
    rules: &HashMap<&str, Vec<&str>>,
) -> bool {
    if let Some(allowed) = rules.get(krate) {
        !allowed.contains(&dep)
    } else {
        false
    }
}

//
// ─────────────────────────────────────────────
// CUSTOM RULES
// ─────────────────────────────────────────────
//

fn forbidden_dependency(krate: &str, dep: &str) -> bool {
    matches!(
        (krate, dep),
        // domain nunca pode depender disso
        ("domain", "tokio")
            | ("domain", "reqwest")
            | ("domain", "serde_json")

        // application não pode acessar infra diretamente
            | ("application", "reqwest")
            | ("application", "sqlx")

        // engine deve ser determinístico
            | ("engine", "tokio")
    )
}

fn domain_must_be_pure(krate: &str, dep: &str) -> bool {
    if krate != "domain" {
        return false;
    }

    matches!(
        dep,
        "tokio" | "reqwest" | "sqlx" | "redis" | "mongodb"
    )
}

//
// ─────────────────────────────────────────────
// UTILS
// ─────────────────────────────────────────────
//

fn violation(rule: &str, krate: &str, dep: &str) {
    println!("❌ [{}] {} -> {}", rule, krate, dep);
}
