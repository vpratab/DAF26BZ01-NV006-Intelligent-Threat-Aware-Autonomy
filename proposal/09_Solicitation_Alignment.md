# Solicitation Alignment

## Topic Basis

- **Track posture:** Phase I
- **Source basis:** DAF 26.BZ Release 1 Phase I topic language, pp. 11-13.
- **Objective summary:** Advance one or more core ITA2 design challenges, especially WEZ modeling, WEZ avoidance, advanced weaponeering, and mutual-support behaviors for ACPs.

## What This Repository Intentionally Covers

- WEZ-aware risk measurement and route sanity checking
- avoidance behavior for multiple static or moving threats
- weapon-task or action feasibility checks tied to mission context
- mutual-support and deconfliction logic for collaborative ACP behaviors

## How The Repository Maps To The Topic

| Solicitation Need | Repository Response |
| --- | --- |
| Topic-specific runtime checks | `core/src/profile.rs` encodes five topic-shaped trust properties tied to this mission area. |
| Repeatable proof and replay | `tooling/replay`, `tooling/eval`, `evidence/`, and `package_manifest.json` provide deterministic reproduction. |
| Integration path | `bindings/include/rt_vlas.h` and `bindings/src/lib.rs` define the C ABI boundary for autonomy-stack insertion. |
| Reviewer-verifiable evidence | `evidence/scorecard_summary.md`, `proof_log.txt`, `timeline.json`, and `trace.svg` make the behavior inspectable. |
| Clear scope discipline | This repository is scoped as: This repository focuses on verifying threat-aware behaviors and mission outputs at runtime; it does not attempt to replace the underlying threat-aware planner or inner-loop vehicle controller. |

## What The Package Is Not Claiming

- it is not a replacement for the underlying autonomy stack
- it is not a certification package
- it is not based on classified program data
- it is not claiming operational fielding approval

## Why The Current Shape Is Credible

The strongest near-term value of RTVLAS is the ability to make autonomy behavior observable,
explainable, and rejectable when it drifts outside mission or safety expectations. That is the
thread this repository follows for this specific topic.
