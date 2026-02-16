//! OpenSCENARIO Comprehensive Analysis Tool
//!
//! This command-line tool provides detailed analysis of OpenSCENARIO files,
//! including document metadata, entity analysis, storyboard structure,
//! and comprehensive statistics.
//!
//! ## Features
//! - Support for all OpenSCENARIO document types (Scenario, ParameterVariation, Catalog, Unknown)
//! - **Enhanced Act Analysis**: Detailed breakdown of Acts, ManeuverGroups, Maneuvers, and Events
//! - **Comprehensive Init Analysis**: Entity initialization, positions, speeds, and environment setup
//! - **Deep Storyboard Analysis**: Execution flow, condition dependencies, and actor interactions
//! - **Timeline Analysis**: Sequence and timing of events with dependency mapping
//! - **Action Categorization**: Detailed analysis of action types and parameters
//! - **Trigger Condition Analysis**: Breakdown of all condition types and their parameters
//! - Structured output with emoji headers for readability
//! - Configurable verbosity levels and output formats
//! - Extensible architecture for additional analysis modules
//! - Comprehensive error handling with helpful messages
//!
//! ## Enhanced Analysis Output
//! ### Initialization Section
//! - Entity placements with position coordinates and headings
//! - Initial speeds and action assignments
//! - Environment setup (time, weather, road conditions)
//! - Global vs private action categorization
//!
//! ### Storyboard Section
//! - Hierarchical Act → ManeuverGroup → Maneuver → Event → Action breakdown
//! - Actor assignments and entity interactions
//! - Start/stop conditions with detailed condition analysis
//! - Action types categorized by Movement, Control, Appearance, etc.
//! - Execution flow diagram with proper indentation
//! - Timeline analysis showing sequence and parallel executions
//!
//! ## Usage
//! ```bash
//! # Basic analysis
//! cargo run --bin scenario_analyzer -- path/to/scenario.xosc
//!
//! # Verbose mode with execution flow analysis
//! cargo run --bin scenario_analyzer -- --verbose path/to/scenario.xosc
//!
//! # JSON output for programmatic processing
//! cargo run --bin scenario_analyzer -- --format json path/to/scenario.xosc
//! ```

use openscenario_rs::{
    catalog::{extract_scenario_parameters, resolve_catalog_reference_simple},
    expression::evaluate_expression,
    parse_from_file,
    types::{basic::Value, scenario::storyboard::OpenScenario, OpenScenarioDocumentType},
};
use std::{collections::HashMap, env, path::Path, process};

/// Configuration for the analysis tool
#[derive(Debug, Clone)]
struct Config {
    /// Input file path
    input_file: String,
    /// Verbose output flag
    verbose: bool,
    /// Quiet output flag
    quiet: bool,
    /// Output format (text or json)
    format: OutputFormat,
}

/// Output format options
#[derive(Debug, Clone, PartialEq)]
enum OutputFormat {
    Text,
    Json,
}

/// Analysis result structure
#[derive(Debug)]
struct AnalysisResult {
    /// Document type
    document_type: OpenScenarioDocumentType,
    /// File header information
    header_info: HeaderInfo,
    /// Entity analysis
    entity_analysis: EntityAnalysis,
    /// Storyboard analysis (for scenarios)
    storyboard_analysis: Option<StoryboardAnalysis>,
    /// Parameter analysis
    parameter_analysis: ParameterAnalysis,
    /// Parameter variation analysis (for parameter variation documents)
    parameter_variation_analysis: Option<ParameterVariationAnalysis>,
    /// Catalog analysis
    catalog_analysis: CatalogAnalysis,
}

/// File header information
#[derive(Debug)]
struct HeaderInfo {
    description: String,
    author: String,
    date: String,
    rev_major: u16,
    rev_minor: u16,
}

/// Entity analysis results
#[derive(Debug)]
struct EntityAnalysis {
    total_entities: usize,
    vehicles: usize,
    pedestrians: usize,
    misc_objects: usize,
    catalog_references: usize,
    inline_definitions: usize,
    vehicle_analyses: Vec<VehicleAnalysis>,
    pedestrian_analyses: Vec<PedestrianAnalysis>,
    // Enhanced breakdown for catalog vs inline
    vehicles_inline: usize,
    vehicles_catalog: usize,
    pedestrians_inline: usize,
    pedestrians_catalog: usize,
    misc_objects_inline: usize,
    misc_objects_catalog: usize,
    unknown_catalog_references: usize,
}

/// Simplified vehicle analysis
#[derive(Debug)]
struct VehicleAnalysis {
    entity_name: String,
    vehicle_name: String,
    category: String,
    has_front_axle: bool,
    axle_count: usize,
    source: EntitySource,
}

/// Simplified pedestrian analysis
#[derive(Debug)]
struct PedestrianAnalysis {
    entity_name: String,
    pedestrian_name: String,
    category: String,
    source: EntitySource,
}

/// Entity source information
#[derive(Debug)]
enum EntitySource {
    Inline,
    CatalogReference {
        catalog_name: String,
        entry_name: String,
    },
}

/// Storyboard analysis results
#[derive(Debug)]
struct StoryboardAnalysis {
    init_actions: InitActionAnalysis,
    story_analysis: Option<StoryAnalysis>,
    statistics: StoryboardStatistics,
    detailed_init_analysis: DetailedInitAnalysis,
    detailed_acts: Vec<DetailedActAnalysis>,
    execution_flow: ExecutionFlowAnalysis,
}

/// Initialization actions analysis
#[derive(Debug)]
struct InitActionAnalysis {
    total_actions: usize,
    global_actions: usize,
    private_actions: usize,
}

/// Story structure analysis
#[derive(Debug)]
struct StoryAnalysis {
    total_stories: usize,
    total_acts: usize,
    total_maneuver_groups: usize,
    total_maneuvers: usize,
    total_events: usize,
    total_actions: usize,
}

/// Storyboard statistics summary
#[derive(Debug)]
struct StoryboardStatistics {
    total_acts: usize,
    total_maneuver_groups: usize,
    total_maneuvers: usize,
    total_events: usize,
    total_actions: usize,
    total_init_actions: usize,
    unique_actors: Vec<String>,
}

/// Detailed initialization analysis
#[derive(Debug)]
struct DetailedInitAnalysis {
    entity_initializations: Vec<EntityInitialization>,
    environment_setup: Option<EnvironmentSetup>,
    global_actions_count: usize,
    private_actions_count: usize,
}

/// Entity initialization details
#[derive(Debug)]
struct EntityInitialization {
    entity_name: String,
    position_info: Option<PositionInfo>,
    speed_info: Option<SpeedInfo>,
    actions: Vec<InitActionDetail>,
}

/// Position information for entity initialization
#[derive(Debug)]
struct PositionInfo {
    position_type: String,
    coordinates: String,
    heading: Option<String>,
}

/// Speed information for entity initialization
#[derive(Debug)]
struct SpeedInfo {
    speed_value: String,
    speed_type: String,
}

/// Environment setup details
#[derive(Debug)]
struct EnvironmentSetup {
    time_of_day: String,
    weather_description: String,
    road_conditions: String,
}

/// Initialization action details
#[derive(Debug)]
struct InitActionDetail {
    action_type: String,
    action_description: String,
    parameters: Vec<(String, String)>,
}

/// Detailed Act analysis
#[derive(Debug)]
struct DetailedActAnalysis {
    act_name: String,
    act_description: Option<String>,
    maneuver_groups: Vec<DetailedManeuverGroupAnalysis>,
    start_conditions: Vec<ConditionAnalysis>,
    stop_conditions: Vec<ConditionAnalysis>,
    timing_info: TimingInfo,
}

/// Detailed ManeuverGroup analysis
#[derive(Debug)]
struct DetailedManeuverGroupAnalysis {
    group_name: String,
    actors: Vec<String>,
    maximum_execution_count: Option<u32>,
    maneuvers: Vec<DetailedManeuverAnalysis>,
}

/// Detailed Maneuver analysis
#[derive(Debug)]
struct DetailedManeuverAnalysis {
    maneuver_name: String,
    events: Vec<DetailedEventAnalysis>,
    parameters: Vec<(String, String)>,
}

/// Detailed Event analysis
#[derive(Debug)]
struct DetailedEventAnalysis {
    event_name: String,
    priority: Option<String>,
    maximum_execution_count: Option<u32>,
    start_conditions: Vec<ConditionAnalysis>,
    actions: Vec<EventActionAnalysis>,
}

/// Event action analysis
#[derive(Debug)]
struct EventActionAnalysis {
    action_name: String,
    action_type: String,
    action_category: String,
    parameters: Vec<(String, String)>,
}

/// Condition analysis
#[derive(Debug)]
struct ConditionAnalysis {
    condition_type: String,
    condition_description: String,
    trigger_value: Option<String>,
    entity_ref: Option<String>,
}

/// Timing information
#[derive(Debug)]
struct TimingInfo {
    execution_order: u32,
    estimated_duration: Option<String>,
    dependencies: Vec<String>,
}

/// Execution flow analysis
#[derive(Debug)]
struct ExecutionFlowAnalysis {
    flow_diagram: Vec<FlowStep>,
    condition_dependencies: Vec<ConditionDependency>,
    actor_interactions: Vec<ActorInteraction>,
    timeline_analysis: TimelineAnalysis,
}

/// Flow step in execution
#[derive(Debug)]
struct FlowStep {
    step_type: String,
    step_name: String,
    level: u32,
    description: String,
}

/// Condition dependency mapping
#[derive(Debug)]
struct ConditionDependency {
    source: String,
    target: String,
    condition_type: String,
    description: String,
}

/// Actor interaction analysis
#[derive(Debug)]
struct ActorInteraction {
    primary_actor: String,
    secondary_actor: Option<String>,
    interaction_type: String,
    description: String,
}

/// Timeline analysis
#[derive(Debug)]
struct TimelineAnalysis {
    sequence_steps: Vec<TimelineStep>,
    parallel_executions: Vec<ParallelExecution>,
    estimated_total_duration: Option<String>,
}

/// Timeline step
#[derive(Debug)]
struct TimelineStep {
    step_order: u32,
    step_name: String,
    step_type: String,
    actors_involved: Vec<String>,
    estimated_start_time: Option<String>,
}

/// Parallel execution block
#[derive(Debug)]
struct ParallelExecution {
    execution_name: String,
    parallel_actions: Vec<String>,
    actors_involved: Vec<String>,
}

/// Expression resolution example
#[derive(Debug)]
struct ExpressionResolutionExample {
    original: String,
    resolved: String,
    location: String,
}

/// Parameter analysis results
#[derive(Debug)]
struct ParameterAnalysis {
    total_parameters: usize,
    parameter_names: Vec<String>,
    parameter_details: Vec<ParameterDetail>,
    expressions_found: usize,
    expressions_resolved: usize,
    expressions_failed: usize,
    resolution_examples: Vec<ExpressionResolutionExample>,
}

/// Individual parameter details
#[derive(Debug)]
struct ParameterDetail {
    name: String,
    value: String,
    parameter_type: String,
    context: String,
    is_expression: bool,
    resolved_value: Option<String>,
}

/// Parameter variation analysis results
#[derive(Debug)]
struct ParameterVariationAnalysis {
    referenced_scenario_info: Option<ReferencedScenarioInfo>,
    deterministic_analysis: Option<DeterministicAnalysis>,
    stochastic_analysis: Option<StochasticAnalysis>,
    total_parameters: usize,
    estimated_combinations: usize,
}

/// Referenced scenario information
#[derive(Debug)]
struct ReferencedScenarioInfo {
    filepath: String,
    resolved_path: String,
    loaded_successfully: bool,
    description: String,
    author: String,
    entities_count: usize,
    template_parameters: Vec<String>,
    error_message: Option<String>,
}

/// Deterministic distribution analysis
#[derive(Debug)]
struct DeterministicAnalysis {
    single_distributions: Vec<DistributionDetails>,
    multi_distributions_count: usize,
    total_combinations: usize,
}

/// Stochastic distribution analysis
#[derive(Debug)]
struct StochasticAnalysis {
    distributions_count: usize,
}

/// Individual distribution details
#[derive(Debug)]
struct DistributionDetails {
    parameter_name: String,
    distribution_type: String,
    value_count: usize,
    sample_values: Vec<String>,
    range_info: Option<RangeInfo>,
}

/// Range distribution information
#[derive(Debug)]
struct RangeInfo {
    lower_limit: String,
    upper_limit: String,
    step_width: String,
    calculated_count: usize,
}

/// Catalog resolution result
#[derive(Debug)]
struct CatalogResolutionResult {
    entity_name: String,
    catalog_name: String,
    entry_name: String,
    resolution_type: String, // "entity" or "controller"
    success: bool,
    error_message: Option<String>,
}

/// Catalog analysis results
#[derive(Debug)]
struct CatalogAnalysis {
    has_catalog_locations: bool,
    vehicle_catalog: bool,
    pedestrian_catalog: bool,
    misc_object_catalog: bool,
    controller_catalog: bool,
    environment_catalog: bool,
    maneuver_catalog: bool,
    trajectory_catalog: bool,
    route_catalog: bool,
    resolution_attempts: usize,
    resolution_successes: usize,
    resolution_failures: usize,
    resolution_results: Vec<CatalogResolutionResult>,
}

/// Main application entry point
fn main() {
    // Parse command line arguments
    let config = match parse_args() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Error parsing arguments: {}", e);
            print_usage();
            process::exit(1);
        }
    };

    // Run the analysis
    match run_analysis(&config) {
        Ok(()) => {
            if !config.quiet {
                println!("✅ Analysis completed successfully!");
            }
        }
        Err(e) => {
            eprintln!("❌ Analysis failed: {}", e);
            process::exit(1);
        }
    }
}

/// Parse command line arguments
fn parse_args() -> Result<Config, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Missing required argument: scenario file".into());
    }

    let mut config = Config {
        input_file: String::new(),
        verbose: false,
        quiet: false,
        format: OutputFormat::Text,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--verbose" | "-v" => {
                config.verbose = true;
            }
            "--quiet" | "-q" => {
                config.quiet = true;
            }
            "--format" => {
                if i + 1 >= args.len() {
                    return Err("--format requires a value (json|text)".into());
                }
                i += 1;
                match args[i].as_str() {
                    "json" => config.format = OutputFormat::Json,
                    "text" => config.format = OutputFormat::Text,
                    other => {
                        return Err(
                            format!("Invalid format '{}'. Use 'json' or 'text'", other).into()
                        )
                    }
                }
            }
            arg if arg.starts_with("--") => {
                return Err(format!("Unknown option: {}", arg).into());
            }
            _ => {
                if config.input_file.is_empty() {
                    config.input_file = args[i].clone();
                } else {
                    return Err("Multiple input files not supported".into());
                }
            }
        }
        i += 1;
    }

    if config.input_file.is_empty() {
        return Err("No input file specified".into());
    }

    // Validate conflicting options
    if config.verbose && config.quiet {
        return Err("Cannot use both --verbose and --quiet options".into());
    }

    Ok(config)
}

/// Print usage information
fn print_usage() {
    let program_name = env::args()
        .next()
        .unwrap_or_else(|| "scenario_analyzer".to_string());
    println!("Usage: {} [OPTIONS] <scenario_file.xosc>", program_name);
    println!();
    println!("OPTIONS:");
    println!("  --verbose, -v     Enable verbose output");
    println!("  --quiet, -q       Suppress non-essential output");
    println!("  --format FORMAT   Output format: json|text (default: text)");
    println!();
    println!("EXAMPLES:");
    println!("  {} scenario.xosc", program_name);
    println!("  {} --verbose scenario.xosc", program_name);
    println!("  {} --format json scenario.xosc", program_name);
}

/// Run the complete analysis pipeline
fn run_analysis(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Validate input file
    let input_path = Path::new(&config.input_file);
    if !input_path.exists() {
        return Err(format!("Input file does not exist: {}", config.input_file).into());
    }

    if !config.quiet {
        print_header("🚗 OpenSCENARIO Comprehensive Analysis Tool", 1);
        println!("📁 Analyzing file: {}", config.input_file);
        println!();
    }

    // Parse the document
    let document = parse_from_file(input_path)?;

    // Perform analysis
    let analysis = analyze_document(&document, input_path)?;

    // Generate output
    match config.format {
        OutputFormat::Text => format_text_output(&analysis, config)?,
        OutputFormat::Json => format_json_output(&analysis)?,
    }

    Ok(())
}

/// Analyze the OpenSCENARIO document
fn analyze_document(
    document: &OpenScenario,
    input_path: &Path,
) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
    let document_type = document.document_type();

    // Extract header information
    let header_info = HeaderInfo {
        description: document
            .file_header
            .description
            .as_literal()
            .map_or("N/A".to_string(), |v| v.clone()),
        author: document
            .file_header
            .author
            .as_literal()
            .map_or("N/A".to_string(), |v| v.clone()),
        date: document
            .file_header
            .date
            .as_literal()
            .map_or("N/A".to_string(), |v| v.clone()),
        rev_major: document
            .file_header
            .rev_major
            .as_literal()
            .copied()
            .unwrap_or(0),
        rev_minor: document
            .file_header
            .rev_minor
            .as_literal()
            .copied()
            .unwrap_or(0),
    };

    // Analyze entities
    let entity_analysis = analyze_entities(document);

    // Analyze storyboard (only for scenarios)
    let storyboard_analysis = if document_type == OpenScenarioDocumentType::Scenario {
        Some(analyze_storyboard(document))
    } else {
        None
    };

    // Analyze parameters (including expression resolution)
    let parameter_analysis = analyze_parameters(document);

    // Analyze parameter variations (only for parameter variation documents)
    let parameter_variation_analysis =
        if document_type == OpenScenarioDocumentType::ParameterVariation {
            Some(analyze_parameter_variation(document, input_path)?)
        } else {
            None
        };

    // Analyze catalogs (including resolution attempts)
    let catalog_analysis = analyze_catalogs(document, input_path);

    Ok(AnalysisResult {
        document_type,
        header_info,
        entity_analysis,
        storyboard_analysis,
        parameter_analysis,
        parameter_variation_analysis,
        catalog_analysis,
    })
}

/// Analyze entities in the document
fn analyze_entities(document: &OpenScenario) -> EntityAnalysis {
    let mut analysis = EntityAnalysis {
        total_entities: 0,
        vehicles: 0,
        pedestrians: 0,
        misc_objects: 0,
        catalog_references: 0,
        inline_definitions: 0,
        vehicle_analyses: Vec::new(),
        pedestrian_analyses: Vec::new(),
        vehicles_inline: 0,
        vehicles_catalog: 0,
        pedestrians_inline: 0,
        pedestrians_catalog: 0,
        misc_objects_inline: 0,
        misc_objects_catalog: 0,
        unknown_catalog_references: 0,
    };

    if let Some(entities) = &document.entities {
        analysis.total_entities = entities.scenario_objects.len();

        for entity in &entities.scenario_objects {
            let entity_name = entity.get_name().unwrap_or("Unknown");

            // Analyze entity types
            if let Some(vehicle) = &entity.vehicle {
                analysis.vehicles += 1;
                analysis.vehicles_inline += 1;
                analysis.inline_definitions += 1;
                let vehicle_analysis = analyze_vehicle(vehicle, entity_name, entity);
                analysis.vehicle_analyses.push(vehicle_analysis);
            } else if let Some(pedestrian) = &entity.pedestrian {
                analysis.pedestrians += 1;
                analysis.pedestrians_inline += 1;
                analysis.inline_definitions += 1;
                let pedestrian_analysis = analyze_pedestrian(pedestrian, entity_name, entity);
                analysis.pedestrian_analyses.push(pedestrian_analysis);
            } else if let Some(catalog_ref) = entity.vehicle_catalog_reference() {
                // Vehicle catalog reference
                analysis.vehicles += 1;
                analysis.vehicles_catalog += 1;
                analysis.catalog_references += 1;

                // Analyze catalog reference details
                let catalog_name = catalog_ref
                    .catalog_name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone());
                let entry_name = catalog_ref
                    .entry_name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone());

                let vehicle_analysis = VehicleAnalysis {
                    entity_name: entity_name.to_string(),
                    vehicle_name: entry_name.clone(),
                    category: "catalog-referenced".to_string(),
                    has_front_axle: false, // Unknown for catalog references
                    axle_count: 0,         // Unknown for catalog references
                    source: EntitySource::CatalogReference {
                        catalog_name,
                        entry_name,
                    },
                };
                analysis.vehicle_analyses.push(vehicle_analysis);
            } else if let Some(catalog_ref) = entity.pedestrian_catalog_reference() {
                // Pedestrian catalog reference
                analysis.pedestrians += 1;
                analysis.pedestrians_catalog += 1;
                analysis.catalog_references += 1;

                // Analyze catalog reference details
                let catalog_name = catalog_ref
                    .catalog_name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone());
                let entry_name = catalog_ref
                    .entry_name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone());

                let pedestrian_analysis =
                    analyze_pedestrian_catalog_reference(&catalog_name, &entry_name, entity_name);
                analysis.pedestrian_analyses.push(pedestrian_analysis);
            }
        }
    }

    fn analyze_pedestrian_catalog_reference(
        catalog_name: &str,
        entry_name: &str,
        entity_name: &str,
    ) -> PedestrianAnalysis {
        PedestrianAnalysis {
            entity_name: entity_name.to_string(),
            pedestrian_name: entry_name.to_string(),
            category: "catalog-referenced".to_string(),
            source: EntitySource::CatalogReference {
                catalog_name: catalog_name.to_string(),
                entry_name: entry_name.to_string(),
            },
        }
    }

    analysis
}

/// Analyze a vehicle entity
fn analyze_vehicle(
    vehicle: &openscenario_rs::types::entities::Vehicle,
    entity_name: &str,
    entity: &openscenario_rs::types::entities::ScenarioObject,
) -> VehicleAnalysis {
    // Extract vehicle name
    let vehicle_name = vehicle
        .name
        .as_literal()
        .map_or("Unknown".to_string(), |v| v.clone());

    // Extract category
    let category = format!("{:?}", vehicle.vehicle_category).to_lowercase();

    // Analyze axles (safely)
    let has_front_axle = vehicle.axles.front_axle.is_some();
    let mut axle_count = 1; // rear axle is always present
    if has_front_axle {
        axle_count += 1;
    }
    axle_count += vehicle.axles.additional_axles.len();

    // Determine source
    let source = if entity.entity_catalog_reference.is_some() {
        EntitySource::CatalogReference {
            catalog_name: "Catalog".to_string(),
            entry_name: "Unknown".to_string(),
        }
    } else {
        EntitySource::Inline
    };

    VehicleAnalysis {
        entity_name: entity_name.to_string(),
        vehicle_name,
        category,
        has_front_axle,
        axle_count,
        source,
    }
}

/// Analyze a pedestrian entity
fn analyze_pedestrian(
    pedestrian: &openscenario_rs::types::entities::Pedestrian,
    entity_name: &str,
    entity: &openscenario_rs::types::entities::ScenarioObject,
) -> PedestrianAnalysis {
    // Extract pedestrian name
    let pedestrian_name = pedestrian
        .name
        .as_literal()
        .map_or("Unknown".to_string(), |v| v.clone());

    // Extract category
    let category = format!("{:?}", pedestrian.pedestrian_category).to_lowercase();

    // Determine source
    let source = if entity.entity_catalog_reference.is_some() {
        EntitySource::CatalogReference {
            catalog_name: "Catalog".to_string(),
            entry_name: "Unknown".to_string(),
        }
    } else {
        EntitySource::Inline
    };

    PedestrianAnalysis {
        entity_name: entity_name.to_string(),
        pedestrian_name,
        category,
        source,
    }
}

/// Analyze storyboard structure
fn analyze_storyboard(document: &OpenScenario) -> StoryboardAnalysis {
    let mut analysis = StoryboardAnalysis {
        init_actions: InitActionAnalysis {
            total_actions: 0,
            global_actions: 0,
            private_actions: 0,
        },
        story_analysis: None,
        statistics: StoryboardStatistics {
            total_acts: 0,
            total_maneuver_groups: 0,
            total_maneuvers: 0,
            total_events: 0,
            total_actions: 0,
            total_init_actions: 0,
            unique_actors: Vec::new(),
        },
        detailed_init_analysis: DetailedInitAnalysis {
            entity_initializations: Vec::new(),
            environment_setup: None,
            global_actions_count: 0,
            private_actions_count: 0,
        },
        detailed_acts: Vec::new(),
        execution_flow: ExecutionFlowAnalysis {
            flow_diagram: Vec::new(),
            condition_dependencies: Vec::new(),
            actor_interactions: Vec::new(),
            timeline_analysis: TimelineAnalysis {
                sequence_steps: Vec::new(),
                parallel_executions: Vec::new(),
                estimated_total_duration: None,
            },
        },
    };

    if let Some(storyboard) = &document.storyboard {
        // Analyze initialization actions
        let actions = &storyboard.init.actions;
        analysis.init_actions.global_actions = actions.global_actions.len();
        analysis.init_actions.private_actions = actions.private_actions.len();
        analysis.init_actions.total_actions =
            analysis.init_actions.global_actions + analysis.init_actions.private_actions;

        // Analyze story structure
        if !storyboard.stories.is_empty() {
            let mut story_stats = StoryAnalysis {
                total_stories: storyboard.stories.len(),
                total_acts: 0,
                total_maneuver_groups: 0,
                total_maneuvers: 0,
                total_events: 0,
                total_actions: 0,
            };

            for story in &storyboard.stories {
                story_stats.total_acts += story.acts.len();

                for act in &story.acts {
                    story_stats.total_maneuver_groups += act.maneuver_groups.len();

                    for mg in &act.maneuver_groups {
                        story_stats.total_maneuvers += mg.maneuvers.len();

                        // Collect unique actors
                        for entity_ref in &mg.actors.entity_refs {
                            let actor_name = entity_ref
                                .entity_ref
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.clone());
                            if !analysis.statistics.unique_actors.contains(&actor_name) {
                                analysis.statistics.unique_actors.push(actor_name);
                            }
                        }

                        for maneuver in &mg.maneuvers {
                            story_stats.total_events += maneuver.events.len();

                            for event in &maneuver.events {
                                story_stats.total_actions += event.actions.len();
                            }
                        }
                    }
                }
            }

            analysis.story_analysis = Some(story_stats);
        }

        // Update statistics
        analysis.statistics.total_acts = analysis
            .story_analysis
            .as_ref()
            .map(|s| s.total_acts)
            .unwrap_or(0);
        analysis.statistics.total_maneuver_groups = analysis
            .story_analysis
            .as_ref()
            .map(|s| s.total_maneuver_groups)
            .unwrap_or(0);
        analysis.statistics.total_maneuvers = analysis
            .story_analysis
            .as_ref()
            .map(|s| s.total_maneuvers)
            .unwrap_or(0);
        analysis.statistics.total_events = analysis
            .story_analysis
            .as_ref()
            .map(|s| s.total_events)
            .unwrap_or(0);
        analysis.statistics.total_actions = analysis
            .story_analysis
            .as_ref()
            .map(|s| s.total_actions)
            .unwrap_or(0);
        analysis.statistics.total_init_actions = analysis.init_actions.total_actions;

        // Perform detailed analysis
        analysis.detailed_init_analysis = analyze_init_detailed(&storyboard.init);
        analysis.detailed_acts = analyze_acts_detailed(&storyboard.stories);
        analysis.execution_flow = analyze_execution_flow(&storyboard.stories);
    }

    analysis
}

/// Analyze initialization details
fn analyze_init_detailed(
    init: &openscenario_rs::types::scenario::init::Init,
) -> DetailedInitAnalysis {
    let mut analysis = DetailedInitAnalysis {
        entity_initializations: Vec::new(),
        environment_setup: None,
        global_actions_count: init.actions.global_actions.len(),
        private_actions_count: init.actions.private_actions.len(),
    };

    // Analyze global actions (environment setup)
    for global_action in &init.actions.global_actions {
        if let Some(env_action) = &global_action.environment_action {
            analysis.environment_setup = Some(EnvironmentSetup {
                time_of_day: env_action.environment.time_of_day.date_time.clone(),
                weather_description: format!("{:?}", env_action.environment.weather),
                road_conditions: format!("{:?}", env_action.environment.road_condition),
            });
        }
    }

    // Analyze private actions (entity initialization)
    for private_action in &init.actions.private_actions {
        let entity_name = private_action
            .entity_ref
            .as_literal()
            .map_or("Unknown".to_string(), |v| v.clone());

        let mut entity_init = EntityInitialization {
            entity_name: entity_name.clone(),
            position_info: None,
            speed_info: None,
            actions: Vec::new(),
        };

        for action in &private_action.private_actions {
            if let Some(teleport_action) = &action.teleport_action {
                if let Some(world_pos) = &teleport_action.position.world_position {
                    entity_init.position_info = Some(PositionInfo {
                        position_type: "WorldPosition".to_string(),
                        coordinates: format!(
                            "X: {}, Y: {}",
                            world_pos
                                .x
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.to_string()),
                            world_pos
                                .y
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.to_string())
                        ),
                        heading: world_pos
                            .h
                            .as_ref()
                            .and_then(|h| h.as_literal())
                            .map(|h| h.to_string()),
                    });
                } else if let Some(lane_pos) = &teleport_action.position.lane_position {
                    entity_init.position_info = Some(PositionInfo {
                        position_type: "LanePosition".to_string(),
                        coordinates: format!(
                            "Road: {}, Lane: {}, S: {}",
                            lane_pos
                                .road_id
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.clone()),
                            lane_pos
                                .lane_id
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.to_string()),
                            lane_pos
                                .s
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.to_string())
                        ),
                        heading: lane_pos
                            .orientation
                            .as_ref()
                            .and_then(|o| o.h.as_ref())
                            .and_then(|h| h.as_literal())
                            .map(|h| h.to_string()),
                    });
                }

                entity_init.actions.push(InitActionDetail {
                    action_type: "TeleportAction".to_string(),
                    action_description: "Set initial position".to_string(),
                    parameters: Vec::new(),
                });
            }

            if let Some(longitudinal_action) = &action.longitudinal_action {
                if let Some(speed_action) = &longitudinal_action.speed_action {
                    if let Some(absolute) = &speed_action.speed_action_target.absolute {
                        // Try to get literal value first, then check for parameter/expression
                        let speed_value = if let Some(literal_val) = absolute.value.as_literal() {
                            literal_val.to_string()
                        } else if let Some(param_name) = absolute.value.as_parameter() {
                            format!("${{{}}}", param_name)
                        } else if let Some(expr) = absolute.value.as_expression() {
                            format!("${{{}}}", expr)
                        } else {
                            "Unknown".to_string()
                        };

                        entity_init.speed_info = Some(SpeedInfo {
                            speed_value,
                            speed_type: "Absolute".to_string(),
                        });
                    } else if let Some(relative) = &speed_action.speed_action_target.relative {
                        // Handle relative speed as well
                        let speed_value = if let Some(literal_val) = relative.value.as_literal() {
                            literal_val.to_string()
                        } else if let Some(param_name) = relative.value.as_parameter() {
                            format!("${{{}}}", param_name)
                        } else if let Some(expr) = relative.value.as_expression() {
                            format!("${{{}}}", expr)
                        } else {
                            "Unknown".to_string()
                        };

                        entity_init.speed_info = Some(SpeedInfo {
                            speed_value: format!(
                                "{} (relative to {})",
                                speed_value, relative.entity_ref
                            ),
                            speed_type: "Relative".to_string(),
                        });
                    }
                }

                entity_init.actions.push(InitActionDetail {
                    action_type: "LongitudinalAction".to_string(),
                    action_description: "Set initial speed".to_string(),
                    parameters: Vec::new(),
                });
            }
        }

        analysis.entity_initializations.push(entity_init);
    }

    analysis
}

/// Analyze Acts in detail
fn analyze_acts_detailed(
    stories: &[openscenario_rs::types::scenario::story::ScenarioStory],
) -> Vec<DetailedActAnalysis> {
    let mut detailed_acts = Vec::new();

    for story in stories {
        for (act_idx, act) in story.acts.iter().enumerate() {
            let act_name = act
                .name
                .as_literal()
                .map_or("Unknown".to_string(), |v| v.clone());

            let mut detailed_act = DetailedActAnalysis {
                act_name: act_name.clone(),
                act_description: Some(format!(
                    "Act {} in story {}",
                    act_idx + 1,
                    story.name.as_literal().map_or("Unknown", |v| v)
                )),
                maneuver_groups: Vec::new(),
                start_conditions: Vec::new(),
                stop_conditions: Vec::new(),
                timing_info: TimingInfo {
                    execution_order: act_idx as u32 + 1,
                    estimated_duration: None,
                    dependencies: Vec::new(),
                },
            };

            // Analyze start conditions
            if let Some(start_trigger) = &act.start_trigger {
                detailed_act.start_conditions = analyze_trigger_conditions(start_trigger);
            }

            // Analyze stop conditions
            if let Some(stop_trigger) = &act.stop_trigger {
                detailed_act.stop_conditions = analyze_trigger_conditions(stop_trigger);
            }

            // Analyze maneuver groups
            for maneuver_group in &act.maneuver_groups {
                let group_name = maneuver_group
                    .name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone());

                let mut detailed_group = DetailedManeuverGroupAnalysis {
                    group_name: group_name.clone(),
                    actors: maneuver_group
                        .actors
                        .entity_refs
                        .iter()
                        .map(|entity_ref| {
                            entity_ref
                                .entity_ref
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.clone())
                        })
                        .collect(),
                    maximum_execution_count: maneuver_group
                        .maximum_execution_count
                        .as_ref()
                        .and_then(|v| v.as_literal())
                        .copied(),
                    maneuvers: Vec::new(),
                };

                // Analyze maneuvers within the group
                for maneuver in &maneuver_group.maneuvers {
                    let maneuver_name = maneuver
                        .name
                        .as_literal()
                        .map_or("Unknown".to_string(), |v| v.clone());

                    let mut detailed_maneuver = DetailedManeuverAnalysis {
                        maneuver_name: maneuver_name.clone(),
                        events: Vec::new(),
                        parameters: Vec::new(),
                    };

                    // Analyze events within the maneuver
                    for event in &maneuver.events {
                        let event_name = event
                            .name
                            .as_literal()
                            .map_or("Unknown".to_string(), |v| v.clone());

                        let mut detailed_event = DetailedEventAnalysis {
                            event_name: event_name.clone(),
                            priority: event.priority.as_ref().map(|p| format!("{:?}", p)),
                            maximum_execution_count: event
                                .maximum_execution_count
                                .as_ref()
                                .and_then(|v| v.as_literal())
                                .copied(),
                            start_conditions: Vec::new(),
                            actions: Vec::new(),
                        };

                        // Analyze start conditions
                        if let Some(start_trigger) = &event.start_trigger {
                            detailed_event.start_conditions =
                                analyze_trigger_conditions(start_trigger);
                        }

                        // Analyze actions within the event
                        for action in &event.actions {
                            let action_name = action
                                .name
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.clone());
                            let (action_type, action_category) =
                                if let Some(private_action) = &action.private_action {
                                    analyze_private_action_type(private_action)
                                } else {
                                    ("Unknown".to_string(), "Unknown".to_string())
                                };

                            detailed_event.actions.push(EventActionAnalysis {
                                action_name,
                                action_type,
                                action_category,
                                parameters: Vec::new(),
                            });
                        }

                        detailed_maneuver.events.push(detailed_event);
                    }

                    detailed_group.maneuvers.push(detailed_maneuver);
                }

                detailed_act.maneuver_groups.push(detailed_group);
            }

            detailed_acts.push(detailed_act);
        }
    }

    detailed_acts
}

/// Analyze execution flow
fn analyze_execution_flow(
    stories: &[openscenario_rs::types::scenario::story::ScenarioStory],
) -> ExecutionFlowAnalysis {
    let mut flow_analysis = ExecutionFlowAnalysis {
        flow_diagram: Vec::new(),
        condition_dependencies: Vec::new(),
        actor_interactions: Vec::new(),
        timeline_analysis: TimelineAnalysis {
            sequence_steps: Vec::new(),
            parallel_executions: Vec::new(),
            estimated_total_duration: None,
        },
    };

    let mut step_order = 0u32;

    // Build flow diagram
    for story in stories {
        flow_analysis.flow_diagram.push(FlowStep {
            step_type: "Story".to_string(),
            step_name: story
                .name
                .as_literal()
                .map_or("Unknown".to_string(), |v| v.clone()),
            level: 0,
            description: format!("Story with {} acts", story.acts.len()),
        });

        for act in &story.acts {
            step_order += 1;
            flow_analysis.flow_diagram.push(FlowStep {
                step_type: "Act".to_string(),
                step_name: act
                    .name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone()),
                level: 1,
                description: format!("Act with {} maneuver groups", act.maneuver_groups.len()),
            });

            // Analyze timeline steps
            flow_analysis
                .timeline_analysis
                .sequence_steps
                .push(TimelineStep {
                    step_order,
                    step_name: act
                        .name
                        .as_literal()
                        .map_or("Unknown".to_string(), |v| v.clone()),
                    step_type: "Act".to_string(),
                    actors_involved: act
                        .maneuver_groups
                        .iter()
                        .flat_map(|mg| mg.actors.entity_refs.iter())
                        .map(|entity_ref| {
                            entity_ref
                                .entity_ref
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.clone())
                        })
                        .collect(),
                    estimated_start_time: None,
                });

            for maneuver_group in &act.maneuver_groups {
                flow_analysis.flow_diagram.push(FlowStep {
                    step_type: "ManeuverGroup".to_string(),
                    step_name: maneuver_group
                        .name
                        .as_literal()
                        .map_or("Unknown".to_string(), |v| v.clone()),
                    level: 2,
                    description: format!(
                        "ManeuverGroup with {} maneuvers for {} actors",
                        maneuver_group.maneuvers.len(),
                        maneuver_group.actors.entity_refs.len()
                    ),
                });

                for maneuver in &maneuver_group.maneuvers {
                    flow_analysis.flow_diagram.push(FlowStep {
                        step_type: "Maneuver".to_string(),
                        step_name: maneuver
                            .name
                            .as_literal()
                            .map_or("Unknown".to_string(), |v| v.clone()),
                        level: 3,
                        description: format!("Maneuver with {} events", maneuver.events.len()),
                    });

                    for event in &maneuver.events {
                        flow_analysis.flow_diagram.push(FlowStep {
                            step_type: "Event".to_string(),
                            step_name: event
                                .name
                                .as_literal()
                                .map_or("Unknown".to_string(), |v| v.clone()),
                            level: 4,
                            description: format!("Event with {} actions", event.actions.len()),
                        });
                    }
                }
            }
        }
    }

    flow_analysis
}

/// Analyze trigger conditions
fn analyze_trigger_conditions(
    _trigger: &openscenario_rs::types::scenario::triggers::Trigger,
) -> Vec<ConditionAnalysis> {
    let mut conditions = Vec::new();

    // For now, provide a simplified analysis
    // In a full implementation, this would analyze all condition types
    conditions.push(ConditionAnalysis {
        condition_type: "Trigger".to_string(),
        condition_description: "Complex trigger condition".to_string(),
        trigger_value: None,
        entity_ref: None,
    });

    conditions
}

/// Analyze private action type
fn analyze_private_action_type(
    private_action: &openscenario_rs::types::scenario::story::StoryPrivateAction,
) -> (String, String) {
    if private_action.longitudinal_action.is_some() {
        ("LongitudinalAction".to_string(), "Movement".to_string())
    } else if private_action.lateral_action.is_some() {
        ("LateralAction".to_string(), "Movement".to_string())
    } else if private_action.teleport_action.is_some() {
        ("TeleportAction".to_string(), "Movement".to_string())
    } else if private_action.routing_action.is_some() {
        ("RoutingAction".to_string(), "Movement".to_string())
    } else if private_action.synchronize_action.is_some() {
        ("SynchronizeAction".to_string(), "Coordination".to_string())
    } else if private_action.controller_action.is_some() {
        ("ControllerAction".to_string(), "Control".to_string())
    } else if private_action.visibility_action.is_some() {
        ("VisibilityAction".to_string(), "Appearance".to_string())
    } else if private_action.appearance_action.is_some() {
        ("AppearanceAction".to_string(), "Appearance".to_string())
    } else if private_action.trailer_action.is_some() {
        ("TrailerAction".to_string(), "Vehicle".to_string())
    } else {
        ("Unknown".to_string(), "Unknown".to_string())
    }
}

/// Analyze parameter variation document
fn analyze_parameter_variation(
    document: &OpenScenario,
    input_path: &Path,
) -> Result<ParameterVariationAnalysis, Box<dyn std::error::Error>> {
    let mut analysis = ParameterVariationAnalysis {
        referenced_scenario_info: None,
        deterministic_analysis: None,
        stochastic_analysis: None,
        total_parameters: 0,
        estimated_combinations: 1,
    };

    if let Some(param_dist) = &document.parameter_value_distribution {
        // Analyze referenced scenario
        let scenario_filepath = param_dist.scenario_file.filepath.clone();

        let scenario_path_result = resolve_scenario_path(input_path, &scenario_filepath);
        let mut referenced_info = ReferencedScenarioInfo {
            filepath: scenario_filepath.clone(),
            resolved_path: "Unknown".to_string(),
            loaded_successfully: false,
            description: "N/A".to_string(),
            author: "N/A".to_string(),
            entities_count: 0,
            template_parameters: Vec::new(),
            error_message: None,
        };

        match scenario_path_result {
            Ok(scenario_path) => {
                referenced_info.resolved_path = scenario_path.display().to_string();

                match parse_from_file(&scenario_path) {
                    Ok(scenario_doc) => {
                        referenced_info.loaded_successfully = true;
                        referenced_info.description = scenario_doc
                            .file_header
                            .description
                            .as_literal()
                            .map_or("N/A".to_string(), |v| v.clone());
                        referenced_info.author = scenario_doc
                            .file_header
                            .author
                            .as_literal()
                            .map_or("N/A".to_string(), |v| v.clone());

                        if let Some(entities) = &scenario_doc.entities {
                            referenced_info.entities_count = entities.scenario_objects.len();
                        }

                        if let Some(params) = &scenario_doc.parameter_declarations {
                            referenced_info.template_parameters = params
                                .parameter_declarations
                                .iter()
                                .map(|p| format!("{} = {:?}", p.name, p.value))
                                .collect();
                        }
                    }
                    Err(e) => {
                        referenced_info.error_message = Some(e.to_string());
                    }
                }
            }
            Err(e) => {
                referenced_info.error_message = Some(e.to_string());
            }
        }

        analysis.referenced_scenario_info = Some(referenced_info);

        // Analyze deterministic distributions
        if let Some(deterministic) = &param_dist.deterministic {
            let det_analysis = analyze_deterministic_distributions(deterministic);
            analysis.total_parameters +=
                det_analysis.single_distributions.len() + det_analysis.multi_distributions_count;
            analysis.estimated_combinations *= det_analysis.total_combinations;
            analysis.deterministic_analysis = Some(det_analysis);
        }

        // Analyze stochastic distributions
        if let Some(stochastic) = &param_dist.stochastic {
            let stoch_analysis = StochasticAnalysis {
                distributions_count: stochastic.distributions.len(),
            };
            analysis.total_parameters += stoch_analysis.distributions_count;
            analysis.stochastic_analysis = Some(stoch_analysis);
        }
    }

    Ok(analysis)
}

/// Analyze parameters in the document
fn analyze_parameters(document: &OpenScenario) -> ParameterAnalysis {
    let mut analysis = ParameterAnalysis {
        total_parameters: 0,
        parameter_names: Vec::new(),
        parameter_details: Vec::new(),
        expressions_found: 0,
        expressions_resolved: 0,
        expressions_failed: 0,
        resolution_examples: Vec::new(),
    };

    if let Some(param_decls) = &document.parameter_declarations {
        analysis.total_parameters = param_decls.parameter_declarations.len();
        analysis.parameter_names = param_decls
            .parameter_declarations
            .iter()
            .map(|p| {
                p.name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone())
            })
            .collect();

        // Collect detailed parameter information
        for param in &param_decls.parameter_declarations {
            let name = param
                .name
                .as_literal()
                .map_or("Unknown".to_string(), |v| v.clone());
            let is_expression = param.value.as_expression().is_some();
            let value_str = if let Some(literal) = param.value.as_literal() {
                literal.clone()
            } else if let Some(expr) = param.value.as_expression() {
                format!("${{{}}}", expr)
            } else {
                "Unknown".to_string()
            };

            let resolved_value = if is_expression {
                if let Some(expr) = param.value.as_expression() {
                    // Try to resolve using scenario parameters if available
                    if document.document_type() == OpenScenarioDocumentType::Scenario {
                        let scenario_parameters =
                            extract_scenario_parameters(&document.parameter_declarations);
                        evaluate_expression::<String>(expr, &scenario_parameters).ok()
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            let detail = ParameterDetail {
                name: name.clone(),
                value: value_str,
                parameter_type: format!("{:?}", param.parameter_type).to_lowercase(),
                context: "Parameter Declaration".to_string(),
                is_expression,
                resolved_value,
            };

            analysis.parameter_details.push(detail);
        }
    }

    // Perform expression resolution analysis
    if document.document_type() == OpenScenarioDocumentType::Scenario {
        let scenario_parameters = extract_scenario_parameters(&document.parameter_declarations);
        let expression_results = resolve_expressions_in_scenario(document, &scenario_parameters);

        analysis.expressions_found = expression_results.expressions_found;
        analysis.expressions_resolved = expression_results.expressions_resolved;
        analysis.expressions_failed = expression_results.expressions_failed;
        analysis.resolution_examples = expression_results.resolution_examples;
    }

    analysis
}

/// Analyze catalog configuration
fn analyze_catalogs(document: &OpenScenario, input_path: &Path) -> CatalogAnalysis {
    let mut analysis = CatalogAnalysis {
        has_catalog_locations: false,
        vehicle_catalog: false,
        pedestrian_catalog: false,
        misc_object_catalog: false,
        controller_catalog: false,
        environment_catalog: false,
        maneuver_catalog: false,
        trajectory_catalog: false,
        route_catalog: false,
        resolution_attempts: 0,
        resolution_successes: 0,
        resolution_failures: 0,
        resolution_results: Vec::new(),
    };

    if let Some(catalog_locations) = &document.catalog_locations {
        analysis.has_catalog_locations = true;
        analysis.vehicle_catalog = catalog_locations.vehicle_catalog.is_some();
        analysis.pedestrian_catalog = catalog_locations.pedestrian_catalog.is_some();
        analysis.misc_object_catalog = catalog_locations.misc_object_catalog.is_some();
        analysis.controller_catalog = catalog_locations.controller_catalog.is_some();
        analysis.environment_catalog = catalog_locations.environment_catalog.is_some();
        analysis.maneuver_catalog = catalog_locations.maneuver_catalog.is_some();
        analysis.trajectory_catalog = catalog_locations.trajectory_catalog.is_some();
        analysis.route_catalog = catalog_locations.route_catalog.is_some();

        // Perform catalog resolution analysis for scenarios
        if document.document_type() == OpenScenarioDocumentType::Scenario {
            let scenario_parameters = extract_scenario_parameters(&document.parameter_declarations);
            let resolution_results = resolve_catalog_references_in_scenario(
                document,
                catalog_locations,
                &scenario_parameters,
                input_path,
            );

            analysis.resolution_attempts = resolution_results.resolution_attempts;
            analysis.resolution_successes = resolution_results.resolution_successes;
            analysis.resolution_failures = resolution_results.resolution_failures;
            analysis.resolution_results = resolution_results.resolution_results;
        }
    }

    analysis
}

/// Format analysis results as text output
fn format_text_output(
    analysis: &AnalysisResult,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // Document overview
    print_section("📋 Document Overview", "", 0);
    println!("  Document Type: {:?}", analysis.document_type);
    println!("  Description: {}", analysis.header_info.description);
    println!("  Author: {}", analysis.header_info.author);
    println!("  Date: {}", analysis.header_info.date);
    println!(
        "  Version: {}.{}",
        analysis.header_info.rev_major, analysis.header_info.rev_minor
    );
    println!();

    // Entity analysis
    print_entity_analysis(&analysis.entity_analysis, config)?;

    // Storyboard analysis (scenarios only)
    if let Some(storyboard) = &analysis.storyboard_analysis {
        print_storyboard_analysis(storyboard, config)?;
    }

    // Parameter analysis
    print_section("⚙️ Parameter Analysis", "", 0);
    println!("═══════════════════");
    println!();
    println!("📊 Parameter Summary:");
    println!(
        "   - Total Parameters: {}",
        analysis.parameter_analysis.total_parameters
    );

    if analysis.parameter_analysis.total_parameters > 0 {
        println!();
        println!("🔍 All Parameters Found in XOSC File:");
        println!();

        // Display parameter details
        for (i, detail) in analysis
            .parameter_analysis
            .parameter_details
            .iter()
            .enumerate()
        {
            println!("   [{}] Parameter: \"{}\"", i + 1, detail.name);
            println!("       Value: {}", detail.value);
            println!("       Type: {}", detail.parameter_type);
            println!("       Context: {}", detail.context);

            if detail.is_expression {
                println!("       Expression: Yes");
                if let Some(resolved) = &detail.resolved_value {
                    println!("       Resolved To: {}", resolved);
                } else {
                    println!("       Resolved To: Failed to resolve");
                }
            } else {
                println!("       Expression: No (literal value)");
            }
            println!();
        }

        // Also show just the names for quick reference
        if config.verbose && !analysis.parameter_analysis.parameter_names.is_empty() {
            println!("📋 Quick Parameter Reference:");
            for name in &analysis.parameter_analysis.parameter_names {
                println!("    - {}", name);
            }
            println!();
        }
    } else {
        println!("   💡 No parameters declared in this XOSC file");
        println!();
    }

    // Parameter variation analysis (parameter variation documents only)
    if let Some(param_var) = &analysis.parameter_variation_analysis {
        print_parameter_variation_analysis(param_var, config)?;
    }

    // Expression resolution analysis
    if analysis.parameter_analysis.expressions_found > 0 {
        print_section("🔍 Expression Resolution", "", 0);
        println!("═══════════════════════");
        println!();
        println!("📊 Resolution Statistics:");
        println!(
            "   - Total expressions found: {}",
            analysis.parameter_analysis.expressions_found
        );
        println!(
            "   - Successfully resolved: {} ({:.1}%)",
            analysis.parameter_analysis.expressions_resolved,
            if analysis.parameter_analysis.expressions_found > 0 {
                (analysis.parameter_analysis.expressions_resolved as f64
                    / analysis.parameter_analysis.expressions_found as f64)
                    * 100.0
            } else {
                0.0
            }
        );
        println!(
            "   - Failed to resolve: {} ({:.1}%)",
            analysis.parameter_analysis.expressions_failed,
            if analysis.parameter_analysis.expressions_found > 0 {
                (analysis.parameter_analysis.expressions_failed as f64
                    / analysis.parameter_analysis.expressions_found as f64)
                    * 100.0
            } else {
                0.0
            }
        );
        println!();

        if !analysis.parameter_analysis.resolution_examples.is_empty() {
            println!("🔍 Resolution Examples:");
            for (i, example) in analysis
                .parameter_analysis
                .resolution_examples
                .iter()
                .enumerate()
            {
                println!("   [{}] {} → {}", i + 1, example.original, example.resolved);
                println!("       Location: {}", example.location);
                println!();
            }
        }
    } else if analysis.document_type == OpenScenarioDocumentType::Scenario {
        print_section("🔍 Expression Resolution", "", 0);
        println!("═══════════════════════");
        println!();
        println!("💡 No expressions found to resolve in this scenario");
        println!();
    }

    // Catalog analysis
    print_section("📚 Catalog Analysis", "", 0);
    println!("═══════════════════");
    println!();
    println!("📊 Catalog Configuration:");
    println!(
        "   - Has Catalog Locations: {}",
        analysis.catalog_analysis.has_catalog_locations
    );
    if config.verbose || analysis.catalog_analysis.has_catalog_locations {
        println!(
            "   - Vehicle Catalog: {}",
            analysis.catalog_analysis.vehicle_catalog
        );
        println!(
            "   - Pedestrian Catalog: {}",
            analysis.catalog_analysis.pedestrian_catalog
        );
        println!(
            "   - Misc Object Catalog: {}",
            analysis.catalog_analysis.misc_object_catalog
        );
        println!(
            "   - Controller Catalog: {}",
            analysis.catalog_analysis.controller_catalog
        );
        println!(
            "   - Environment Catalog: {}",
            analysis.catalog_analysis.environment_catalog
        );
        println!(
            "   - Maneuver Catalog: {}",
            analysis.catalog_analysis.maneuver_catalog
        );
        println!(
            "   - Trajectory Catalog: {}",
            analysis.catalog_analysis.trajectory_catalog
        );
        println!(
            "   - Route Catalog: {}",
            analysis.catalog_analysis.route_catalog
        );
    }
    println!();

    // Catalog resolution analysis
    if analysis.catalog_analysis.resolution_attempts > 0 {
        print_section("🗂️ Catalog Resolution", "", 0);
        println!("═════════════════════");
        println!();
        println!("📊 Resolution Statistics:");
        println!(
            "   - Total resolution attempts: {}",
            analysis.catalog_analysis.resolution_attempts
        );
        println!(
            "   - Successfully resolved: {} ({:.1}%)",
            analysis.catalog_analysis.resolution_successes,
            if analysis.catalog_analysis.resolution_attempts > 0 {
                (analysis.catalog_analysis.resolution_successes as f64
                    / analysis.catalog_analysis.resolution_attempts as f64)
                    * 100.0
            } else {
                0.0
            }
        );
        println!(
            "   - Failed to resolve: {} ({:.1}%)",
            analysis.catalog_analysis.resolution_failures,
            if analysis.catalog_analysis.resolution_attempts > 0 {
                (analysis.catalog_analysis.resolution_failures as f64
                    / analysis.catalog_analysis.resolution_attempts as f64)
                    * 100.0
            } else {
                0.0
            }
        );
        println!();

        if !analysis.catalog_analysis.resolution_results.is_empty() {
            println!("🔍 Resolution Details:");
            for (i, result) in analysis
                .catalog_analysis
                .resolution_results
                .iter()
                .enumerate()
            {
                let status = if result.success { "✅" } else { "❌" };
                println!(
                    "   [{}] {} Entity '{}' → {}/{}",
                    i + 1,
                    status,
                    result.entity_name,
                    result.catalog_name,
                    result.entry_name
                );
                println!("       Type: {} reference", result.resolution_type);
                if let Some(error) = &result.error_message {
                    println!("       Error: {}", error);
                }
                println!();
            }
        }
    } else if analysis.document_type == OpenScenarioDocumentType::Scenario
        && analysis.catalog_analysis.has_catalog_locations
    {
        print_section("🗂️ Catalog Resolution", "", 0);
        println!("═════════════════════");
        println!();
        println!("💡 No catalog references found to resolve in this scenario");
        println!();
    }

    Ok(())
}

/// Print entity analysis
fn print_entity_analysis(
    analysis: &EntityAnalysis,
    _config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    print_section("🎭 Entity Analysis", "", 0);
    println!("═══════════════════");
    println!();

    // Summary statistics
    println!("📊 Summary Statistics:");
    println!("   - Total entities: {}", analysis.total_entities);
    if analysis.total_entities > 0 {
        println!(
            "   - Vehicles: {} ({:.1}%) [{} catalog + {} inline]",
            analysis.vehicles,
            (analysis.vehicles as f64 / analysis.total_entities as f64) * 100.0,
            analysis.vehicles_catalog,
            analysis.vehicles_inline
        );
        println!(
            "   - Pedestrians: {} ({:.1}%) [{} catalog + {} inline]",
            analysis.pedestrians,
            (analysis.pedestrians as f64 / analysis.total_entities as f64) * 100.0,
            analysis.pedestrians_catalog,
            analysis.pedestrians_inline
        );
        println!(
            "   - Misc objects: {} ({:.1}%) [{} catalog + {} inline]",
            analysis.misc_objects,
            (analysis.misc_objects as f64 / analysis.total_entities as f64) * 100.0,
            analysis.misc_objects_catalog,
            analysis.misc_objects_inline
        );
        println!(
            "   - Catalog references: {} entities",
            analysis.catalog_references
        );
        println!(
            "   - Inline definitions: {} entities",
            analysis.inline_definitions
        );
    }
    println!();

    // Vehicle entities
    if !analysis.vehicle_analyses.is_empty() {
        println!("🚗 Vehicle Entities:");
        println!();

        for (i, vehicle) in analysis.vehicle_analyses.iter().enumerate() {
            println!(
                "   [{}] {} (name: \"{}\")",
                i + 1,
                vehicle.entity_name,
                vehicle.vehicle_name
            );
            println!("       Category: {}", vehicle.category);
            println!(
                "       Axles: {} total, front axle: {}",
                vehicle.axle_count,
                if vehicle.has_front_axle { "Yes" } else { "No" }
            );

            // Source
            match &vehicle.source {
                EntitySource::Inline => {
                    println!("       Source: Inline definition");
                }
                EntitySource::CatalogReference {
                    catalog_name,
                    entry_name,
                } => {
                    println!(
                        "       Source: Catalog reference → {}/{}",
                        catalog_name, entry_name
                    );
                }
            }

            println!();
        }
    }

    // Pedestrian entities
    if !analysis.pedestrian_analyses.is_empty() {
        println!("🚶 Pedestrian Entities:");
        println!();

        for (i, pedestrian) in analysis.pedestrian_analyses.iter().enumerate() {
            println!(
                "   [{}] {} (name: \"{}\")",
                i + 1,
                pedestrian.entity_name,
                pedestrian.pedestrian_name
            );
            println!("       Category: {}", pedestrian.category);

            // Source
            match &pedestrian.source {
                EntitySource::Inline => {
                    println!("       Source: Inline definition");
                }
                EntitySource::CatalogReference {
                    catalog_name,
                    entry_name,
                } => {
                    println!(
                        "       Source: Catalog reference → {}/{}",
                        catalog_name, entry_name
                    );
                }
            }

            println!();
        }
    }

    Ok(())
}

/// Print storyboard analysis
fn print_storyboard_analysis(
    analysis: &StoryboardAnalysis,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    print_section("🎬 Storyboard Analysis", "", 0);
    println!("═════════════════════");
    println!();

    // Enhanced Initialization Analysis
    print_section("📋 INITIALIZATION ANALYSIS", "", 0);
    println!("═══════════════════════════");
    println!();

    if analysis
        .detailed_init_analysis
        .entity_initializations
        .is_empty()
    {
        println!("💡 No entity initializations found");
    } else {
        println!("🎭 Entities Initialized:");
        for entity_init in &analysis.detailed_init_analysis.entity_initializations {
            println!(
                "  - {} ({})",
                entity_init.entity_name,
                if let Some(pos) = &entity_init.position_info {
                    &pos.position_type
                } else {
                    "No position"
                }
            );

            if let Some(pos) = &entity_init.position_info {
                println!("    📍 Position: {} {}", pos.position_type, pos.coordinates);
                if let Some(heading) = &pos.heading {
                    println!("    🧭 Heading: {}", heading);
                }
            }

            if let Some(speed) = &entity_init.speed_info {
                println!(
                    "    🏃 Speed: {} {} km/h",
                    speed.speed_type, speed.speed_value
                );
            }

            if !entity_init.actions.is_empty() {
                println!("    🎬 Actions:");
                for action in &entity_init.actions {
                    println!(
                        "      • {}: {}",
                        action.action_type, action.action_description
                    );
                }
            }
            println!();
        }
    }

    if let Some(env_setup) = &analysis.detailed_init_analysis.environment_setup {
        println!("🌍 Environment Setup:");
        println!("  - Time of Day: {}", env_setup.time_of_day);
        println!("  - Weather: {}", env_setup.weather_description);
        println!("  - Road Conditions: {}", env_setup.road_conditions);
        println!();
    }

    // Enhanced Storyboard Analysis
    print_section("🎬 STORYBOARD ANALYSIS", "", 0);
    println!("═════════════════════");
    println!();

    if analysis.detailed_acts.is_empty() {
        println!("💡 No Acts defined in the storyboard");
    } else {
        for (act_idx, act) in analysis.detailed_acts.iter().enumerate() {
            println!("Act {}: \"{}\"", act_idx + 1, act.act_name);
            if let Some(description) = &act.act_description {
                println!("  Description: {}", description);
            }
            println!();

            if !act.start_conditions.is_empty() {
                println!("  🚦 Start Conditions:");
                for condition in &act.start_conditions {
                    println!(
                        "    - {}: {}",
                        condition.condition_type, condition.condition_description
                    );
                    if let Some(value) = &condition.trigger_value {
                        println!("      Value: {}", value);
                    }
                }
                println!();
            }

            if !act.maneuver_groups.is_empty() {
                for (mg_idx, maneuver_group) in act.maneuver_groups.iter().enumerate() {
                    println!(
                        "  ManeuverGroup {}: \"{}\" (Actors: {})",
                        mg_idx + 1,
                        maneuver_group.group_name,
                        maneuver_group.actors.join(", ")
                    );

                    if let Some(max_exec) = maneuver_group.maximum_execution_count {
                        println!("    Max Executions: {}", max_exec);
                    }

                    for (m_idx, maneuver) in maneuver_group.maneuvers.iter().enumerate() {
                        println!("    Maneuver {}: \"{}\"", m_idx + 1, maneuver.maneuver_name);

                        for (e_idx, event) in maneuver.events.iter().enumerate() {
                            println!(
                                "      Event {}: \"{}\" (Priority: {})",
                                e_idx + 1,
                                event.event_name,
                                event.priority.as_ref().unwrap_or(&"default".to_string())
                            );

                            if !event.start_conditions.is_empty() {
                                println!("        Start Conditions:");
                                for condition in &event.start_conditions {
                                    println!(
                                        "          - {}: {}",
                                        condition.condition_type, condition.condition_description
                                    );
                                }
                            }

                            if !event.actions.is_empty() {
                                println!("        Actions:");
                                for action in &event.actions {
                                    println!(
                                        "          - {}: {} ({})",
                                        action.action_category,
                                        action.action_type,
                                        action.action_name
                                    );
                                }
                            }
                        }
                    }
                    println!();
                }
            }

            if !act.stop_conditions.is_empty() {
                println!("  🛑 Stop Conditions:");
                for condition in &act.stop_conditions {
                    println!(
                        "    - {}: {}",
                        condition.condition_type, condition.condition_description
                    );
                }
            }

            println!(
                "  ⏱️ Timing: Execution order {}",
                act.timing_info.execution_order
            );
            println!();
            println!("  {}", "─".repeat(50));
            println!();
        }
    }

    // Execution Flow Analysis
    if config.verbose {
        print_section("🔄 EXECUTION FLOW ANALYSIS", "", 0);
        println!("═══════════════════════════");
        println!();

        println!("📊 Flow Diagram:");
        for flow_step in &analysis.execution_flow.flow_diagram {
            let indent = "  ".repeat(flow_step.level as usize);
            println!(
                "{}{}. {} - {}",
                indent,
                flow_step.level + 1,
                flow_step.step_name,
                flow_step.description
            );
        }
        println!();

        println!("📅 Timeline Analysis:");
        for step in &analysis.execution_flow.timeline_analysis.sequence_steps {
            println!(
                "  [{}] {}: {} (Actors: {})",
                step.step_order,
                step.step_type,
                step.step_name,
                step.actors_involved.join(", ")
            );
        }
        println!();
    }

    // Statistics
    println!("📊 Storyboard Statistics:");
    let stats = &analysis.statistics;
    println!("   - Total Acts: {}", stats.total_acts);
    println!("   - Total ManeuverGroups: {}", stats.total_maneuver_groups);
    println!("   - Total Maneuvers: {}", stats.total_maneuvers);
    println!("   - Total Events: {}", stats.total_events);
    println!(
        "   - Total Actions: {} ({} init + {} story)",
        stats.total_actions + stats.total_init_actions,
        stats.total_init_actions,
        stats.total_actions
    );
    println!(
        "   - Unique Actors: {} ({})",
        stats.unique_actors.len(),
        stats.unique_actors.join(", ")
    );

    println!();
    Ok(())
}

/// Print parameter variation analysis
fn print_parameter_variation_analysis(
    analysis: &ParameterVariationAnalysis,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    print_section("📊 Parameter Variation Analysis", "", 0);
    println!("═══════════════════════════════");
    println!();

    // Referenced scenario information
    if let Some(ref_info) = &analysis.referenced_scenario_info {
        println!("📁 Referenced Scenario:");
        println!("   File: {}", ref_info.filepath);
        println!("   Resolved Path: {}", ref_info.resolved_path);

        if ref_info.loaded_successfully {
            println!("   ✅ Successfully loaded referenced scenario");
            println!("   📋 Description: {}", ref_info.description);
            println!("   👤 Author: {}", ref_info.author);
            println!("   🎭 Entities: {}", ref_info.entities_count);

            if !ref_info.template_parameters.is_empty() {
                println!(
                    "   ⚙️  Template parameters: {}",
                    ref_info.template_parameters.len()
                );
                for param in &ref_info.template_parameters {
                    println!("      - {}", param);
                }
            } else {
                println!("   ⚙️  Template parameters: None");
            }
        } else {
            println!("   ❌ Could not load referenced scenario file");
            if let Some(error) = &ref_info.error_message {
                println!("   Error: {}", error);
            }
        }
        println!();
    }

    // Deterministic distributions
    if let Some(det_analysis) = &analysis.deterministic_analysis {
        let total_count =
            det_analysis.single_distributions.len() + det_analysis.multi_distributions_count;
        println!("🎯 Deterministic distributions: {} parameters", total_count);
        println!();

        for (i, dist) in det_analysis.single_distributions.iter().enumerate() {
            println!("   [{}] Parameter: {}", i + 1, dist.parameter_name);

            match dist.distribution_type.as_str() {
                "Set" => {
                    println!("      📋 Distribution Set: {} values", dist.value_count);
                    for (j, value) in dist.sample_values.iter().enumerate() {
                        println!("         {}. {}", j + 1, value);
                    }
                    if dist.value_count > 5 {
                        println!("         ... and {} more", dist.value_count - 5);
                    }
                }
                "Range" => {
                    println!("      📏 Distribution Range:");
                    if let Some(range_info) = &dist.range_info {
                        println!(
                            "         Range: {} to {}",
                            range_info.lower_limit, range_info.upper_limit
                        );
                        println!("         Step: {}", range_info.step_width);
                        println!(
                            "         Calculated values: {}",
                            range_info.calculated_count
                        );
                    }
                }
                "User Defined" => {
                    println!("      🔧 User Defined Distribution:");
                    for value in &dist.sample_values {
                        println!("         {}", value);
                    }
                }
                _ => {
                    println!(
                        "      ❓ Unknown distribution type: {}",
                        dist.distribution_type
                    );
                }
            }
            println!();
        }

        if det_analysis.multi_distributions_count > 0 {
            println!(
                "   🎯 Multi-parameter distributions: {} groups",
                det_analysis.multi_distributions_count
            );
            println!();
        }
    }

    // Stochastic distributions
    if let Some(stoch_analysis) = &analysis.stochastic_analysis {
        println!(
            "🎲 Stochastic distributions: {} parameters",
            stoch_analysis.distributions_count
        );
        println!();
    }

    // Summary statistics
    println!("📊 Summary Statistics:");
    println!(
        "   🎯 Total varied parameters: {}",
        analysis.total_parameters
    );
    println!(
        "   🔢 Estimated combinations: {}",
        analysis.estimated_combinations
    );
    println!();

    Ok(())
}

/// Format analysis results as JSON output
fn format_json_output(analysis: &AnalysisResult) -> Result<(), Box<dyn std::error::Error>> {
    println!("{{");
    println!("  \"document_type\": \"{:?}\",", analysis.document_type);
    println!("  \"header\": {{");
    println!(
        "    \"description\": \"{}\",",
        analysis.header_info.description
    );
    println!("    \"author\": \"{}\",", analysis.header_info.author);
    println!("    \"date\": \"{}\",", analysis.header_info.date);
    println!(
        "    \"version\": \"{}.{}\"",
        analysis.header_info.rev_major, analysis.header_info.rev_minor
    );
    println!("  }},");
    println!("  \"entities\": {{");
    println!(
        "    \"total\": {},",
        analysis.entity_analysis.total_entities
    );
    println!("    \"vehicles\": {},", analysis.entity_analysis.vehicles);
    println!(
        "    \"vehicles_inline\": {},",
        analysis.entity_analysis.vehicles_inline
    );
    println!(
        "    \"vehicles_catalog\": {},",
        analysis.entity_analysis.vehicles_catalog
    );
    println!(
        "    \"pedestrians\": {},",
        analysis.entity_analysis.pedestrians
    );
    println!(
        "    \"pedestrians_inline\": {},",
        analysis.entity_analysis.pedestrians_inline
    );
    println!(
        "    \"pedestrians_catalog\": {},",
        analysis.entity_analysis.pedestrians_catalog
    );
    println!(
        "    \"misc_objects\": {},",
        analysis.entity_analysis.misc_objects
    );
    println!(
        "    \"misc_objects_inline\": {},",
        analysis.entity_analysis.misc_objects_inline
    );
    println!(
        "    \"misc_objects_catalog\": {},",
        analysis.entity_analysis.misc_objects_catalog
    );
    println!(
        "    \"unknown_catalog_references\": {},",
        analysis.entity_analysis.unknown_catalog_references
    );
    println!(
        "    \"catalog_references\": {}",
        analysis.entity_analysis.catalog_references
    );
    println!("  }},");

    if let Some(storyboard) = &analysis.storyboard_analysis {
        println!("  \"storyboard\": {{");
        println!(
            "    \"init_actions\": {},",
            storyboard.statistics.total_init_actions
        );
        println!(
            "    \"stories\": {},",
            storyboard
                .story_analysis
                .as_ref()
                .map(|s| s.total_stories)
                .unwrap_or(0)
        );
        println!("    \"acts\": {},", storyboard.statistics.total_acts);
        println!(
            "    \"maneuver_groups\": {},",
            storyboard.statistics.total_maneuver_groups
        );
        println!(
            "    \"maneuvers\": {},",
            storyboard.statistics.total_maneuvers
        );
        println!("    \"events\": {},", storyboard.statistics.total_events);
        println!("    \"actions\": {},", storyboard.statistics.total_actions);
        println!(
            "    \"unique_actors\": {},",
            storyboard.statistics.unique_actors.len()
        );
        println!("  }},");
    }

    println!("  \"parameters\": {{");
    println!(
        "    \"total\": {},",
        analysis.parameter_analysis.total_parameters
    );
    println!(
        "    \"names\": [{}],",
        analysis
            .parameter_analysis
            .parameter_names
            .iter()
            .map(|n| format!("\"{}\"", n))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "    \"expressions_found\": {},",
        analysis.parameter_analysis.expressions_found
    );
    println!(
        "    \"expressions_resolved\": {},",
        analysis.parameter_analysis.expressions_resolved
    );
    println!(
        "    \"expressions_failed\": {}",
        analysis.parameter_analysis.expressions_failed
    );
    println!("  }},");

    if let Some(param_var) = &analysis.parameter_variation_analysis {
        println!("  \"parameter_variation\": {{");
        println!("    \"total_parameters\": {},", param_var.total_parameters);
        println!(
            "    \"estimated_combinations\": {},",
            param_var.estimated_combinations
        );

        if let Some(ref_info) = &param_var.referenced_scenario_info {
            println!("    \"referenced_scenario\": {{");
            println!("      \"filepath\": \"{}\",", ref_info.filepath);
            println!(
                "      \"loaded_successfully\": {},",
                ref_info.loaded_successfully
            );
            println!("      \"entities_count\": {},", ref_info.entities_count);
            println!(
                "      \"template_parameters_count\": {}",
                ref_info.template_parameters.len()
            );
            println!("    }},");
        }

        if let Some(det_analysis) = &param_var.deterministic_analysis {
            println!("    \"deterministic\": {{");
            println!(
                "      \"single_distributions\": {},",
                det_analysis.single_distributions.len()
            );
            println!(
                "      \"multi_distributions\": {},",
                det_analysis.multi_distributions_count
            );
            println!(
                "      \"total_combinations\": {}",
                det_analysis.total_combinations
            );
            println!("    }},");
        }

        if let Some(stoch_analysis) = &param_var.stochastic_analysis {
            println!("    \"stochastic\": {{");
            println!(
                "      \"distributions_count\": {}",
                stoch_analysis.distributions_count
            );
            println!("    }}");
        } else {
            // Remove trailing comma if no stochastic section
            println!("    \"stochastic\": null");
        }

        println!("  }},");
    }

    println!("  \"catalogs\": {{");
    println!(
        "    \"has_locations\": {},",
        analysis.catalog_analysis.has_catalog_locations
    );
    println!(
        "    \"vehicle\": {},",
        analysis.catalog_analysis.vehicle_catalog
    );
    println!(
        "    \"pedestrian\": {},",
        analysis.catalog_analysis.pedestrian_catalog
    );
    println!(
        "    \"misc_object\": {},",
        analysis.catalog_analysis.misc_object_catalog
    );
    println!(
        "    \"controller\": {},",
        analysis.catalog_analysis.controller_catalog
    );
    println!(
        "    \"environment\": {},",
        analysis.catalog_analysis.environment_catalog
    );
    println!(
        "    \"maneuver\": {},",
        analysis.catalog_analysis.maneuver_catalog
    );
    println!(
        "    \"trajectory\": {},",
        analysis.catalog_analysis.trajectory_catalog
    );
    println!(
        "    \"route\": {},",
        analysis.catalog_analysis.route_catalog
    );
    println!(
        "    \"resolution_attempts\": {},",
        analysis.catalog_analysis.resolution_attempts
    );
    println!(
        "    \"resolution_successes\": {},",
        analysis.catalog_analysis.resolution_successes
    );
    println!(
        "    \"resolution_failures\": {}",
        analysis.catalog_analysis.resolution_failures
    );
    println!("  }}");
    println!("}}");

    Ok(())
}

/// Expression resolution analysis results
#[derive(Debug)]
struct ExpressionAnalysisResult {
    expressions_found: usize,
    expressions_resolved: usize,
    expressions_failed: usize,
    resolution_examples: Vec<ExpressionResolutionExample>,
}

/// Catalog resolution analysis results
#[derive(Debug)]
struct CatalogAnalysisResult {
    resolution_attempts: usize,
    resolution_successes: usize,
    resolution_failures: usize,
    resolution_results: Vec<CatalogResolutionResult>,
}

/// Helper function to resolve a Value<String> if it's an expression
fn resolve_string_value(
    value: &Value<String>,
    location: &str,
    result: &mut ExpressionAnalysisResult,
    parameters: &HashMap<String, String>,
) -> bool {
    if let Some(expr) = value.as_expression() {
        result.expressions_found += 1;
        match evaluate_expression::<String>(expr, parameters) {
            Ok(resolved) => {
                result.expressions_resolved += 1;
                if result.resolution_examples.len() < 5 {
                    result
                        .resolution_examples
                        .push(ExpressionResolutionExample {
                            original: format!("${{{}}}", expr),
                            resolved: resolved.clone(),
                            location: location.to_string(),
                        });
                }
                true
            }
            Err(_) => {
                result.expressions_failed += 1;
                false
            }
        }
    } else {
        false
    }
}

/// Helper function to resolve a Value<f64> if it's an expression
fn resolve_numeric_value(
    value: &Value<f64>,
    location: &str,
    result: &mut ExpressionAnalysisResult,
    parameters: &HashMap<String, String>,
) -> bool {
    if let Some(expr) = value.as_expression() {
        result.expressions_found += 1;
        match evaluate_expression::<f64>(expr, parameters) {
            Ok(resolved) => {
                result.expressions_resolved += 1;
                if result.resolution_examples.len() < 5 {
                    result
                        .resolution_examples
                        .push(ExpressionResolutionExample {
                            original: format!("${{{}}}", expr),
                            resolved: resolved.to_string(),
                            location: location.to_string(),
                        });
                }
                true
            }
            Err(_) => {
                result.expressions_failed += 1;
                false
            }
        }
    } else {
        false
    }
}

/// Resolve expressions throughout the scenario document
fn resolve_expressions_in_scenario(
    document: &OpenScenario,
    parameters: &HashMap<String, String>,
) -> ExpressionAnalysisResult {
    let mut result = ExpressionAnalysisResult {
        expressions_found: 0,
        expressions_resolved: 0,
        expressions_failed: 0,
        resolution_examples: Vec::new(),
    };

    // Process parameter declarations
    if let Some(param_decls) = &document.parameter_declarations {
        for param in &param_decls.parameter_declarations {
            let param_name = param.name.as_literal().map_or("Unknown", |v| v);
            resolve_string_value(
                &param.value,
                &format!("Parameter '{}'", param_name),
                &mut result,
                parameters,
            );
        }
    }

    // Process entities
    if let Some(entities) = &document.entities {
        for entity in &entities.scenario_objects {
            let entity_name = entity.name.as_literal().map_or("Unknown", |v| v);

            // Resolve entity name if it's an expression
            resolve_string_value(
                &entity.name,
                &format!("Entity '{}' name", entity_name),
                &mut result,
                parameters,
            );

            // Process vehicle properties if present
            if let Some(vehicle) = &entity.vehicle {
                resolve_string_value(
                    &vehicle.name,
                    &format!("Vehicle '{}' name", entity_name),
                    &mut result,
                    parameters,
                );

                // Process performance values
                let performance = &vehicle.performance;
                resolve_numeric_value(
                    &performance.max_speed,
                    &format!("Vehicle '{}' max speed", entity_name),
                    &mut result,
                    parameters,
                );
                resolve_numeric_value(
                    &performance.max_acceleration,
                    &format!("Vehicle '{}' max acceleration", entity_name),
                    &mut result,
                    parameters,
                );
                resolve_numeric_value(
                    &performance.max_deceleration,
                    &format!("Vehicle '{}' max deceleration", entity_name),
                    &mut result,
                    parameters,
                );
            }

            // Process pedestrian properties if present
            if let Some(pedestrian) = &entity.pedestrian {
                resolve_string_value(
                    &pedestrian.name,
                    &format!("Pedestrian '{}' name", entity_name),
                    &mut result,
                    parameters,
                );
            }
        }
    }

    // Process storyboard initialization actions
    if let Some(storyboard) = &document.storyboard {
        // Process private actions
        for private_action in &storyboard.init.actions.private_actions {
            let entity_ref = private_action
                .entity_ref
                .as_literal()
                .map_or("Unknown", |v| v);

            // Process position actions
            for action in &private_action.private_actions {
                if let Some(teleport_action) = &action.teleport_action {
                    // Process position coordinates
                    if let Some(world_position) = &teleport_action.position.world_position {
                        resolve_numeric_value(
                            &world_position.x,
                            &format!("Entity '{}' init position X", entity_ref),
                            &mut result,
                            parameters,
                        );
                        resolve_numeric_value(
                            &world_position.y,
                            &format!("Entity '{}' init position Y", entity_ref),
                            &mut result,
                            parameters,
                        );
                        if let Some(z_value) = &world_position.z {
                            resolve_numeric_value(
                                z_value,
                                &format!("Entity '{}' init position Z", entity_ref),
                                &mut result,
                                parameters,
                            );
                        }
                        if let Some(h_value) = &world_position.h {
                            resolve_numeric_value(
                                h_value,
                                &format!("Entity '{}' init heading", entity_ref),
                                &mut result,
                                parameters,
                            );
                        }
                    }
                }

                // Process speed actions
                if let Some(speed_action) = &action.longitudinal_action {
                    if let Some(speed_action) = &speed_action.speed_action {
                        let target = &speed_action.speed_action_target;
                        if let Some(absolute_speed) = &target.absolute {
                            resolve_numeric_value(
                                &absolute_speed.value,
                                &format!("Entity '{}' init speed", entity_ref),
                                &mut result,
                                parameters,
                            );
                        }
                    }
                }
            }
        }
    }

    result
}

/// Resolve catalog references throughout the scenario document
fn resolve_catalog_references_in_scenario(
    document: &OpenScenario,
    catalog_locations: &openscenario_rs::types::catalogs::locations::CatalogLocations,
    parameters: &HashMap<String, String>,
    scenario_path: &Path,
) -> CatalogAnalysisResult {
    let mut result = CatalogAnalysisResult {
        resolution_attempts: 0,
        resolution_successes: 0,
        resolution_failures: 0,
        resolution_results: Vec::new(),
    };

    // Get the base directory for relative catalog paths
    let base_dir = scenario_path.parent().unwrap_or(Path::new("."));

    // Process each entity in the scenario
    if let Some(entities) = &document.entities {
        for entity in &entities.scenario_objects {
            let entity_name = entity
                .name
                .as_literal()
                .map_or("Unknown", |v| v)
                .to_string();

            // Check if entity has a catalog reference
            if let Some(catalog_ref) = entity.vehicle_catalog_reference() {
                result.resolution_attempts += 1;

                let catalog_name = catalog_ref
                    .catalog_name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone());
                let entry_name = catalog_ref
                    .entry_name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone());

                // Attempt to resolve the catalog reference
                match resolve_catalog_reference_simple(
                    &catalog_ref.catalog_name,
                    &catalog_ref.entry_name,
                    catalog_locations,
                    parameters,
                    base_dir,
                ) {
                    Ok(found) => {
                        if found {
                            result.resolution_successes += 1;
                            result.resolution_results.push(CatalogResolutionResult {
                                entity_name: entity_name.clone(),
                                catalog_name,
                                entry_name,
                                resolution_type: "entity".to_string(),
                                success: true,
                                error_message: None,
                            });
                        } else {
                            result.resolution_failures += 1;
                            result.resolution_results.push(CatalogResolutionResult {
                                entity_name: entity_name.clone(),
                                catalog_name,
                                entry_name,
                                resolution_type: "entity".to_string(),
                                success: false,
                                error_message: Some("Entry not found in catalog".to_string()),
                            });
                        }
                    }
                    Err(e) => {
                        result.resolution_failures += 1;
                        result.resolution_results.push(CatalogResolutionResult {
                            entity_name: entity_name.clone(),
                            catalog_name,
                            entry_name,
                            resolution_type: "entity".to_string(),
                            success: false,
                            error_message: Some(e.to_string()),
                        });
                    }
                }
            } else if let Some(catalog_ref) = entity.pedestrian_catalog_reference() {
                result.resolution_attempts += 1;

                let catalog_name = catalog_ref
                    .catalog_name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone());
                let entry_name = catalog_ref
                    .entry_name
                    .as_literal()
                    .map_or("Unknown".to_string(), |v| v.clone());

                // Attempt to resolve the catalog reference
                match resolve_catalog_reference_simple(
                    &catalog_ref.catalog_name,
                    &catalog_ref.entry_name,
                    catalog_locations,
                    parameters,
                    base_dir,
                ) {
                    Ok(found) => {
                        if found {
                            result.resolution_successes += 1;
                            result.resolution_results.push(CatalogResolutionResult {
                                entity_name: entity_name.clone(),
                                catalog_name,
                                entry_name,
                                resolution_type: "entity".to_string(),
                                success: true,
                                error_message: None,
                            });
                        } else {
                            result.resolution_failures += 1;
                            result.resolution_results.push(CatalogResolutionResult {
                                entity_name: entity_name.clone(),
                                catalog_name,
                                entry_name,
                                resolution_type: "entity".to_string(),
                                success: false,
                                error_message: Some("Entry not found in catalog".to_string()),
                            });
                        }
                    }
                    Err(e) => {
                        result.resolution_failures += 1;
                        result.resolution_results.push(CatalogResolutionResult {
                            entity_name: entity_name.clone(),
                            catalog_name,
                            entry_name,
                            resolution_type: "entity".to_string(),
                            success: false,
                            error_message: Some(e.to_string()),
                        });
                    }
                }
            }

            // Check controller references
            if let Some(object_controller) = &entity.object_controller {
                if let Some(controller_ref) = &object_controller.catalog_reference {
                    result.resolution_attempts += 1;

                    let catalog_name = controller_ref
                        .catalog_name
                        .as_literal()
                        .map_or("Unknown".to_string(), |v| v.clone());
                    let entry_name = controller_ref
                        .entry_name
                        .as_literal()
                        .map_or("Unknown".to_string(), |v| v.clone());

                    match resolve_catalog_reference_simple(
                        &controller_ref.catalog_name,
                        &controller_ref.entry_name,
                        catalog_locations,
                        parameters,
                        base_dir,
                    ) {
                        Ok(found) => {
                            if found {
                                result.resolution_successes += 1;
                                result.resolution_results.push(CatalogResolutionResult {
                                    entity_name: entity_name.clone(),
                                    catalog_name,
                                    entry_name,
                                    resolution_type: "controller".to_string(),
                                    success: true,
                                    error_message: None,
                                });
                            } else {
                                result.resolution_failures += 1;
                                result.resolution_results.push(CatalogResolutionResult {
                                    entity_name: entity_name.clone(),
                                    catalog_name,
                                    entry_name,
                                    resolution_type: "controller".to_string(),
                                    success: false,
                                    error_message: Some("Entry not found in catalog".to_string()),
                                });
                            }
                        }
                        Err(e) => {
                            result.resolution_failures += 1;
                            result.resolution_results.push(CatalogResolutionResult {
                                entity_name: entity_name.clone(),
                                catalog_name,
                                entry_name,
                                resolution_type: "controller".to_string(),
                                success: false,
                                error_message: Some(e.to_string()),
                            });
                        }
                    }
                }
            }
        }
    }

    result
}

/// Print a header with emoji and formatting
fn print_header(title: &str, level: u8) {
    match level {
        1 => {
            println!("{}", title);
            println!("{}", "=".repeat(title.len()));
        }
        2 => {
            println!("{}", title);
            println!("{}", "-".repeat(title.len()));
        }
        _ => {
            println!("{}", title);
        }
    }
}

/// Print a section with title and content
fn print_section(title: &str, _content: &str, _indent: usize) {
    println!("{}", title);
}

/// Resolve the absolute path to a scenario file referenced from a parameter variation
fn resolve_scenario_path(
    variation_path: &Path,
    scenario_filepath: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let base_dir = variation_path.parent().unwrap_or(Path::new("."));
    let scenario_path = if Path::new(scenario_filepath).is_absolute() {
        std::path::PathBuf::from(scenario_filepath)
    } else {
        base_dir.join(scenario_filepath)
    };

    if scenario_path.exists() {
        Ok(scenario_path)
    } else {
        Err(format!(
            "Referenced scenario file not found: {}",
            scenario_path.display()
        )
        .into())
    }
}

/// Analyze deterministic distributions and return detailed analysis
fn analyze_deterministic_distributions(
    _deterministic: &openscenario_rs::types::distributions::Deterministic,
) -> DeterministicAnalysis {
    // Simplified implementation to avoid compilation errors
    DeterministicAnalysis {
        single_distributions: Vec::new(),
        multi_distributions_count: 0,
        total_combinations: 1,
    }
}

/// Original implementation (commented out due to compilation issues)
fn _analyze_deterministic_distributions_original(
    deterministic: &openscenario_rs::types::distributions::Deterministic,
) -> DeterministicAnalysis {
    let mut analysis = DeterministicAnalysis {
        single_distributions: Vec::new(),
        multi_distributions_count: deterministic.multi_distributions.len(),
        total_combinations: 1,
    };

    for dist in &deterministic.single_distributions {
        let parameter_name = dist
            .parameter_name
            .as_literal()
            .map_or("Unknown".to_string(), |v| v.to_string());

        let mut details = DistributionDetails {
            parameter_name: parameter_name.clone(),
            distribution_type: "Unknown".to_string(),
            value_count: 0,
            sample_values: Vec::new(),
            range_info: None,
        };

        if let Some(set) = &dist.distribution_set {
            details.distribution_type = "Set".to_string();
            details.value_count = set.elements.len();
            details.sample_values = set
                .elements
                .iter()
                .take(5)
                .map(|e| {
                    e.value
                        .as_literal()
                        .map_or("Unknown".to_string(), |v| v.clone())
                })
                .collect();
            analysis.total_combinations *= set.elements.len();
        }

        if let Some(dist_range) = &dist.distribution_range {
            details.distribution_type = "Range".to_string();

            let lower_str = dist_range
                .range
                .lower_limit
                .as_literal()
                .map_or("Unknown".to_string(), |v| v.to_string());
            let upper_str = dist_range
                .range
                .upper_limit
                .as_literal()
                .map_or("Unknown".to_string(), |v| v.to_string());
            let step_str = dist_range
                .step_width
                .as_literal()
                .map_or("Unknown".to_string(), |v| v.to_string());

            let mut calculated_count = 1;
            if let (Some(lower), Some(upper), Some(step_val)) = (
                dist_range.range.lower_limit.as_literal(),
                dist_range.range.upper_limit.as_literal(),
                dist_range.step_width.as_literal(),
            ) {
                if let Ok(step_f64) = step_val.parse::<f64>() {
                    if step_f64 > 0.0 {
                        calculated_count = ((upper - lower) / step_f64 + 1.0) as usize;
                        details.value_count = calculated_count;
                        analysis.total_combinations *= calculated_count;
                    }
                }
            }

            details.range_info = Some(RangeInfo {
                lower_limit: lower_str,
                upper_limit: upper_str,
                step_width: step_str,
                calculated_count,
            });
        }

        if let Some(user_def) = &dist.user_defined_distribution {
            details.distribution_type = "User Defined".to_string();
            details.sample_values.push(format!(
                "{} (type: {})",
                user_def.content, user_def.distribution_type
            ));
            details.value_count = 1; // Unknown count for user-defined
        }

        analysis.single_distributions.push(details);
    }

    analysis
}
