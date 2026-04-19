# FFI Idioms

Load when working on string/error handling at FFI boundaries.

## Source Summary

FFI idioms focus on minimizing unsafe surface area and making lifetime/ownership rules explicit at language boundaries.

## Error Mapping

- Flat error enums: map variants to stable integer error codes.
- Structured Rust errors: map to integer code plus optional detail string channel.
- Custom Rust error structs: expose transparent C-compatible representations when needed.

## String Boundaries

- Accepting strings from foreign code:
  - Treat inputs as borrowed C strings.
  - Convert through `CStr` with explicit safety preconditions.
  - Keep unsafe scope minimal and local.
- Passing strings to foreign code:
  - Keep `CString` alive for the entire foreign call.
  - Do not create temporary pointers that outlive owning values.
  - Use mutable byte buffers when foreign functions may mutate output.

## FFI Idiom Checklist

- Are lifetime expectations explicit in docs and signatures?
- Is each unsafe block justified by a tight invariant statement?
- Are ownership transfer rules explicit for every allocated buffer?
