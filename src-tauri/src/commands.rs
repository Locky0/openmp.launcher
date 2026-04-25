use crate::{errors::LauncherError, helpers, injector, samp};
use log::{error, info, warn};
use md5::compute;
use sevenz_rust::decompress_file;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tauri::api::path::resource_dir;

const D3DX9_RUNTIME_DLL: &str = "d3dx9_25.dll";

#[tauri::command]
pub async fn inject(
    name: &str,
    ip: &str,
    port: i32,
    exe: &str,
    dll: &str,
    omp_file: &str,
    password: &str,
    custom_game_exe: &str,
) -> std::result::Result<(), String> {
    let actual_omp_file = if *crate::NO_OMP_FLAG.lock().unwrap() {
        ""
    } else {
        omp_file
    };

    match injector::run_samp(
        name,
        ip,
        port,
        exe,
        dll,
        actual_omp_file,
        password,
        custom_game_exe,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("{}", e);
            match e {
                LauncherError::AccessDenied(_) => {
                    return Err("need_admin".to_string());
                }
                _ => return Err(e.to_string()),
            }
        }
    }
}

#[tauri::command]
pub fn get_gtasa_path_from_samp() -> String {
    samp::get_gtasa_path()
}

#[tauri::command]
pub fn get_nickname_from_samp() -> String {
    samp::get_nickname()
}

#[tauri::command]
pub fn rerun_as_admin() -> std::result::Result<String, String> {
    let exe_path = std::env::current_exe()
        .map_err(|_| "Failed to get current executable path".to_string())?
        .into_os_string()
        .into_string()
        .map_err(|_| "Failed to convert path to string".to_string())?;

    runas::Command::new(exe_path)
        .arg("")
        .status()
        .map_err(|_| "Failed to restart as administrator".to_string())?;

    Ok("SUCCESS".to_string())
}

#[tauri::command]
pub fn get_samp_favorite_list() -> String {
    samp::get_samp_favorite_list()
}

#[tauri::command]
pub fn resolve_hostname(hostname: String) -> std::result::Result<String, String> {
    use std::net::{IpAddr, ToSocketAddrs};

    if hostname.is_empty() {
        return Err("Hostname cannot be empty".to_string());
    }

    let addr = format!("{}:80", hostname);
    let addrs = addr
        .to_socket_addrs()
        .map_err(|e| format!("Failed to resolve hostname '{}': {}", hostname, e))?;

    for ip in addrs {
        if let IpAddr::V4(ipv4) = ip.ip() {
            return Ok(ipv4.to_string());
        }
    }

    Err(format!("No IPv4 address found for hostname '{}'", hostname))
}

#[tauri::command]
pub fn is_process_alive(pid: u32) -> bool {
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

    unsafe {
        let handle: HANDLE = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle != 0 {
            CloseHandle(handle);
            true
        } else {
            false
        }
    }
}

#[tauri::command]
pub fn get_checksum_of_files(list: Vec<String>) -> std::result::Result<Vec<String>, String> {
    let mut result = Vec::new();

    for file in list {
        let mut f =
            File::open(&file).map_err(|e| format!("Failed to open file '{}': {}", file, e))?;

        let mut contents = Vec::new();
        f.read_to_end(&mut contents)
            .map_err(|e| format!("Failed to read file '{}': {}", file, e))?;

        let digest = compute(&contents);
        let checksum_entry = format!("{}|{:x}", file, digest);
        result.push(checksum_entry);
    }

    Ok(result)
}

#[tauri::command]
pub fn extract_7z(path: String, output_path: String) -> std::result::Result<(), String> {
    decompress_file(&path, &output_path)
        .map_err(|e| format!("Failed to extract archive '{}': {}", path, e))
}

#[tauri::command]
pub fn copy_files_to_gtasa(
    app_handle: tauri::AppHandle,
    src: String,
    gtasa_dir: String,
) -> std::result::Result<(), String> {
    let result = (|| -> crate::errors::Result<()> {
        helpers::copy_files(&src, &gtasa_dir)?;

        let support_file = resolve_support_file(&app_handle, D3DX9_RUNTIME_DLL).ok_or_else(|| {
            LauncherError::InternalError(format!(
                "Missing bundled runtime dependency '{}'",
                D3DX9_RUNTIME_DLL
            ))
        })?;

        let destination = Path::new(&gtasa_dir).join(D3DX9_RUNTIME_DLL);
        helpers::copy_file(&support_file, destination)?;
        Ok(())
    })();

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("{}", e);
            match e {
                LauncherError::AccessDenied(_) => {
                    return Err("need_admin".to_string());
                }
                _ => return Err(e.to_string()),
            }
        }
    }
}

fn resolve_support_file(app_handle: &tauri::AppHandle, file_name: &str) -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if let Some(resource_path) = resource_dir(&app_handle.config()) {
        candidates.push(resource_path.join(file_name));
        candidates.push(resource_path.join("extra").join(file_name));
    }

    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            candidates.push(parent.join(file_name));
            candidates.push(parent.join("resources").join(file_name));
        }
    }

    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("extra").join(file_name));

    candidates.into_iter().find(|candidate| candidate.exists())
}

#[tauri::command]
pub fn log_info(msg: &str) -> () {
    info!("Frontend info: {}", msg);
}

#[tauri::command]
pub fn log_warn(msg: &str) -> () {
    warn!("Frontend warning: {}", msg);
}

#[tauri::command]
pub fn log_error(msg: &str) -> () {
    error!("Frontend error: {}", msg);
}
