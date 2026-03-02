# Deliverables Design Guidelines

> Single source of truth for all CAPPY investigation deliverables.
> Referenced by CORTEX-CAPPY agent and `/investigate` skill.

## Overview

CAPPY generates four types of deliverables from investigation context:

| Deliverable | File | Format | Audience | Purpose |
|-------------|------|--------|----------|---------|
| Customer Response | `customer_response.txt` | Plain text | Customer | Professional email with root cause and steps |
| JIRA Draft | `JIRA_DRAFT.txt` | Combined | Engineering | Plaintext + wiki markup in one file |
| HTML Case Infographic | `investigation_summary.html` | HTML | Engineering | Visual investigation summary with interactive diagram |
| Living Notes | `living_note.md` | Markdown | Internal | Investigation narrative + RCA content |

**Note**: RCA content is consolidated into `living_note.md` - no separate RCA file.

## Design Principles

### 1. Evidence-Based Content Only
- Every claim must have a citation (`file:line` or `file:entry`)
- Never include AI assumptions without verification
- Mark unverified observations explicitly

### 2. Professional Tone
- Technical but accessible
- No internal jargon in customer-facing deliverables
- Confidence scores are INTERNAL ONLY - never expose to customers

### 3. Plain Text for Customer Responses
- No markdown formatting (##, **, -, etc.)
- Ready to paste directly into email
- Clear section headers using CAPS or underlines

### 4. Combined JIRA Format
- Single file with both plaintext and wiki markup
- Clear separator between sections
- User copies whichever format they need

### 5. PAN Branding (HTML only)
- Primary: `#FA582D` (PAN Orange)
- Dark backgrounds: `#0f0f1a`, `#1a1a2e`
- Success: `#00d26a`, Error: `#e94560`

---

## Template Files

- [`customer_response.txt`](./customer_response.txt) - Plain text customer email template
- [`JIRA_DRAFT.txt`](./JIRA_DRAFT.txt) - Combined plaintext + wiki markup template
- [`case_infographic.html`](./case_infographic.html) - **NEW** Interactive HTML case infographic (production-ready)
- [`INFOGRAPHIC_GUIDE.md`](./INFOGRAPHIC_GUIDE.md) - **NEW** Comprehensive implementation guide for the HTML infographic

---

## Deliverable File Structure

```
{case_dir}/
├── living_note.md                    # Running notes + RCA content (consolidated)
├── inv_context.json                  # Investigation state
└── deliverables/
    ├── customer_response.txt         # Plain text email (customer-facing)
    ├── JIRA_DRAFT.txt                # Combined plaintext + wiki markup (engineering)
    └── investigation_summary.html    # Interactive infographic (engineering - NEW)
```

---

## JIRA_DRAFT.txt Structure

```
================================================================================
                           PLAINTEXT VERSION
              (Copy this for email, Slack, or case notes)
================================================================================

[plaintext content - no special formatting]

================================================================================
                          JIRA WIKI MARKUP
                    (Copy this to paste into JIRA)
================================================================================

{panel:title=...|titleBGColor=#FA582D|bgColor=#FFFFFF}
[wiki markup content]
{panel}
```

---

## Data Flow

```
inv_context.json (v3 schema)
    │
    ├─ caseMetadata.* → Header info (case ID, customer, severity)
    ├─ environment.* → Product, version, build
    ├─ evidence.items[] → Key observations with citations
    ├─ phases.* → Investigation progress and findings
    ├─ confidenceEvolution[] → Internal tracking (not for customers)
    └─ artifacts.evidenceFiles[] → Source files analyzed
         │
         ▼
    timeline_util::build_timeline()
         │
         ├─ Timeline.events[] → Chronological events
         ├─ Timeline.causality → Mermaid diagram + nodes with full_text
         ├─ Timeline.observations[] → Key findings
         └─ Timeline.evidence_files[] → File metadata
              │
              ▼
    generate_deliverables() → TXT/HTML output
```

---

## Features by Deliverable

### HTML Case Infographic (`investigation_summary.html`)
- ✅ **Interactive Mermaid diagram** - Click to expand, zoom controls
- ✅ **Glassmorphism design** - Modern aesthetic with PAN branding
- ✅ **Evidence table** - Key findings with citations and confidence
- ✅ **Event timeline** - Chronological sequence of events
- ✅ **Root cause highlight** - Clearly identified issue with explanation
- ✅ **Resolution steps** - Numbered action items with details
- ✅ **Statistics dashboard** - Claims verified, evidence files, verification rate
- ✅ **Alternatives ruled out** - Why other hypotheses were rejected
- ✅ **Responsive design** - Works on desktop, tablet, mobile
- ✅ **Print-friendly** - Professional black/white output
- ✅ **Dark theme** - Reduces eye strain, matches engineering aesthetics

### Customer Response (`customer_response.txt`)
- ✅ Plain text (no markdown formatting)
- ✅ Ready to paste directly into email
- ✅ Professional but non-technical tone
- ✅ Clear action items for customer

### JIRA Draft (`JIRA_DRAFT.txt`)
- ✅ Combined plaintext and wiki markup
- ✅ Copy/paste plaintext to Slack or case notes
- ✅ Copy/paste wiki markup directly to JIRA
- ✅ Single file, dual format

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 3.0.0 | 2026-02-06 | **NEW**: Added HTML case infographic with interactive diagram, Mermaid integration, glassmorphism design |
| 2.0.0 | 2026-01-30 | Consolidated format: plain text customer response, combined JIRA file, RCA merged into living_note |
| 1.0.0 | 2026-01-30 | Initial consolidation from toolkit CLAUDE.md |
