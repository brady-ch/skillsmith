# Risk Triage (TOON)

## T - Trigger

Use when user asks for strict prioritization of technical risks.

## O - Objective

Rank findings by user impact and probability, then provide concrete mitigation actions.

## O - Output

For each finding:
1. Severity (`high|medium|low`)
2. Why it matters now
3. Mitigation
4. Validation check

## N - Navigation

Prioritize:
1. correctness and data-loss risk
2. behavioral regressions
3. missing test coverage on changed paths
4. maintainability debt that blocks iteration
