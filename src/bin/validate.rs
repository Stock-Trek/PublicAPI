use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    fmt::Display,
    fs,
    process::ExitCode,
};
use stock_trek::validate::{
    validate,
    validator::{Location, ValidationError},
};

fn main() -> ExitCode {
    match file_path_from_arg() {
        Err(error) => {
            println!("{}", error);
            return ExitCode::from(1);
        }
        Ok(path) => match file_contents_from_path(&path) {
            Err(error) => {
                println!("{}", error);
                return ExitCode::from(2);
            }
            Ok(contents) => match validate_contents(&contents) {
                Err(errors) => {
                    errors.iter().for_each(|error| println!("{}", error));
                    return ExitCode::from(4);
                }
                Ok(success) => {
                    println!("{}", success);
                    return ExitCode::from(0);
                }
            },
        },
    }
}

fn file_path_from_arg() -> Result<String, String> {
    let mut args = env::args();
    let _program = args.next();
    let arg = args.next();
    match arg {
        Some(path) => Ok(path),
        None => Err("A file path must be provided as the argument".to_string()),
    }
}

fn file_contents_from_path(path: &str) -> Result<String, String> {
    match fs::read_to_string(&path) {
        Ok(contents) => return Ok(contents),
        Err(e) => {
            return Err(format!(
                "Could not read contents from path {}\n{}",
                &path,
                e.to_string()
            ))
        }
    }
}

fn validate_contents(contents: &str) -> Result<String, Vec<String>> {
    match validate::validate(contents) {
        Ok(..) => {
            Ok("This code is supported for use with stock-trek.com, happy signalling!".to_string())
        }
        Err(error) => match error {
            ValidationError::Parse(parse_error) => {
                let line_column = parse_error.span().start();
                let error = format!(
                    "Could not parse code at [line:{},col:{}]",
                    line_column.line, line_column.column,
                );
                return Err(vec![error]);
            }
            ValidationError::Invalid(invalid_error) => {
                fn add_error_locations<T: Display>(
                    error_vec: &mut Vec<String>,
                    error_locations: &BTreeMap<T, BTreeSet<Location>>,
                ) {
                    error_locations.iter().for_each(|(key, key_locations)| {
                        let error_locations_string = format!(
                            "{}",
                            key_locations
                                .iter()
                                .map(|location| location.to_string())
                                .collect::<Vec<_>>()
                                .join(",")
                        );
                        let error = format!(
                            "{} at {} locations: {}",
                            key,
                            key_locations.len(),
                            error_locations_string
                        );
                        error_vec.push(error);
                    });
                }
                let mut errors = Vec::new();
                add_error_locations(&mut errors, &invalid_error.invalid_node_locations);
                add_error_locations(&mut errors, &invalid_error.invalid_path_locations);
                errors.push(format!(
                    "Found {} validation errors",
                    &invalid_error.invalid_node_locations.len()
                        + &invalid_error.invalid_path_locations.len()
                ));
                Err(errors)
            }
        },
    }
}
