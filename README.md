# fw

```
fw

USAGE:
    fw <PATH> <COMMAND>

ARGS:
    <PATH>       File to watch
    <COMMAND>    Command to execute

OPTIONS:
    -h, --help    Print help information
```

### Example 

Want to run a *justfile* script that compiles a `.tex` file each time it is saved.

    fw main.tex just
    
Hence, `just` is executed each time main.tex changes.
