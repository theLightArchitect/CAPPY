---
description: Analyze log bundles, HAR files, and other evidence
argument-hint: <evidence-path> [--depth shallow|deep]
allowed-tools:
  - mcp__cappy__call-tool
---

# /evidence Command

Analyze customer-provided evidence files (log bundles, HAR files, configs) for troubleshooting.

## Usage

```
/evidence <evidence-path> [--depth shallow|deep]
```

## Examples

- `/evidence ~/Downloads/xsoar-support-bundle.tar.gz`
- `/evidence ./case/api-trace.har --depth deep`
- `/evidence ./configs/server.conf`

## Supported Evidence Types

| Type | Extension | Analysis |
|------|-----------|----------|
| Log Bundle | `.tar.gz`, `.zip` | Extract, parse logs, find errors |
| HAR File | `.har` | HTTP errors, timing, API traces |
| Config | `.conf`, `.json`, `.yaml` | Configuration validation |
| Screenshot | `.png`, `.jpg` | OCR text extraction |
| Video | `.mp4`, `.webm` | Frame extraction, timeline |

## Analysis Depth

- **shallow**: Quick scan for obvious errors (default)
- **deep**: Full analysis with timeline correlation, pattern matching

## Output

Returns:
- Extracted errors and warnings with timestamps
- Environment info (product, version, build)
- Timeline of events
- Anomalies detected
- Citation-ready file:line references
