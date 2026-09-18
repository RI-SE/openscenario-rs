//! Nine `Vec` fields below elements the XSD marks required (`minOccurs >= 1`) used to carry
//! `#[serde(default)]`, so a missing child list deserialized into an empty vector instead of
//! failing to parse. Each test below documents one field: the empty document must now be
//! rejected, and the error text is asserted so a future change that silently restores `default`
//! is caught by CI rather than by another audit.

use openscenario_rs::types::actions::movement::SpeedProfileAction;
use openscenario_rs::types::actions::traffic::{
    ControllerDistribution, VehicleCategoryDistribution, VehicleRoleDistribution,
};
use openscenario_rs::types::geometry::shapes::{ClothoidSpline, Nurbs, Polyline};
use openscenario_rs::types::road::UsedArea;

fn parse<T: for<'de> serde::Deserialize<'de>>(xml: &str) -> Result<T, quick_xml::de::DeError> {
    quick_xml::de::from_str(xml)
}

#[test]
fn used_area_without_position_is_rejected() {
    let err = parse::<UsedArea>("<UsedArea/>").unwrap_err();
    assert_eq!(err.to_string(), "missing field `Position`");
}

#[test]
fn speed_profile_action_without_entries_is_rejected() {
    let err = parse::<SpeedProfileAction>(r#"<SpeedProfileAction followingMode="position"/>"#)
        .unwrap_err();
    assert_eq!(err.to_string(), "missing field `SpeedProfileEntry`");
}

#[test]
fn vehicle_role_distribution_without_entries_is_rejected() {
    let err = parse::<VehicleRoleDistribution>("<VehicleRoleDistribution/>").unwrap_err();
    assert_eq!(
        err.to_string(),
        "missing field `VehicleRoleDistributionEntry`"
    );
}

#[test]
fn vehicle_category_distribution_without_entries_is_rejected() {
    let err = parse::<VehicleCategoryDistribution>("<VehicleCategoryDistribution/>").unwrap_err();
    assert_eq!(
        err.to_string(),
        "missing field `VehicleCategoryDistributionEntry`"
    );
}

#[test]
fn controller_distribution_without_entries_is_rejected() {
    let err = parse::<ControllerDistribution>("<ControllerDistribution/>").unwrap_err();
    assert_eq!(
        err.to_string(),
        "missing field `ControllerDistributionEntry`"
    );
}

#[test]
fn clothoid_spline_without_segments_is_rejected() {
    let err = parse::<ClothoidSpline>("<ClothoidSpline/>").unwrap_err();
    assert_eq!(err.to_string(), "missing field `ClothoidSplineSegment`");
}

#[test]
fn nurbs_without_control_points_is_rejected() {
    let err = parse::<Nurbs>(r#"<Nurbs order="3"/>"#).unwrap_err();
    assert_eq!(err.to_string(), "missing field `ControlPoint`");
}

#[test]
fn nurbs_without_knots_is_rejected() {
    // `ControlPoint` is declared first in the type's field order, so once it is present the
    // next missing required field reported is `Knot`.
    let err = parse::<Nurbs>(
        r#"<Nurbs order="3"><ControlPoint><Position/></ControlPoint><ControlPoint><Position/></ControlPoint></Nurbs>"#,
    )
    .unwrap_err();
    assert_eq!(err.to_string(), "missing field `Knot`");
}

#[test]
fn polyline_without_vertices_is_rejected() {
    let err = parse::<Polyline>("<Polyline/>").unwrap_err();
    assert_eq!(err.to_string(), "missing field `Vertex`");
}
