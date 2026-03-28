use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::SystemTime;

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("mcp-runner: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> io::Result<ExitCode> {
    let mut args = env::args_os().skip(1);
    let Some(tool_name_os) = args.next() else {
        eprintln!("Usage: mcp-runner <tool-name> [args...]");
        eprintln!("Example: mcp-runner doc-viewer --mcp");
        return Ok(ExitCode::from(2));
    };

    let tool_name = tool_name_os.to_string_lossy().to_string();
    let tool_args: Vec<OsString> = args.collect();

    let workspace_root = find_workspace_root()?;
    let tool_dir = workspace_root.join("tools").join("mcp").join(&tool_name);
    let tool_manifest = tool_dir.join("Cargo.toml");
    if !tool_manifest.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Unknown tool '{tool_name}' (missing {})", tool_manifest.display()),
        ));
    }

    let exe_path = preferred_exe_path(&workspace_root, &tool_name);
    if should_build(&workspace_root, &tool_name, &tool_dir, &exe_path)? {
        build_tool(&tool_manifest, &tool_name)?;
    }

    let final_exe = existing_exe_path(&workspace_root, &tool_name).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Build finished but executable not found for tool '{tool_name}' in {}",
                workspace_root.join("target/release").display()
            ),
        )
    })?;

    let status = Command::new(&final_exe)
        .args(&tool_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(exit_code_from_status(status.code()))
}

fn find_workspace_root() -> io::Result<PathBuf> {
    if let Ok(root) = env::var("WORKSPACE_ROOT") {
        if !root.trim().is_empty() {
            let path = PathBuf::from(root);
            if is_workspace_root(&path) {
                return Ok(path);
            }
        }
    }

    let cwd = env::current_dir()?;
    if let Some(root) = find_workspace_from(&cwd) {
        return Ok(root);
    }

    let exe = env::current_exe()?;
    if let Some(root) = find_workspace_from(exe.parent().unwrap_or(Path::new("."))) {
        return Ok(root);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Unable to locate workspace root (expected Cargo.toml and tools/)",
    ))
}

fn find_workspace_from(start: &Path) -> Option<PathBuf> {
    for dir in start.ancestors() {
        if is_workspace_root(dir) {
            return Some(dir.to_path_buf());
        }
    }
    None
}

fn is_workspace_root(path: &Path) -> bool {
    path.join("Cargo.toml").is_file() && path.join("tools").is_dir()
}

fn preferred_exe_path(workspace_root: &Path, tool_name: &str) -> PathBuf {
    #[cfg(windows)]
    {
        workspace_root
            .join("target")
            .join("release")
            .join(format!("{tool_name}.exe"))
    }

    #[cfg(not(windows))]
    {
        workspace_root
            .join("target")
            .join("release")
            .join(tool_name)
    }
}

fn existing_exe_path(workspace_root: &Path, tool_name: &str) -> Option<PathBuf> {
    let release_dir = workspace_root.join("target").join("release");
    let exe_with_ext = release_dir.join(format!("{tool_name}.exe"));
    let exe_no_ext = release_dir.join(tool_name);

    if exe_with_ext.exists() {
        Some(exe_with_ext)
    } else if exe_no_ext.exists() {
        Some(exe_no_ext)
    } else {
        None
    }
}

fn should_build(
    workspace_root: &Path,
    tool_name: &str,
    tool_dir: &Path,
    exe_path: &Path,
) -> io::Result<bool> {
    if !exe_path.exists() && existing_exe_path(workspace_root, tool_name).is_none() {
        return Ok(true);
    }

    let existing_exe =
        existing_exe_path(workspace_root, tool_name).unwrap_or_else(|| exe_path.to_path_buf());

    if !existing_exe.exists() {
        return Ok(true);
    }

    let exe_mtime = modified_time(&existing_exe)?;

    let tool_manifest = tool_dir.join("Cargo.toml");
    if tool_manifest.exists() && modified_time(&tool_manifest)? > exe_mtime {
        return Ok(true);
    }

    let tool_src = tool_dir.join("src");
    if newest_rs_mtime(&tool_src)?
        .map(|t| t > exe_mtime)
        .unwrap_or(false)
    {
        return Ok(true);
    }

    let viewer_manifest = workspace_root.join("tools").join("viewer-api").join("Cargo.toml");
    if viewer_manifest.exists() && modified_time(&viewer_manifest)? > exe_mtime {
        return Ok(true);
    }

    let viewer_src = workspace_root.join("tools").join("viewer-api").join("src");
    if newest_rs_mtime(&viewer_src)?
        .map(|t| t > exe_mtime)
        .unwrap_or(false)
    {
        return Ok(true);
    }

    Ok(false)
}

fn build_tool(tool_manifest: &Path, tool_name: &str) -> io::Result<()> {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(tool_manifest)
        .output()?;

    if output.status.success() {
        return Ok(());
    }

    eprintln!("mcp-runner: build failed for {tool_name}");
    if !output.stdout.is_empty() {
        eprintln!("----- cargo stdout -----");
        eprint!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprintln!("----- cargo stderr -----");
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Err(io::Error::other("cargo build failed"))
}

fn modified_time(path: &Path) -> io::Result<SystemTime> {
    fs::metadata(path)?.modified()
}

fn newest_rs_mtime(path: &Path) -> io::Result<Option<SystemTime>> {
    if !path.is_dir() {
        return Ok(None);
    }

    let mut newest: Option<SystemTime> = None;
    walk_rs_files(path, &mut |file_path| {
        if let Ok(time) = modified_time(file_path) {
            newest = Some(match newest {
                Some(current) if current >= time => current,
                _ => time,
            });
        }
    })?;

    Ok(newest)
}

fn walk_rs_files(path: &Path, on_file: &mut dyn FnMut(&Path)) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry.file_type()?.is_dir() {
            walk_rs_files(&entry_path, on_file)?;
        } else if entry_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            on_file(&entry_path);
        }
    }

    Ok(())
}

fn exit_code_from_status(code: Option<i32>) -> ExitCode {
    match code {
        Some(value) if (0..=255).contains(&value) => ExitCode::from(value as u8),
        Some(_) => ExitCode::from(1),
        None => ExitCode::from(1),
    }
}
