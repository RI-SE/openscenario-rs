//! Test for position builders to verify Phase 2 fixes

#[cfg(feature = "builder")]
mod position_builder_tests {
    use openscenario_rs::builder::positions::{WorldPositionBuilder, RelativePositionBuilder, LanePositionBuilder};
    use openscenario_rs::builder::positions::PositionBuilder;

    #[test]
    fn test_world_position_builder() {
        let result = WorldPositionBuilder::new()
            .x(100.0)
            .y(200.0)
            .z(0.0)
            .heading(1.57)
            .finish();
        
        assert!(result.is_ok(), "WorldPositionBuilder should work: {:?}", result.err());
        
        let position = result.unwrap();
        assert!(position.world_position.is_some());
        
        let world_pos = position.world_position.unwrap();
        assert_eq!(world_pos.x.as_literal().unwrap(), &100.0);
        assert_eq!(world_pos.y.as_literal().unwrap(), &200.0);
        assert_eq!(world_pos.z.as_ref().unwrap().as_literal().unwrap(), &0.0);
        assert_eq!(world_pos.h.as_ref().unwrap().as_literal().unwrap(), &1.57);
    }

    #[test]
    fn test_relative_position_builder() {
        let result = RelativePositionBuilder::new()
            .to_entity("target_vehicle")
            .world_offset(5.0, -2.0, 0.0)
            .finish();
        
        assert!(result.is_ok(), "RelativePositionBuilder should work: {:?}", result.err());
        
        let position = result.unwrap();
        assert!(position.relative_world_position.is_some());
        
        let rel_pos = position.relative_world_position.unwrap();
        assert_eq!(rel_pos.entity_ref.as_literal().unwrap(), "target_vehicle");
        assert_eq!(rel_pos.dx.as_literal().unwrap(), &5.0);
        assert_eq!(rel_pos.dy.as_literal().unwrap(), &-2.0);
        assert_eq!(rel_pos.dz.as_literal().unwrap(), &0.0);
    }

    #[test]
    fn test_lane_position_builder() {
        let result = LanePositionBuilder::new()
            .road("highway_1")
            .lane("1")
            .s(100.0)
            .offset(0.0)
            .finish();
        
        assert!(result.is_ok(), "LanePositionBuilder should work: {:?}", result.err());
        
        let position = result.unwrap();
        assert!(position.lane_position.is_some());
        
        let lane_pos = position.lane_position.unwrap();
        assert_eq!(lane_pos.road_id.as_literal().unwrap(), "highway_1");
        assert_eq!(lane_pos.lane_id.as_literal().unwrap(), "1");
        assert_eq!(lane_pos.s.as_literal().unwrap(), &100.0);
        assert_eq!(lane_pos.offset.as_literal().unwrap(), &0.0);
    }

    #[test]
    fn test_lane_position_builder_with_integer_lane() {
        let result = LanePositionBuilder::new()
            .road("test_road")
            .right_lane("test_road", 2, 50.0)
            .finish();
        
        assert!(result.is_ok(), "LanePositionBuilder with integer lane should work: {:?}", result.err());
        
        let position = result.unwrap();
        assert!(position.lane_position.is_some());
        
        let lane_pos = position.lane_position.unwrap();
        assert_eq!(lane_pos.road_id.as_literal().unwrap(), "test_road");
        assert_eq!(lane_pos.lane_id.as_literal().unwrap(), "2");
        assert_eq!(lane_pos.s.as_literal().unwrap(), &50.0);
    }

    #[test]
    fn test_relative_lane_position_builder() {
        let result = RelativePositionBuilder::new()
            .to_entity("lead_vehicle")
            .lane_offset(10.0, 0.0)
            .finish();
        
        assert!(result.is_ok(), "RelativePositionBuilder for lane should work: {:?}", result.err());
        
        let position = result.unwrap();
        assert!(position.relative_lane_position.is_some());
        
        let rel_lane_pos = position.relative_lane_position.unwrap();
        assert_eq!(rel_lane_pos.entity_ref.as_literal().unwrap(), "lead_vehicle");
        assert_eq!(rel_lane_pos.ds.as_literal().unwrap(), &10.0);
        assert_eq!(rel_lane_pos.d_lane.as_literal().unwrap(), &0);
        assert_eq!(rel_lane_pos.offset.as_literal().unwrap(), &0.0);
    }
}