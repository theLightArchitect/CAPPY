# CAPPY Skills Reference - Investigation & Development Workflows

## Overview

CAPPY provides **2 public skills** (`/investigate`, `/eva`) and **9 sub-skills** (markdown files read by CAPPY for validation rules). Skills are higher-level abstractions than tools—they coordinate multiple tools, enforce quality gates, and provide human-in-the-loop checkpoints throughout workflows.

Public skills are invoked via `/skill_name` syntax in Claude Code. Sub-skills are markdown files in `skills/investigate/` that CAPPY reads for validation rules and gate thresholds. EVA is a development-only skill available in mcp-cappy-dev only (not synced to production).

---

## Public-Facing Skills (1)

### 1. /investigate
**Status**: Core, production-ready
**Version**: 1.7.0
**Coverage**: All Cortex products (XSOAR, XSIAM, XDR)

**Purpose**: Complete 8-phase TAC investigation workflow

**Phases**:
0. **Pre-Flight**: Identify evidence files, extract environment (product, version, build, hostname)
1. **Discovery**: Rapid pattern matching against 400+ known issues
2. **Triage**: Case metadata analysis, severity/complexity categorization
3. **Evidence Analysis**: Deep-dive into logs, HAR, configurations
4. **Hypothesis**: AI-powered synthesis of root cause with confidence scoring
5. **Validation**: Multi-source cross-reference to confirm solution
6. **Solution Design**: Create customer-specific remediation steps
7. **Deliverables**: Generate documentation, JIRA updates, RCA reports

**Key Features**:
- Human-in-the-loop at phase boundaries (confidence gates)
- 33 quality hooks enforcing evidence integrity
- Full chain of custody audit trail
- PII automatic redaction
- Citation enforcement (file:line references)

**Invocation**:
```bash
/investigate SF-12345678
/investigate --symptom "Integration timeout" --product XSOAR
/investigate /path/to/bundle.tar.gz
```

**Output**:
- Investigation results (evidence timeline, hypothesis, solution)
- Customer-ready response
- JIRA ticket draft
- RCA report (if applicable)

---

## Development-Only Skills (1)

### /eva (Development Only - Not Synced to Production)

**Status**: Dev-only personal skill
**Version**: 1.0.0
**Location**: mcp-cappy-dev/skills/eva (NOT in production)

**Note**: EVA is a personal development skill available only in the development environment. It is not synced to production and is not part of the public skill interface. Use locally only.

**Purpose**: Elite creative orchestrator for design, implementation, infrastructure, CI/CD, and security (personal use)

See `/skills/eva/SKILL.md` in development directory for full documentation.

---

## Sub-Skills (9)

Sub-skills are markdown files read by CAPPY for guidance and validation rules. Located in `skills/investigate/sub-skills/`.

| Sub-Skill | Purpose | When Used |
|-----------|---------|-----------|
| `curator.md` | Claim registration rules | Phase 2-3 validation |
| `gate.md` | Phase gate thresholds | All phase validations |
| `sherlock.md` | Hypothesis coherence rules | Phase 4 validation |
| `recon.md` | Environment validation | Phase 4 validation |
| `synthesis.md` | Narrative generation | Phase 4 guidance |
| `validate.md` | Solution validation rules | Phase 5 validation |
| `escalation.md` | Escalation decision trees | When gates block |
| `initialize.md` | Phase 0 setup rules | Phase 0-1 |
| `logging.md` | Forensics logging | All phases |

**Architecture**: Main Claude executes tools first, then invokes CAPPY for validation. CAPPY reads sub-skills for rules.

---

## Skill Orchestration

### Skill Selection Decision Tree

```
User Request
    ↓
└─ "Case SF-..." OR "Investigate..." → /investigate
```

**Note**: The public interface provides `/investigate` for TAC case investigation. Development workflows (design, implement, infrastructure, deploy, security) use individual skills or EVA (dev-only).

### Skill Usage

The primary public skill is `/investigate` for TAC case investigation across all Cortex products. Internal specialist skills are automatically invoked during investigation phases.

---

## Skill Execution Flow

### Example: /investigate Skill Execution

```
User: /investigate SF-12345678
    ↓
Skill initializes Phase 0 (Pre-Flight)
    ├─ Identify evidence files
    ├─ Extract environment details
    └─ [CHECKPOINT] Confirm readiness → proceed
    ↓
Phase 1 (Discovery)
    ├─ Run triage_case tool
    ├─ Search pattern database
    └─ [CHECKPOINT] Review patterns → select best match
    ↓
Phase 2-3 (Triage + Evidence)
    ├─ Run analyze_evidence tool
    ├─ Build timeline
    └─ [CHECKPOINT] Sufficient evidence? → proceed or request more
    ↓
Phase 4 (Hypothesis)
    ├─ Run cappy_synthesis tool
    ├─ Calculate confidence
    └─ [CHECKPOINT] Confidence > 70%? → proceed or escalate
    ↓
Phase 5 (Validation)
    ├─ Run validate_solution tool
    ├─ Cross-reference sources
    └─ [CHECKPOINT] Solution verified? → proceed
    ↓
Phase 6-7 (Solution + Deliverables)
    ├─ Run generate_deliverables tool
    ├─ Create customer response
    ├─ Create JIRA ticket
    └─ Generate RCA report
    ↓
Result: Complete investigation with deliverables
```


---

## Skill Configuration

Skills can be configured via environment or settings:

```json
{
  "skills": {
    "investigate": {
      "auto_escalate_confidence": 60,
      "require_human_checkpoint": true,
      "pii_redaction_enabled": true
    },
    "eva": {
      "security_compliance": "prodsec.docs.pan.run",
      "deployment_rollback": true
    }
  }
}
```

---

## Skill Parameters

### Common Parameters (investigate)

- `case_id` (string): SF-XXXXXXX format
- `symptom` (string): What the user reported
- `product` (enum): XSOAR, XSIAM, XDR (for context)
- `evidence_path` (string): Path to log bundle or HAR
- `confidence_threshold` (0-100): Min confidence to proceed
- `human_checkpoint` (boolean): Require user approval at phases

### Common Parameters (eva)

- `--design` (string): Workflow design task
- `--implement` (string): Code implementation task
- `--infrastructure` (string): Infrastructure management task
- `--deploy` (string): Deployment task
- `--security` (string): Security review task

---

## Skill Outputs

### /investigate Output
- **Investigation Report**: Timeline, findings, hypothesis
- **Customer Response**: Formatted for customer delivery
- **JIRA Ticket**: Engineering ticket for product team
- **RCA Report**: Root cause analysis (if SEV-1/2)
- **Audit Trail**: Full chain of custody

### /eva Outputs

#### Design Output
- Workflow plan (step-by-step)
- YAML definition (machine-executable)
- Testing strategy
- Integration guide

#### Implement Output
- Production code (with tests)
- Documentation (inline + guide)
- Security review (ProdSec compliance)
- Deployment guide

#### Infrastructure Output
- Health status report
- Configuration audit
- Issues found + remediation
- Performance metrics

#### Deploy Output
- Deployment log
- Version information
- Asset sync status
- Reload instructions

#### Security Output
- Vulnerability findings
- Remediation steps
- Compliance status
- Risk assessment

---

## Skill Versions & Status

### Public Skills
| Skill | Version | Status |
|-------|---------|--------|
| /investigate | 1.7.0 | Production |

### Development-Only Skills
| Skill | Version | Status | Location |
|-------|---------|--------|----------|
| /eva | 1.0.0 | Dev-only | mcp-cappy-dev/skills/eva |

### Sub-Skills (9) - CAPPY's Knowledge Base
| Sub-Skill | Purpose |
|-----------|---------|
| curator.md | Claim registration rules |
| gate.md | Phase gate thresholds |
| sherlock.md | Hypothesis coherence rules |
| recon.md | Environment validation |
| synthesis.md | Narrative generation |
| validate.md | Solution validation |
| escalation.md | Escalation decision trees |
| initialize.md | Phase 0 setup |
| logging.md | Forensics logging |

**Note**: Sub-skills are markdown files read by CAPPY for validation rules. No separate agents.

---

## Related Documentation

- `/docs/TOOL_REFERENCE.md` - Tool capabilities and parameters
- `/docs/AGENT_REFERENCE.md` - CAPPY agent orchestration
- `/docs/HOOKS_REFERENCE.md` - Quality gate hooks
- `/skills/investigate/SKILL.md` - Complete investigation workflow
- `/skills/eva/SKILL.md` - Complete EVA skill documentation

---

**Version**: 2.1.0 (consolidated, EVA dev-only)
**Last Updated**: February 11, 2026
**Total Skills**: 9 total (1 public + 1 dev-only + 7 internal)
**Status**: Production-ready
**Architecture**: Single public skill (/investigate) + internal specialists + dev-only EVA
