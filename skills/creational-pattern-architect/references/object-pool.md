# Object Pool

Load when expensive short-lived objects or resources should be reused instead of repeatedly created and destroyed.

## Intent

- Keep initialized objects ready in a pool and return them for reuse.

## Core Shape

- Clients borrow an object, use it, then return it instead of destroying it.
- The article emphasizes performance motivation and the extra lifecycle complexity that follows.

## Applicability

- Connection pools, thread/worker pools, scarce external handles, expensive reusable buffers.

## Nearby Alternatives

- Reject when objects are cheap to create or state-reset rules are unsafe.
- Reject when a single deferred instance is enough; that is `Lazy Initialization`.

## Main Tradeoff

- Pools can reduce allocation/acquisition cost, but object lifetime and reset semantics become explicit design work.

## Operational Risk

- Dirty or leaked pooled objects. Mitigate with strict reset contracts, return validation, and narrow pooling to genuinely expensive resources.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Object_pool_pattern
