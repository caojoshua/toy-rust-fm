# toy-rust-fm
A toy rust file manager, so I can practice development for Rust (because its hard) and TUI (cause
I've never done before. This project can browse through the file system, and open files in a text
editor.

I thought it would be cool to implement a file manager that is very integrated with the command
line. For example, we could have side-by-side tmux-like panes, one with a shell and the other with
the file manager. Changing directories in the file manager would also change directories in the
shell, and vice-versa. We could also have expandable macros, i.e. select multiple files in the file
manager, and then `mv %selected` from the shell.

In practice, this would be a greater effort than anticipated. Getting a native shell in the TUI app
is non-trivial task. I think there might be some libraries (i.e. libvterm, which neovim uses), but
I did not look into them too much. I could try manual implementation, by monitoring the child shell
stdin/stdout, but I'm not sure yet how I would deal with colors, special characters, or anything
else unexpected. I'm also not sure how to implement the features in the previous paragraph, as I
would have to somehow modiy the shell process to be able to communicate with the file manager.

I think this is a cool idea. Maybe I will come back to this later.
