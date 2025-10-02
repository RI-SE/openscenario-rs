use openscenario_rs::parser::xml::parse_from_str;

fn main() {
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="3" date="2021-07-09T10:00:00" description="Test" author="Test">
    </FileHeader>
    <Entities>
        <ScenarioObject name="Ego">
        </ScenarioObject>
    </Entities>
    <Storyboard>
        <Init>
            <Actions>
            </Actions>
        </Init>
        <Story name="TestStory">
            <Act name="TestAct">
                <ManeuverGroup maximumExecutionCount="1" name="TestGroup">
                    <Actors selectTriggeringEntities="false">
                        <EntityRef entityRef="Ego" />
                    </Actors>
                    <Maneuver name="TestManeuver">
                        <Event name="TestEvent" priority="override">
                            <Action name="TestAction">
                                <PrivateAction>
                                </PrivateAction>
                            </Action>
                            <StartTrigger>
                                <ConditionGroup>
                                    <Condition name="TestCondition" delay="0" conditionEdge="rising">
                                        <ByEntityCondition>
                                            <TriggeringEntities triggeringEntitiesRule="any">
                                                <EntityRef entityRef="Ego" />
                                            </TriggeringEntities>
                                            <EntityCondition>
                                                <RelativeDistanceCondition entityRef="CutInVehicle" relativeDistanceType="longitudinal" value="$CutInVehicle_HeadwayDistanceTrigger_dx0_m" freespace="true" rule="lessThan" coordinateSystem="entity" />
                                            </EntityCondition>
                                        </ByEntityCondition>
                                    </Condition>
                                </ConditionGroup>
                            </StartTrigger>
                        </Event>
                    </Maneuver>
                </ManeuverGroup>
                <StartTrigger>
                    <ConditionGroup>
                        <Condition name="ActStart" delay="0" conditionEdge="none">
                            <ByValueCondition>
                                <SimulationTimeCondition value="0" rule="greaterOrEqual" />
                            </ByValueCondition>
                        </Condition>
                    </ConditionGroup>
                </StartTrigger>
            </Act>
        </Story>
        <StopTrigger>
            <ConditionGroup>
                <Condition name="End" delay="10.0" conditionEdge="rising">
                    <ByValueCondition>
                        <SimulationTimeCondition value="100.0" rule="greaterOrEqual" />
                    </ByValueCondition>
                </Condition>
            </ConditionGroup>
        </StopTrigger>
    </Storyboard>
</OpenSCENARIO>"#;

    match parse_from_str(xml) {
        Ok(scenario) => {
            println!("✅ Successfully parsed scenario with RelativeDistanceCondition");
            println!("Document type: {:?}", scenario.document_type());
        }
        Err(e) => {
            println!("❌ Failed to parse: {}", e);
        }
    }
}
