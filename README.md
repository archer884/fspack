# fspack

Use this program to automatically update `manifest.json` and `layout.json` files for Microsoft Flight Simulator 2020 packages.

```shell
$ fspack --help
fspack 0.1.0
Update manifest.json and layout.json

USAGE:
    fspack.exe [FLAGS] [path]

FLAGS:
    -f, --force      Set this flag to overwrite existing manifest and layout files. Defaults to stdout
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>    The path to your package. Defaults to your current directory
```

To try the program without overwriting your files, just run the following command:

```shell
$ fspack <path to your package>
```

To update your package manifest and layout, use the same command with the `-f` flag.

```shell
$ fspack <path to your package> -f
```
