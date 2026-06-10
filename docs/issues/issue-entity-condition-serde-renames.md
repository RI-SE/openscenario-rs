# fix(serde): add missing @-prefixed renames to entity condition structs in `conditions/entity.rs`

**Repo:** https://github.com/RI-SE/openscenario-rs  
**Labels:** `bug`, `serde`, `xml`

---

## Summary

Eleven structs in `src/types/conditions/entity.rs` are missing explicit
`#[serde(rename = ...)]` annotations entirely. Because quick-xml 0.38 requires
`@name` for XML attributes and `PascalCase` for child elements, every field in
these structs serialises/deserialises with the wrong key — causing silent data
loss or hard parse failures.

This is the **Priority 3** item from the serde annotation audit.  Priorities 1,
2, 4, and 5 from that audit were fixed as low-risk inline changes; this one is
tracked separately because it touches ~30 fields across 11 structs and warrants
its own review + test pass.

---

## Affected structs and required fixes

### `EndOfRoadCondition`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `duration: Double` | `<duration>` child element | required attr `duration` | `#[serde(rename = "@duration")]` |

### `OffroadCondition`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `duration: Double` | `<duration>` child element | required attr `duration` | `#[serde(rename = "@duration")]` |

### `CollisionCondition`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `target: Option<OSString>` | `<target>` | child `<EntityRef entityRef="...">` | `#[serde(rename = "EntityRef", skip_serializing_if = "Option::is_none")]` |
| `by_type: Option<CollisionTarget>` | `<by_type>` | child `<ByObjectType objectType="...">` | `#[serde(rename = "ByObjectType", skip_serializing_if = "Option::is_none")]` |
| `position: Option<Position>` | `<position>` | child `<Position>` | `#[serde(rename = "Position", skip_serializing_if = "Option::is_none")]` |

### `CollisionTarget`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `target_type: OSString` | `<target_type>` | required attr `objectType` on `<ByObjectType>` | `#[serde(rename = "@objectType")]` |

### `TimeToCollisionCondition`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `target: TimeToCollisionTarget` | `<target>` | child `<TimeToCollisionConditionTarget>` | `#[serde(rename = "TimeToCollisionConditionTarget")]` |

### `TimeToCollisionTarget`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `entity_ref: Option<EntityRef>` | `<entity_ref>` | child `<EntityRef>` | `#[serde(rename = "EntityRef", skip_serializing_if = "Option::is_none")]` |
| `position: Option<Position>` | `<position>` | child `<Position>` | `#[serde(rename = "Position", skip_serializing_if = "Option::is_none")]` |

### `AngleCondition`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `angle_type: AngleType` | `<angle_type>` | required attr `angleType` | `#[serde(rename = "@angleType")]` |
| `angle: Double` | `<angle>` | required attr `angle` | `#[serde(rename = "@angle")]` |
| `angle_tolerance: Double` | `<angle_tolerance>` | required attr `angleTolerance` | `#[serde(rename = "@angleTolerance")]` |
| `coordinate_system: Option<CoordinateSystem>` | `<coordinate_system>` | optional attr `coordinateSystem` | `#[serde(rename = "@coordinateSystem", default, skip_serializing_if = "Option::is_none")]` |

### `RelativeSpeedCondition`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `entity_ref: OSString` | `<entity_ref>` | required attr `entityRef` | `#[serde(rename = "@entityRef")]` |
| `rule: Rule` | `<rule>` | required attr `rule` | `#[serde(rename = "@rule")]` |
| `value: Double` | `<value>` | required attr `value` | `#[serde(rename = "@value")]` |
| `direction: Option<DirectionalDimension>` | `<direction>` | optional attr `direction` | `#[serde(rename = "@direction", default, skip_serializing_if = "Option::is_none")]` |

### `RelativeLaneRange`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `from: Option<Int>` | `<from>` | optional attr `from` | `#[serde(rename = "@from", default, skip_serializing_if = "Option::is_none")]` |
| `to: Option<Int>` | `<to>` | optional attr `to` | `#[serde(rename = "@to", default, skip_serializing_if = "Option::is_none")]` |

### `RelativeClearanceCondition`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `relative_lane_ranges: Vec<RelativeLaneRange>` | `<relative_lane_ranges>` | repeated child `<RelativeLaneRange>` | `#[serde(rename = "RelativeLaneRange", default)]` |
| `entity_refs: Vec<EntityRef>` | `<entity_refs>` | repeated child `<EntityRef>` | `#[serde(rename = "EntityRef", default)]` |
| `opposite_lanes: Boolean` | `<opposite_lanes>` | required attr `oppositeLanes` | `#[serde(rename = "@oppositeLanes")]` |
| `distance_forward: Option<Double>` | `<distance_forward>` | optional attr `distanceForward` | `#[serde(rename = "@distanceForward", default, skip_serializing_if = "Option::is_none")]` |
| `distance_backward: Option<Double>` | `<distance_backward>` | optional attr `distanceBackward` | `#[serde(rename = "@distanceBackward", default, skip_serializing_if = "Option::is_none")]` |
| `free_space: Boolean` | `<free_space>` | required attr `freeSpace` | `#[serde(rename = "@freeSpace")]` |

### `RelativeAngleCondition`

| Field | Current (broken) | XSD says | Fix |
|---|---|---|---|
| `entity_ref: OSString` | `<entity_ref>` | required attr `entityRef` | `#[serde(rename = "@entityRef")]` |
| `angle_type: AngleType` | `<angle_type>` | required attr `angleType` | `#[serde(rename = "@angleType")]` |
| `angle: Double` | `<angle>` | required attr `angle` | `#[serde(rename = "@angle")]` |
| `angle_tolerance: Double` | `<angle_tolerance>` | required attr `angleTolerance` | `#[serde(rename = "@angleTolerance")]` |
| `coordinate_system: Option<CoordinateSystem>` | `<coordinate_system>` | optional attr `coordinateSystem` | `#[serde(rename = "@coordinateSystem", default, skip_serializing_if = "Option::is_none")]` |

---

## Acceptance criteria

- [ ] All 11 structs above have per-field explicit `#[serde(rename = ...)]` annotations matching the tables above
- [ ] A round-trip XML serialise→deserialise test is added for each affected struct
- [ ] A deserialise-from-raw-XML test is added for `EndOfRoadCondition`, `OffroadCondition`, `AngleCondition`, and `RelativeClearanceCondition` confirming correct attribute names are parsed
- [ ] `cargo test` passes with no regressions

---

## XSD reference lines (Schema/OpenSCENARIO.xsd)

| Type | Line |
|---|---|
| `EndOfRoadCondition` | 1119 |
| `OffroadCondition` | 1529 |
| `CollisionCondition` | 923 |
| `AngleCondition` | 734 |
| `RelativeSpeedCondition` | 1883 |
| `RelativeLaneRange` | 1862 |
| `RelativeClearanceCondition` | 1833 |
| `RelativeAngleCondition` | 1826 |
| `TimeToCollisionCondition` | 2179 |

---

## Pattern reference

Correct annotation style already used in the same repo (e.g. `WorldPosition`, `CatalogVertex`,
`TrajectoryPosition`):

```rust
#[serde(rename = "@entityRef")]
pub entity_ref: OSString,

#[serde(rename = "@coordinateSystem", default, skip_serializing_if = "Option::is_none")]
pub coordinate_system: Option<CoordinateSystem>,

#[serde(rename = "EntityRef", default)]
pub entity_refs: Vec<EntityRef>,
```
