
	fw 0.2.0
	file-watcher
	
	USAGE:
	    fw [OPTIONS] <PATH> <COMMAND>
	
	ARGS:
	    <PATH>       File to watch
	    <COMMAND>    Command to execute
	
	OPTIONS:
	    -h, --help           Print help information
	    -t, --time <TIME>    Check interval time [default: 2.5]
	    -V, --version        Print version information

</br>

Want to run a *justfile* script that compiles a `.tex` file each time it is saved.

	fw main.tex just

Hence, `just` is executed each time main.tex changes.

</br>

## Installation

	cargo install --git https://github.com/AOx0/fw
