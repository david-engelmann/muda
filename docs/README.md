# Muda Documentation

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/lockup-dark.svg">
    <img src="media/lockup-light.svg" alt="muda" width="760">
  </picture>
</p>

Welcome to the documentation suite for `muda` (Japanese for *waste*), the zero-dependency Git rework meter and monthly statement generator in the Retrospective Line.

## Documentation Index

| Document | Audience | Contents |
| :--- | :--- | :--- |
| [Operator & Integrator Guide](GUIDE.md) | Engineering leads & CI authors | Installation, `statement` flags, UTC month windowing, `rates.json`, GitHub Actions `fetch-depth: 0` recipe |
| [Technical Architecture](ARCHITECTURE.md) | Systems engineers & contributors | Reachable Git DAG walker, three rework classifiers & cause joins, exact integer minor-unit pricer, repo write guard |
| [Wire Formats & Schemas](FORMATS.md) | FinOps & dashboard integrators | `rates.json` (`schema/rates.schema.json`) and monthly statement JSON (`schema/statement.schema.json`) |
| [Ecosystem Comparison](COMPARISON.md) | Engineering leaders | How `muda` differs from DORA/git-churn dashboards (LinearB/Swarmia), token billing, and `catchledger` |
| [Performance & Scale](PERFORMANCE.md) | Infrastructure teams | Measured Git DAG billing latency (`50` to `500` commits in `< 1s` on Apple M3 Max) |
| [Frequently Asked Questions](FAQ.md) | All users | Crates.io/PyPI naming warning (`muda` collision), `unjoined` items, UTC month boundaries, shallow clones |
| [Storefront README Draft](STOREFRONT_README_DRAFT.md) | Maintainers & Muse seat | Drop-in visual storefront upgrade preserving `32 of 100` rounds pin |

## Machine-Readable Contracts

- Schemas: [`../schema/rates.schema.json`](../schema/rates.schema.json), [`../schema/statement.schema.json`](../schema/statement.schema.json)
- Example Rate Card: [`../examples/rates.json`](../examples/rates.json)
