//! Road-based position types for highway and street positioning
//!
//! This file contains:
//! - RoadPosition for road-relative coordinates (road ID, s, t)
//! - RelativeRoadPosition for entity-relative road positioning
//! - LanePosition for lane-specific positioning with offsets
//! - RelativeLanePosition for lane-relative positioning
//! - Road network integration and coordinate validation
//!
//! Contributes to project by:
//! - Supporting OpenDRIVE and road network integration
//! - Providing natural positioning for automotive scenarios
//! - Enabling lane-aware positioning and lane change operations
//! - Facilitating road-following and path planning algorithms
//! - Supporting both absolute and relative road-based positioning