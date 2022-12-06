# `coverage-badge-generator`

A command-line badge generator for generic code coverage reports. Designed for use in CI pipelines.

## Example

- `coverage-badge-generator 98.91`: ![green badge](assets/green-badge.svg)
- `coverage-badge-generator 55.19`: ![yellow badge](assets/yellow-badge.svg)
- `coverage-badge-generator 26.45`: ![red badge](assets/red-badge.svg)

## Usage

```
Generate an SVG code coverage badge from the command line.

USAGE:
    coverage-badge-generator [OPTIONS] <PROJECT_COVERAGE_PERCENTAGE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --decimal-places <DECIMAL_PLACES>              Decimal places to round coverage by [default: 1]
    -b, --badge-label <label>                          Badge label [default: coverage]
    -o, --output <OUTPUT_FILE>                         The output file [default: badge.svg]
    -F, --failing-color <CSS_HEX_COLOR>                Color of failing badge (e.g. red) [default: #e43]
    -W, --warning-color <CSS_HEX_COLOR>                Color of warning badge (e.g. yellow) [default: #db1]
    -P, --passing-color <CSS_HEX_COLOR>                Color of passing badge (e.g. green) [default: #3c1]
    -p, --passing-percent <MIN_COVERAGE_PERCENTAGE>    Min percentage for passing badge (Floating Point) [default: 95.0]
    -w, --warning-percent <MIN_COVERAGE_PERCENTAGE>    Min percentage for warning badge (Floating Point) [default: 50.0]

ARGS:
    <PROJECT_COVERAGE_PERCENTAGE>     Actual Coverage Percentage to display in the badge
```

## Shamelessly Stolen From

[romainreignier/lcov_badge_generator](https://github.com/romainreignier/lcov_badge_generator)