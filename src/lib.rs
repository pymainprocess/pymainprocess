use pyo3::prelude::*;
use std::process::{Command, Stdio};
use pyo3::exceptions::PyException;
use pyo3::create_exception;
use which::which;
use std::ffi::CString;
use dotenv::dotenv;
use std::io::{BufRead};
use sys_info;
use std::env;
use pytype::PyInt;

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
    #[cfg(any(target_os = "unix", target_os = "linux"))]
    m.add_function(wrap_pyfunction!(py_fork, m)?)?;
    #[cfg(any(target_os = "unix", target_os = "linux"))]
    m.add_function(wrap_pyfunction!(py_execvp, m)?)?;
    m.add_function(wrap_pyfunction!(env_get, m)?)?;
    m.add_function(wrap_pyfunction!(env_get_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(env_set, m)?)?;
    m.add_function(wrap_pyfunction!(env_items, m)?)?;
    m.add_function(wrap_pyfunction!(py_exit, m)?)?;
    m.add_function(wrap_pyfunction!(env_reset, m)?)?;
    m.add_function(wrap_pyfunction!(env_os_data, m)?)?;
    m.add_function(wrap_pyfunction!(chdir, m)?)?;
    m.add_function(wrap_pyfunction!(mkdir, m)?)?;
    m.add("ProcessBaseError", m.py().get_type_bound::<ProcessBaseError>())?;
    m.add("CommandFailed", m.py().get_type_bound::<CommandFailed>())?;
    m.add("UnixOnly", m.py().get_type_bound::<UnixOnly>())?;
    m.add("WindowsOnly", m.py().get_type_bound::<WindowsOnly>())?;
    Ok(())
}