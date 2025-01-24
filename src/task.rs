use std::env;
use std::error;
use std::fmt;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use chrono::{Local, NaiveDate};

use crate::color::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Task {
    name: String,
    creation_date: NaiveDate,
    due_date: Option<NaiveDate>,
    color: Option<Color>,
    note: String
}

impl Task {
    fn new(name: String) -> Self {
        let dt = Local::now();
        let creation_date = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap();

        Task{
            name,
            creation_date,
            due_date: None,
            color: None,
            note: String::new()
        }
    }
}

// -- Error handling --
type Result<T> = std::result::Result<T, ArgError>;

#[derive(Debug, PartialEq)]
pub enum ArgError {
    ArgMissing(String),
    TooManyArgs(String),
    InvalidTaskId(String),
    TaskNotFound,
    IncorrectDateFormat,
    InvalidColor(String),
}

impl error::Error for ArgError { }

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgError::ArgMissing(e) => writeln!(f, "Expected additional argument specifying the {}", e),
            ArgError::TooManyArgs(e) => writeln!(f, "Too many arguments provided: {}", e),
            ArgError::InvalidTaskId(e) => writeln!(f, "Invalid task id provided: {}", e),
            ArgError::TaskNotFound => writeln!(f, "Task not found"),
            ArgError::IncorrectDateFormat => writeln!(f, "Incorrectly formatted date (should be of YYYY-MM-DD format)"),
            ArgError::InvalidColor(e) => writeln!(f, "The requested color is not available: {}", e),
        }
    }
}
// -- End error handling --

// Parse the task ID and check it is valid and exists. Return task_id - 1.
fn parse_task_id(tasks: &[Task], task_id_opt: &Option<String>) -> Result<usize> {
    let task_id_string = match task_id_opt {
        None => return Err(ArgError::ArgMissing(String::from("task id"))),
        Some(task_id_string) => task_id_string.to_owned()
    };

    let task_id = task_id_string
        .parse::<usize>()
        .map_err(|_| ArgError::InvalidTaskId(task_id_string))?;

    if task_id > tasks.len() || task_id == 0 {
        Err(ArgError::TaskNotFound)
    } else {
        Ok(task_id - 1)
    }
}

// Check if there are additional arguments
pub fn check_for_more_args<T>(args_iter: T) -> Result<()>
where
    T: Iterator<Item = String> {
    let more_args = args_iter
        .collect::<Vec<String>>()
        .join(" ");

    match more_args.is_empty() {
        true => Ok(()),
        false => Err(ArgError::TooManyArgs(more_args))
    }
}

// Print all tasks the screen in a formatted way
pub fn list_tasks(tasks: &[Task], args_iter: env::Args) -> Result<()> {
    check_for_more_args(args_iter)?;

    println!("   ID  Task name                                                                   Creation date  Due date    Note");

    for (i, task) in tasks.iter().enumerate() {
        let name = if task.name.len() >= 75{
            &format!("{:.71}...", task.name)
        } else {
            &task.name
        };

        let color = match task.color {
            Some(Color::Red) => " ".red_bg(),
            Some(Color::Yellow) => " ".yellow_bg(),
            Some(Color::Green) => " ".green_bg(),
            Some(Color::Blue) => " ".blue_bg(),
            Some(Color::Purple) => " ".purple_bg(),
            None => String::from(" "),
        };

        let creation_date = task.creation_date.format("%Y-%m-%d").to_string();

        let due_date = match task.due_date {
            Some(date) => {
                let mut due_date = date
                    .format("%Y-%m-%d")
                    .to_string();
                // Color red if due date is in the past
                let dt = Local::now();
                let today = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap();
                if date < today {
                    due_date = due_date.red_fg();
                }
                due_date
            }
            None => String::new()
        };

        let note = if !task.note.is_empty() {
            String::from("✓")
        } else {
            String::new()
        };

        println!("{} {:>3}  {:<75} {:14} {:11} {}", color, i+1, name, creation_date, due_date, note)
    }
    println!();

    Ok(())
}

// Create task and add to vector
pub fn create_task<T>(tasks: &mut Vec<Task>, args_iter: T) -> Result<()>
where
    T: Iterator<Item = String> {
    let task_name = args_iter.collect::<Vec<String>>().join(" ");
    if task_name.is_empty() {
        return Err(ArgError::ArgMissing(String::from("task name")));
    };

    tasks.push(Task::new(task_name));
    println!("Task created with ID {}", tasks.len());

    Ok(())
}

// Provide a summary of the task
pub fn show_task(tasks: &[Task], mut args_iter: env::Args) -> Result<()> {
    let task_id = parse_task_id(tasks, &args_iter.next())?;
    check_for_more_args(args_iter)?;
    let task = &tasks[task_id];

    // Format creation date
    let creation_date = tasks[task_id].creation_date
        .format("%Y-%m-%d")
        .to_string();

    // Format due date
    let due_date = match task.due_date {
        Some(date) => {
            let mut due_date = date
                .format("%Y-%m-%d")
                .to_string();
            // Color red if due date is in the past
            let dt = Local::now();
            let today = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap();
            if date < today {
                due_date = due_date.red_fg();
            }
            due_date
        }
        None => String::new()
    };

    // Format color
    let mut color = task.color
        .as_ref()
        .map_or(String::from("None"), |c| c.to_string());
    color = match task.color {
        Some(Color::Red) => color.red_fg(),
        Some(Color::Yellow) => color.yellow_fg(),
        Some(Color::Green) => color.green_fg(),
        Some(Color::Blue) => color.blue_fg(),
        Some(Color::Purple) => color.purple_fg(),
        None => color
    };

    // Print all to screen
    let width = 75;
    println!("{:>15} {:<width$}", "ID:", task_id + 1);
    println!("{:>15} {:<width$}", "Name:", task.name);
    println!("{:>15} {:<width$}", "Creation date:", creation_date);
    println!("{:>15} {:<width$}", "Due date:", due_date);
    println!("{:>15} {:<width$}", "Color:", color);

    // Print the note as well
    let mut identifier = String::from("Note:");
    for line in task.note.split('\n') {
        let mut printline = String::new();
        for word in line.split(' ') {
            if printline.is_empty() {
                printline.push_str(word);
            } else if !printline.is_empty() && (printline.len() + word.len() < width) {
                printline.push(' ');
                printline.push_str(word);
            } else {
                println!("{:>15} {:<width$}", identifier, printline);
                printline = String::from(word); // New line
                if !identifier.is_empty() {
                    identifier = String::new(); // Don't show 'Note:' more than once
                }
            }
        }
        println!("{:>15} {:<width$}", identifier, printline);
        identifier = String::new(); // Don't show 'Note:' more than once
    }

    // Finally, an empty line
    println!();

    Ok(())
}

// Delete a task from the Vec
pub fn delete_task<T>(tasks: &mut Vec<Task>, mut args_iter: T) -> Result<()>
where
    T: Iterator<Item = String> {
    let task_id = parse_task_id(tasks, &args_iter.next())?;

    check_for_more_args(args_iter)?;

    let task_name = tasks[task_id].name.to_owned();
    tasks.remove(task_id);
    println!("Removed task \'{}\'", task_name);

    Ok(())
}

// Set or clear a task color
pub fn set_task_color<T>(tasks: &mut [Task], mut args_iter: T) -> Result<()>
where
    T: Iterator<Item = String> {
    let task_id = parse_task_id(tasks, &args_iter.next())?;

    // Get the color string from the argument and look up the color. Change the string
    // color for the message to the user
    let mut color_string = args_iter.next()
        .ok_or(ArgError::ArgMissing(String::from("task name")))?;
    let color = match color_string.as_str() {
        "red" => {
            color_string = color_string.red_fg();
            Some(Color::Red)
        },
        "yellow" => {
            color_string = color_string.yellow_fg();
            Some(Color::Yellow)
        },
        "green" => {
            color_string = color_string.green_fg();
            Some(Color::Green)
        },
        "blue" => {
            color_string = color_string.blue_fg();
            Some(Color::Blue)
        },
        "purple" => {
            color_string = color_string.purple_fg();
            Some(Color::Purple)
        }
        "clear" => {
            color_string = String::new();
            None
        },
        other => { return Err(ArgError::InvalidColor(other.to_string())); }
    };

    check_for_more_args(args_iter)?;

    // Set the color
    tasks[task_id].color = color;

    // Print the result
    if color_string.is_empty() {
        println!("Color removed for task \'{}\'", tasks[task_id].name);
    } else {
        println!("Color for task \'{}\' was set to {}", tasks[task_id].name, color_string);
    }

    Ok(())
}

// Adds a note to the task
pub fn add_note<T>(tasks: &mut [Task], mut args_iter: T) -> Result<()>
where
    T: Iterator<Item = String> {
    let task_id = parse_task_id(tasks, &args_iter.next())?;
    let note = args_iter.collect::<Vec<String>>().join(" ");

    if note == *"clear" {
        tasks[task_id].note = String::new();
        return Ok(());
    }

    if !tasks[task_id].note.is_empty() {
        tasks[task_id].note.push('\n');
    }
    tasks[task_id].note.push_str(&note);

    Ok(())
}

// Sort tasks by color, then due date
pub fn sort_tasks<T>(tasks: &mut [Task], args_iter: T) -> Result<()>
where
    T: Iterator<Item = String> {
    check_for_more_args(args_iter)?;

    tasks.sort_by_key(|task| (task.due_date));
    tasks.sort_by_key(|task| (task.due_date.is_none())); // Order 'None' values to the bottom
    tasks.sort_by(|task1, task2| task1.color.cmp(&task2.color)); // A bit contrived because this cannot be written as:
                                                                 //     tasks.sort_by_key(|task| (task.color));
    tasks.sort_by_key(|task| (task.color.is_none()));  // Order 'None' values to the bottom

    Ok(())
}

// Add a due date to the task
pub fn add_duedate<T>(tasks: &mut [Task], mut args_iter: T) -> Result<()>
where
    T: Iterator<Item = String> {
    let task_id = parse_task_id(tasks, &args_iter.next())?;

    let date_string = args_iter.next().ok_or(ArgError::ArgMissing(String::from("date")))?;
    let due_date = NaiveDate::parse_from_str(date_string.as_str(), "%Y-%m-%d")
        .map_err(|_| ArgError::IncorrectDateFormat)?;

    check_for_more_args(args_iter)?;

    tasks[task_id].due_date = Some(due_date);

    Ok(())
}

pub fn show_help(args_iter: env::Args) -> Result<()> {
    check_for_more_args(args_iter)?;

    let help_str = include_str!("help.txt");
    println!("{help_str}\n");

    Ok(())
}

// Rename a task
pub fn rename_task<T>(tasks: &mut [Task], mut args_iter: T) -> Result<()>
where
    T: Iterator<Item = String> {
    let task_id = parse_task_id(tasks, &args_iter.next())?;
    let name_old = tasks[task_id].name.to_owned();
    let name_new = args_iter.collect::<Vec<String>>().join(" ");

    tasks[task_id].name = name_new;

    println!("Renamed task \'{}\' to \'{}\'", name_old, tasks[task_id].name);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::IntoIter;

    #[test]
    fn test_create_task() {

        let args_iter_correct: IntoIter<String> = vec![String::from("test")].into_iter();
        let args_iter_correct2: IntoIter<String> = vec![String::from("test"), String::from("2")].into_iter();
        let args_iter_missing: IntoIter<String> = vec![].into_iter();

        let mut tasks: Vec<Task> = vec![];
        assert!(matches!(
            create_task(&mut tasks, args_iter_correct),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].name, String::from("test"));

        tasks = vec![];
        assert!(matches!(
            create_task(&mut tasks,args_iter_correct2),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].name, String::from("test 2"));

        assert!(matches!(
            create_task(&mut vec![], args_iter_missing),
            Result::Err(ArgError::ArgMissing(..))
        ));
    }

    #[test]
    fn test_delete_task() {
        let mut tasks = vec![Task::new(String::from("test"))];

        let args_iter_incorrect: IntoIter<String> = vec![String::from("2")].into_iter();
        let args_iter_invalid: IntoIter<String> = vec![String::from("foobar")].into_iter();
        let args_iter_too_many: IntoIter<String> = vec![String::from("1"), String::from("more")].into_iter();
        let args_iter_missing: IntoIter<String> = vec![].into_iter();
        let args_iter_correct: IntoIter<String> = vec![String::from("1")].into_iter();

        assert!(matches!(
            delete_task(&mut tasks, args_iter_incorrect),
            Result::Err(ArgError::TaskNotFound)
        ));

        assert!(matches!(
            delete_task(&mut tasks, args_iter_invalid),
            Result::Err(ArgError::InvalidTaskId(..))
        ));

        assert!(matches!(
            delete_task(&mut tasks, args_iter_too_many),
            Result::Err(ArgError::TooManyArgs(..))
        ));

        assert!(matches!(
            delete_task(&mut tasks, args_iter_missing),
            Result::Err(ArgError::ArgMissing(..))
        ));

        assert!(matches!(
            delete_task(&mut tasks, args_iter_correct),
            Result::Ok(..)
        ));
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_rename_task() {
        let mut tasks = vec![Task::new(String::from("test"))];
        let taskname_new = String::from("test renamed");

        let args_iter_incorrect: IntoIter<String> = vec![String::from("2")].into_iter();
        let args_iter_invalid: IntoIter<String> = vec![String::from("foobar")].into_iter();
        let args_iter_missing: IntoIter<String> = vec![].into_iter();
        let args_iter_correct: IntoIter<String> = vec![String::from("1"), taskname_new.clone()].into_iter();

        assert!(matches!(
            rename_task(&mut tasks, args_iter_incorrect),
            Result::Err(ArgError::TaskNotFound)
        ));

        assert!(matches!(
            rename_task(&mut tasks, args_iter_invalid),
            Result::Err(ArgError::InvalidTaskId(..))
        ));

        assert!(matches!(
            rename_task(&mut tasks, args_iter_missing),
            Result::Err(ArgError::ArgMissing(..))
        ));

        assert!(matches!(
            rename_task(&mut tasks, args_iter_correct),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].name, taskname_new);
    }

    #[test]
    fn test_add_duedate() {
        let mut tasks = vec![Task::new("test".to_owned())];
        let due_date = String::from("2025-12-12");

        let args_iter_incorrect_1: IntoIter<String> = vec![String::from("2"), due_date.clone()].into_iter();
        let args_iter_incorrect_2: IntoIter<String> = vec![String::from("1"), String::from("20251212")].into_iter();
        let args_iter_invalid: IntoIter<String> = vec![String::from("foobar"), due_date.clone()].into_iter();
        let args_iter_too_many: IntoIter<String> = vec![String::from("1"), due_date.clone(), String::from("more")].into_iter();
        let args_iter_missing_1: IntoIter<String> = vec![].into_iter();
        let args_iter_missing_2: IntoIter<String> = vec![String::from("1")].into_iter();
        let args_iter_correct: IntoIter<String> = vec![String::from("1"), due_date.clone()].into_iter();

        assert!(matches!(
            add_duedate(&mut tasks, args_iter_incorrect_1),
            Result::Err(ArgError::TaskNotFound)
        ));

        assert!(matches!(
            add_duedate(&mut tasks, args_iter_incorrect_2),
            Result::Err(ArgError::IncorrectDateFormat)
        ));

        assert!(matches!(
            add_duedate(&mut tasks, args_iter_invalid),
            Result::Err(ArgError::InvalidTaskId(..))
        ));

        assert!(matches!(
            add_duedate(&mut tasks, args_iter_too_many),
            Result::Err(ArgError::TooManyArgs(..))
        ));

        assert!(matches!(
            add_duedate(&mut tasks, args_iter_missing_1),
            Result::Err(ArgError::ArgMissing(..))
        ));

        assert!(matches!(
            add_duedate(&mut tasks, args_iter_missing_2),
            Result::Err(ArgError::ArgMissing(..))
        ));

        assert!(matches!(
            add_duedate(&mut tasks, args_iter_correct),
            Result::Ok(..)
        ));
        assert_eq!(
            tasks[0].due_date
                .unwrap()
                .format("%Y-%m-%d")
                .to_string(),
            due_date
        );
    }

    #[test]
    fn test_set_task_color() {
        let mut tasks = vec![Task::new( String::from("test") )];

        assert_eq!(tasks[0].color, None);

        let args_iter_incorrect_1: IntoIter<String> = vec![String::from("2"), String::from("red")].into_iter();
        let args_iter_incorrect_2: IntoIter<String> = vec![String::from("1"), String::from("orange")].into_iter();
        let args_iter_invalid: IntoIter<String> = vec![String::from("foobar"), String::from("red")].into_iter();
        let args_iter_too_many: IntoIter<String> = vec![String::from("1"), String::from("red"), String::from("more")].into_iter();
        let args_iter_missing_1: IntoIter<String> = vec![].into_iter();
        let args_iter_missing_2: IntoIter<String> = vec![String::from("1")].into_iter();

        let args_iter_correct_r: IntoIter<String> = vec![String::from("1"), String::from("red")].into_iter();
        let args_iter_correct_y: IntoIter<String> = vec![String::from("1"), String::from("yellow")].into_iter();
        let args_iter_correct_g: IntoIter<String> = vec![String::from("1"), String::from("green")].into_iter();
        let args_iter_correct_b: IntoIter<String> = vec![String::from("1"), String::from("blue")].into_iter();
        let args_iter_correct_p: IntoIter<String> = vec![String::from("1"), String::from("purple")].into_iter();
        let args_iter_correct_n: IntoIter<String> = vec![String::from("1"), String::from("clear")].into_iter();

        // Test all failures
        assert!(matches!(
            set_task_color(&mut tasks, args_iter_incorrect_1),
            Result::Err(ArgError::TaskNotFound)
        ));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_incorrect_2),
            Result::Err(ArgError::InvalidColor(..))
        ));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_invalid),
            Result::Err(ArgError::InvalidTaskId(..))
        ));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_too_many),
            Result::Err(ArgError::TooManyArgs(..))
        ));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_missing_1),
            Result::Err(ArgError::ArgMissing(..))
        ));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_missing_2),
            Result::Err(ArgError::ArgMissing(..))
        ));

        // Test correct behavior
        assert!(matches!(
            set_task_color(&mut tasks, args_iter_correct_r),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].color, Some(Color::Red));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_correct_y),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].color, Some(Color::Yellow));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_correct_g),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].color, Some(Color::Green));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_correct_b),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].color, Some(Color::Blue));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_correct_p),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].color, Some(Color::Purple));

        assert!(matches!(
            set_task_color(&mut tasks, args_iter_correct_n),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].color, None);
    }

    #[test]
    fn test_add_note() {
        let mut tasks = vec![Task::new( String::from("test") )];

        let args_iter_incorrect_1: IntoIter<String> = vec![String::from("2"), String::from("red")].into_iter();
        let args_iter_new: IntoIter<String> = vec![String::from("1"), String::from("Line1")].into_iter();
        let args_iter_add: IntoIter<String> = vec![String::from("1"), String::from("Line2")].into_iter();
        let args_iter_clear: IntoIter<String> = vec![String::from("1"), String::from("clear")].into_iter();

        assert!(matches!(
            add_note(&mut tasks, args_iter_incorrect_1),
            Result::Err(ArgError::TaskNotFound)
        ));

        assert!(matches!(
            add_note(&mut tasks, args_iter_new),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].note, String::from("Line1"));

        assert!(matches!(
            add_note(&mut tasks, args_iter_add),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].note, String::from("Line1\nLine2"));

        assert!(matches!(
            add_note(&mut tasks, args_iter_clear),
            Result::Ok(..)
        ));
        assert_eq!(tasks[0].note, String::from(""));
    }

    #[test]
    fn test_sort_tasks() {
        let mut tasks = vec![
            Task {name: String::from("Task green 1"),  creation_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(), due_date: NaiveDate::from_ymd_opt(2025, 8, 9), color: Some(Color::Green),  note: String::new()},
            Task {name: String::from("Task purple 1"), creation_date: NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(), due_date: None,                                color: Some(Color::Purple), note: String::new()},
            Task {name: String::from("Task green 2"),  creation_date: NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(), due_date: NaiveDate::from_ymd_opt(2025, 6, 1), color: Some(Color::Green),  note: String::new()},
            Task {name: String::from("Task blue 1"),   creation_date: NaiveDate::from_ymd_opt(2024, 2, 7).unwrap(), due_date: NaiveDate::from_ymd_opt(2025, 6, 1), color: Some(Color::Blue),   note: String::new()},
            Task {name: String::from("Task black 1"),  creation_date: NaiveDate::from_ymd_opt(2024, 5, 6).unwrap(), due_date: None,                                color: None,                note: String::new()},
            Task {name: String::from("Task green 3"),  creation_date: NaiveDate::from_ymd_opt(2024, 8, 3).unwrap(), due_date: NaiveDate::from_ymd_opt(2024, 9, 8), color: Some(Color::Green),  note: String::new()},
            Task {name: String::from("Task red 1"),    creation_date: NaiveDate::from_ymd_opt(2024, 2, 4).unwrap(), due_date: None,                                color: Some(Color::Red),    note: String::new()},
            Task {name: String::from("Task black 2"),  creation_date: NaiveDate::from_ymd_opt(2024, 1, 4).unwrap(), due_date: NaiveDate::from_ymd_opt(2025, 6, 1), color: None,                note: String::new()},
            Task {name: String::from("Task green 4"),  creation_date: NaiveDate::from_ymd_opt(2024, 5, 7).unwrap(), due_date: None,                                color: Some(Color::Green),  note: String::new()},
            Task {name: String::from("Task green 5"),  creation_date: NaiveDate::from_ymd_opt(2024, 3, 5).unwrap(), due_date: NaiveDate::from_ymd_opt(2025, 1, 7), color: Some(Color::Green),  note: String::new()},
            Task {name: String::from("Task red 2"),    creation_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), due_date: NaiveDate::from_ymd_opt(2025, 3, 9), color: Some(Color::Red),    note: String::new()},
        ];

        let args_iter_correct: IntoIter<String> = vec![].into_iter();
        let args_iter_too_many = vec![String::from("foo")].into_iter();

        assert!(matches!(
            sort_tasks(&mut tasks, args_iter_too_many),
            Result::Err(ArgError::TooManyArgs(..))
        ));

        let order_expected = vec![
            String::from("Task red 2"),
            String::from("Task red 1"),
            String::from("Task green 3"),
            String::from("Task green 5"),
            String::from("Task green 2"),
            String::from("Task green 1"),
            String::from("Task green 4"),
            String::from("Task blue 1"),
            String::from("Task purple 1"),
            String::from("Task black 2"),
            String::from("Task black 1"),
        ];

        assert!(matches!(
            sort_tasks(&mut tasks, args_iter_correct),
            Result::Ok(..)
        ));
        for (task, name) in std::iter::zip(tasks, order_expected) {
            assert_eq!(task.name, name);
        }
    }

}
