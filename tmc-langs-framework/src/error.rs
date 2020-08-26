use crate::io::tmc_zip;

use crate::command::CommandString;
use std::path::PathBuf;
use std::process::ExitStatus;
use thiserror::Error;

// todo: make util error type and move variants there
#[derive(Error, Debug)]
pub enum TmcError {
    // IO
    #[error("File IO error")]
    FileIo(#[from] FileIo),
    #[error("{0} exists and is not a directory")]
    UnexpectedFile(PathBuf),
    #[error("Failed to read stdio")]
    ReadStdio(#[source] std::io::Error),

    #[error("Failed to read file inside zip archive with path {0}")]
    ZipRead(PathBuf, #[source] std::io::Error),
    #[error("Failed to write file at {0} to zip archive")]
    ZipWrite(PathBuf, #[source] std::io::Error),

    #[error("Error appending to tar")]
    TarAppend(#[source] std::io::Error),
    #[error("Error finishing tar")]
    TarFinish(#[source] std::io::Error),
    #[error("Failed to read line")]
    ReadLine(#[source] std::io::Error),
    #[error("Failed to canonicalize path {0}")]
    Canonicalize(PathBuf, #[source] std::io::Error),
    #[error("Error occurred in a child process")]
    Process(#[source] std::io::Error),
    #[error("Failed to set permissions for {0}")]
    SetPermissions(PathBuf, #[source] std::io::Error),
    #[error("Invalid parameter value: {0}")]
    InvalidParam(String),
    #[error("File {0} not in given project root {1}")]
    FileNotInProject(PathBuf, PathBuf),
    #[error("Path {0} is not absolute")]
    PathNotAbsolute(PathBuf),

    #[error("Path {0} contained invalid UTF8")]
    UTF8(PathBuf),
    #[error("Path {0} contained no file name")]
    NoFileName(PathBuf),

    #[error("No matching plugin found for {0}")]
    PluginNotFound(PathBuf),
    #[error("No project directory found in archive during unzip")]
    NoProjectDirInZip,

    #[error("Error in plugin")]
    Plugin(#[from] Box<dyn std::error::Error + 'static + Send + Sync>),

    #[error("Failed to run command")]
    Command(#[from] CommandError),

    #[error(transparent)]
    YamlDeserialization(#[from] serde_yaml::Error),
    #[error(transparent)]
    ZipError(#[from] tmc_zip::ZipError),
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
}

// == Collection of errors likely to be useful in multiple plugins which can be special cased without needing a plugin's specific error type ==
/// An error caused by a failed attempt to execute an external command.
#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Failed to spawn command: {0:?}")]
    Spawn(CommandString, #[source] std::io::Error),
    #[error("The executable for \"{name}\" could not be found ({path}). Please make sure you have installed it correctly.")]
    NotFound {
        name: String,
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Failed to run command {0:?}")]
    FailedToRun(CommandString, #[source] std::io::Error),
    #[error("Command {command} exited with status {status}")]
    Failed {
        command: CommandString,
        status: ExitStatus,
        stdout: String,
        stderr: String,
    },
}

/// A wrapper for std::io::Error that provides more context for the failed operations.
#[derive(Error, Debug)]
pub enum FileIo {
    #[error("Failed to open file at {0}")]
    FileOpen(PathBuf, #[source] std::io::Error),
    #[error("Failed to read file at {0}")]
    FileRead(PathBuf, #[source] std::io::Error),
    #[error("Failed to write file at {0}")]
    FileWrite(PathBuf, #[source] std::io::Error),
    #[error("Failed to create file at {0}")]
    FileCreate(PathBuf, #[source] std::io::Error),
    #[error("Failed to remove file at {0}")]
    FileRemove(PathBuf, #[source] std::io::Error),
    #[error("Failed to copy file from {from} to {to}")]
    FileCopy {
        from: PathBuf,
        to: PathBuf,
        source: std::io::Error,
    },
    #[error("Failed to move file from {from} to {to}")]
    FileMove {
        from: PathBuf,
        to: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to open directory at {0}")]
    DirOpen(PathBuf, #[source] std::io::Error),
    #[error("Failed to read directory at {0}")]
    DirRead(PathBuf, #[source] std::io::Error),
    #[error("Failed to create directory at {0}")]
    DirCreate(PathBuf, #[source] std::io::Error),
    #[error("Failed to remove directory at {0}")]
    DirRemove(PathBuf, #[source] std::io::Error),

    #[error("Failed to rename file {from} to {to}")]
    Rename {
        from: PathBuf,
        to: PathBuf,
        source: std::io::Error,
    },

    #[error("Path {0} has no file name")]
    NoFileName(PathBuf),
    #[error("Expected {0} to be a directory, but it was a file")]
    UnexpectedFile(PathBuf),

    #[error("Directory walk error")]
    Walkdir(#[from] walkdir::Error),
}
