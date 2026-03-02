---
description: Quick pattern matching and similar case lookup for symptoms
argument-hint: <symptom> [--product XSOAR|XSIAM|XDR]
allowed-tools:
  - mcp__cappy__call-tool
---

# /triage Command

Perform quick pattern matching against the CAPPY pattern database and search for similar cases.

## Usage

```
/triage <symptom> [--product XSOAR|XSIAM|XDR]
```

## Examples

- `/triage War Room Edit button missing`
- `/triage Docker container OOM killed --product XSOAR`
- `/triage correlation rule not matching bind variables --product XSIAM`

## What It Does

1. **Pattern Match**: Search 400+ known patterns for symptom matches
2. **Case Search**: Find similar TAC cases from history
3. **JIRA Search**: Look for related engineering tickets
4. **Confidence Score**: Returns confidence level (DEFINITIVE/STRONG/MODERATE)

## Output

Returns:
- Matched patterns with confidence scores
- Similar cases with resolutions
- Related JIRA tickets
- Required evidence if confidence < 70%

## When to Use

- Quick lookup for known issues
- Before starting full investigation
- Initial customer response preparation
