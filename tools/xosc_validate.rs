//! OpenSCENARIO XSD Validation Tool
//!
//! This binary validates OpenSCENARIO (.xosc) files against the XSD schema
//! using libxml for comprehensive schema validation.

use clap::{Parser, ValueEnum};
use libxml::parser::Parser as XmlParser;
use libxml::schemas::{SchemaParserContext, SchemaValidationContext};
use libxml::tree::Document;
use libxml::error::StructuredError;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(name = "xosc-validate")]
#[command(about = "Validate OpenSCENARIO files against XSD schema")]
#[command(version = "0.1.0")]
struct Args {
    /// Input XOSC file(s) to validate
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Path to XSD schema file
    #[arg(short, long, default_value = "Schema/OpenSCENARIO.xsd")]
    schema: PathBuf,

    /// Output format
    #[arg(short = 'f', long, default_value = "human")]
    format: OutputFormat,

    /// Quiet mode - only show summary
    #[arg(short, long)]
    quiet: bool,

    /// Validate all .xosc files in directory recursively
    #[arg(short, long)]
    recursive: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Human,
    Json,
    Junit,
}

#[derive(Debug)]
struct ValidationResult {
    file_path: PathBuf,
    is_valid: bool,
    errors: Vec<ValidationError>,
}

#[derive(Debug, Clone)]
struct ValidationError {
    line: Option<u32>,
    column: Option<u32>,
    message: String,
    error_type: String,
}

#[derive(Debug)]
enum ExitCode {
    Success = 0,
    ValidationErrors = 1,
    FileNotFound = 2,
    SchemaError = 3,
    InternalError = 99,
}

struct XsdValidator {
    schema_validation_context: SchemaValidationContext,
}

impl XsdValidator {
    fn new(xsd_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !xsd_path.exists() {
            return Err(format!("Schema file not found: {}", xsd_path.display()).into());
        }

        let xsd_content = fs::read_to_string(xsd_path)
            .map_err(|e| format!("Failed to read schema file: {}", e))?;

        let mut schema_parser = SchemaParserContext::from_buffer(&xsd_content);

        let schema_validation_context = SchemaValidationContext::from_parser(&mut schema_parser)
            .map_err(|e| format!("Failed to create validation context: {:?}", e))?;

        Ok(Self {
            schema_validation_context,
        })
    }

    fn validate_file(&mut self, xosc_path: &Path) -> Result<ValidationResult, Box<dyn std::error::Error>> {
        if !xosc_path.exists() {
            return Ok(ValidationResult {
                file_path: xosc_path.to_path_buf(),
                is_valid: false,
                errors: vec![ValidationError {
                    line: None,
                    column: None,
                    message: "File not found".to_string(),
                    error_type: "FileNotFound".to_string(),
                }],
            });
        }

        let xml_content = fs::read_to_string(xosc_path)
            .map_err(|e| format!("Failed to read XOSC file: {}", e))?;

        let parser = XmlParser::default();
        let document = parser
            .parse_string(&xml_content)
            .map_err(|e| format!("Failed to parse XML: {}", e))?;

        let validation_result = self.validate_document(&document, xosc_path);

        Ok(validation_result)
    }

    fn validate_document(&mut self, document: &Document, file_path: &Path) -> ValidationResult {
        match self.schema_validation_context.validate_document(document) {
            Ok(_) => ValidationResult {
                file_path: file_path.to_path_buf(),
                is_valid: true,
                errors: vec![],
            },
            Err(errors) => ValidationResult {
                file_path: file_path.to_path_buf(),
                is_valid: false,
                errors: parse_libxml_errors(errors),
            },
        }
    }
}

fn parse_libxml_errors(errors: Vec<StructuredError>) -> Vec<ValidationError> {
    errors
        .into_iter()
        .map(|err| ValidationError {
            line: err.line.map(|l| l as u32),
            column: err.col.map(|c| c as u32),
            message: err.message.as_ref().map(|s| s.trim().to_string()).unwrap_or_else(|| "Unknown error".to_string()),
            error_type: classify_error_type(err.message.as_ref().unwrap_or(&"Unknown error".to_string())),
        })
        .collect()
}

fn classify_error_type(message: &str) -> String {
    if message.contains("not expected") || message.contains("not allowed") {
        "ElementNotAllowed".to_string()
    } else if message.contains("missing") || message.contains("required") {
        "MissingRequired".to_string()
    } else if message.contains("invalid value") || message.contains("not valid") {
        "InvalidValue".to_string()
    } else if message.contains("type") {
        "TypeMismatch".to_string()
    } else {
        "ValidationError".to_string()
    }
}

fn collect_files(paths: &[PathBuf], recursive: bool) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    for path in paths {
        if path.is_file() {
            if path.extension().and_then(|s| s.to_str()) == Some("xosc") {
                files.push(path.clone());
            }
        } else if path.is_dir() && recursive {
            collect_files_from_dir(path, &mut files)?;
        } else if path.is_dir() {
            return Err(format!("Directory specified but --recursive not used: {}", path.display()).into());
        }
    }

    Ok(files)
}

fn collect_files_from_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("xosc") {
            files.push(path);
        } else if path.is_dir() {
            collect_files_from_dir(&path, files)?;
        }
    }

    Ok(())
}

fn format_results(results: &[ValidationResult], format: &OutputFormat, quiet: bool) -> String {
    match format {
        OutputFormat::Human => format_human(results, quiet),
        OutputFormat::Json => format_json(results),
        OutputFormat::Junit => format_junit(results),
    }
}

fn format_human(results: &[ValidationResult], quiet: bool) -> String {
    let mut output = String::new();

    if !quiet {
        for result in results {
            if result.is_valid {
                output.push_str(&format!("✓ {}\n", result.file_path.display()));
            } else {
                output.push_str(&format!("✗ {} ({} errors)\n", 
                    result.file_path.display(), result.errors.len()));
                
                for error in &result.errors {
                    let location = match (error.line, error.column) {
                        (Some(line), Some(col)) => format!("{}:{}", line, col),
                        (Some(line), None) => line.to_string(),
                        _ => "?".to_string(),
                    };
                    output.push_str(&format!("  Line {}: {} ({})\n", 
                        location, error.message, error.error_type));
                }
                output.push('\n');
            }
        }
    }

    // Summary
    let valid_count = results.iter().filter(|r| r.is_valid).count();
    let total_count = results.len();
    let error_count: usize = results.iter().map(|r| r.errors.len()).sum();

    output.push_str(&format!("Summary: {}/{} files valid", valid_count, total_count));
    if error_count > 0 {
        output.push_str(&format!(", {} total errors", error_count));
    }
    output.push('\n');

    output
}

fn format_json(results: &[ValidationResult]) -> String {
    use serde_json::{json, Value};

    let valid_count = results.iter().filter(|r| r.is_valid).count();
    let total_count = results.len();
    let error_count: usize = results.iter().map(|r| r.errors.len()).sum();

    let files: Vec<Value> = results
        .iter()
        .map(|result| {
            json!({
                "path": result.file_path.display().to_string(),
                "valid": result.is_valid,
                "errors": result.errors.iter().map(|e| {
                    json!({
                        "line": e.line,
                        "column": e.column,
                        "message": e.message,
                        "type": e.error_type
                    })
                }).collect::<Vec<_>>()
            })
        })
        .collect();

    let output = json!({
        "summary": {
            "total": total_count,
            "valid": valid_count,
            "invalid": total_count - valid_count,
            "total_errors": error_count
        },
        "files": files
    });

    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}

fn format_junit(results: &[ValidationResult]) -> String {
    let mut output = String::new();
    output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    
    let valid_count = results.iter().filter(|r| r.is_valid).count();
    let total_count = results.len();
    let failure_count = total_count - valid_count;
    
    output.push_str(&format!(
        "<testsuite tests=\"{}\" failures=\"{}\" name=\"OpenSCENARIO Validation\">\n",
        total_count, failure_count
    ));
    
    for result in results {
        let test_name = result.file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
            
        if result.is_valid {
            output.push_str(&format!("  <testcase name=\"{}\"/>\n", test_name));
        } else {
            output.push_str(&format!("  <testcase name=\"{}\">\n", test_name));
            for error in &result.errors {
                output.push_str(&format!(
                    "    <failure message=\"{}\" type=\"{}\">{}</failure>\n",
                    error.message, error.error_type, error.message
                ));
            }
            output.push_str("  </testcase>\n");
        }
    }
    
    output.push_str("</testsuite>\n");
    output
}

fn main() {
    let args = Args::parse();

    let exit_code = match run_validation(args) {
        Ok(has_errors) => {
            if has_errors {
                ExitCode::ValidationErrors
            } else {
                ExitCode::Success
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            if e.to_string().contains("not found") {
                ExitCode::FileNotFound
            } else if e.to_string().contains("schema") {
                ExitCode::SchemaError
            } else {
                ExitCode::InternalError
            }
        }
    };

    process::exit(exit_code as i32);
}

fn run_validation(args: Args) -> Result<bool, Box<dyn std::error::Error>> {
    // Create validator
    let mut validator = XsdValidator::new(&args.schema)?;

    // Collect files to validate
    let files = collect_files(&args.files, args.recursive)?;

    if files.is_empty() {
        return Err("No .xosc files found to validate".into());
    }

    // Validate all files
    let mut results = Vec::new();
    for file in files {
        let result = validator.validate_file(&file)?;
        results.push(result);
    }

    // Format and print results
    let output = format_results(&results, &args.format, args.quiet);
    print!("{}", output);

    // Return whether there were any validation errors
    let has_errors = results.iter().any(|r| !r.is_valid);
    Ok(has_errors)
}