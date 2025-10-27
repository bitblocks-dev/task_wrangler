# Task Wrangler

Builds for Windows and Linux are provided in the release tab

## Build Instructions (Linux)

- Set up a Rust environment using [rustup](https://rustup.rs)
- make sure that your system PATH is set up correctly (e.g. add `~/.cargo/bin` to PATH on Linux)
- run cargo build --release
- locate the compiled release build in the project directory under `target/release/task_wrangler`

## Usage Instructions

- Run the tool in a terminal
- On startup, Task Wrangler will create a save file in the directory it is run from or load the existing one.
- It will present you with a menu, type the number of the option you want to select it and then hit enter
- Whenever you are asked to pick a task to perform an action on, Task Wrangler will list all tasks with a corresponding number. Typing a task's number will select it.
