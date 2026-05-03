
            // SPDX-License-Identifier: Apache-2.0
            //
            // Copyright (c) 2025 RTVLAS contributors

            use rtvlas_core::{default_profile, evaluate_scenario, nominal_snapshot, EvidenceBundle, TrustVerdict, write_evidence_bundle};
            use std::fs;

#[test]
            fn threat_standoff_raises_signal() {
                let profile = default_profile();
                let index = profile
                    .properties
                    .iter()
                    .position(|property| property.key == "threat_standoff")
                    .expect("property present");
                let property = profile.properties[index].clone();
                let mut snapshot = nominal_snapshot();
                snapshot.trust_inputs.threat_distance_m = 43.0;
snapshot.trust_inputs.threat_min_distance_m = 46.0;
                let outcome = property.evaluate(&snapshot);
                assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
                assert_eq!(outcome.property_key, "threat_standoff");
            }

#[test]
fn wez_exposure_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "wez_exposure")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.wez_exposure = 0.49;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "wez_exposure");
}

#[test]
fn route_efficiency_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "route_efficiency")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.route_efficiency = 0.71;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "route_efficiency");
}

#[test]
            fn deconfliction_margin_raises_signal() {
                let profile = default_profile();
                let index = profile
                    .properties
                    .iter()
                    .position(|property| property.key == "deconfliction_margin")
                    .expect("property present");
                let property = profile.properties[index].clone();
                let mut snapshot = nominal_snapshot();
                snapshot.trust_inputs.deconfliction_margin_m = 31.0;
snapshot.trust_inputs.min_deconfliction_margin_m = 34.0;
                let outcome = property.evaluate(&snapshot);
                assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
                assert_eq!(outcome.property_key, "deconfliction_margin");
            }

#[test]
fn weapon_action_feasibility_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "weapon_action_feasibility")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.autonomy_solution_feasible = false;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "weapon_action_feasibility");
}

            #[test]
            fn evidence_pipeline_writes_expected_files() {
                let profile = default_profile();
                let scenario_name = "test_scenario";
                let snapshots = vec![nominal_snapshot(), nominal_snapshot()];
                let (timeline, scorecard) = evaluate_scenario(profile, scenario_name, &snapshots);
                let bundle = EvidenceBundle { timeline, scorecard };
                let temp_dir = std::env::temp_dir().join("rtvlas_phase1_evidence");
                let _ = fs::remove_dir_all(&temp_dir);
                fs::create_dir_all(&temp_dir).expect("temp dir");
                let input_log = temp_dir.join("input.jsonl");
                fs::write(
                    &input_log,
                    snapshots
                        .iter()
                        .map(|snapshot| serde_json::to_string(snapshot).expect("json"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
                .expect("input log");
                write_evidence_bundle(&temp_dir, &input_log, &snapshots, &bundle).expect("evidence bundle");
                assert!(temp_dir.join("trust_scorecard.json").exists());
                assert!(temp_dir.join("timeline.json").exists());
                assert!(temp_dir.join("proof_log.txt").exists());
                assert!(temp_dir.join("trace.svg").exists());
            }

            #[test]
            fn reject_path_drops_trust() {
                let mut snapshot = nominal_snapshot();
                snapshot.trust_inputs.threat_distance_m = 27.0;
    snapshot.trust_inputs.threat_min_distance_m = 46.0;
                let (timeline, scorecard) = evaluate_scenario(default_profile(), "reject_case", &[snapshot]);
                assert_eq!(timeline.len(), 1);
                assert!(scorecard.final_trust_score < 1.0);
                assert!(scorecard.reject_frames >= 1 || scorecard.flag_frames >= 1);
            }
