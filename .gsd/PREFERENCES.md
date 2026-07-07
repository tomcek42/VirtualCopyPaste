---
version: 1
skill_staleness_days: 0
uat_dispatch: false
unique_milestone_ids: false
token_profile: balanced
budget_ceiling: 50.00

models:
  research:
    model: claude-sonnet-4-6
    fallbacks:
      - github-copilot/gpt-5
  planning:
    model: claude-opus-4-8
    fallbacks:
      - github-copilot/claude-opus-4.8
  execution:
    model: claude-sonnet-4-6
    fallbacks:
      - github-copilot/gpt-5
  execution_simple:
    model: github-copilot/gpt-5-mini
    fallbacks:
      - claude-haiku-4-5
  completion:
    model: github-copilot/gpt-5-mini
    fallbacks:
      - claude-haiku-4-5
  subagent:
    model: github-copilot/gpt-5-mini
    fallbacks:
      - claude-haiku-4-5
dynamic_routing:
  enabled: true
  capability_routing: true
  escalate_on_failure: true
  budget_pressure: false
  cross_provider: true
  allow_flat_rate_providers: true
  hooks: true
  tier_models:
    light: github-copilot/gpt-5-mini
    standard: claude-sonnet-4-6
    heavy: claude-opus-4-8

context_management:
  observation_masking: true
  observation_mask_turns: 6
  tool_result_max_chars: 800
  compaction_threshold_percent: 0.70

context_selection: smart

uok:
  enabled: true
  legacy_fallback:
    enabled: false
  gates:
    enabled: true
  model_policy:
    enabled: true
  execution_graph:
    enabled: true
  audit_unified:
    enabled: true
  plan_v2:
    enabled: true
  gitops:
    enabled: true
    turn_action: commit
    turn_push: false
---