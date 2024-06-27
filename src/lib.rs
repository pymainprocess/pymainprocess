use pyo3::prelude::*;
use std::process::{Command, Stdio};
use pyo3::exceptions::PyException;
use pyo3::create_exception;
use which::which;
use std::ffi::CString;
use std::fs;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use dotenv::dotenv;
use std::io::BufRead;
use sys_info;
use std::env;
use pytype::PyInt;
use tempfile::{Builder, TempDir, NamedTempFile};

#[cfg(any(target_os = "unix", target_os = "linux", target_os = "macos"))]
use std::os::unix::fs::MetadataExt;

create_exception!(pymainprocess, ProcessBaseError, PyException);
create_exception!(pymainprocess, CommandFailed, ProcessBaseError);
create_exception!(pymainprocess, UnixOnly, ProcessBaseError);
create_exception!(pymainprocess, WindowsOnly, ProcessBaseError);

#[cfg(any(target_os = "unix", target_os = "linux"))]
use nix::unistd::{fork, ForkResult, execvp};

#[pyfunction]
fn call(command: &str) -> PyResult<()> {
    let _output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?;
    } else {
        if which("bash").is_ok() {
            Command::new("bash")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?;
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?;
        }
    };
    Ok(())
}

#[pyfunction]
fn call_and_safe(command: &str) -> PyResult<(String, String)> {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?
    } else {
        if which("bash").is_ok() {
            Command::new("bash")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?
        }
    };

    let stdout = String::from_utf8_lossy(&result.stdout).to_string();
    let stderr = String::from_utf8_lossy(&result.stderr).to_string();
    Ok((stdout, stderr))
}

#[pyfunction]
fn sudo(command: &str, user: &str) -> PyResult<()> {
    if cfg!(target_os = "windows") {
        return Err(WindowsOnly::new_err("Windows is not supported".to_string()));
    } else {
        if which("sudo").is_ok() {
            let _output = if which("bash").is_ok() {
                Command::new("sudo")
                    .arg("-u")
                    .arg(user)
                    .arg("-E")
                    .arg("bash")
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?
            } else {
                Command::new("sudo")
                    .arg("-u")
                    .arg(user)
                    .arg("-E")
                    .arg("sh")
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?
            };
        } else {
            return Err(UnixOnly::new_err("sudo is not installed".to_string()));
        }
    }
    Ok(())
}

#[pyfunction]
fn sudo_and_safe(command: &str, user: &str) -> PyResult<(String, String)> {
    if cfg!(target_os = "windows") {
        return Err(WindowsOnly::new_err("Windows is not supported".to_string()));
    } else {
        if which("sudo").is_ok() {
            let result = if which("bash").is_ok() {
                Command::new("sudo")
                    .arg("-u")
                    .arg(user)
                    .arg("-E")
                    .arg("bash")
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?
            } else {
                Command::new("sudo")
                    .arg("-u")
                    .arg(user)
                    .arg("-E")
                    .arg("sh")
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .map_err(|e| CommandFailed::new_err(format!("Failed to Execute Command: {}", e)))?
            };

            let stdout = String::from_utf8_lossy(&result.stdout).to_string();
            let stderr = String::from_utf8_lossy(&result.stderr).to_string();
            Ok((stdout, stderr))
        } else {
            return Err(UnixOnly::new_err("sudo is not installed".to_string()));
        }
    }
}

#[pyfunction]
fn py_which(command: &str) -> PyResult<String> {
    match which(command) {
        Ok(path) => Ok(path.to_string_lossy().to_string()),
        Err(_) => Err(CommandFailed::new_err(format!("Command not found: {}", command))),
    }
}

#[pyfunction]
fn get_cwd() -> PyResult<String> {
    let cwd = std::env::current_dir()?;
    Ok(cwd.to_str().unwrap().to_string())
}

#[pyfunction]
fn path_join(path: &str, paths: Vec<String>) -> PyResult<String> {
    let path = std::path::Path::new(path);
    let path = paths.iter().fold(path.to_path_buf(), |acc, x| acc.join(x));
    match path.to_str() {
        Some(p) => Ok(p.to_string()),
        None => Err(CommandFailed::new_err("Failed to convert path to string".to_string())),
    }
}

#[pyfunction]
fn path_exists(path: &str) -> PyResult<bool> {
    let _path = std::path::Path::new(path);
    Ok(_path.exists())
}

#[pyfunction]
fn path_is_file(path: &str) -> PyResult<bool> {
    let _path = std::path::Path::new(path);
    Ok(_path.is_file())
}

#[pyfunction]
fn path_is_dir(path: &str) -> PyResult<bool> {
    let _path = std::path::Path::new(path);
    Ok(_path.is_dir())
}

#[pyfunction]
fn path_basename(path: &str) -> PyResult<String> {
    let _path = std::path::Path::new(path);
    Ok(_path.file_name().unwrap().to_str().unwrap().to_string())
}

#[pyfunction]
fn path_splitext(path: &str) -> PyResult<(String, String)> {
    let _path = std::path::Path::new(path);
    let base = _path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| ProcessBaseError::new_err("Failed to get file stem".to_string()))?
        .to_string();
    let ext = _path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    Ok((base, ext))
}

#[pyfunction]
fn path_walk(start_path: &str) -> PyResult<HashMap<String, HashMap<String, Vec<String>>>> {
    fn walk_dir(path: &Path) -> PyResult<HashMap<String, Vec<String>>> {
        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry in fs::read_dir(path).map_err(|e| ProcessBaseError::new_err(format!("Failed to read directory: {}", e)))? {
            let entry = entry.map_err(|e| ProcessBaseError::new_err(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path.file_name().unwrap().to_str().unwrap().to_string());
            } else {
                files.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }

        let mut result = HashMap::new();
        result.insert("dirs".to_string(), dirs);
        result.insert("files".to_string(), files);

        Ok(result)
    }

    let path = Path::new(start_path);
    if !path.is_dir() {
        return Err(ProcessBaseError::new_err("Start path is not a directory".to_string()));
    }

    let result = walk_dir(path)?;
    let mut final_result = HashMap::new();
    let basename = path.file_name().unwrap().to_str().unwrap().to_string();
    final_result.insert(basename, result);
    Ok(final_result)
}

#[cfg(any(target_os = "unix", target_os = "linux"))]
#[pyfunction]
fn py_fork() -> PyResult<i32> {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => Ok(child.as_raw()),
        Ok(ForkResult::Child) => Ok(0),
        Err(err) => Err(ProcessBaseError::new_err(format!("Fork failed: {}", err))),
    }
}

#[cfg(any(target_os = "unix", target_os = "linux"))]
#[pyfunction]
fn py_execvp(file: &str, args: Vec<String>) -> PyResult<()> {
    let c_file = CString::new(file).map_err(|e| ProcessBaseError::new_err(format!("Invalid file name: {}", e)))?;
    let c_args: Vec<CString> = args.iter()
        .map(|arg| CString::new(arg.as_str()).map_err(|e| ProcessBaseError::new_err(format!("Invalid argument: {}", e))))
        .collect::<Result<Vec<CString>, PyErr>>()?;
    execvp(&c_file, &c_args).map_err(|e| CommandFailed::new_err(format!("Execvp failed: {}", e)))?;
    Ok(())
}

#[pyfunction]
fn env_get(key: &str) -> PyResult<String> {
    let value = std::env::var(key).map_err(|e| ProcessBaseError::new_err(format!("Failed to get environment variable: {}", e)))?;
    Ok(value)
}

#[pyfunction]
fn env_get_from_file(filepath: &str, key: &str, dotenv_use: bool) -> PyResult<String> {
    if dotenv_use {
        dotenv().ok();
        let variable = env::var(key).map_err(|e| ProcessBaseError::new_err(format!("Failed to get environment variable: {}", e)))?;
        Ok(variable)
    } else {
        let file = std::fs::File::open(filepath).map_err(|e| ProcessBaseError::new_err(format!("Failed to open file: {}", e)))?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line.map_err(|e| ProcessBaseError::new_err(format!("Failed to read line: {}", e)))?;
            if line.starts_with(&format!("{}=", key)) {
                let value = line.splitn(2, '=').nth(1).ok_or_else(|| ProcessBaseError::new_err(format!("Key not found: {}", key)))?;
                return Ok(value.to_string());
            }
        }
        Err(ProcessBaseError::new_err(format!("Key not found: {}", key)))
    }
}

#[pyfunction]
fn env_set(key: &str, value: &str) -> PyResult<()> {
    std::env::set_var(key, value);
    Ok(())
}

#[pyfunction]
fn env_items() -> PyResult<Vec<(String, String)>> {
    let items = std::env::vars().map(|(k, v)| (k, v.to_string())).collect();
    Ok(items)
}

#[pyfunction]
fn env_reset() -> PyResult<()> {
    std::env::vars().for_each(|(k, _)| std::env::remove_var(k));
    Ok(())
}

#[pyfunction]
fn env_os_data(data: &str) -> PyResult<String> {
    match data.to_lowercase().as_str() {
        "platform" | "os" => sys_info::os_type()
            .map_err(|e| ProcessBaseError::new_err(format!("Failed to get os type: {}", e))),
        "os_version" => sys_info::os_release()
            .map_err(|e| ProcessBaseError::new_err(format!("Failed to get os version: {}", e))),
        "architecture" => Ok(std::env::consts::ARCH.to_string()),
        "kernel" => sys_info::os_release()
            .map_err(|e| ProcessBaseError::new_err(format!("Failed to get kernel version: {}", e))),
        "cpu" => {
            let num = sys_info::cpu_num()
                .map_err(|e| ProcessBaseError::new_err(format!("Failed to get cpu number: {}", e)))?;
            let speed = sys_info::cpu_speed()
                .map_err(|e| ProcessBaseError::new_err(format!("Failed to get cpu speed: {}", e)))?;
            Ok(format!("{} cores at {} MHz", num, speed))
        },
        "hostname" => sys_info::hostname()
            .map_err(|e| ProcessBaseError::new_err(format!("Failed to get hostname: {}", e))),
        _ => Err(ProcessBaseError::new_err(format!("Invalid data: {}", data))),
    }
}

#[pyfunction]
fn py_exit(code: PyInt) -> PyResult<()> {
    std::process::exit(code as i32);
}

#[pyfunction]
fn chdir(path: &str) -> PyResult<()> {
    std::env::set_current_dir(path).map_err(|e| ProcessBaseError::new_err(format!("Failed to change directory: {}", e)))?;
    Ok(())
}

#[pyfunction]
fn mkdir(path: &str, exist_ok: bool) -> PyResult<()> {
    if exist_ok {
        std::fs::create_dir_all(path).map_err(|e| ProcessBaseError::new_err(format!("Failed to create directory: {}", e)))?;
    } else {
        std::fs::create_dir(path).map_err(|e| ProcessBaseError::new_err(format!("Failed to create directory: {}", e)))?;
    }
    Ok(())
}

#[pyfunction]
fn copy(src: &str, dest: &str, is_dir: bool) -> PyResult<()> {
    if is_dir {
        let options = CopyOptions::new(); // Verwende Standardoptionen
        copy_dir(src, dest, &options).map_err(|e| ProcessBaseError::new_err(format!("Failed to copy directory: {}", e)))?;
    } else {
        fs::copy(src, dest).map_err(|e| ProcessBaseError::new_err(format!("Failed to copy file: {}", e)))?;
    }
    Ok(())
}

#[pyfunction]
fn remove(path: &str, is_dir: bool) -> PyResult<()> {
    if is_dir {
        std::fs::remove_dir_all(path).map_err(|e| ProcessBaseError::new_err(format!("Failed to remove directory: {}", e)))?;
    } else {
        std::fs::remove_file(path).map_err(|e| ProcessBaseError::new_err(format!("Failed to remove file: {}", e)))?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn useradd(username: &str, password: &str) -> PyResult<()> {
    let shell = if which("bash").is_ok() {
        which("bash").unwrap()
    } else {
        which("sh").unwrap()
    };

    let status = Command::new("useradd")
        .arg("-m")
        .arg("-p")
        .arg(password)
        .arg("-s")
        .arg(shell)
        .arg(username)
        .status()
        .map_err(|e| ProcessBaseError::new_err(format!("Failed to create user: {}", e)))?;

    if !status.success() {
        return Err(ProcessBaseError::new_err("Failed to create user".to_string()));
    }

    Ok(())
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn useradd_with_sudo(username: &str, password: &str) -> PyResult<()> {
    let shell = if which("bash").is_ok() {
        which("bash").unwrap()
    } else {
        which("sh").unwrap()
    };

    let status = Command::new("sudo")
        .arg("useradd")
        .arg("-m")
        .arg("-p")
        .arg(password)
        .arg("-s")
        .arg(shell)
        .arg(username)
        .status()
        .map_err(|e| ProcessBaseError::new_err(format!("Failed to create user: {}", e)))?;

    if !status.success() {
        return Err(ProcessBaseError::new_err("Failed to create user".to_string()));
    }

    Ok(())
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn userdel(username: &str) -> PyResult<()> {
    let status = Command::new("userdel")
        .arg(username)
        .status()
        .map_err(|e| ProcessBaseError::new_err(format!("Failed to delete user: {}", e)))?;

    if !status.success() {
        return Err(ProcessBaseError::new_err("Failed to delete user".to_string()));
    }

    Ok(())
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn userdel_with_sudo(username: &str) -> PyResult<()> {
    let status = Command::new("sudo")
        .arg("userdel")
        .arg(username)
        .status()
        .map_err(|e| ProcessBaseError::new_err(format!("Failed to delete user: {}", e)))?;

    if !status.success() {
        return Err(ProcessBaseError::new_err("Failed to delete user".to_string()));
    }

    Ok(())
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn get_uid() -> PyResult<PyInt> {
    let uid = unsafe { libc::getuid() };
    Ok(uid as PyInt)
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn get_gid() -> PyResult<PyInt> {
    let gid = unsafe { libc::getgid() };
    Ok(gid as PyInt)
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn get_euid() -> PyResult<PyInt> {
    let euid = unsafe { libc::geteuid() };
    Ok(euid as PyInt)
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn get_egid() -> PyResult<PyInt> {
    let egid = unsafe { libc::getegid() };
    Ok(egid as PyInt)
}

#[cfg(any(target_os = "linux", target_os = "unix", target_os = "macos"))]
#[pyfunction]
fn chmod(path: &str, mode: u32) -> PyResult<()> {
    use std::os::unix::fs::PermissionsExt;
    use std::fs;

    let metadata = fs::metadata(path).map_err(|e| ProcessBaseError::new_err(format!("Failed to get metadata: {}", e)))?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(mode);
    fs::set_permissions(path, permissions).map_err(|e| ProcessBaseError::new_err(format!("Failed to change mode: {}", e)))?;
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "unix", target_os = "macos"))]
#[pyfunction]
fn chown(path: &str, uid: u32, gid: u32) -> PyResult<()> {
    use std::ffi::CString;
    use libc;

    let c_path = CString::new(path).map_err(|e| ProcessBaseError::new_err(format!("Failed to convert path: {}", e)))?;
    let result = unsafe { libc::chown(c_path.as_ptr(), uid, gid) };
    if result == -1 {
        return Err(ProcessBaseError::new_err(format!("Failed to change owner: {}", std::io::Error::last_os_error())));
    }
    Ok(())
}

#[pyfunction]
fn clear() -> PyResult<()> {
    let _output = if cfg!(any(target_os = "linux", target_os = "unix", target_os = "macos")) {
        Command::new("clear").status().map_err(|e| ProcessBaseError::new_err(format!("Failed to clear: {}", e)))?;
    } else {
        Command::new("cls").status().map_err(|e| ProcessBaseError::new_err(format!("Failed to clear: {}", e)))?;
    };
    Ok(())
}

#[pyfunction]
fn download(url: &str, output: &str, curl: bool, silent: bool) -> PyResult<()> {
    let status = if curl {
        if silent {
            Command::new("curl")
                .arg("-s")
                .arg("-o")
                .arg(output)
                .arg("-L")
                .arg(url)
                .status()
        } else {
            Command::new("curl")
                .arg("-o")
                .arg(output)
                .arg("-L")
                .arg(url)
                .status()
        }
    } else {
        if silent {
            Command::new("wget")
                .arg("-q")
                .arg("-O")
                .arg(output)
                .arg(url)
                .status()
        } else {
            Command::new("wget")
                .arg("-O")
                .arg(output)
                .arg(url)
                .status()
        }
    };

    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(CommandFailed::new_err(format!("Command failed with status: {}", s))),
        Err(e) => Err(CommandFailed::new_err(format!("Failed to execute command: {}", e))),
    }
}

#[pyfunction]
fn path_split(path: &str) -> PyResult<Vec<String>> {
    let path = std::path::Path::new(path);
    let components: Vec<String> = path
        .components()
        .map(|comp| comp.as_os_str().to_string_lossy().to_string())
        .collect();
    Ok(components)
}

#[pyfunction]
fn path_realpath(path: &str) -> PyResult<String> {
    let path = std::path::Path::new(path);
    let realpath = path.canonicalize()?;
    Ok(realpath.to_string_lossy().to_string())
}

#[pyfunction]
fn path_islink(path: &str) -> PyResult<bool> {
    let path = std::path::Path::new(path);
    let metadata = path.symlink_metadata()?; // Verwende symlink_metadata, um Symbolic Links zu erkennen
    Ok(metadata.file_type().is_symlink())
}

#[cfg(any(target_os = "unix", target_os = "linux", target_os = "macos"))]
#[pyfunction]
fn path_ismount(path: &str) -> PyResult<bool> {
    let path = std::path::Path::new(path);
    let metadata = path.metadata()?;
    let parent_metadata = path.parent().ok_or_else(|| ProcessBaseError::new_err("Failed to get parent directory".to_string()))?.metadata()?;
    Ok(metadata.dev() != parent_metadata.dev())
}

#[pyfunction]
fn path_splitroot(path: &str) -> PyResult<Vec<String>> {
    let path = std::path::Path::new(path);
    let components: Vec<String> = path
        .components()
        .map(|comp| comp.as_os_str().to_string_lossy().to_string())
        .collect();
    Ok(components)
}

#[pyfunction]
fn create_temp_file(suffix: Option<&str>) -> PyResult<String> {
    let temp_file = if let Some(suffix) = suffix {
        Builder::new().suffix(suffix).tempfile()?
    } else {
        NamedTempFile::new()?
    };
    let path = temp_file.into_temp_path().to_path_buf();
    Ok(path.to_string_lossy().to_string())
}

#[pyfunction]
fn create_temp_dir(suffix: Option<&str>) -> PyResult<String> {
    let temp_dir = if let Some(suffix) = suffix {
        Builder::new().suffix(suffix).tempdir()?
    } else {
        TempDir::new()?
    };
    let path = temp_dir.into_path();
    Ok(path.to_string_lossy().to_string())
}

#[pyfunction]
fn get_temp_path(path: &str) -> PyResult<String> {
    let path_buf = PathBuf::from(path);
    Ok(path_buf.to_string_lossy().to_string())
}

#[pyfunction]
fn cleanup_temp_path(path: &str, is_dir: bool) -> PyResult<()> {
    let path_buf = PathBuf::from(path);
    if is_dir {
        std::fs::remove_dir_all(&path_buf).map_err(|e| ProcessBaseError::new_err(format!("Failed to remove directory: {}", e)))?;
    } else {
        std::fs::remove_file(&path_buf).map_err(|e| ProcessBaseError::new_err(format!("Failed to remove file: {}", e)))?;
    }
    Ok(())
}

#[pyfunction]
fn path_symlink(original: &str, link: &str) -> PyResult<()> {
    #[cfg(target_os = "windows")]
    std::os::windows::fs::symlink_file(original, link)
        .map_err(|e| ProcessBaseError::new_err(format!("Failed to create symlink: {}", e)))?;
    
    #[cfg(not(target_os = "windows"))]
    std::os::unix::fs::symlink(original, link)
        .map_err(|e| ProcessBaseError::new_err(format!("Failed to create symlink: {}", e)))?;
    
    Ok(())
}

#[pymodule]
fn pymainprocess(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(call, m)?)?;
    m.add_function(wrap_pyfunction!(call_and_safe, m)?)?;
    m.add_function(wrap_pyfunction!(sudo, m)?)?;
    m.add_function(wrap_pyfunction!(sudo_and_safe, m)?)?;
    m.add_function(wrap_pyfunction!(py_which, m)?)?;
    m.add_function(wrap_pyfunction!(get_cwd, m)?)?;
    m.add_function(wrap_pyfunction!(path_join, m)?)?;
    m.add_function(wrap_pyfunction!(path_exists, m)?)?;
    m.add_function(wrap_pyfunction!(path_is_file, m)?)?;
    m.add_function(wrap_pyfunction!(path_is_dir, m)?)?;
    m.add_function(wrap_pyfunction!(path_basename, m)?)?;
    m.add_function(wrap_pyfunction!(path_splitext, m)?)?;
    m.add_function(wrap_pyfunction!(path_walk, m)?)?;
    m.add_function(wrap_pyfunction!(path_split, m)?)?;
    m.add_function(wrap_pyfunction!(path_realpath, m)?)?;
    m.add_function(wrap_pyfunction!(path_islink, m)?)?;
    m.add_function(wrap_pyfunction!(path_splitroot, m)?)?;
    #[cfg(any(target_os = "unix", target_os = "linux", target_os = "macos"))]
    m.add_function(wrap_pyfunction!(path_ismount, m)?)?;
    #[cfg(any(target_os = "unix", target_os = "linux"))]
    m.add_function(wrap_pyfunction!(py_fork, m)?)?;
    #[cfg(any(target_os = "unix", target_os = "linux"))]
    m.add_function(wrap_pyfunction!(py_execvp, m)?)?;
    #[cfg(target_os = "linux")]
    m.add_function(wrap_pyfunction!(useradd, m)?)?;
    #[cfg(target_os = "linux")]
    m.add_function(wrap_pyfunction!(useradd_with_sudo, m)?)?;
    #[cfg(target_os = "linux")]
    m.add_function(wrap_pyfunction!(userdel, m)?)?;
    #[cfg(target_os = "linux")]
    m.add_function(wrap_pyfunction!(userdel_with_sudo, m)?)?;
    #[cfg(target_os = "linux")]
    m.add_function(wrap_pyfunction!(get_uid, m)?)?;
    #[cfg(target_os = "linux")]
    m.add_function(wrap_pyfunction!(get_gid, m)?)?;
    #[cfg(target_os = "linux")]
    m.add_function(wrap_pyfunction!(get_euid, m)?)?;
    #[cfg(target_os = "linux")]
    m.add_function(wrap_pyfunction!(get_egid, m)?)?;
    #[cfg(any(target_os = "linux", target_os = "unix"))]
    m.add_function(wrap_pyfunction!(chmod, m)?)?;
    #[cfg(any(target_os = "linux", target_os = "unix"))]
    m.add_function(wrap_pyfunction!(chown, m)?)?;
    m.add_function(wrap_pyfunction!(clear, m)?)?;
    m.add_function(wrap_pyfunction!(env_get, m)?)?;
    m.add_function(wrap_pyfunction!(env_get_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(env_set, m)?)?;
    m.add_function(wrap_pyfunction!(env_items, m)?)?;
    m.add_function(wrap_pyfunction!(py_exit, m)?)?;
    m.add_function(wrap_pyfunction!(env_reset, m)?)?;
    m.add_function(wrap_pyfunction!(env_os_data, m)?)?;
    m.add_function(wrap_pyfunction!(download, m)?)?;
    m.add_function(wrap_pyfunction!(chdir, m)?)?;
    m.add_function(wrap_pyfunction!(mkdir, m)?)?;
    m.add_function(wrap_pyfunction!(copy, m)?)?;
    m.add_function(wrap_pyfunction!(remove, m)?)?;
    m.add_function(wrap_pyfunction!(create_temp_file, m)?)?;
    m.add_function(wrap_pyfunction!(create_temp_dir, m)?)?;
    m.add_function(wrap_pyfunction!(get_temp_path, m)?)?;
    m.add_function(wrap_pyfunction!(cleanup_temp_path, m)?)?;
    m.add_function(wrap_pyfunction!(path_symlink, m)?)?;
    m.add("ProcessBaseError", m.py().get_type_bound::<ProcessBaseError>())?;
    m.add("CommandFailed", m.py().get_type_bound::<CommandFailed>())?;
    m.add("UnixOnly", m.py().get_type_bound::<UnixOnly>())?;
    m.add("WindowsOnly", m.py().get_type_bound::<WindowsOnly>())?;
    Ok(())
}