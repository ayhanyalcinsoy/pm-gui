use anyhow::Result;
use roxmltree::Document;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub version: String,
    pub release: u32,
    pub license: String,
    pub part_of: String,
    pub package_size: u64,
    pub installed_size: u64,
    pub package_format: String,
    pub distribution: String,
    pub distribution_release: String,
    pub architecture: String,
    pub source: Option<Source>,
    pub history: Vec<PackageHistory>,
    pub dependencies: Vec<Dependency>,
    pub files: Vec<PackageFile>,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub name: String,
    pub homepage: String,
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
    pub version: Option<String>,
    pub release: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct PackageFile {
    pub path: String,
    pub file_type: String,
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
                name: node.attribute("name").unwrap_or("Unknown").to_string(),
                summary: Self::get_text(&node, "Summary").unwrap_or_default(),
                description: Self::get_text(&node, "Description").unwrap_or_default(),
                version: Self::get_text(&node, "Version").unwrap_or_default(),
                release: node.attribute("release")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1),
                license: Self::get_text(&node, "License").unwrap_or_default(),
                part_of: node.attribute("partOf").unwrap_or("Unknown").to_string(),
                package_size: node.attribute("packageSize")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0),
                installed_size: node.attribute("installedSize")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0),
                package_format: node.attribute("packageFormat").unwrap_or("1.0").to_string(),
                distribution: node.attribute("distribution").unwrap_or("PisiLinux").to_string(),
                distribution_release: node.attribute("distributionRelease").unwrap_or("2.0").to_string(),
                architecture: node.attribute("architecture").unwrap_or("x86_64").to_string(),
                source: Self::parse_source(&node),
                history: Self::parse_history(&node),
                dependencies: Self::parse_dependencies(&node),
                files: Self::parse_files(&node),
            };
            packages.push(package);
        }

        println!("Parsed {} packages from XML", packages.len());
        Ok(packages)
    }

    pub fn parse_components(packages: &[PackageInfo]) -> Vec<Component> {
        let mut component_counts: HashMap<String, usize> = HashMap::new();
        
        for package in packages {
            *component_counts.entry(package.part_of.clone()).or_insert(0) += 1;
        }

        let mut components: Vec<Component> = component_counts
            .into_iter()
            .map(|(name, count)| Component {
                name: if name == "Unknown" { "Other".to_string() } else { name },
                package_count: count,
            })
            .collect();

        // "All" component'ını ekle
        let total_packages = packages.len();
        components.insert(0, Component {
            name: "All".to_string(),
            package_count: total_packages,
        });

        components.sort_by(|a, b| a.name.cmp(&b.name));
        components
    }

    fn get_text(node: &roxmltree::Node, tag_name: &str) -> Option<String> {
        node.descendants()
            .find(|n| n.has_tag_name(tag_name))
            .and_then(|n| n.text())
            .map(|s| s.trim().to_string())
    }

    fn parse_source(node: &roxmltree::Node) -> Option<Source> {
        if let Some(source_node) = node.descendants().find(|n| n.has_tag_name("Source")) {
            let name = source_node.attribute("name").unwrap_or("").to_string();
            let homepage = source_node.attribute("homepage").unwrap_or("").to_string();
            
            if !name.is_empty() {
                return Some(Source { name, homepage });
            }
        }
        None
    }

    fn parse_history(node: &roxmltree::Node) -> Vec<PackageHistory> {
        let mut history = Vec::new();
        
        if let Some(history_node) = node.descendants().find(|n| n.has_tag_name("History")) {
            for update_node in history_node.descendants().filter(|n| n.has_tag_name("Update")) {
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
        
        if let Some(deps_node) = node.descendants().find(|n| n.has_tag_name("Dependencies")) {
            for package_node in deps_node.descendants().filter(|n| n.has_tag_name("Package")) {
                let name = package_node.text().unwrap_or("").trim().to_string();
                let version = package_node.attribute("version").map(|s| s.to_string());
                let release = package_node.attribute("release").and_then(|s| s.parse().ok());
                
                if !name.is_empty() {
                    deps.push(Dependency { name, version, release });
                }
            }
        }
        
        deps
    }

    fn parse_files(node: &roxmltree::Node) -> Vec<PackageFile> {
        let mut files = Vec::new();
        
        if let Some(files_node) = node.descendants().find(|n| n.has_tag_name("Files")) {
            for file_node in files_node.descendants().filter(|n| n.has_tag_name("File")) {
                let path = file_node.text().unwrap_or("").trim().to_string();
                let file_type = file_node.attribute("type").unwrap_or("file").to_string();
                
                if !path.is_empty() {
                    files.push(PackageFile { path, file_type });
                }
            }
        }
        
        files
    }

    /// URL'den XML içeriğini indir
    pub fn download_xml(url: &str) -> Result<String> {
        let response = minreq::get(url).send()?;
        if response.status_code == 200 {
            Ok(response.as_str()?.to_string())
        } else {
            Err(anyhow::anyhow!("HTTP {}: Failed to download XML", response.status_code))
        }
    }

    /// Dosyadan XML içeriğini oku (fallback için)
    pub fn read_xml_file(path: &str) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }
}
