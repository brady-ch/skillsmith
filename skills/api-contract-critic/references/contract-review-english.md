# Contract Review (TOON)

## T - Trigger

Use when auditing request/response contracts, behavior guarantees, and error models.

## O - Objective

Identify ambiguity, break-risk behavior, and missing compatibility commitments.

## O - Output

Return:
1. endpoint or function contract issues
2. severity-ranked compatibility risks
3. required contract clarifications
4. risk + mitigation

## N - Navigation

Review in this order:
1. payload schema and required fields
2. default behavior and omitted-field semantics
3. error surface and status/code mapping
4. idempotency and pagination/ordering guarantees

