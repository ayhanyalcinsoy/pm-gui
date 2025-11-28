use anyhow::Result;
use roxmltree::Document;

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub version: String,
    pub license: String,
    pub part_of: String,
    pub isa: String,
    pub package_size: u64,
    pub installed_size: u64,
    pub history: Vec<PackageHistory>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone)]
pub struct PackageHistory {
    pub version: String,
    pub release: u32,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub package_count: usize,
}

pub struct XmlParser;

impl XmlParser {
    pub fn parse_pisi_index(xml_content: &str) -> Result<Vec<PackageInfo>> {
        let doc = Document::parse(xml_content)?;
        let mut packages = Vec::new();

        for node in doc.descendants().filter(|n| n.has_tag_name("Package")) {
            let package = PackageInfo {
                name: node.attribute("name").unwrap_or("").to_string(),
                summary: Self::get_text(&node, "Summary").unwrap_or_default(),
                description: Self::get_text(&node, "Description").unwrap_or_default(),
                version: Self::get_text(&node, "Version").unwrap_or_default(),
                license: Self::get_text(&node, "License").unwrap_or_default(),
                part_of: node.attribute("partOf").unwrap_or("").to_string(),
                isa: node.attribute("isa").unwrap_or("").to_string(),
                package_size: node.attribute("packageSize")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
                installed_size: node.attribute("installedSize")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
                history: Self::parse_history(&node),
                dependencies: Self::parse_dependencies(&node),
            };
            packages.push(package);
        }

        Ok(packages)
    }

    pub fn parse_components(packages: &[PackageInfo]) -> Vec<Component> {
        use std::collections::HashMap;

        let mut component_counts: HashMap<String, usize> = HashMap::new();

        for package in packages {
            *component_counts.entry(package.part_of.clone()).or_insert(0) += 1;
        }

        let mut components: Vec<Component> = component_counts
        .into_iter()
        .map(|(name, count)| Component {
            name,
            package_count: count,
        })
        .collect();

        components.sort_by(|a, b| a.name.cmp(&b.name));
        components
    }

    fn get_text(node: &roxmltree::Node, tag_name: &str) -> Option<String> {
        node.descendants()
        .find(|n| n.has_tag_name(tag_name))
        .and_then(|n| n.text())
        .map(|s| s.to_string())
    }

    fn parse_history(node: &roxmltree::Node) -> Vec<PackageHistory> {
        let mut history = Vec::new();

        for history_node in node.descendants().filter(|n| n.has_tag_name("History")) {
            if let Some(update_node) = history_node.descendants().find(|n| n.has_tag_name("Update")) {
                let version = update_node.attribute("version").unwrap_or("").to_string();
                let release = update_node.attribute("release")
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);
                let date = update_node.attribute("date").unwrap_or("").to_string();

                history.push(PackageHistory { version, release, date });
            }
        }

        history
    }

    fn parse_dependencies(node: &roxmltree::Node) -> Vec<Dependency> {
        let mut deps = Vec::new();

        for deps_node in node.descendants().filter(|n| n.has_tag_name("Dependencies")) {
            for dep_node in deps_node.descendants().filter(|n| n.has_tag_name("Dependency")) {
                let name = dep_node.text().unwrap_or("").to_string();
                let version = dep_node.attribute("version").unwrap_or("").to_string();

                deps.push(Dependency { name, version });
            }
        }

        deps
    }
}
