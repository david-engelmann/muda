# Ecosystem Comparison

| Dimension | `muda` | Engineering Metrics SaaS (LinearB, Swarmia, Jellyfish) | LLM Token Billing Dashboards | `catchledger` (Fleet Sibling) |
| :--- | :--- | :--- | :--- | :--- |
| **Core question** | Which commits caused reverts, fix-forwards, and redone work this month, and what did they cost? | Team velocity, PR cycle time, generic line-churn % | How many API tokens were consumed per model/key? | What do CI gates and verification checks cost vs catch? |
| **Causal attribution** | Exact SHA-to-SHA join (`revert subject`, `Fixes` trailer, `redone task`) + explicit `unjoined[]` | Heuristic line-churn (<21 days) with no causal SHA join | None (session-level only) | Gate-level cost accounting |
| **Deployment** | Single `791 KiB` zero-dep Rust binary over local Git | Cloud SaaS requiring GitHub org OAuth & webhook sync | Cloud billing export | Single local Rust binary |
| **Arithmetic** | Exact integer minor units | Floating-point aggregates | Vendor invoice floats | Deterministic local ledger |
