# dust
A dhall parser and AST in rust.

Contents:

* [Info]
  * [Status]
* [Development and Usage]
  * [Input]
  * [Stages/Operations]
    * [Parsing]
    * [Resolution]
      * [HTTP Imports]
      * [Local Imports]
      * [Evaluation]
  
[Info]: #Info
[Status]: #Status
[Development and Usage]: #Development-and-Usage
[Input]: #Input
[Stages/Operations]: #StagesOperations
[Parsing]: #Parsing
[Resolution]: #Resolution
[HTTP Imports]: #HTTPImports
[Local Imports]: #LocalImports
[Evaluation]: #Evaluation

## Info

`dust` is a dhall re-implementation in Rust, for efficiency, extensibility and fun.

### Status

This project is in alpha-development stage. Not recomended to try to use it for manipulating dhall (just yet).

## Development and Usage

Building with `cargo build` will generate the `dust` binary in `target/`.

### Input

`dust` uses the last non-option argument as the root source filepath.

    printf 'let a = "This is a dhall" in a' | tee a.dhall
    
    # By default only parse and resolve, do not eval/normalize
    dust a.dhall

If no such, it will use the `/dev/stdin` file (yes, not STDIN).

    printf 'let a = "This is a dhall" in a' |
    dust  # it thinks it's reading /dev/stdin
    
The stdin point is important for import resolution. It is assumed that relative imports start rom `/dev/`, not CWD.
Use a local file for that.

### Stages/Operations

Input source goes (optionally) through stages:

- Parsing
- Resolution
- Evaluation

These can be enabled/disabled, by using one of the flags shown in `--help`. Some combinations don't make sense.

#### Parsing

Translates source code in an AST structure.

#### Resolution

Resolves imports (both local and http). The imported source is re-emitted under `let` bindings under the fully resolved import path-ids.

##### HTTP Imports

HTTP imported files are expected to be found in `~/.cache/dust` (to be configurable in the future), under the full HTTP URI as path.

You can manually populate this, or you can use `--fetch`, which will do this job specifically. `--fetch` needs to be run only once, or
once every time you'd like to refresh/update the imports.

##### Local Imports

Local imports can only be relative, and are resolved relative to the including file's path.

The including file will either have been imported with the same process, receiving and absolute import path (unique path-import-id),
or is the root source specified on command line.

The only file able to be resolved relative to the CWD is the root source file specified on command line.

#### Evaluation

Pass the `--eval` flag to get an easter egg.
