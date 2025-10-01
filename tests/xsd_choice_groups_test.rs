//! Tests for XSD Choice Group Infrastructure
//!
//! This test file validates the core infrastructure for handling XSD choice groups
//! with unbounded occurrences. It tests the trait system, XML event parsing logic,
//! element reading functionality, and integration with simple XML examples.

use openscenario_rs::parser::choice_groups::{
    ChoiceGroupParser, ChoiceGroupRegistry, XsdChoiceGroup, parse_choice_group,
};
use openscenario_rs::{Error, Result};

// Test types for comprehensive testing
#[derive(Debug, PartialEq, Clone)]
enum ComplexVariant {
    SimpleElement(String),
    ElementWithAttributes { id: String, value: i32 },
    NestedElement { name: String, children: Vec<String> },
    EmptyElement,
}

#[derive(Debug, PartialEq)]
struct ComplexChoiceGroup {
    elements: Vec<ComplexVariant>,
    metadata: Option<String>,
}

impl XsdChoiceGroup for ComplexChoiceGroup {
    type Variant = ComplexVariant;

    fn choice_element_names() -> &'static [&'static str] {
        &["SimpleElement", "ElementWithAttributes", "NestedElement", "EmptyElement"]
    }

    fn parse_choice_element(element_name: &str, xml: &str) -> Result<Self::Variant> {
        match element_name {
            "SimpleElement" => {
                let start = xml.find('>').ok_or_else(|| Error::validation_error("xml", "Invalid SimpleElement"))?;
                let end = xml.rfind('<').ok_or_else(|| Error::validation_error("xml", "Invalid SimpleElement"))?;
                let content = &xml[start + 1..end];
                Ok(ComplexVariant::SimpleElement(content.to_string()))
            }
            "ElementWithAttributes" => {
                // Extract id attribute
                let id_start = xml.find("id=\"").ok_or_else(|| Error::validation_error("xml", "Missing id attribute"))?;
                let id_value_start = id_start + 4;
                let id_end = xml[id_value_start..].find('"').ok_or_else(|| Error::validation_error("xml", "Invalid id attribute"))?;
                let id = xml[id_value_start..id_value_start + id_end].to_string();

                // Extract value attribute
                let value_start = xml.find("value=\"").ok_or_else(|| Error::validation_error("xml", "Missing value attribute"))?;
                let value_value_start = value_start + 7;
                let value_end = xml[value_value_start..].find('"').ok_or_else(|| Error::validation_error("xml", "Invalid value attribute"))?;
                let value_str = &xml[value_value_start..value_value_start + value_end];
                let value = value_str.parse::<i32>()
                    .map_err(|_| Error::validation_error("xml", "Invalid integer in value attribute"))?;

                Ok(ComplexVariant::ElementWithAttributes { id, value })
            }
            "NestedElement" => {
                // Extract name attribute
                let name_start = xml.find("name=\"").ok_or_else(|| Error::validation_error("xml", "Missing name attribute"))?;
                let name_value_start = name_start + 6;
                let name_end = xml[name_value_start..].find('"').ok_or_else(|| Error::validation_error("xml", "Invalid name attribute"))?;
                let name = xml[name_value_start..name_value_start + name_end].to_string();

                // Extract child elements (simplified parsing)
                let mut children = Vec::new();
                let mut pos = 0;
                while let Some(child_start) = xml[pos..].find("<Child>") {
                    let absolute_start = pos + child_start + 7;
                    if let Some(child_end) = xml[absolute_start..].find("</Child>") {
                        let child_content = &xml[absolute_start..absolute_start + child_end];
                        children.push(child_content.to_string());
                        pos = absolute_start + child_end + 8;
                    } else {
                        break;
                    }
                }

                Ok(ComplexVariant::NestedElement { name, children })
            }
            "EmptyElement" => {
                Ok(ComplexVariant::EmptyElement)
            }
            _ => Err(Error::choice_group_error("Unknown element")),
        }
    }

    fn from_choice_variants(variants: Vec<Self::Variant>) -> Result<Self> {
        Ok(ComplexChoiceGroup {
            elements: variants,
            metadata: Some("Parsed by choice group infrastructure".to_string()),
        })
    }
}

#[test]
fn test_trait_system_compilation() {
    // Test that the trait system compiles and provides clear interfaces
    let element_names = ComplexChoiceGroup::choice_element_names();
    assert_eq!(element_names.len(), 4);
    assert!(element_names.contains(&"SimpleElement"));
    assert!(element_names.contains(&"ElementWithAttributes"));
    assert!(element_names.contains(&"NestedElement"));
    assert!(element_names.contains(&"EmptyElement"));
}

#[test]
fn test_simple_element_parsing() {
    let xml = "<SimpleElement>Hello World</SimpleElement>";
    let variant = ComplexChoiceGroup::parse_choice_element("SimpleElement", xml).unwrap();
    
    match variant {
        ComplexVariant::SimpleElement(content) => {
            assert_eq!(content, "Hello World");
        }
        _ => panic!("Expected SimpleElement variant"),
    }
}

#[test]
fn test_element_with_attributes_parsing() {
    let xml = r#"<ElementWithAttributes id="test123" value="456">content</ElementWithAttributes>"#;
    let variant = ComplexChoiceGroup::parse_choice_element("ElementWithAttributes", xml).unwrap();
    
    match variant {
        ComplexVariant::ElementWithAttributes { id, value } => {
            assert_eq!(id, "test123");
            assert_eq!(value, 456);
        }
        _ => panic!("Expected ElementWithAttributes variant"),
    }
}

#[test]
fn test_nested_element_parsing() {
    let xml = r#"<NestedElement name="parent"><Child>child1</Child><Child>child2</Child></NestedElement>"#;
    let variant = ComplexChoiceGroup::parse_choice_element("NestedElement", xml).unwrap();
    
    match variant {
        ComplexVariant::NestedElement { name, children } => {
            assert_eq!(name, "parent");
            assert_eq!(children.len(), 2);
            assert_eq!(children[0], "child1");
            assert_eq!(children[1], "child2");
        }
        _ => panic!("Expected NestedElement variant"),
    }
}

#[test]
fn test_empty_element_parsing() {
    let xml = "<EmptyElement/>";
    let variant = ComplexChoiceGroup::parse_choice_element("EmptyElement", xml).unwrap();
    
    match variant {
        ComplexVariant::EmptyElement => {
            // Success
        }
        _ => panic!("Expected EmptyElement variant"),
    }
}

#[test]
fn test_variant_combination() {
    let variants = vec![
        ComplexVariant::SimpleElement("test".to_string()),
        ComplexVariant::EmptyElement,
        ComplexVariant::ElementWithAttributes { id: "id1".to_string(), value: 100 },
    ];
    
    let choice_group = ComplexChoiceGroup::from_choice_variants(variants).unwrap();
    assert_eq!(choice_group.elements.len(), 3);
    assert!(choice_group.metadata.is_some());
    assert_eq!(choice_group.metadata.unwrap(), "Parsed by choice group infrastructure");
}

#[test]
fn test_xml_event_parsing_basic() {
    let xml = r#"
    <Container>
        <SimpleElement>content1</SimpleElement>
        <EmptyElement/>
        <SimpleElement>content2</SimpleElement>
    </Container>
    "#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    assert_eq!(result.elements.len(), 3);
    
    match &result.elements[0] {
        ComplexVariant::SimpleElement(content) => assert_eq!(content, "content1"),
        _ => panic!("Expected SimpleElement"),
    }
    
    match &result.elements[1] {
        ComplexVariant::EmptyElement => {},
        _ => panic!("Expected EmptyElement"),
    }
    
    match &result.elements[2] {
        ComplexVariant::SimpleElement(content) => assert_eq!(content, "content2"),
        _ => panic!("Expected SimpleElement"),
    }
}

#[test]
fn test_xml_event_parsing_complex() {
    let xml = r#"
    <Container>
        <ElementWithAttributes id="first" value="123">some content</ElementWithAttributes>
        <NestedElement name="nested">
            <Child>nested_child1</Child>
            <Child>nested_child2</Child>
        </NestedElement>
        <SimpleElement>simple content</SimpleElement>
    </Container>
    "#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    assert_eq!(result.elements.len(), 3);
    
    match &result.elements[0] {
        ComplexVariant::ElementWithAttributes { id, value } => {
            assert_eq!(id, "first");
            assert_eq!(value, &123);
        }
        _ => panic!("Expected ElementWithAttributes"),
    }
    
    match &result.elements[1] {
        ComplexVariant::NestedElement { name, children } => {
            assert_eq!(name, "nested");
            assert_eq!(children.len(), 2);
            assert_eq!(children[0], "nested_child1");
            assert_eq!(children[1], "nested_child2");
        }
        _ => panic!("Expected NestedElement"),
    }
    
    match &result.elements[2] {
        ComplexVariant::SimpleElement(content) => assert_eq!(content, "simple content"),
        _ => panic!("Expected SimpleElement"),
    }
}

#[test]
fn test_element_reading_functionality() {
    // Test that complete elements are read correctly including nested content
    let xml = r#"
    <Container>
        <NestedElement name="test">
            <Child>content with <SubChild>nested</SubChild> elements</Child>
            <Child>another child</Child>
        </NestedElement>
    </Container>
    "#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    assert_eq!(result.elements.len(), 1);
    
    match &result.elements[0] {
        ComplexVariant::NestedElement { name, children } => {
            assert_eq!(name, "test");
            assert_eq!(children.len(), 2);
            // Note: Our simplified parser doesn't handle nested XML perfectly,
            // but it demonstrates the element reading functionality
            assert!(children[0].contains("nested"));
            assert_eq!(children[1], "another child");
        }
        _ => panic!("Expected NestedElement"),
    }
}

#[test]
fn test_mixed_order_elements() {
    let xml = r#"
    <Container>
        <EmptyElement/>
        <SimpleElement>first</SimpleElement>
        <ElementWithAttributes id="attr1" value="100"/>
        <SimpleElement>second</SimpleElement>
        <EmptyElement/>
        <ElementWithAttributes id="attr2" value="200"/>
    </Container>
    "#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    assert_eq!(result.elements.len(), 6);
    
    // Verify order is preserved
    assert!(matches!(result.elements[0], ComplexVariant::EmptyElement));
    assert!(matches!(result.elements[1], ComplexVariant::SimpleElement(_)));
    assert!(matches!(result.elements[2], ComplexVariant::ElementWithAttributes { .. }));
    assert!(matches!(result.elements[3], ComplexVariant::SimpleElement(_)));
    assert!(matches!(result.elements[4], ComplexVariant::EmptyElement));
    assert!(matches!(result.elements[5], ComplexVariant::ElementWithAttributes { .. }));
}

#[test]
fn test_empty_container() {
    let xml = r#"<Container></Container>"#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    assert_eq!(result.elements.len(), 0);
    assert!(result.metadata.is_some());
}

#[test]
fn test_container_with_whitespace() {
    let xml = r#"
    <Container>
        
        <SimpleElement>content</SimpleElement>
        
        <EmptyElement/>
        
    </Container>
    "#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    assert_eq!(result.elements.len(), 2);
}

#[test]
fn test_container_not_found_error() {
    let xml = r#"<WrongContainer><SimpleElement>test</SimpleElement></WrongContainer>"#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: Result<ComplexChoiceGroup> = parser.parse_choice_group("Container");

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Container element 'Container' not found"));
}

#[test]
fn test_invalid_xml_error() {
    let xml = r#"<Container><SimpleElement>unclosed element</Container>"#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: Result<ComplexChoiceGroup> = parser.parse_choice_group("Container");

    // Should get an XML parsing error
    assert!(result.is_err());
}

#[test]
fn test_unknown_element_ignored() {
    let xml = r#"
    <Container>
        <SimpleElement>valid</SimpleElement>
        <UnknownElement>should be ignored</UnknownElement>
        <EmptyElement/>
    </Container>
    "#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    // Should only parse known elements
    assert_eq!(result.elements.len(), 2);
    assert!(matches!(result.elements[0], ComplexVariant::SimpleElement(_)));
    assert!(matches!(result.elements[1], ComplexVariant::EmptyElement));
}

#[test]
fn test_choice_group_registry() {
    let registry = ChoiceGroupRegistry::new();
    let xml = r#"
    <Container>
        <SimpleElement>registry test</SimpleElement>
        <EmptyElement/>
    </Container>
    "#;

    let result: ComplexChoiceGroup = registry.parse("Container", xml).unwrap();

    assert_eq!(result.elements.len(), 2);
    assert!(matches!(result.elements[0], ComplexVariant::SimpleElement(_)));
    assert!(matches!(result.elements[1], ComplexVariant::EmptyElement));
}

#[test]
fn test_global_registry_function() {
    let xml = r#"
    <Container>
        <ElementWithAttributes id="global" value="999"/>
        <SimpleElement>global test</SimpleElement>
    </Container>
    "#;

    let result: ComplexChoiceGroup = parse_choice_group("Container", xml).unwrap();

    assert_eq!(result.elements.len(), 2);
    
    match &result.elements[0] {
        ComplexVariant::ElementWithAttributes { id, value } => {
            assert_eq!(id, "global");
            assert_eq!(value, &999);
        }
        _ => panic!("Expected ElementWithAttributes"),
    }
    
    match &result.elements[1] {
        ComplexVariant::SimpleElement(content) => assert_eq!(content, "global test"),
        _ => panic!("Expected SimpleElement"),
    }
}

#[test]
fn test_nested_containers_ignored() {
    let xml = r#"
    <Container>
        <SimpleElement>outer</SimpleElement>
        <NestedContainer>
            <SimpleElement>should be ignored</SimpleElement>
        </NestedContainer>
        <EmptyElement/>
    </Container>
    "#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    // Should only parse direct children of Container
    assert_eq!(result.elements.len(), 2);
    
    match &result.elements[0] {
        ComplexVariant::SimpleElement(content) => assert_eq!(content, "outer"),
        _ => panic!("Expected SimpleElement"),
    }
    
    assert!(matches!(result.elements[1], ComplexVariant::EmptyElement));
}

#[test]
fn test_self_closing_container() {
    let xml = r#"<Container/>"#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    assert_eq!(result.elements.len(), 0);
}

#[test]
fn test_multiple_containers_first_match() {
    let xml = r#"
    <Container>
        <SimpleElement>first container</SimpleElement>
    </Container>
    <Container>
        <SimpleElement>second container</SimpleElement>
    </Container>
    "#;

    let mut parser = ChoiceGroupParser::from_str(xml);
    let result: ComplexChoiceGroup = parser.parse_choice_group("Container").unwrap();

    // Should only parse the first matching container
    assert_eq!(result.elements.len(), 1);
    
    match &result.elements[0] {
        ComplexVariant::SimpleElement(content) => assert_eq!(content, "first container"),
        _ => panic!("Expected SimpleElement"),
    }
}

#[test]
fn test_registry_default_trait() {
    let registry = ChoiceGroupRegistry::default();
    let xml = r#"<Container><EmptyElement/></Container>"#;

    let result: ComplexChoiceGroup = registry.parse("Container", xml).unwrap();
    assert_eq!(result.elements.len(), 1);
}

// Integration test with realistic XML structure
#[test]
fn test_integration_with_realistic_xml() {
    let xml = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <Actions>
        <SimpleElement>Action 1</SimpleElement>
        <ElementWithAttributes id="action2" value="42">
            Action 2 Content
        </ElementWithAttributes>
        <NestedElement name="complex_action">
            <Child>parameter1</Child>
            <Child>parameter2</Child>
            <Child>parameter3</Child>
        </NestedElement>
        <EmptyElement/>
        <SimpleElement>Action 5</SimpleElement>
    </Actions>
    "#;

    let result: ComplexChoiceGroup = parse_choice_group("Actions", xml).unwrap();

    assert_eq!(result.elements.len(), 5);
    assert!(result.metadata.is_some());
    
    // Verify all element types are parsed correctly
    let element_types: Vec<_> = result.elements.iter().map(|e| {
        match e {
            ComplexVariant::SimpleElement(_) => "Simple",
            ComplexVariant::ElementWithAttributes { .. } => "WithAttributes",
            ComplexVariant::NestedElement { .. } => "Nested",
            ComplexVariant::EmptyElement => "Empty",
        }
    }).collect();
    
    assert_eq!(element_types, vec!["Simple", "WithAttributes", "Nested", "Empty", "Simple"]);
}