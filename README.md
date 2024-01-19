# Rustybox
# Rustybox - Command Line Utility

## Overview

The main program uses the `args().collect()` function to retrieve command line
parameters. Subsequently, it checks if `args[1]` (the second argument, as the first
will be `rustybox`) is among the possible parameters. If it's not a valid command, the
message "Invalid command" is displayed. The variable `ok` is used, initially set to 0,
and when the argument is a valid command, its value changes to 1. Although the `match`
statement could be used, I opted for multiple `if` statements (similar to my
preference in C, where I'm not a fan of `switch`).

All functions except `pwd` receive parameters. They all have the `args` parameter,
which is a vector of strings containing the command line arguments.

## Commands:

### `pwd`:
- Fetches and displays the current directory, transforming it into a string.

### `echo`:
- Constructs a new string using `push` and `push_str`. Depending on the presence
of `-n`, the result is printed with either `print!` or `println!`.

### `cat`:
- Reads the command line argument as a string and displays it.

### `mkdir`:
- Uses `create_dir` to create a new directory if it doesn't already exist.

### `mv`:
- Uses the `rename` function and `args[2]` and `args[3]` to move/rename.

### `touch`:
- If the file doesn't exist and `-c` is not present, a new file is created.
- If `-c` is present, the access and modification times are modified.

### `ln`:
- Uses `symlink` and `hard_link` to create a symbolic link and a hard link, respectively.

### `rmdir`:
- Deletes the directory using `remove_dir`.

### `rm`:
- Deletes using `remove_dir_all` for recursive deletion, `remove_file` for a file,
and `remove_dir` for an empty directory.
- The `dir_error` variable checks if a directory needs to be deleted when `-d` is
not present. In this situation, it deletes the remaining files and displays an error
for attempting to delete the directory.

### `cp`:
- If not recursive, checks for a file as the source and a directory as the destination,
then copies them.
- If recursive, calls the `copy_recursive` function.
- `copy_recursive` iterates through all files and directories in the current directory
and moves them to the destination path.

### `ls`:
- Can receive a variety of parameters, but the most important ones are `recursive` and `all`.
- If only recursive is required, `ls_recursive`, a recursive function, is called.
- If `all` is also present, `ls_recursive_all` is called.
- The two functions are similar, with the only difference being that the one with
`all` displays ".", "..", and hidden files/directories.

### `chmod`:
- Checks if `args[2]` is a number.
- If it is a number, converts `args[2]` from a string to an octal number 
modifies permissions.
- If it's not a number, checks which permissions are being changed (read, write, exec.),
whether permissions are added or subtracted (1 for adding, 2 for subtracting), and for whom (all, group, ...).
- If adding, iterates through all possibilities, checks for whom and what permissions
are added, and performs a bitwise OR with an octal number (e.g., if adding read permission
for all, we know that all modifies all fields and read is 4, so it performs a bitwise OR with `0o444`).
- The same logic is used for `plus == 2` (minus), except that it subtracts (instead of
`|=` it uses `-=`).

## Run

Open terminal in the folder `Rustybox` and run `src/main` and the command you want.
