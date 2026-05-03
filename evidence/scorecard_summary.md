# Evidence Scorecard Summary

- Topic: `DAF26BZ01-NV006 Intelligent Threat Aware Autonomy`
- Generated: `2026-05-03T04:54:57Z`
- Git Head: `6924e86c309ffba6d16384bbf13ed201659d8d9f`
- Scenario Pass Rate: `3/3 (100.0%)`
- Evidence Type: `deterministic synthetic autonomy traces for submission-stage feasibility review`

| Scenario | Mode | Result | Final Trust | First Reject | Scorecard |
| --- | --- | --- | --- | --- | --- |
| Nominal WEZ-Aware Mutual Support Route | `nominal` | `PASS` | `1.000` | `None` | [scenario_01_nominal_threat_route](scenario_01_nominal_threat_route/trust_scorecard.json) |
| Overaggressive WEZ Skirt | `degraded` | `PASS` | `0.460` | `None` | [scenario_02_overaggressive_avoidance](scenario_02_overaggressive_avoidance/trust_scorecard.json) |
| Unsafe WEZ Penetration and Action Failure | `fault` | `PASS` | `0.003` | `20` | [scenario_03_unsafe_wez_penetration](scenario_03_unsafe_wez_penetration/trust_scorecard.json) |

## Notes

- Nominal scenarios are expected to remain fully accepted.
- Degraded scenarios are expected to produce concern signals without hard reject behavior.
- Fault scenarios are expected to produce deterministic reject behavior.
- This summary is generated automatically from the underlying per-scenario scorecards.
