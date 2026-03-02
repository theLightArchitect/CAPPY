# Pattern Database Schema

## Overview

CAPPY's pattern database maps known issues to structured diagnostic information. Each pattern represents a known problem with its symptoms, root cause, confidence level, and resolution steps.

## Pattern Structure

```json
{
  "pattern_id": "P001",
  "title": "Short description of the known issue",
  "product": "XSOAR | XSIAM | XDR | SHARED",
  "component": "Component or subsystem affected",
  "confidence": "Definitive | Strong | Moderate",
  "version_range": {
    "min": "6.0.0",
    "max": "8.99.99",
    "notes": "Optional version-specific notes"
  },
  "symptoms": [
    "Observable symptom description"
  ],
  "match_patterns": [
    "regex pattern to match in logs/evidence"
  ],
  "exclude_if": [
    "regex pattern that rules out this pattern"
  ],
  "causality_chain": [
    "Step 1: Initial trigger",
    "Step 2: Propagation",
    "Step 3: Observable failure"
  ],
  "root_cause": "Technical explanation of the root cause",
  "resolution": {
    "steps": [
      "Step-by-step resolution instructions"
    ],
    "workaround": "Temporary workaround if available",
    "fix_version": "Version where this was fixed (if applicable)"
  },
  "tags": ["tag1", "tag2"],
  "created": "2026-01-15",
  "updated": "2026-02-01"
}
```

## Confidence Levels

| Level | Definition | Evidence Required |
|-------|-----------|-------------------|
| **Definitive** | Exact match to known issue | Specific error string or log signature |
| **Strong** | High probability match | Multiple correlated symptoms |
| **Moderate** | Possible match, needs verification | Single symptom or partial match |

## Database Statistics (v2.0.0)

| Metric | Value |
|--------|-------|
| Total patterns | 540 |
| Definitive | 178 (33%) |
| Strong | 307 (57%) |
| Moderate | 55 (10%) |

## Sample File

See `cappy-cache_sample.json` for 10 example patterns demonstrating the schema structure.
