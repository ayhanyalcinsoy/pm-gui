use std::process::Command;
use anyhow::Result;

pub struct PackageManager;

impl PackageManager {
    pub fn install_package(package_name: &str) -> Result<()> {
        let output = Command::new("pisi")
        .args(["install", "-y", package_name])
        .output()?;

        if output.status.success() {
            println!("Package {} installed successfully", package_name);
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to install package: {}", error))
        }
    }

    pub fn remove_package(package_name: &str) -> Result<()> {
        let output = Command::new("pisi")
        .args(["remove", "-y", package_name])
        .output()?;

        if output.status.success() {
            println!("Package {} removed successfully", package_name);
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to remove package: {}", error))
        }
    }

    pub fn update_package(package_name: &str) -> Result<()> {
        let output = Command::new("pisi")
        .args(["update", "-y", package_name])
        .output()?;

        if output.status.success() {
            println!("Package {} updated successfully", package_name);
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to update package: {}", error))
        }
    }

    pub fn update_all_packages() -> Result<()> {
        let output = Command::new("pisi")
        .args(["update", "-y"])
        .output()?;

        if output.status.success() {
            println!("All packages updated successfully");
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to update packages: {}", error))
        }
    }

    pub fn search_packages(query: &str) -> Result<Vec<String>> {
        let output = Command::new("pisi")
        .args(["search", query])
        .output()?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let packages: Vec<String> = output_str
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
            Ok(packages)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Search failed: {}", error))
        }
    }

    pub fn get_installed_packages() -> Result<Vec<String>> {
        let output = Command::new("pisi")
        .args(["list-installed"])
        .output()?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let packages: Vec<String> = output_str
            .lines()
            .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
            .collect();
            Ok(packages)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to get installed packages: {}", error))
        }
    }
}
