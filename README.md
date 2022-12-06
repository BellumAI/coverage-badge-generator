# `coverage-badge-generator`

A command-line badge generator for generic code coverage reports. Designed for use in CI pipelines.

## Example

![green badge](assets/green-badge.svg)
![yellow badge](assets/yellow-badge.svg)
![red badge](assets/red-badge.svg)

## Usage

```
Generate an SVG code coverage badge from the command line.

USAGE:
    coverage-badge-generator [OPTIONS] <PROJECT_COVERAGE_PERCENTAGE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <OUTPUT_FILE>                 The output file [default: badge.svg]
    -t, --target <MIN_COVERAGE_PERCENTAGE>     Minimum Percentage for Green badge (Floating Point) [default: 95.0]
    -w, --warning <MIN_COVERAGE_PERCENTAGE>    Minimum Percentage for Yellow badge (Floating Point) [default: 50.0]
```

## Shamelessly Stolen From

https://github.com/romainreignier/lcov_badge_generator