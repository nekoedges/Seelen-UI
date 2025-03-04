use std::collections::HashMap;
use std::path::PathBuf;

use tauri::{Builder, WebviewWindow, Wry};
use tauri_plugin_shell::ShellExt;

use crate::error_handler::Result;
use crate::hook::HookManager;
use crate::modules::input::Keyboard;
use crate::modules::virtual_desk::get_vd_manager;
use crate::seelen::{get_app_handle, Seelen};
use crate::seelen_rofi::handler::*;
use crate::seelen_weg::handler::*;
use crate::seelen_weg::icon_extractor::{
    extract_and_save_icon_from_file, extract_and_save_icon_umid,
};
use crate::seelen_wm_v2::handler::*;
use crate::state::infrastructure::*;
use crate::system::brightness::*;
use crate::utils::{
    is_running_as_appx_package, is_virtual_desktop_supported as virtual_desktop_supported,
};
use crate::windows_api::WindowsApi;
use crate::winevent::{SyntheticFullscreenData, WinEvent};
use crate::{log_error, utils};

use crate::modules::media::infrastructure::*;
use crate::modules::monitors::infrastructure::*;
use crate::modules::network::infrastructure::*;
use crate::modules::notifications::infrastructure::*;
use crate::modules::power::infrastructure::*;
use crate::modules::system_settings::infrastructure::*;
use crate::modules::tray::infrastructure::*;

#[tauri::command(async)]
fn select_file_on_explorer(path: String) -> Result<()> {
    get_app_handle()
        .shell()
        .command("explorer")
        .args(["/select,", &path])
        .spawn()?;
    Ok(())
}

#[tauri::command(async)]
fn open_file(path: String) -> Result<()> {
    get_app_handle()
        .shell()
        .command("explorer")
        .arg(&path)
        .spawn()?;
    Ok(())
}

#[tauri::command(async)]
async fn run_as_admin(program: String, args: Vec<String>) -> Result<()> {
    let command = if args.is_empty() {
        format!("Start-Process '{}' -Verb runAs", program)
    } else {
        format!(
            "Start-Process '{}' -Verb runAs -ArgumentList '{}'",
            program,
            args.join(" ")
        )
    };
    get_app_handle()
        .shell()
        .command("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &command,
        ])
        .status()
        .await?;
    Ok(())
}

#[tauri::command(async)]
async fn run(program: String, args: Vec<String>) -> Result<()> {
    // we create a link file to trick with explorer into a separated process
    // and without elevation in case Seelen UI was running as admin
    // this could take some delay like is creating a file but just are some milliseconds
    // and this exposed funtion is intended to just run certain times
    let lnk_file = WindowsApi::create_temp_shortcut(&program, &args.join(" "))?;
    get_app_handle()
        .shell()
        .command("explorer")
        .arg(&lnk_file)
        .status()
        .await?;
    std::fs::remove_file(&lnk_file)?;
    Ok(())
}

#[tauri::command(async)]
fn is_dev_mode() -> bool {
    tauri::is_dev()
}

#[tauri::command(async)]
fn is_appx_package() -> bool {
    is_running_as_appx_package()
}

#[tauri::command(async)]
pub fn get_user_envs() -> HashMap<String, String> {
    std::env::vars().collect::<HashMap<String, String>>()
}

// https://docs.rs/tauri/latest/tauri/window/struct.WindowBuilder.html#known-issues
// https://github.com/tauri-apps/wry/issues/583
#[tauri::command(async)]
fn show_app_settings() {
    log_error!(Seelen::show_settings());
}

#[tauri::command(async)]
async fn set_auto_start(enabled: bool) -> Result<()> {
    Seelen::set_auto_start(enabled).await
}

#[tauri::command(async)]
async fn get_auto_start_status() -> Result<bool> {
    Seelen::is_auto_start_enabled().await
}

#[tauri::command(async)]
fn switch_workspace(idx: usize) -> Result<()> {
    get_vd_manager().switch_to(idx)
}

#[tauri::command(async)]
fn send_keys(keys: String) -> Result<()> {
    Keyboard::new().send_keys(&keys)
}

#[tauri::command(async)]
fn get_icon(path: String) -> Option<PathBuf> {
    if path.starts_with("shell:AppsFolder") {
        let umid = path.replace("shell:AppsFolder\\", "");
        return extract_and_save_icon_umid(&umid).ok();
    }
    extract_and_save_icon_from_file(&path).ok()
}

#[tauri::command(async)]
fn is_virtual_desktop_supported() -> bool {
    virtual_desktop_supported()
}

#[tauri::command(async)]
fn simulate_fullscreen(webview: WebviewWindow<tauri::Wry>, value: bool) -> Result<()> {
    let handle = webview.hwnd()?;
    let monitor = WindowsApi::monitor_from_window(handle);
    let event = if value {
        WinEvent::SyntheticFullscreenStart(SyntheticFullscreenData { handle, monitor })
    } else {
        WinEvent::SyntheticFullscreenEnd(SyntheticFullscreenData { handle, monitor })
    };
    HookManager::emit_event(event, handle);
    Ok(())
}

#[tauri::command(async)]
async fn check_for_updates() -> Result<bool> {
    Ok(utils::updater::check_for_updates().await?.is_some())
}

#[tauri::command(async)]
async fn install_last_available_update() -> Result<()> {
    let update = utils::updater::check_for_updates()
        .await?
        .ok_or("There is no update available")?;
    utils::updater::trace_update_intallation(update).await?;
    get_app_handle().restart();
    #[allow(unreachable_code)]
    Ok(())
}

pub fn register_invoke_handler(app_builder: Builder<Wry>) -> Builder<Wry> {
    use crate::modules::language;

    app_builder.invoke_handler(tauri::generate_handler![
        // General
        run,
        is_dev_mode,
        is_appx_package,
        open_file,
        run_as_admin,
        select_file_on_explorer,
        is_virtual_desktop_supported,
        get_user_envs,
        show_app_settings,
        switch_workspace,
        send_keys,
        get_icon,
        get_system_colors,
        simulate_fullscreen,
        check_for_updates,
        install_last_available_update,
        get_connected_monitors,
        // Seelen Settings
        set_auto_start,
        get_auto_start_status,
        state_get_icon_packs,
        state_get_themes,
        state_get_placeholders,
        state_get_layouts,
        state_get_weg_items,
        state_get_settings,
        state_get_default_settings,
        state_get_default_monitor_settings,
        state_write_settings,
        state_write_weg_items,
        state_get_specific_apps_configurations,
        state_get_wallpaper,
        state_set_wallpaper,
        state_get_history,
        state_get_plugins,
        state_get_widgets,
        state_get_profiles,
        // Media
        media_prev,
        media_toggle_play_pause,
        media_next,
        set_volume_level,
        media_toggle_mute,
        media_set_default_device,
        // Brightness
        get_main_monitor_brightness,
        set_main_monitor_brightness,
        // Power
        log_out,
        suspend,
        restart,
        shutdown,
        // SeelenWeg
        weg_get_items_for_widget,
        weg_close_app,
        weg_kill_app,
        weg_toggle_window_state,
        weg_request_update_previews,
        weg_pin_item,
        // Windows Manager
        set_window_position,
        request_focus,
        // App Launcher
        launcher_get_apps,
        // tray icons
        temp_get_by_event_tray_info,
        on_click_tray_icon,
        on_context_menu_tray_icon,
        // network
        wlan_get_profiles,
        wlan_start_scanning,
        wlan_stop_scanning,
        wlan_connect,
        wlan_disconnect,
        // notifications
        notifications_close,
        notifications_close_all,
        language::get_system_languages,
    ])
}
