# OpenSCENARIO-rs Integration Guide

This guide covers how to integrate OpenSCENARIO-rs into various development environments and workflows, including simulation tools, testing frameworks, and CI/CD pipelines.

## Table of Contents

1. [Integration Overview](#integration-overview)
2. [Simulation Tool Integration](#simulation-tool-integration)
3. [Testing Framework Integration](#testing-framework-integration)
4. [CI/CD Pipeline Integration](#cicd-pipeline-integration)
5. [IDE and Development Environment Setup](#ide-and-development-environment-setup)
6. [Performance Optimization](#performance-optimization)
7. [Troubleshooting](#troubleshooting)

## Integration Overview

OpenSCENARIO-rs provides multiple integration points:

- **Parser API**: For reading existing OpenSCENARIO files
- **Builder API**: For programmatic scenario creation
- **Validation API**: For schema and semantic validation
- **Serialization API**: For XML output generation
- **Catalog System**: For reusable component management

### Core Integration Patterns

```rust
// 1. Parse existing scenarios
let scenario = openscenario_rs::parse_file("scenario.xosc")?;

// 2. Build new scenarios programmatically
let scenario = ScenarioBuilder::new()
    .with_header("Test", "Author")
    .with_entities()
        .add_vehicle("ego").car().finish()
    .build()?;

// 3. Validate scenarios
scenario.validate()?;

// 4. Serialize to XML
let xml = openscenario_rs::serialize_to_string(&scenario)?;
```

## Simulation Tool Integration

### CARLA Integration

```rust
use openscenario_rs::{ScenarioBuilder, parse_file};
use carla_client::Client;

struct CarlaScenarioRunner {
    client: Client,
    scenario: OpenScenario,
}

impl CarlaScenarioRunner {
    pub fn new(scenario_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new("localhost", 2000)?;
        let scenario = parse_file(scenario_path)?;
        
        Ok(Self { client, scenario })
    }
    
    pub fn setup_world(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Extract road network
        if let Some(road_network) = &self.scenario.road_network {
            if let Some(logic_file) = &road_network.logic_file {
                let xodr_path = logic_file.filepath.as_literal()
                    .ok_or("Invalid road file path")?;
                self.client.load_world(xodr_path)?;
            }
        }
        
        Ok(())
    }
    
    pub fn spawn_entities(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(entities) = &self.scenario.entities {
            for entity in &entities.scenario_objects {
                if let Some(vehicle) = &entity.vehicle {
                    self.spawn_vehicle(entity, vehicle)?;
                }
                if let Some(pedestrian) = &entity.pedestrian {
                    self.spawn_pedestrian(entity, pedestrian)?;
                }
            }
        }
        
        Ok(())
    }
    
    fn spawn_vehicle(&mut self, entity: &ScenarioObject, vehicle: &Vehicle) 
        -> Result<(), Box<dyn std::error::Error>> {
        // Convert OpenSCENARIO vehicle to CARLA vehicle
        let blueprint = match vehicle.vehicle_category {
            VehicleCategory::Car => "vehicle.tesla.model3",
            VehicleCategory::Truck => "vehicle.carlamotors.firetruck",
            _ => "vehicle.tesla.model3",
        };
        
        // Spawn at initial position
        // Implementation depends on CARLA client API
        Ok(())
    }
    
    fn spawn_pedestrian(&mut self, entity: &ScenarioObject, pedestrian: &Pedestrian) 
        -> Result<(), Box<dyn std::error::Error>> {
        // Spawn pedestrian in CARLA
        Ok(())
    }
    
    pub fn execute_storyboard(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(storyboard) = &self.scenario.storyboard {
            for story in &storyboard.stories {
                self.execute_story(story)?;
            }
        }
        
        Ok(())
    }
    
    fn execute_story(&mut self, story: &Story) -> Result<(), Box<dyn std::error::Error>> {
        for act in &story.acts {
            self.execute_act(act)?;
        }
        Ok(())
    }
    
    fn execute_act(&mut self, act: &Act) -> Result<(), Box<dyn std::error::Error>> {
        // Convert OpenSCENARIO actions to CARLA commands
        for maneuver_group in &act.maneuver_groups {
            for maneuver in &maneuver_group.maneuvers {
                self.execute_maneuver(maneuver)?;
            }
        }
        Ok(())
    }
    
    fn execute_maneuver(&mut self, maneuver: &Maneuver) -> Result<(), Box<dyn std::error::Error>> {
        for event in &maneuver.events {
            for action in &event.actions {
                self.execute_action(action)?;
            }
        }
        Ok(())
    }
    
    fn execute_action(&mut self, action: &Action) -> Result<(), Box<dyn std::error::Error>> {
        // Convert OpenSCENARIO actions to CARLA vehicle commands
        if let Some(private_action) = &action.private_action {
            for private_action_wrapper in &private_action.private_actions {
                if let Some(speed_action) = &private_action_wrapper.longitudinal_action
                    .as_ref().and_then(|la| la.speed_action.as_ref()) {
                    self.apply_speed_action(&private_action.entity_ref, speed_action)?;
                }
                
                if let Some(teleport_action) = &private_action_wrapper.teleport_action {
                    self.apply_teleport_action(&private_action.entity_ref, teleport_action)?;
                }
            }
        }
        
        Ok(())
    }
    
    fn apply_speed_action(&mut self, entity_ref: &OSString, action: &SpeedAction) 
        -> Result<(), Box<dyn std::error::Error>> {
        // Apply speed to CARLA vehicle
        Ok(())
    }
    
    fn apply_teleport_action(&mut self, entity_ref: &OSString, action: &TeleportAction) 
        -> Result<(), Box<dyn std::error::Error>> {
        // Teleport CARLA vehicle
        Ok(())
    }
}

// Usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runner = CarlaScenarioRunner::new("highway_scenario.xosc")?;
    
    runner.setup_world()?;
    runner.spawn_entities()?;
    runner.execute_storyboard()?;
    
    Ok(())
}
```

### SUMO Integration

```rust
use openscenario_rs::{ScenarioBuilder, parse_file};
use sumo_client::TraciClient;

struct SumoScenarioRunner {
    client: TraciClient,
    scenario: OpenScenario,
}

impl SumoScenarioRunner {
    pub fn new(scenario_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = TraciClient::new("localhost", 8813)?;
        let scenario = parse_file(scenario_path)?;
        
        Ok(Self { client, scenario })
    }
    
    pub fn setup_simulation(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Load SUMO network file
        if let Some(road_network) = &self.scenario.road_network {
            if let Some(logic_file) = &road_network.logic_file {
                let net_path = logic_file.filepath.as_literal()
                    .ok_or("Invalid network file path")?;
                self.client.load_network(net_path)?;
            }
        }
        
        Ok(())
    }
    
    pub fn add_vehicles(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(entities) = &self.scenario.entities {
            for entity in &entities.scenario_objects {
                if let Some(vehicle) = &entity.vehicle {
                    let vehicle_id = entity.name.as_literal()
                        .ok_or("Invalid entity name")?;
                    
                    // Add vehicle to SUMO
                    self.client.vehicle_add(
                        vehicle_id,
                        "DEFAULT_VEHTYPE", // Map from OpenSCENARIO vehicle type
                        "route1", // Extract from scenario
                        0.0, // Start time
                        0.0, // Position
                        0.0, // Speed
                        0, // Lane
                    )?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn run_simulation(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut step = 0;
        
        while self.client.simulation_get_min_expected_number()? > 0 {
            self.client.simulation_step()?;
            
            // Process OpenSCENARIO events for current time step
            self.process_events_at_time(step as f64 * 0.1)?; // 0.1s time step
            
            step += 1;
        }
        
        Ok(())
    }
    
    fn process_events_at_time(&mut self, time: f64) -> Result<(), Box<dyn std::error::Error>> {
        // Check conditions and trigger actions based on simulation time
        // This requires implementing a condition evaluation engine
        Ok(())
    }
}
```

## Testing Framework Integration

### Unit Testing with OpenSCENARIO-rs

```rust
use openscenario_rs::{ScenarioBuilder, types::enums::ParameterType};

#[cfg(test)]
mod scenario_tests {
    use super::*;
    
    #[test]
    fn test_highway_scenario_creation() {
        let scenario = ScenarioBuilder::new()
            .with_header("Highway Test", "Test Suite")
            .add_parameter("ego_speed", ParameterType::Double, "25.0")
            .with_entities()
                .add_vehicle("ego")
                    .car()
                    .finish()
                .add_vehicle("target")
                    .car()
                    .finish()
            .build()
            .expect("Failed to build scenario");
        
        // Validate scenario structure
        assert!(scenario.entities.is_some());
        assert_eq!(scenario.entities.unwrap().scenario_objects.len(), 2);
        assert!(scenario.parameter_declarations.is_some());
    }
    
    #[test]
    fn test_scenario_serialization_roundtrip() {
        let original = ScenarioBuilder::new()
            .with_header("Roundtrip Test", "Test Suite")
            .with_entities()
                .add_vehicle("test_vehicle")
                    .car()
                    .finish()
            .build()
            .expect("Failed to build scenario");
        
        // Serialize to XML
        let xml = openscenario_rs::serialize_to_string(&original)
            .expect("Failed to serialize");
        
        // Parse back from XML
        let parsed = openscenario_rs::parse_from_str(&xml)
            .expect("Failed to parse");
        
        // Verify key properties are preserved
        assert_eq!(
            original.file_header.description.as_literal(),
            parsed.file_header.description.as_literal()
        );
    }
    
    #[test]
    fn test_parameter_resolution() {
        let scenario = ScenarioBuilder::new()
            .with_header("Parameter Test", "Test Suite")
            .add_parameter("test_speed", ParameterType::Double, "30.0")
            .with_entities()
                .add_vehicle("ego")
                    .car()
                    .finish()
            .build()
            .expect("Failed to build scenario");
        
        // Test parameter resolution
        if let Some(params) = &scenario.parameter_declarations {
            let speed_param = params.parameter_declarations
                .iter()
                .find(|p| p.name.as_literal() == Some("test_speed"))
                .expect("Speed parameter not found");
            
            assert_eq!(speed_param.value.as_literal(), Some("30.0"));
        }
    }
}

// Property-based testing with proptest
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_scenario_builder_with_random_parameters(
            description in "[a-zA-Z ]{1,50}",
            author in "[a-zA-Z ]{1,30}",
            speed in 0.0f64..100.0
        ) {
            let scenario = ScenarioBuilder::new()
                .with_header(&description, &author)
                .add_parameter("speed", ParameterType::Double, &speed.to_string())
                .with_entities()
                    .add_vehicle("test")
                        .car()
                        .finish()
                .build();
            
            prop_assert!(scenario.is_ok());
            
            let scenario = scenario.unwrap();
            prop_assert_eq!(
                scenario.file_header.description.as_literal(),
                Some(description.as_str())
            );
            prop_assert_eq!(
                scenario.file_header.author.as_literal(),
                Some(author.as_str())
            );
        }
    }
}
```

### Integration Testing

```rust
use openscenario_rs::{ScenarioBuilder, parse_file};
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_scenario_validation_with_external_tool() {
    // Create a test scenario
    let scenario = ScenarioBuilder::new()
        .with_header("Validation Test", "Integration Test")
        .with_entities()
            .add_vehicle("ego")
                .car()
                .finish()
        .build()
        .expect("Failed to build scenario");
    
    // Serialize to temporary file
    let mut temp_file = NamedTempFile::new()
        .expect("Failed to create temp file");
    
    let xml = openscenario_rs::serialize_to_string(&scenario)
        .expect("Failed to serialize");
    
    std::fs::write(temp_file.path(), xml)
        .expect("Failed to write temp file");
    
    // Validate with external tool (if available)
    if let Ok(output) = Command::new("openscenario-validator")
        .arg(temp_file.path())
        .output() {
        assert!(output.status.success(), 
                "External validation failed: {}", 
                String::from_utf8_lossy(&output.stderr));
    }
}

#[test]
fn test_large_scenario_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Create a large scenario
    let mut builder = ScenarioBuilder::new()
        .with_header("Large Scenario", "Performance Test")
        .with_entities();
    
    // Add many entities
    for i in 0..1000 {
        builder = builder
            .add_vehicle(&format!("vehicle_{}", i))
                .car()
                .finish();
    }
    
    let scenario = builder.build()
        .expect("Failed to build large scenario");
    
    let build_time = start.elapsed();
    
    // Serialize
    let start = Instant::now();
    let _xml = openscenario_rs::serialize_to_string(&scenario)
        .expect("Failed to serialize large scenario");
    let serialize_time = start.elapsed();
    
    // Performance assertions
    assert!(build_time.as_millis() < 1000, 
            "Build time too slow: {:?}", build_time);
    assert!(serialize_time.as_millis() < 5000, 
            "Serialization time too slow: {:?}", serialize_time);
}
```

## CI/CD Pipeline Integration

### GitHub Actions

```yaml
# .github/workflows/openscenario-validation.yml
name: OpenSCENARIO Validation

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  validate-scenarios:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        components: rustfmt, clippy
    
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Build OpenSCENARIO-rs
      run: cargo build --release --features builder
    
    - name: Run scenario validation tests
      run: cargo test --features builder scenario_validation
    
    - name: Validate example scenarios
      run: |
        for scenario in scenarios/*.xosc; do
          echo "Validating $scenario"
          cargo run --bin validate-scenario -- "$scenario"
        done
    
    - name: Generate test scenarios
      run: |
        cargo run --example generate_test_scenarios --features builder
        
    - name: Upload generated scenarios
      uses: actions/upload-artifact@v3
      with:
        name: generated-scenarios
        path: generated_scenarios/

  performance-test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Run performance benchmarks
      run: |
        cargo bench --features builder
        
    - name: Check performance regression
      run: |
        # Compare with baseline performance metrics
        cargo run --example builder_performance_demo --features builder --release
```

### Jenkins Pipeline

```groovy
// Jenkinsfile
pipeline {
    agent any
    
    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo"
        PATH = "${CARGO_HOME}/bin:${PATH}"
    }
    
    stages {
        stage('Setup') {
            steps {
                sh 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
                sh 'rustup component add rustfmt clippy'
            }
        }
        
        stage('Build') {
            steps {
                sh 'cargo build --release --features builder'
            }
        }
        
        stage('Test') {
            parallel {
                stage('Unit Tests') {
                    steps {
                        sh 'cargo test --features builder'
                    }
                }
                
                stage('Integration Tests') {
                    steps {
                        sh 'cargo test --features builder --test integration_tests'
                    }
                }
                
                stage('Performance Tests') {
                    steps {
                        sh 'cargo run --example builder_performance_demo --features builder --release'
                    }
                }
            }
        }
        
        stage('Validate Scenarios') {
            steps {
                script {
                    def scenarios = sh(
                        script: 'find scenarios/ -name "*.xosc"',
                        returnStdout: true
                    ).trim().split('\n')
                    
                    for (scenario in scenarios) {
                        sh "cargo run --bin validate-scenario -- '${scenario}'"
                    }
                }
            }
        }
        
        stage('Generate Reports') {
            steps {
                sh 'cargo run --example generate_scenario_report --features builder'
                
                publishHTML([
                    allowMissing: false,
                    alwaysLinkToLastBuild: true,
                    keepAll: true,
                    reportDir: 'reports',
                    reportFiles: 'scenario_report.html',
                    reportName: 'Scenario Validation Report'
                ])
            }
        }
    }
    
    post {
        always {
            archiveArtifacts artifacts: 'target/release/openscenario-rs', fingerprint: true
            junit 'target/test-results.xml'
        }
        
        failure {
            emailext (
                subject: "OpenSCENARIO Build Failed: ${env.JOB_NAME} - ${env.BUILD_NUMBER}",
                body: "Build failed. Check console output at ${env.BUILD_URL}",
                to: "${env.CHANGE_AUTHOR_EMAIL}"
            )
        }
    }
}
```

## IDE and Development Environment Setup

### VS Code Configuration

```json
// .vscode/settings.json
{
    "rust-analyzer.cargo.features": ["builder"],
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.extraArgs": ["--", "-W", "clippy::all"],
    "files.associations": {
        "*.xosc": "xml"
    },
    "xml.validation.enabled": true,
    "xml.validation.schema": [
        {
            "fileMatch": ["*.xosc"],
            "url": "./Schema/OpenSCENARIO.xsd"
        }
    ]
}
```

```json
// .vscode/tasks.json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Build OpenSCENARIO-rs",
            "type": "shell",
            "command": "cargo",
            "args": ["build", "--features", "builder"],
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            }
        },
        {
            "label": "Test Builder System",
            "type": "shell",
            "command": "cargo",
            "args": ["test", "--features", "builder"],
            "group": "test",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            }
        },
        {
            "label": "Run Builder Demo",
            "type": "shell",
            "command": "cargo",
            "args": ["run", "--example", "builder_comprehensive_demo", "--features", "builder"],
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            }
        },
        {
            "label": "Validate Scenario",
            "type": "shell",
            "command": "cargo",
            "args": ["run", "--bin", "xosc-validate", "--", "${file}"],
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            }
        }
    ]
}
```

### IntelliJ IDEA / CLion Configuration

```xml
<!-- .idea/runConfigurations/Build_OpenSCENARIO_rs.xml -->
<component name="ProjectRunConfigurationManager">
  <configuration default="false" name="Build OpenSCENARIO-rs" type="CargoCommandRunConfiguration" factoryName="Cargo Command">
    <option name="command" value="build --features builder" />
    <option name="workingDirectory" value="file://$PROJECT_DIR$" />
    <option name="channel" value="DEFAULT" />
    <option name="requiredFeatures" value="true" />
    <option name="allFeatures" value="false" />
    <option name="emulateTerminal" value="false" />
    <option name="withSudo" value="false" />
    <option name="buildTarget" value="REMOTE" />
    <option name="backtrace" value="SHORT" />
    <envs />
    <option name="isRedirectInput" value="false" />
    <option name="redirectInputPath" value="" />
    <method v="2">
      <option name="CARGO.BUILD_TASK_PROVIDER" enabled="true" />
    </method>
  </configuration>
</component>
```

## Performance Optimization

### Memory Usage Optimization

```rust
use openscenario_rs::{ScenarioBuilder, types::basic::Value};

// Use string interning for repeated values
use string_cache::DefaultAtom;

struct OptimizedScenarioBuilder {
    string_cache: std::collections::HashMap<String, DefaultAtom>,
}

impl OptimizedScenarioBuilder {
    pub fn new() -> Self {
        Self {
            string_cache: std::collections::HashMap::new(),
        }
    }
    
    fn intern_string(&mut self, s: &str) -> DefaultAtom {
        if let Some(atom) = self.string_cache.get(s) {
            atom.clone()
        } else {
            let atom = DefaultAtom::from(s);
            self.string_cache.insert(s.to_string(), atom.clone());
            atom
        }
    }
    
    pub fn build_large_scenario(&mut self, entity_count: usize) 
        -> Result<OpenScenario, Box<dyn std::error::Error>> {
        let mut builder = ScenarioBuilder::new()
            .with_header("Large Scenario", "Optimizer")
            .with_entities();
        
        // Reuse common strings
        let car_type = self.intern_string("car");
        
        for i in 0..entity_count {
            let entity_name = format!("vehicle_{}", i);
            builder = builder
                .add_vehicle(&entity_name)
                    .car() // Reuses interned string
                    .finish();
        }
        
        Ok(builder.build()?)
    }
}
```

### Compilation Time Optimization

```rust
// Use feature flags to reduce compilation time during development
#[cfg(feature = "dev-mode")]
mod dev_utils {
    use openscenario_rs::ScenarioBuilder;
    
    pub fn quick_scenario() -> openscenario_rs::OpenScenario {
        ScenarioBuilder::new()
            .with_header("Dev Test", "Developer")
            .with_entities()
                .add_vehicle("test")
                    .car()
                    .finish()
            .build()
            .unwrap()
    }
}

// Cargo.toml
// [features]
// default = []
// builder = []
// dev-mode = ["builder"]
// full = ["builder", "validation", "catalog"]
```

### Runtime Performance Optimization

```rust
use openscenario_rs::{ScenarioBuilder, OpenScenario};
use std::sync::Arc;
use rayon::prelude::*;

// Parallel scenario processing
fn process_scenarios_parallel(scenarios: Vec<OpenScenario>) -> Vec<ProcessedScenario> {
    scenarios
        .into_par_iter()
        .map(|scenario| process_scenario(scenario))
        .collect()
}

// Scenario caching
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref SCENARIO_CACHE: Mutex<HashMap<String, Arc<OpenScenario>>> = 
        Mutex::new(HashMap::new());
}

fn get_cached_scenario(path: &str) -> Result<Arc<OpenScenario>, Box<dyn std::error::Error>> {
    let mut cache = SCENARIO_CACHE.lock().unwrap();
    
    if let Some(scenario) = cache.get(path) {
        Ok(scenario.clone())
    } else {
        let scenario = Arc::new(openscenario_rs::parse_file(path)?);
        cache.insert(path.to_string(), scenario.clone());
        Ok(scenario)
    }
}
```

## Troubleshooting

### Common Integration Issues

#### 1. Feature Flag Problems

```rust
// Problem: Builder not available
// Solution: Enable the builder feature
// Cargo.toml:
[dependencies]
openscenario-rs = { version = "0.1", features = ["builder"] }

// Or in code:
#[cfg(feature = "builder")]
use openscenario_rs::ScenarioBuilder;

#[cfg(not(feature = "builder"))]
compile_error!("Builder feature is required for this functionality");
```

#### 2. XML Schema Validation Errors

```rust
use openscenario_rs::{parse_file, Error};

fn handle_parsing_errors(path: &str) {
    match parse_file(path) {
        Ok(scenario) => println!("Parsed successfully"),
        Err(Error::XmlError(xml_err)) => {
            eprintln!("XML parsing error: {}", xml_err);
            eprintln!("Check XML syntax and schema compliance");
        },
        Err(Error::ValidationError(val_err)) => {
            eprintln!("Validation error: {}", val_err);
            eprintln!("Check OpenSCENARIO schema compliance");
        },
        Err(e) => eprintln!("Other error: {}", e),
    }
}
```

#### 3. Memory Usage Issues

```rust
// Problem: High memory usage with large scenarios
// Solution: Process efficiently with buffered writing

use openscenario_rs::{parse_file, serialize_to_writer};
use std::fs::File;
use std::io::BufWriter;

fn process_large_scenario(input_path: &str, output_path: &str)
    -> Result<(), Box<dyn std::error::Error>> {
    // Parse scenario
    let scenario = parse_file(input_path)?;

    // Process with buffered output
    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);

    serialize_to_writer(writer, &scenario)?;

    // Scenario is dropped here, freeing memory
    Ok(())
}
```

#### 4. Performance Issues

```rust
// Problem: Slow scenario building
// Solution: Use bulk operations and pre-allocation

use openscenario_rs::ScenarioBuilder;

fn build_scenario_efficiently(entity_count: usize) 
    -> Result<OpenScenario, Box<dyn std::error::Error>> {
    let mut builder = ScenarioBuilder::new()
        .with_header("Efficient Scenario", "Optimizer")
        .with_entities();
    
    // Pre-allocate if possible (depends on implementation)
    // builder.reserve_entities(entity_count);
    
    // Use batch operations
    for i in 0..entity_count {
        builder = builder
            .add_vehicle(&format!("v{}", i))
                .car()
                .finish();
    }
    
    Ok(builder.build()?)
}
```

### Debug Utilities

```rust
use openscenario_rs::{OpenScenario, ScenarioBuilder};

// Debug scenario structure
fn debug_scenario_structure(scenario: &OpenScenario) {
    println!("=== Scenario Debug Info ===");
    println!("Description: {:?}", scenario.file_header.description.as_literal());
    println!("Author: {:?}", scenario.file_header.author.as_literal());
    
    if let Some(entities) = &scenario.entities {
        println!("Entity count: {}", entities.scenario_objects.len());
        for (i, entity) in entities.scenario_objects.iter().enumerate() {
            println!("  Entity {}: {:?}", i, entity.name.as_literal());
        }
    }
    
    if let Some(storyboard) = &scenario.storyboard {
        println!("Story count: {}", storyboard.stories.len());
        for (i, story) in storyboard.stories.iter().enumerate() {
            println!("  Story {}: {:?}", i, story.name.as_literal());
            println!("    Act count: {}", story.acts.len());
        }
    }
}

// Validate scenario completeness
fn validate_scenario_completeness(scenario: &OpenScenario) -> Vec<String> {
    let mut issues = Vec::new();
    
    if scenario.entities.is_none() {
        issues.push("No entities defined".to_string());
    }
    
    if scenario.storyboard.is_none() {
        issues.push("No storyboard defined".to_string());
    }
    
    if let Some(entities) = &scenario.entities {
        if entities.scenario_objects.is_empty() {
            issues.push("No scenario objects defined".to_string());
        }
    }
    
    issues
}
```

This integration guide provides comprehensive coverage of how to integrate OpenSCENARIO-rs into various development workflows and environments. The examples show practical implementations for common use cases and provide solutions for typical integration challenges.