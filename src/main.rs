mod color;
mod file_io;
mod task;

use std::env;
use std::path::PathBuf;

use file_io::get_filename;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_LICENSE: &str = env!("CARGO_PKG_LICENSE");


fn main() {
    let filename: PathBuf = get_filename();
    let mut tasks: Vec<task::Task> = vec![];

    // Load tasks if any
    match file_io::load_tasks(&filename, &mut tasks) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Read the command argument
    let mut args_iter = env::args();
    let _ = args_iter.next(); // Skip the first argument
    let command: String = match args_iter.next() {
        Some(arg) => arg,
        None => {
            eprintln!("No arguments given. Specify \'todo help\' to learn how to use this program\n");
            std::process::exit(1);
        }
    };

    // Call the corresponding method
    let command_str = command.as_str();
    let result = match command_str {
        "add"     => task::create_task(&mut tasks, args_iter),

        "due"     => task::add_duedate(&mut tasks, args_iter),
        "note"    => task::add_note(&mut tasks, args_iter),
        "color"   => task::set_task_color(&mut tasks, args_iter),
        "rename"  => task::rename_task(&mut tasks, args_iter),
        "remove"  => task::delete_task(&mut tasks, args_iter),

        "list"    => task::list_tasks(&tasks, args_iter),
        "show"    => task::show_task(&tasks, args_iter),
        "sort"    => task::sort_tasks(&mut tasks, args_iter),
        "undo"    => task::check_for_more_args(args_iter), // Only check args, nothing else to do
        "info" => {
            println!("{PKG_NAME} version {PKG_VERSION}, written by {PKG_AUTHORS} and released under the {PKG_LICENSE} license\n{PKG_REPOSITORY}");
            std::process::exit(0);
        }
        "help"    => task::show_help(args_iter),
        other     => {
            eprintln!("Unknown command given: {}\n", other);
            std::process::exit(1);
        }
    };

    // Check if method ran successfully and set flag for saving/undo
    let mut undo_flag = false;
    let mut save_flag = false;
    match result {
        Ok(..) => {
            if matches!(command_str, "add" | "due" | "note" | "color" | "rename" | "remove" | "sort") {
                save_flag = true;
            } else if matches!(command_str, "undo") {
                undo_flag = true;
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    // Save tasks to file OR roll back previous version of file (undo)
    if save_flag {
        match file_io::save_file(&filename, &tasks) {
            Ok(..) => (),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    } else if undo_flag {
        match file_io::roll_back_file(&filename) {
            Ok(..) => (),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
