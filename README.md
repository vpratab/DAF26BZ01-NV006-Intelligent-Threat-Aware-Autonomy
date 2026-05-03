# DAF26BZ01-NV006 Intelligent Threat Aware Autonomy

[![Topic](https://img.shields.io/badge/Topic-DAF26BZ01-NV006-0A3D62)](proposal/02_Technical_Volume.md)
[![Core](https://img.shields.io/badge/Core-Rust_Runtime_Monitor-1F618D)](core/src/lib.rs)
[![Bindings](https://img.shields.io/badge/ABI-C_Compatible-117A65)](bindings/include/rt_vlas.h)
[![Evidence](https://img.shields.io/badge/Evidence-Prebuilt-9A7D0A)](evidence/)
[![Package](https://img.shields.io/badge/Submission-Rebuildable-7C3AED)](scripts/prepare_package.sh)

This repository packages **RTVLAS** for **DAF26BZ01-NV006 Intelligent Threat Aware Autonomy** as a **sanity checker for threat-aware autonomy; mission path/constraint assurance layer**.

> RTVLAS adapted as a runtime assurance layer for threat-aware autonomy, checking whether generated paths and mission actions remain tactically sane in the presence of dynamic threats and deconfliction constraints.

**End product form:** Runtime verification module that checks threat-aware pathing and mission autonomy outputs using a low-compute safety property framework and replay/evaluation tooling.
**Solicitation track:** Phase I

## Reviewer Start

- [Submission Index](proposal/00_Submission_Index.md)
- [Executive Summary](proposal/01_Executive_Summary.md)
- [Technical Volume](proposal/02_Technical_Volume.md)
- [Reviewer Guide](proposal/04_Reviewer_Guide.md)
- [Claim / Artifact Matrix](proposal/05_Claim_Artifact_Matrix.md)
- [Risk Register](proposal/07_Risk_Register.md)
- [Data Provenance](proposal/08_Data_Provenance.md)
- [Solicitation Alignment](proposal/09_Solicitation_Alignment.md)
- [Submission Checklist](proposal/10_Submission_Checklist.md)
- [Required Inputs](proposal/11_Required_Inputs.md)
- [Docs Index](docs/README.md)
- [Evidence Guide](evidence/README.md)
- [Evidence Summary](evidence/scorecard_summary.md)
- [Package Manifest](package_manifest.json)

## Why This Repo Exists

RTVLAS is not positioned here as the autonomy stack. It is positioned as the **runtime trust layer**
that independently monitors autonomy outputs, applies topic-specific safety and mission properties,
and emits structured evidence for operator review, recovery logic, and technical due diligence.

## Solicitation Focus This Repo Targets

- WEZ-aware risk measurement and route sanity checking
- avoidance behavior for multiple static or moving threats
- weapon-task or action feasibility checks tied to mission context
- mutual-support and deconfliction logic for collaborative ACP behaviors

## System Shape

```mermaid
flowchart LR
    A["Autonomy State Snapshot"] --> B["RTVLAS Monitor"]
    B --> C["Topic-Specific Property Set"]
    C --> D["Trust Verdict"]
    D --> E["Evidence Bundle"]
    D --> F["Integration Hooks"]
    E --> G["Reviewer Package"]
```

## Evidence Snapshot

| Scenario | Expected Outcome |
| --- | --- |
| [Nominal WEZ-Aware Mutual Support Route](evidence/scenario_01_nominal_threat_route/trust_scorecard.json) | Threat-aware planner preserves standoff and route quality while maintaining cooperative spacing and action feasibility. |
| [Overaggressive WEZ Skirt](evidence/scenario_02_overaggressive_avoidance/trust_scorecard.json) | The planner remains feasible but accepts elevated WEZ exposure and reduced route efficiency while mutual-support margins begin to erode. |
| [Unsafe WEZ Penetration and Action Failure](evidence/scenario_03_unsafe_wez_penetration/trust_scorecard.json) | Threat-aware autonomy collapses standoff, deconfliction, and action feasibility constraints, producing reject-grade mission behavior. |

## One Command Rebuild

```bash
./scripts/prepare_package.sh
```

Rebuild output:

- regenerated `evidence/`
- regenerated `evidence/scorecard_summary.md` and `package_manifest.json`
- refreshed `submission_package/`
- rebuilt Rust workspace and tests

## Current Evidence Boundaries

This package is intentionally honest about maturity. Current evidence is based on deterministic,
topic-shaped autonomy traces generated inside this repository for repeatable feasibility or readiness review.
See [proposal/08_Data_Provenance.md](proposal/08_Data_Provenance.md) and [package_manifest.json](package_manifest.json).

## Repository Map

- [core/](core/): runtime monitor, property framework, evidence writer
- [bindings/](bindings/): C ABI for external autonomy stacks
- [tooling/](tooling/): replay, evaluation, and optional viewer tooling
- [evidence/](evidence/): pre-generated artifacts for all scenarios
- [proposal/](proposal/): reviewer-facing submission package
- [docs/](docs/): architecture and API references
- [scenarios/](scenarios/): deterministic input traces used to generate evidence
- [scripts/](scripts/): package rebuild and scenario execution
