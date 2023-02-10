# fspack

Use this program to automatically update `manifest.json` and `layout.json` files for Microsoft Flight Simulator 2020 packages.

```shell
$ fspack --help
  [PATH]
          The path to your package. Defaults to your current directory

Options:
  -s, --size
          print package size

          In the original version of these packages, there was a member in the package manifest that contained the total size for the files in the       
          package. This appears to no longer be necessary, but in the vent you need to update that value, this will print the appropriate size.

  -l, --layout
          write layout changes

          By default, we just print the new layout file to stdout, but you can pass this flag to have us overwrite the file directly.

  -h, --help
          Print help (see a summary with '-h')
```

To try the program without overwriting your files, just run the following command:

```shell
$ fspack <path to your package>
```

To update your package layout, use the same command with the `--layout` flag.

```shell
$ fspack <path to your package> --layout
```
