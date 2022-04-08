
	fw 0.5.0
    file-watcher
    
    USAGE:
    fw [OPTIONS] <PATH> <COMMAND>
    
    ARGS:
    <PATH>       File to watch
    <COMMAND>    Command to execute
    
    OPTIONS:
    -e, --error-skip     Ignore errors when executing command and don't panic
    -h, --help           Print help information
    -i, --interactive    Interactive
    -l, --length         Whether a deep check must be made by contents length
    -s, --sum            Whether a deep check must be made by contents sum
    -t, --time <TIME>    Check interval time [default: 2.5]
    -v, --verbose        Show command
    -V, --version        Print version information




</br>

Want to run a *justfile* script that compiles a `.tex` file each time it is saved.

	fw main.tex just

Hence, `just` is executed each time main.tex changes.

</br>

## Installation

	cargo install --git https://github.com/AOx0/fw
