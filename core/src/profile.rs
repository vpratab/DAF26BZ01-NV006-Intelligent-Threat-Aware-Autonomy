
        // SPDX-License-Identifier: Apache-2.0
        //
        // Copyright (c) 2025 RTVLAS contributors

        use crate::model::{AutonomySnapshot, BoolField, NumericField, PropertyKind, PropertySpec, TrustInputs};
        use crate::monitor::MonitorProfile;

        pub fn default_profile() -> MonitorProfile {
            MonitorProfile {
                topic_id: "DAF26BZ01-NV006".to_string(),
                title: "Intelligent Threat Aware Autonomy".to_string(),
                framing: "sanity checker for threat-aware autonomy; mission path/constraint assurance layer".to_string(),
                properties: vec![
        PropertySpec::new(
            "threat_standoff",
            "Threat Standoff Margin",
            "Ensures planned paths preserve minimum standoff distance from dynamic threats and engagement envelopes.",
            PropertyKind::MinMargin { field: NumericField::ThreatDistanceM, reference: NumericField::ThreatMinDistanceM },
            1.2,
        ),
        PropertySpec::new(
            "wez_exposure",
            "WEZ Exposure Bound",
            "Flags paths that accept excessive time inside modeled weapon engagement exposure corridors.",
            PropertyKind::MaxValue { field: NumericField::WezExposure, max: 0.42 },
            1.0,
        ),
        PropertySpec::new(
            "route_efficiency",
            "Route Efficiency Floor",
            "Detects threat avoidance plans that degrade mission efficiency below an acceptable tactical threshold.",
            PropertyKind::MinValue { field: NumericField::RouteEfficiency, min: 0.75 },
            0.8,
        ),
        PropertySpec::new(
            "deconfliction_margin",
            "Mutual Support Deconfliction Margin",
            "Preserves minimum spacing from friendly cooperative assets while threat responses are being executed.",
            PropertyKind::MinMargin { field: NumericField::DeconflictionMarginM, reference: NumericField::MinDeconflictionMarginM },
            1.1,
        ),
        PropertySpec::new(
            "weapon_action_feasibility",
            "Weapon / Action Feasibility",
            "Ensures the threat-aware autonomy has not collapsed into an internally infeasible route-and-engagement pairing.",
            PropertyKind::BooleanGate { field: BoolField::AutonomySolutionFeasible, reject_on_false: true },
            0.9,
        )
                ],
            }
        }

        pub fn nominal_snapshot() -> AutonomySnapshot {
            AutonomySnapshot {
    timestamp_ms: 0,
    position_m: [0.0, 0.0, 180.0],
    velocity_mps: [22.0, 1.5, 0.0],
    heading_rad: 0.08,
    trust_inputs: TrustInputs {
        gps_valid: true,
        operator_link: true,
        autonomy_solution_feasible: true,
        mission_plan_valid: true,
        emergency_response_ready: true,
        temporal_skew_ms: 12.0,
        corridor_error_m: 8.0,
        corridor_half_width_m: 24.0,
        command_speed_mps: 26.0,
        max_safe_speed_mps: 38.0,
        deconfliction_margin_m: 55.0,
        min_deconfliction_margin_m: 25.0,
        formation_spacing_m: 40.0,
        desired_spacing_m: 40.0,
        heading_error_rad: 0.05,
        threat_distance_m: 76.0,
        threat_min_distance_m: 46.0,
        wez_exposure: 0.18,
        route_efficiency: 0.91,
        decision_latency_ms: 140.0,
        operator_intent_alignment: 0.94,
        evidence_completeness: 0.97,
        hazard_distance_m: 74.0,
        min_hazard_distance_m: 42.0,
        safe_altitude_margin_m: 48.0,
        recovery_zone_distance_m: 920.0,
        max_recovery_zone_distance_m: 1600.0,
        autonomy_solution_optimality: 0.91,
    },
}
        }
