
# Test and Evaluation Plan

## Devices

- x86_64 laptop
- ARM64 laptop or SBC
- Android phone as recipient
- home router with UPnP enabled
- restrictive network path if available

## Functional tests

- upload a large file
- interrupt transfer mid-stream
- resume successfully
- export evidence bundle
- expire link and confirm access rejection
- fallback from failed direct path to relay

## Security tests

- tamper with manifest or chunk order
- replay receipt events
- test revoked or expired capability tokens
- verify relay never sees plaintext in logs or metrics

## Research benchmarks

- proof size and verification latency
- energy per gigabyte
- time to public reachability
- relay overhead
- operator comprehension of explanation cards
