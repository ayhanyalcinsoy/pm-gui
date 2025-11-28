use anyhow::Result;
use std::process::Command;

pub struct RepositoryManager;

impl RepositoryManager {
    pub fn add_repository(name: &str, url: &str) -> Result<()> {
        let output = Command::new("pisi")
        .args(["add-repo", name, url])
        .output()?;

        if output.status.success() {
            println!("Repository {} added successfully", name);
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to add repository: {}", error))
        }
    }

    pub fn remove_repository(name: &str) -> Result<()> {
        let output = Command::new("pisi")
        .args(["remove-repo", name])
        .output()?;

        if output.status.success() {
            println!("Repository {} removed successfully", name);
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to remove repository: {}", error))
        }
    }

    pub fn enable_repository(name: &str) -> Result<()> {
        let output = Command::new("pisi")
        .args(["enable-repo", name])
        .output()?;

        if output.status.success() {
            println!("Repository {} enabled successfully", name);
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to enable repository: {}", error))
        }
    }

    pub fn disable_repository(name: &str) -> Result<()> {
        let output = Command::new("pisi")
        .args(["disable-repo", name])
        .output()?;

        if output.status.success() {
            println!("Repository {} disabled successfully", name);
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to disable repository: {}", error))
        }
    }

    pub fn update_repositories() -> Result<()> {
        let output = Command::new("pisi")
        .args(["update-repo"])
        .output()?;

        if output.status.success() {
            println!("Repositories updated successfully");
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to update repositories: {}", error))
        }
    }

    pub fn list_repositories() -> Result<Vec<(String, bool)>> {
        let output = Command::new("pisi")
        .args(["list-repo"])
        .output()?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let repos: Vec<(String, bool)> = output_str
            .lines()
            .skip(1) // Skip header
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[0].to_string();
                    let enabled = parts[1] == "enabled";
                    Some((name, enabled))
                } else {
                    None
                }
            })
            .collect();
            Ok(repos)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to list repositories: {}", error))
        }
    }
}
