# ToDo

ToDo is a simple command-line based task manager. Although created as an programming exercise for learning Rust, it is intended to be practical in use.

## Description

ToDo allows one to create tasks, set due dates, add notes, and label and categorize tasks by color. It is operated from a terminal.

![1](https://github.com/user-attachments/assets/58e0b3ec-e98f-4acd-8a12-a8a644dca4e1)

![2](https://github.com/user-attachments/assets/82c7342f-3196-4c33-b86d-de7333d01e55)

## Getting Started

### Dependencies

ToDo is OS agnostic, so it should run on Windows, Linux, and macOS. Perhaps also on BSD. 

### Installing

Right now, installing is simply a matter of downloading the repo and running `cargo build --release`. Then, put the executable in a directory where you have read/write permissing and add it to your shells's path.

### Executing program

Run the program as `todo [command] [arguments]`. The following command creates a task:

* `todo add [task name]` adds a task with name `task name`. Spaces in the name are allowed. Returned is the task's ID (needed for all further operations on this task).

A task can be viewed with the command below:

* `todo show [task_id]` shows details for the task with ID `task_id`.

Tasks can be modified with the following commands:

* `todo due [task_id] [due_date]` sets the due date for the task with ID `task_id`. The required format for `due_date` is YYYY-MM-DD. If `due_date` equals `clear`, the due date is removed.
* `todo note [task_id] [text]` sets a note or description for the task with ID `task_id`. All arguments after the ID are taken as the note. If a note already exists, `text` is added to it. If `text` equals `clear`, the note is removed.
* `todo color [task_id] [color]` sets a color for the task with ID `task_id`. Colors can be used to group and order tasks. Available colors are `red`, `yellow`, `green`, `blue`, and `purple`. Specifying `clear` removes the color.
* `todo rename [task_id] [task name]` renames the task with ID `task_id` to `task name`.
* `todo remove [task_id]` removes the task with ID `task_id`. Remaining tasks have their `task_id` renumbered. 

Finally, the following non-task specific commands are available:

* `todo list` lists all tasks.
* `todo sort` groups tasks by color and sorts them by colors of the rainbow (red -> purple). Within each group, tasks are sorted by due date. Note: this operations renumbers the IDs of the tasks.
* `todo undo` undoes the latest change to tasks. 10 undoes are available.
* `todo info` displays info about the program, such as the version number.
* `todo help` displays how this program can be used.

## Data location
Data is saved in the user’s data directory. This is `%APPDATA%\Roaming` on Windows, `$HOME/.local/share` on Linux, and `$HOME/Library/Application Support` on macOS.

## Caveat emptor

This is a project that I started to learn Rust. Everyone is free to use it, but I can provide no guarantee that it works as intended. As such, I cannot be held responsible for unintended data loss nor any other ill effects.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details
