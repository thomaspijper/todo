use std::error;
use std::fmt;
use std::fs::{File, read_to_string, rename, create_dir};
use std::io::Write;
use std::path::PathBuf;

use crate::task::Task;
use dirs::data_dir;

// -- Error handling --
type Result<T> = std::result::Result<T, FileError>;

#[derive(Debug)]
pub enum FileError {
    Deserialize(serde_json::Error),
    Serialize(serde_json::Error),
    Io(std::io::Error),
    CreateDir(std::io::Error),
    BackupMissing,
}

impl error::Error for FileError { }

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::Deserialize(e) => writeln!(f, "Unable to deserialize save file contents. Details:\n    {}", e),
            FileError::Serialize(e) => writeln!(f, "Unable to serialize data for saving. Details:\n    {}", e),
            FileError::Io(e) => writeln!(f, "Unable to save data. Details:\n    {}", e),
            FileError::CreateDir(e) => writeln!(f, "Unable to create directory for saving data. Details:\n    {}", e),
            FileError::BackupMissing => writeln!(f, "Unable to undo. The backup file does not exist")
        }
    }
}
// -- End error handling --

// Builds the filename (with full path)
pub fn get_filename() -> PathBuf {

    let directory = data_dir();
    if directory.is_none() {
        println!("Error: could not discern the user's data directory. Exiting...\n")
    }
    
    let mut filename = directory.unwrap();
    filename.push("todo-rs");
    filename.push("tasks.json");

    filename
}

// Read tasks from the json file, if available
pub fn load_tasks(filename: &PathBuf, tasks: &mut Vec<Task>) -> Result<()> {
    if filename.exists() {
        let json_string = read_to_string(filename).map_err(FileError::Io)?;
        let mut loaded_tasks: Vec<Task> = serde_json::from_str(json_string.as_str()).map_err(FileError::Deserialize)?;
        tasks.append(&mut loaded_tasks);
    } else {
        println!("No previous tasks file found. Is this the first time you run this program?\n")
    };

    Ok(())
}

// Serialize data and save file
pub fn save_file(filename: &PathBuf, tasks: &Vec<Task>) -> Result<()> {
    let data_json = serde_json::to_string(&tasks).map_err(FileError::Serialize)?;

    // Create directory if it does not yet exist
    let parent_dir = filename.parent().unwrap();
    if !parent_dir.exists() {
        create_dir(parent_dir).map_err(FileError::CreateDir)?;
        println!("Creating tasks file: {:?}\n", filename)
    };

    // Create backup file for undo operation
    let mut filename_backup = PathBuf::from(filename);
    filename_backup.set_extension("old");
    if filename.exists() {
        rename(filename, filename_backup).map_err(FileError::Io)?;
    }

    let mut file = File::create(filename).map_err(FileError::Io)?;
    write!(file, "{data_json}").map_err(FileError::Io)?;

    Ok(())
}

// Undo last operation by rolling back file
pub fn roll_back_file(filename: &PathBuf) -> Result<()> {
    let mut filename_backup = PathBuf::from(filename);
    filename_backup.set_extension("old");

    if filename_backup.exists() {
        rename(filename_backup, filename).map_err(FileError::Io)?;
        Ok(())
    } else {
        Err(FileError::BackupMissing)
    }
}
