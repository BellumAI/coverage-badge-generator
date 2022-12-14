use badge::{Badge, BadgeOptions};
use clap::{crate_version, App, Arg};
use rust_decimal::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // CLI arguments handling
    let matches = App::new("lcov_badge_generator")
        .version(crate_version!())
        .about("Generate an SVG code coverage badge from the command line.")
        .arg(
            Arg::with_name("coverage_percentage")
            	.value_name("PROJECT_COVERAGE_PERCENTAGE")
                .help(" Actual Coverage Percentage to display in the badge")
                .required(true),
        )
        .arg(
            Arg::with_name("badge-output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("The output file")
                .default_value("badge.svg")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("badge-decimal-places")
                .short("d")
                .long("decimal-places")
                .value_name("DECIMAL_PLACES")
                .help("Decimal places to round coverage by")
                .default_value("1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("badge-label")
                .short("b")
                .long("badge-label")
                .value_name("label")
                .help("Badge label")
                .default_value("coverage")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("z-warning-percent")
                .short("w")
                .long("warning-percent")
                .value_name("MIN_COVERAGE_PERCENTAGE")
                .help("Min percentage for warning badge (Floating Point)")
                .default_value("50.0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("z-passing-percent")
                .short("p")
                .long("passing-percent")
                .value_name("MIN_COVERAGE_PERCENTAGE")
                .help("Min percentage for passing badge (Floating Point)")
                .default_value("95.0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("failing-color")
                .short("F")
                .long("failing-color")
                .value_name("CSS_HEX_COLOR")
                .help("Color of failing badge (e.g. red)")
                .default_value("#e43")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("warning-color")
                .short("W")
                .long("warning-color")
                .value_name("CSS_HEX_COLOR")
                .help("Color of warning badge (e.g. yellow)")
                .default_value("#db1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("x-passing-color")
                .short("P")
                .long("passing-color")
                .value_name("CSS_HEX_COLOR")
                .help("Color of passing badge (e.g. green)")
                .default_value("#3c1")
                .takes_value(true),
        )
        .get_matches();
        
    let decimal_places_input = matches.value_of("badge-decimal-places").expect("Option `decimal-places` requires an integer value");
    let decimal_places: u32 = decimal_places_input.parse().expect("Option `decimal-places` requires an integer value");
        
    let lines_coverage_input = matches.value_of("coverage_percentage").expect("<MIN_COVERAGE_PERCENTAGE> should be a floating point value");
    let coverage_float: f32 = lines_coverage_input.parse().expect("<MIN_COVERAGE_PERCENTAGE> should be a floating point value");
    
    let lines_coverage_decimal = Decimal::from_str(lines_coverage_input).expect("<MIN_COVERAGE_PERCENTAGE> should be a floating point value");
    let lines_coverage = lines_coverage_decimal.round_dp(decimal_places).to_string();
    
    let min_warn = matches.value_of("z-warning-percent").expect("Option `warning-percent` requires a floating point value");
    let min_warn_float: f32 = min_warn.parse().expect("Option `warning-percent` requires a floating point value");
    
    let min_target = matches.value_of("z-passing-percent").expect("Option `passing-percent` requires a floating point value");
    let min_target_float: f32 = min_target.parse().expect("Option `passing-percent` requires a floating point value");
    
    let pass_color = matches.value_of("x-passing-color").expect("Option `passing-color` requires a value");
    let warn_color = matches.value_of("warning-color").expect("Option `warning-color` requires a value");
    let fail_color = matches.value_of("failing-color").expect("Option `failing-color` requires a value");
    
    let color;
    if coverage_float >= min_target_float {
    	color = pass_color // e.g. Green
    } else if coverage_float < min_warn_float {
    	color = fail_color // e.g. Red
    } else {
    	color = warn_color // e.g. Yellow
    }
    
    let label = matches.value_of("badge-label").expect("Option `badge-label` requires a value");

	// Generate the badge
	let badge = Badge::new(BadgeOptions {
		subject: label.to_string(),
		status: lines_coverage.to_string(),
		color: color.to_string(),
	})
	.unwrap();
	// Write the svg file
	let mut badge_file = File::create(matches.value_of("badge-output").unwrap())
		.expect("Failed to create badge file");
	badge_file
		.write_all(badge.to_svg().as_bytes())
		.expect("Failed to write badge svg file");
}