# Pipes And Filters

Load when processing should be decomposed into sequential transformation stages.

## Intent

- Chain processing stages so the output of each stage is the input of the next.

## Core Shape

- Filters transform data and pipes carry the intermediate output between stages.

## Applicability

- ETL flows, request transformation pipelines, compilers, media processing.

## Nearby Alternatives

- Reject when stages need tight shared mutable state or random access to one evolving object.

## Main Tradeoff

- Pipes and Filters improve stage isolation, but intermediate contracts become part of the architecture.

## Operational Risk

- Format drift between stages and hard debugging across boundaries. Mitigate with explicit intermediate contracts.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Pipes_and_filters
