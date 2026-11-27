# Muda Wire Formats & Schemas

## 1. Rates Card Input (`schema/rates.schema.json`)

```json
{
  "currency": "USD",
  "per_item": {
    "revert": "15.00",
    "fix-forward": "7.50",
    "redone": "4.00"
  }
}
```

- `currency`: Exact 3-letter uppercase ASCII ISO currency code (`USD`, `EUR`, `GBP`, ...).
- `per_item`: Non-negative decimal string rates for all three classes (`revert`, `fix-forward`, `redone`).

## 2. Monthly Statement Output (`schema/statement.schema.json`)

```json
{
  "month": "2026-01",
  "window": { "from": "2026-01-01", "to": "2026-02-01" },
  "commits_read": 6,
  "merges_read": 0,
  "causes": [
    {
      "cause": "6b103c2866...",
      "items": [
        { "sha": "91fa0c...", "class": "revert", "join": "revert subject" }
      ]
    }
  ],
  "unjoined": [],
  "totals": {
    "items": 1,
    "per_class": { "revert": 1, "fix-forward": 0, "redone": 0 },
    "per_cause": { "6b103c2866...": 1 },
    "unjoined": 0
  },
  "priced": {
    "currency": "USD",
    "items_priced": 1,
    "per_class": {
      "revert": { "items": 1, "rate": "15.00", "amount": "15.00" },
      "fix-forward": { "items": 0, "rate": "7.50", "amount": "0.00" },
      "redone": { "items": 0, "rate": "4.00", "amount": "0.00" }
    },
    "per_cause": [
      { "cause": "6b103c2866...", "items": 1, "amount": "15.00" }
    ],
    "total": "15.00"
  }
}
```
