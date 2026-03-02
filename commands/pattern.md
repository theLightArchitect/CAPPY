---
description: Search CAPPY pattern database for known issues
argument-hint: <search-query> [--product XSOAR|XSIAM|XDR] [--confidence DEFINITIVE|STRONG|MODERATE]
allowed-tools:
  - mcp__cappy__call-tool
---

# /pattern Command

Search the CAPPY pattern database for known issues and their resolutions.

## Usage

```
/pattern <search-query> [--product XSOAR|XSIAM|XDR] [--confidence DEFINITIVE|STRONG|MODERATE]
```

## Examples

- `/pattern memory leak`
- `/pattern Docker container --product XSOAR`
- `/pattern API timeout --confidence DEFINITIVE`

## Pattern Database

The CAPPY pattern database contains **540 patterns**:

| Confidence | Count | Description |
|------------|-------|-------------|
| DEFINITIVE | 178 | 100% match, known root cause |
| STRONG | 307 | High confidence, tested resolution |
| MODERATE | 55 | Requires additional validation |

| Product | Count |
|---------|-------|
| ALL | 305 |
| XSIAM | 123 |
| XSOAR | 87 |
| Kubernetes | 17 |
| XDR | 8 |

## Output

Returns matching patterns with:
- Symptom description
- Root cause
- Resolution steps
- Related JIRA tickets
- Precedent cases
- Confidence level
