use serde::{Deserialize, Serialize};

// SettingsTab'ı basitleştiriyoruz
#[derive(Debug, Clone, PartialEq)]
pub enum SettingsTab {
    General,
    Cache,
    Repos,
    Proxy,
}

impl Default for SettingsTab {
    fn default() -> Self {
        Self::General
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageListSettings {
    pub show_only_desktop_apps: bool,
    pub show_components_info: bool,
    pub show_package_type_labels: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettings {
    pub check_updates: bool,
    pub update_check_interval: u32, // minutes
    pub install_updates_automatically: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSettings {
    pub use_disk_cache: bool,
    pub cache_size_mb: u32,
    pub cache_directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxySettings {
    pub use_proxy: bool,
    pub http_proxy: String,
    pub https_proxy: String,
    pub ftp_proxy: String,
    pub domain: String,
    pub username: String,
    pub password: String,
    pub use_http_proxy_for_all: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub enabled: bool,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub package_list: PackageListSettings,
    pub update: UpdateSettings,
    pub cache: CacheSettings,
    pub proxy: ProxySettings,
    pub repositories: Vec<Repository>,
    pub system_tray_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            package_list: PackageListSettings {
                show_only_desktop_apps: false,
                show_components_info: true,
                show_package_type_labels: true,
            },
            update: UpdateSettings {
                check_updates: true,
                update_check_interval: 60,
                install_updates_automatically: false,
            },
            cache: CacheSettings {
                use_disk_cache: true,
                cache_size_mb: 1024,
                cache_directory: "/var/cache/pisi/packages".to_string(),
            },
            proxy: ProxySettings {
                use_proxy: false,
                http_proxy: String::new(),
                https_proxy: String::new(),
                ftp_proxy: String::new(),
                domain: String::new(),
                username: String::new(),
                password: String::new(),
                use_http_proxy_for_all: false,
            },
            repositories: vec![
                Repository {
                    enabled: true,
                    name: "Stable".to_string(),
                    url: "https://stable2.pisilinux.org/pisi-index.xml.xz".to_string(),
                },
                Repository {
                    enabled: true,
                    name: "Contrib".to_string(),
                    url: "https://contrib.pisilinux.org/pisi-index.xml.xz".to_string(),
                },
            ],
            system_tray_enabled: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsModalState {
    pub current_tab: SettingsTab,
}

impl Default for SettingsModalState {
    fn default() -> Self {
        Self {
            current_tab: SettingsTab::General,
        }
    }
}
