Produces a one bit dithered version of the input
image.

```
USAGE:
    dither [OPTIONS] <INPUT> --out <o>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --algorithm <algo>    Algorithm, floyd or atkinson, defaults to atkinson
    -o, --out <o>             Output path

ARGS:
    <INPUT>    Input file
```