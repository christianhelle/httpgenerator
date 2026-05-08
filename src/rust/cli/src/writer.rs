use std::{
    fs,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::Duration,
};

use httpgenerator_core::HttpFile;

use crate::CliError;

pub(crate) fn write_files(
    output_folder: &Path,
    files: Vec<HttpFile>,
    timeout_seconds: u64,
) -> Result<Vec<PathBuf>, CliError> {
    if !output_folder.exists() {
        fs::create_dir_all(output_folder).map_err(|error| CliError::CreateOutputDirectory {
            path: output_folder.to_path_buf(),
            reason: error.to_string(),
        })?;
    }

    let output_folder = output_folder.to_path_buf();
    let (sender, receiver) = mpsc::channel();

    thread::spawn({
        let output_folder = output_folder.clone();
        move || {
            let result = write_files_worker(&output_folder, files);
            let _ = sender.send(result);
        }
    });

    receiver
        .recv_timeout(Duration::from_secs(timeout_seconds))
        .map_err(|error| match error {
            mpsc::RecvTimeoutError::Timeout => CliError::WriteTimeout {
                seconds: timeout_seconds,
            },
            mpsc::RecvTimeoutError::Disconnected => CliError::WriteChannelClosed,
        })?
}

fn write_files_worker(
    output_folder: &Path,
    files: Vec<HttpFile>,
) -> Result<Vec<PathBuf>, CliError> {
    let mut written_paths = Vec::with_capacity(files.len());

    for file in files {
        let path = output_folder.join(&file.filename);
        fs::write(&path, file.content).map_err(|error| CliError::WriteFiles {
            path: path.clone(),
            reason: error.to_string(),
        })?;
        written_paths.push(path);
    }

    Ok(written_paths)
}
