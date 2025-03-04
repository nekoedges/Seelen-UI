use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use tauri::{path::BaseDirectory, Manager};

use crate::{error_handler::Result, seelen::get_app_handle};

lazy_static! {
    static ref ICONS: Icons = Icons::instance().expect("Failed to load icons paths");

    pub static ref SEELEN_COMMON: Arc<SeelenCommon> = Arc::new(SeelenCommon::new());

    /**
     * Some UWP apps like WhatsApp are resized after be opened,
     * this list will be used to resize them back after a delay.
     */
    pub static ref FORCE_RETILING_AFTER_ADD: Vec<String> = ["WhatsApp"]
    .iter()
    .map(|x| x.to_string())
    .collect_vec();
}

pub static NATIVE_UI_POPUP_CLASSES: [&str; 3] = [
    "ForegroundStaging",            // Task Switching and Task View
    "XamlExplorerHostIslandWindow", // Task Switching, Task View and other popups
    "ControlCenterWindow",          // Windows 11 right panel with quick settings
];

pub static OVERLAP_BLACK_LIST_BY_EXE: [&str; 6] = [
    "msedgewebview2.exe",
    "SearchHost.exe",
    "StartMenuExperienceHost.exe",
    "ShellExperienceHost.exe",
    "GameBar.exe",      // Windows Xbox Game Bar
    "SnippingTool.exe", // Windows Snipping Tool
];

pub struct Icons {
    missing_app: PathBuf,
}

impl Icons {
    fn instance() -> Result<Self> {
        let handle = get_app_handle();
        Ok(Self {
            missing_app: handle
                .path()
                .resolve("static/icons/missing.png", BaseDirectory::Resource)?,
        })
    }

    pub fn missing_app() -> PathBuf {
        ICONS.missing_app.clone()
    }
}

pub struct SeelenCommon {
    history: PathBuf,
    settings: PathBuf,
    weg_items: PathBuf,
    icons: PathBuf,
    user_themes: PathBuf,
    bundled_themes: PathBuf,
    user_plugins: PathBuf,
    bundled_plugins: PathBuf,
    user_app_configs: PathBuf,
    bundled_app_configs: PathBuf,
    user_layouts: PathBuf,
    bundled_layouts: PathBuf,
    user_placeholders: PathBuf,
    bundled_placeholders: PathBuf,
    widgets: PathBuf,
    bundled_widgets: PathBuf,
    wallpapers: PathBuf,
    profiles: PathBuf,
    bundled_profiles: PathBuf,
}

impl SeelenCommon {
    pub fn new() -> Self {
        let handle = get_app_handle();
        let data_dir = handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir");
        let resource_dir = handle
            .path()
            .resource_dir()
            .expect("Failed to get resource dir");

        Self {
            history: data_dir.join("history"),
            settings: data_dir.join("settings.json"),
            weg_items: data_dir.join("seelenweg_items_v2.yml"),
            icons: data_dir.join("icons"),
            user_themes: data_dir.join("themes"),
            bundled_themes: resource_dir.join("static/themes"),
            user_plugins: data_dir.join("plugins"),
            bundled_plugins: resource_dir.join("static/plugins"),
            user_app_configs: data_dir.join("applications.yml"),
            bundled_app_configs: resource_dir.join("static/apps_templates"),
            user_layouts: data_dir.join("layouts"),
            bundled_layouts: resource_dir.join("static/layouts"),
            user_placeholders: data_dir.join("placeholders"),
            bundled_placeholders: resource_dir.join("static/placeholders"),
            widgets: data_dir.join("widgets"),
            bundled_widgets: resource_dir.join("static/widgets"),
            wallpapers: data_dir.join("wallpapers"),
            profiles: data_dir.join("profiles"),
            bundled_profiles: resource_dir.join("static/profiles"),
        }
    }

    pub fn settings_path(&self) -> &Path {
        &self.settings
    }

    pub fn weg_items_path(&self) -> &Path {
        &self.weg_items
    }

    pub fn history_path(&self) -> &Path {
        &self.history
    }

    pub fn icons_path(&self) -> &Path {
        &self.icons
    }

    pub fn user_themes_path(&self) -> &Path {
        &self.user_themes
    }

    pub fn bundled_themes_path(&self) -> &Path {
        &self.bundled_themes
    }

    pub fn user_plugins_path(&self) -> &Path {
        &self.user_plugins
    }

    pub fn bundled_plugins_path(&self) -> &Path {
        &self.bundled_plugins
    }

    pub fn user_app_configs_path(&self) -> &Path {
        &self.user_app_configs
    }

    pub fn bundled_app_configs_path(&self) -> &Path {
        &self.bundled_app_configs
    }

    pub fn user_layouts_path(&self) -> &Path {
        &self.user_layouts
    }

    pub fn bundled_layouts_path(&self) -> &Path {
        &self.bundled_layouts
    }

    pub fn user_placeholders_path(&self) -> &Path {
        &self.user_placeholders
    }

    pub fn bundled_placeholders_path(&self) -> &Path {
        &self.bundled_placeholders
    }

    pub fn user_widgets_path(&self) -> &Path {
        &self.widgets
    }

    pub fn bundled_widgets_path(&self) -> &Path {
        &self.bundled_widgets
    }

    pub fn wallpapers_path(&self) -> &Path {
        &self.wallpapers
    }

    pub fn user_profiles_path(&self) -> &Path {
        &self.profiles
    }

    pub fn bundled_profiles_path(&self) -> &Path {
        &self.bundled_profiles
    }
}
