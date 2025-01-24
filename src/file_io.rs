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
            FileError::Deserialize(e) => write!(f, "Unable to deserialize save file contents. Details:\n    {}", e),
            FileError::Serialize(e) => write!(f, "Unable to serialize data for saving. Details:\n    {}", e),
            FileError::Io(e) => write!(f, "Unable to save data. Details:\n    {}", e),
            FileError::CreateDir(e) => write!(f, "Unable to create directory for saving data. Details:\n    {}", e),
            FileError::BackupMissing => write!(f, "Unable to undo. No undos are available")
        }
    }
}
// -- End error handling --

// Builds the filename (with full path)
pub fn get_filename() -> PathBuf {

    let directory = data_dir();
    if directory.is_none() {
        println!("Error: could not find the user's data directory. Exiting...\n")
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

    // Create a backup
    create_backup(filename)?;

    // Save the file
    let mut file = File::create(filename).map_err(FileError::Io)?;
    write!(file, "{data_json}").map_err(FileError::Io)?;

    Ok(())
}

// Create a backup file for undo. Maximum number of backup files is 10. The newest file 
// has extension ".000", the oldest extension ".010". 
fn create_backup(filename: &PathBuf) -> Result<()> {
    let max_undos = 10;

    // Rename all existing backup files
    for i in (0..max_undos).rev() {
        let mut backup_older = PathBuf::from(filename);
        backup_older.set_extension( format!("{:03}", i+1) );

        let mut backup_newer = PathBuf::from(filename);
        backup_newer.set_extension( format!("{:03}", i) );

        if backup_newer.exists() {
            rename(backup_newer, backup_older).map_err(FileError::Io)?;
            println!("renamed");
        }
    }

    // Create newest backup file
    let mut backup_newest = PathBuf::from(filename);
    backup_newest.set_extension("000");
    if filename.exists() {
        rename(filename, backup_newest).map_err(FileError::Io)?;
    }

    Ok(())
}

// Undo last operation by rolling back files
pub fn roll_back_file(filename: &PathBuf) -> Result<()> {
    let max_undos = 10;

    for i in 0..max_undos+1 {
        if i == 0 {
            // Restore newest backup file
            let mut backup_newest = PathBuf::from(filename);
            backup_newest.set_extension( format!("{:03}", i) );
            if backup_newest.exists() {
                rename(backup_newest, filename).map_err(FileError::Io)?;
            } else {
                return Err(FileError::BackupMissing);
            }
        } else {
            // Rename older backup files
            let mut backup_older = PathBuf::from(filename);
            backup_older.set_extension( format!("{:03}", i) );
    
            let mut backup_newer = PathBuf::from(filename);
            backup_newer.set_extension( format!("{:03}", i - 1) );

            if backup_older.exists() {
                rename(backup_older, backup_newer).map_err(FileError::Io)?;
            }
        }   
    }

    Ok(())
}
