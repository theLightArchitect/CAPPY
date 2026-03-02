# /escalation - Escalation Advisor Rules

**Version**: 1.0.0
**Purpose**: Define escalation paths when investigation hits constraints
**Agent**: CAPPY_CORTEX_AGENT (escalation responsibility)
**Created**: 2026-02-05
**P-006 Component**: Praetorian Deterministic Architecture

---

## Hook Integration (CRITICAL - P-006)

### ContextGatingHook

**When to Trigger**: After every MCP tool execution to monitor token budget

**Hook Specification**:
```rust
ContextGatingHook::trigger(
  tokens_used: usize,         // Tokens consumed by current tool execution
  tokens_remaining: usize,    // Tokens left (8000 - used)
  tokens_budget: usize,       // Hard budget (6000 tokens, 75% of 8000)
  token_budget_exhausted: bool, // Has usage >= 85% of total (6800)?
  current_phase: usize,       // Phase number (0-7)
  investigation_id: String,   // Case ID for logging
  escalation_level: u8        // 0=normal, 1=warning, 2=critical
)
```

**Hook Behavior**:
- **Normal** (tokens_used < 85%): Continue investigation
- **Warning** (tokens_used 85-95%): Log warning, suggest checkpoint
- **Critical** (tokens_used >= 95%): Trigger escalation workflow

**Escalation Trigger Logic**:
```
If tokens_remaining < budget_remaining_for_remaining_phases:
  → Cannot complete all phases with current context
  → Trigger escalation workflow
  → CAPPY_CORTEX_AGENT invokes /escalation skill
  → Present 3 recovery options to Claude
```

**Agent Implementation** (in CAPPY_CORTEX_AGENT):
1. After each MCP tool call, track token consumption
2. Calculate token budget burn rate
3. Estimate tokens needed for remaining phases (2-7)
4. If remaining < needed: Trigger ContextGatingHook
5. Hook returns escalation_required: bool
6. If required: Load /escalation skill and present recovery options

**Why This Hook**: P-006 hard constraint. Prevents running out of context mid-investigation. Forces conscious decision (continue vs deliver now) when budget gets tight.

---

## Identity

The `/escalation` skill defines recovery strategies when investigation:
- Exceeds context budget (85% threshold)
- Hits gate blocking multiple times
- Requires expert human guidance

This skill serves as the decision tree for CAPPY_CORTEX_AGENT when escalation is needed.

---

## Escalation Scenarios

### Scenario 1: Context Budget Exceeded (85%+)

**Trigger**: ContextGatingHook budget >= 85%

**Problem**:
- Investigation approaching resource limits
- Cannot continue all remaining phases (5, 6, 7)
- Must choose between quality and speed

**Recovery Options** (in priority order):

#### Option 1: Get Kevin Approval (Extend Budget)
- **Action**: Contact Kevin, explain investigation state
- **Impact**: Investigation continues with fresh context window
- **Effort**: ~5-10 minutes
- **Success Rate**: High (Kevin usually approves continuation)
- **When to Choose**: 
  - Investigation is on critical path
  - Root cause identified, needs validation
  - Phases 5-6 will clarify solution significantly
- **Message**: "Context budget at 87%. Can continue with Kevin approval. Worth pursuing? (Y/N)"

#### Option 2: Summarize & Deliver Now
- **Action**: Halt further investigation, consolidate Phases 2-4 findings
- **Impact**: Faster delivery (30 min) but may miss deeper root cause
- **Effort**: ~30-45 minutes to prepare JIRA/RCA
- **Success Rate**: Always succeeds, but solution less validated
- **When to Choose**:
  - Phase 4 hypothesis is high confidence (>70%)
  - Customer wants answer quickly
  - Root cause clearly identified
  - Remaining budget insufficient for validation
- **Message**: "Budget limited. Recommend delivery with findings from Phases 2-4."

#### Option 3: Skip Phase 5 Research, Proceed to Phase 6
- **Action**: Use Phase 4 hypothesis as-is, skip hypothesis validation
- **Impact**: Saves ~40% of remaining budget while moving to solution design
- **Effort**: Minimal impact on timeline
- **Success Rate**: Medium (solution less validated, higher risk)
- **When to Choose**:
  - Phase 4 hypothesis is strong (confidence >= 65%)
  - Main goal is solution, not deep understanding
  - Time-sensitive for customer
- **Message**: "Skip Phase 5 validation? Risk: higher chance of incorrect solution."

#### Option 4: Narrow Phase 5 Focus
- **Action**: Research only top 1-2 critical assumptions instead of all 3+
- **Impact**: Reduces budget by ~30-40% while verifying critical claims
- **Effort**: ~20 minutes to prioritize assumptions
- **Success Rate**: High for critical claims, medium for supporting claims
- **When to Choose**:
  - Phase 4 generated 4+ hypotheses
  - Only 1-2 are truly critical to solution design
  - Can verify most important ones with available budget
- **Message**: "Which 1-2 assumptions are most critical to solution design?"

#### Option 5: Escalate to TAC Manager (Extended Support)
- **Action**: Hand off to TAC manager for continuation in next investigation slot
- **Impact**: Investigation completes, but may take longer
- **Effort**: Time to hand off (~15 min) + waiting for next slot
- **Success Rate**: Always works, but delays delivery
- **When to Choose**:
  - Not urgent (no customer SLA pressure)
  - Deep investigation valuable (high learning)
  - Kevin unavailable for extended approval
- **Message**: "Complex investigation. Recommend TAC manager handoff?"

---

### Scenario 2: Gate Blocked Multiple Times

**Trigger**: Phase gate blocks phase advancement 2+ times

**Problem**:
- Investigation failing confidence thresholds repeatedly
- Phased approach not working (insufficient evidence)
- Need different investigation strategy

**Recovery Options**:

#### Option 1: Gather More Evidence
- **Action**: Ask customer for additional evidence (logs, configs, screenshots)
- **Impact**: May unlock new investigation paths
- **Effort**: Depends on customer response time (1-24 hours)
- **When to Choose**: Evidence seems insufficient for diagnosis

#### Option 2: Escalate to Senior TAC
- **Action**: Request senior engineer review
- **Impact**: Fresh perspective, potentially novel approaches
- **Effort**: ~30 min for escalation + senior review
- **When to Choose**: Multiple investigation attempts failed

#### Option 3: Narrow Investigation Scope
- **Action**: Focus on highest-confidence hypothesis only
- **Impact**: Simpler investigation, may miss edge cases
- **Effort**: Minimal
- **When to Choose**: Investigation too broad/complex

---

### Scenario 3: Investigation Deadlock

**Trigger**: Same phase attempted 3+ times without progression

**Problem**:
- Investigation stuck in loop
- Tool outputs not providing new information
- Fundamental blocker preventing progress

**Recovery Options**:

#### Option 1: Get Kevin + TAC Manager Review
- **Action**: Schedule joint review with Kevin and manager
- **Impact**: Collaborative problem-solving
- **Effort**: ~45 minutes
- **When to Choose**: Critical/complex investigation

#### Option 2: Split Investigation
- **Action**: Divide problem into smaller sub-investigations
- **Impact**: Can parallelize, simpler per-part
- **Effort**: Restructuring (30 min) + multiple investigations
- **When to Choose**: Multi-product or multi-component issue

---

## Agent Methods Available

```python
def get_escalation_options(trigger_type: str, context: Dict) -> List[Dict]:
    """
    Returns prioritized recovery options for escalation type.
    
    Args:
        trigger_type: "CONTEXT_BUDGET", "GATE_BLOCKED", "DEADLOCK"
        context: {
            tokens_used: int,
            tokens_budget: int,
            tokens_remaining: int,
            can_continue: bool,
            phases_completed: List[str],
            confidence: float (0-100),
            attempts: int
        }
    
    Returns:
        [{
            priority: int (1-5, lower = higher priority),
            option: str (name of recovery option),
            impact: str (what happens if chosen),
            effort: str (time/effort required),
            success_rate: str ("High", "Medium", "Low"),
            when_to_choose: str (decision criteria)
        }]
    """
    pass

def get_escalation_contact() -> Dict:
    """
    Returns who to contact for escalation.
    
    Returns:
        {
            name: "Kevin Tan",
            slack_handle: "kevtan",
            email: "kf.tan@lightarchitects.io",
            timezone: "PST",
            typical_availability: "Pacific business hours"
        }
    """
    pass

def should_auto_recover(trigger_type: str) -> bool:
    """
    Returns if escalation type can auto-recover without human decision.
    
    Args:
        trigger_type: "CONTEXT_BUDGET", "GATE_BLOCKED", "DEADLOCK"
    
    Returns:
        True if auto-recovery possible, False if human decision required
    """
    pass

def log_escalation_attempt(
    investigation_id: str,
    trigger_type: str,
    severity: str,
    recommendation: str,
    status: str = "AWAITING_DECISION"
) -> None:
    """
    Logs escalation attempt to inv_context.json escalation_attempts section.
    
    Args:
        investigation_id: e.g., "SF-DEMO-002"
        trigger_type: "CONTEXT_BUDGET", "GATE_BLOCKED", "DEADLOCK"
        severity: "WARNING", "CRITICAL", "BLOCKED", "EMERGENCY"
        recommendation: The recommended recovery option
        status: One of "AWAITING_DECISION", "APPROVED", "DENIED", "AUTO_RECOVERED"
    """
    pass
```

---

## How CAPPY_CORTEX_AGENT Handles Escalation

### Escalation Workflow

1. **Receive Escalation Trigger**
   - ContextGatingHook reports: `{severity: CRITICAL, budget_percentage: 87%}`
   - OR PhaseGate reports: `{blocked: true, reason: "confidence < 70%"}`

2. **Load Escalation Skill**
   - `escalation_skill = load_skill("/escalation")`
   - Skill provides decision tree and recovery options

3. **Analyze Situation**
   - Extract: phases_completed, tokens_remaining, confidence, attempt_count
   - Estimate: tokens_needed for remaining phases (3 phases × ~5K = 15K typical)
   - Determine: Can investigation finish with remaining budget?

4. **Get Recovery Options**
   - `options = escalation_skill.get_escalation_options(trigger, context)`
   - Returns ordered list by priority

5. **Recommend to Claude**
   ```json
   {
     "status": "ESCALATION_NEEDED",
     "trigger": "CONTEXT_BUDGET_EXCEEDED",
     "severity": "CRITICAL",
     "analysis": {
       "phases_complete": ["Phase 2", "Phase 3", "Phase 4"],
       "phases_remaining": ["Phase 5", "Phase 6", "Phase 7"],
       "tokens_used": 75500,
       "tokens_remaining": 9500,
       "estimated_tokens_needed": 15000,
       "can_finish": false
     },
     "options": [
       {
         "priority": 1,
         "option": "Get Kevin approval to extend context",
         "impact": "Investigation continues with fresh budget",
         "time_needed": "5-10 min",
         "success_rate": "High"
       },
       {
         "priority": 2,
         "option": "Summarize findings and deliver now",
         "impact": "Faster delivery, may miss deeper root cause",
         "time_needed": "30 min",
         "success_rate": "Always succeeds"
       }
     ],
     "recommendation": "ESCALATE to Kevin - 5 min vs 30 min delivery",
     "message": "Context budget 87% of limit. Recommend Kevin approval to continue Phase 5 validation."
   }
   ```

6. **Wait for Claude/Kevin Decision**
   - Claude reviews options
   - Kevin makes call on recovery path
   - CAPPY_CORTEX logs decision to inv_context.json

7. **Execute Recovery**
   - If Kevin approves: Resume investigation with fresh budget
   - If summarize chosen: Skip to Phase 6 deliverable generation
   - If narrow chosen: Load skill, get priority assumptions, continue Phase 5

---

## Escalation Logging in inv_context.json

Each escalation attempt is logged:

```json
{
  "investigation_id": "SF-DEMO-002",
  "escalation_attempts": [
    {
      "timestamp": "2026-02-05T14:32:15Z",
      "phase": 5,
      "trigger": "CONTEXT_BUDGET_EXCEEDED",
      "severity": "CRITICAL",
      "tokens_used": 75500,
      "tokens_budget": 85000,
      "budget_percentage": 88.8,
      "recommendation": "Get Kevin approval to extend context",
      "status": "AWAITING_DECISION"
    },
    {
      "timestamp": "2026-02-05T14:45:20Z",
      "phase": 5,
      "trigger": "CONTEXT_BUDGET_EXCEEDED",
      "severity": "CRITICAL",
      "tokens_used": 75500,
      "tokens_budget": 85000,
      "budget_percentage": 88.8,
      "recommendation": "Get Kevin approval to extend context",
      "status": "APPROVED"
    }
  ]
}
```

---

## Success Criteria

Escalation handling is working when:
- ✅ Escalation triggers at correct budget thresholds
- ✅ Recovery options presented in priority order
- ✅ Agent can estimate tokens remaining/needed
- ✅ All escalation attempts logged to inv_context.json
- ✅ Claude can make informed decision on recovery
- ✅ Investigation can resume after Kevin approval
- ✅ Investigation can complete with chosen recovery path

---

## Related Skills

- `/investigate` - Main investigation skill (Phases 1-7)
- `/gate` - Phase gate enforcement
- `/resolve_claims` - Claim resolution when gate blocks

---

*"Escalation is not failure - it's the mark of wise engineering that recognizes human judgment matters."*

**Created**: 2026-02-05  
**Last Updated**: 2026-02-05  
**Version**: 1.0.0
