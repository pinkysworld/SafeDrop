# SafeDrop Master Research Agenda (50 Tracks)

This document expands SafeDrop into a 50-track research program. The purpose is to let you publish in stages while still shipping a usable product. Each track is written so it can become either a GitHub issue, a milestone, or a standalone paper direction.

## Executive framing

- **Core promise**: a single executable that lets a user share files securely from a home PC whenever it is online.
- **Systems posture**: signed evidence first, succinct proofs later.
- **Research posture**: every ambitious claim must reduce to a falsifiable systems hypothesis.
- **Product posture**: non-expert users must be able to understand what the system can actually prove.

## Priority ladder

- **P0** - required for the first real demo or public alpha.
- **P1** - strong publishable extensions that should land after the MVP stabilizes.
- **P2** - frontier or experimental tracks that belong behind feature flags.

## Bundles and candidate paper themes

### Verifiable Home Reachability

Make a home-hosted link reachable whenever the PC is online through UPnP, ICE, relay fallback, continuity records, and availability forecasting.

**Tracks:** R06, R08, R09, R35, R36, R37, R39, R40

**Candidate paper:** Home-hosted reachability with signed continuity and availability evidence.

### Relay Trust and Fallbacks

Treat relays as optional fallback infrastructure while minimizing metadata and investigating verifiable forwarding.

**Tracks:** R07, R22, R32, R33, R34, R38

**Candidate paper:** Encrypted relays, route diversity, and honesty evidence for home servers.

### Proof-Carrying Sharing

Attach signed or succinct evidence to link lifecycle events such as delivery, expiry, deletion, and multi-recipient completion.

**Tracks:** R02, R10, R11, R13, R17, R27, R28

**Candidate paper:** Proof-carrying delivery receipts and verifiable link lifecycle.

### Private Edge Sharing

Reduce metadata leakage, add local-first privacy controls, and explore high-sensitivity storage modes without external cloud dependence.

**Tracks:** R03, R15, R16, R19, R24, R31, R43, R49

**Candidate paper:** Practical privacy layers for home-hosted file sharing.

### PQ and Supply Chain Trust

Prepare SafeDrop for long-lived evidence and trustworthy releases through crypto agility, reproducible builds, and protocol specifications.

**Tracks:** R12, R21, R29, R41, R42, R45, R50

**Candidate paper:** Post-quantum migration and release trust for single-binary secure systems.

### Runtime and UX Foundation

Make the system efficient, operable, and comprehensible so advanced research tracks sit on top of a usable product.

**Tracks:** R01, R04, R05, R14, R18, R20, R23, R25, R26, R30, R44, R46, R47, R48

**Candidate paper:** Usable proof-aware file sharing on constrained personal hardware.

## Full track catalog

### R01 - Learned Bandwidth Throttling for Mobile Users

**Category:** Core Runtime  
**Priority:** P0  
**Difficulty:** Medium  
**Bundle:** Runtime and UX Foundation

**Problem statement.** Adapt upload pacing to recipient network quality on phones, LTE links, and unstable Wi-Fi without manual tuning.

**Methods path.** Start with EWMA-based RTT/loss estimation, then layer a small contextual bandit or TinyML policy over QUIC pacing and chunk scheduling.

**Key evaluation metrics.** Completion time, goodput, tail latency, fairness, retransmission rate, and energy per gigabyte.

**First implementation wedge.** Ship heuristic pacing first and treat the model as a learnable replacement once telemetry is available.

### R02 - Zero-Knowledge Proof of File Delivery

**Category:** Verifiability & Security  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** Proof-Carrying Sharing

**Problem statement.** Prove that the recipient received the file matching a published Merkle root without exposing the file contents.

**Methods path.** Use staged evidence: signed segment receipts, a final authenticated delivery receipt, and optional Halo2 or Plonky-style succinct proof wrapping the receipt chain.

**Key evaluation metrics.** Proof size, prover time, verifier time, and additional transfer overhead per file.

**First implementation wedge.** Do not block MVP on ZK; implement signed receipts first and add succinct proof generation behind a feature flag.

### R03 - Per-File Differential Privacy for Exposed Metadata

**Category:** Privacy & Policy  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Private Edge Sharing

**Problem statement.** Protect outward-facing metadata such as public filenames, approximate sizes, and timestamps without corrupting canonical internal file records.

**Methods path.** Apply alias labels, size bucketing, timestamp coarsening, and a bounded privacy-budget ledger for metadata surfaces that leave the local host.

**Key evaluation metrics.** Privacy loss budget, inference error, usability cost, and storage overhead introduced by padding.

**First implementation wedge.** Make the feature opt-in and separate user-visible aliases from internal immutable manifests.

### R04 - Verifiable Conflict-Free Delivery

**Category:** Core Runtime  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** Runtime and UX Foundation

**Problem statement.** Allow concurrent updates or multi-uploader handoff while preserving deterministic final state and auditability.

**Methods path.** Combine a Merkle DAG, CRDT-style causal metadata, and merge receipts signed by participating devices.

**Key evaluation metrics.** Convergence rate, merge determinism, proof size, and user-visible conflict rate.

**First implementation wedge.** Begin with append-only immutable files and later extend to multi-writer manifests.

### R05 - Energy-Aware Upload Scheduling

**Category:** Intelligence & Operations  
**Priority:** P0  
**Difficulty:** Low  
**Bundle:** Runtime and UX Foundation

**Problem statement.** Delay or reshape large uploads when the host is on battery or under thermal pressure while still honoring urgent transfers.

**Methods path.** Use OS battery state, thermal hints, and a priority queue with deadline-aware admission control.

**Key evaluation metrics.** Joules per gigabyte, completion-rate under deadlines, battery drain, and user override frequency.

**First implementation wedge.** Implement rule-based scheduling first; later learn better thresholds from local history.

### R06 - UPnP / NAT-PMP Automatic Port Forwarding with Proof

**Category:** Reachability  
**Priority:** P0  
**Difficulty:** Medium  
**Bundle:** Verifiable Home Reachability

**Problem statement.** Automatically request router mappings so a home PC can expose a share link while it is online, then record evidence of successful reachability.

**Methods path.** Probe IGD or NAT-PMP, create mappings, run an external echo test, and sign a reachability descriptor bound to the current share session.

**Key evaluation metrics.** Mapping success rate, time to public reachability, failure causes, and relay fallback rate.

**First implementation wedge.** Use signed external probes in the first release; any ZK representation of reachability is strictly a later optimization.

### R07 - Privacy-Preserving Public Relay

**Category:** Reachability  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** Relay Trust and Fallbacks

**Problem statement.** Provide an optional relay for cases where home routers or ISPs block direct access, without exposing file contents to the relay operator.

**Methods path.** Terminate only encrypted transport metadata, use blinded session identifiers, and forward ciphertext over QUIC streams.

**Key evaluation metrics.** Relay overhead, latency inflation, metadata surface, and relay utilization ratio.

**First implementation wedge.** Keep relay stateless where possible and separate accounting from payload forwarding.

### R08 - WebRTC / ICE Hole-Punching with Verifiable Direct Connection

**Category:** Reachability  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** Verifiable Home Reachability

**Problem statement.** Attempt direct peer connectivity before resorting to a relay, and keep evidence of which path was selected.

**Methods path.** Use ICE candidate gathering with STUN and TURN assistance, then sign the selected candidate pair and bind it to the share session.

**Key evaluation metrics.** Direct-path success rate across NAT types, setup latency, and relay avoidance percentage.

**First implementation wedge.** Treat ICE as a secondary transport path after ordinary public-port reachability.

### R09 - Dynamic DNS with Cryptographic Binding

**Category:** Reachability  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Verifiable Home Reachability

**Problem statement.** Bind a short name to the currently reachable host address while preventing stale or unauthorized rebinding.

**Methods path.** Publish signed address updates linked to the host identity key and optionally mirrored in an append-only public binding log.

**Key evaluation metrics.** Update latency, stale-resolution rate, key continuity, and rebinding failure rate.

**First implementation wedge.** Start with a simple rendezvous name and signed JSON descriptor before integrating external DNS APIs.

### R10 - Verifiable Link Expiration

**Category:** Reachability  
**Priority:** P0  
**Difficulty:** Medium  
**Bundle:** Proof-Carrying Sharing

**Problem statement.** Ensure shared links expire on time and leave auditable evidence that the capability is no longer valid.

**Methods path.** Use capability tokens with explicit expiry epochs, revocation manifests, and signed expiry events tied into the audit tree.

**Key evaluation metrics.** Expiry precision, post-expiry access rejection rate, and revocation propagation delay.

**First implementation wedge.** Start with deterministic token expiry and then add deletion evidence for associated storage objects.

### R11 - Merkle-Based Verifiable Audit Logs

**Category:** Verifiability & Security  
**Priority:** P0  
**Difficulty:** Medium  
**Bundle:** Proof-Carrying Sharing

**Problem statement.** Record all upload, link-creation, download, and expiry events in a tamper-evident structure that yields compact inclusion proofs.

**Methods path.** Use an append-only Merkle tree or log-backed DAG with signed checkpoints and exportable witness paths.

**Key evaluation metrics.** Append throughput, proof size, checkpoint cost, and audit export latency.

**First implementation wedge.** Make every important state transition append to the log before introducing advanced proof compression.

### R12 - Post-Quantum Secure Link Protection

**Category:** Verifiability & Security  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** PQ and Supply Chain Trust

**Problem statement.** Protect long-lived links and exported evidence against future quantum attacks by using hybrid classical and post-quantum primitives.

**Methods path.** Adopt crypto agility with X25519 plus ML-KEM for key establishment and Ed25519 plus ML-DSA for signatures, while preserving backward compatibility.

**Key evaluation metrics.** Handshake size, CPU cost, verification latency, and compatibility across devices.

**First implementation wedge.** Ship with hybrid mode before considering PQ-only modes.

### R13 - Verifiable Deletion with Proof of Erasure

**Category:** Verifiability & Security  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** Proof-Carrying Sharing

**Problem statement.** Provide practical deletion evidence for consumer hardware by combining key destruction, garbage collection evidence, and audit commitments.

**Methods path.** Use envelope encryption, per-object key shredding, storage compaction reports, and attested deletion events rather than impossible claims of perfect physical wipe.

**Key evaluation metrics.** Time to logical erasure, residual accessibility probability, and user confidence signals.

**First implementation wedge.** Document clearly that the first implementation proves crypto-shredding and system state transition, not microscopic physical media erasure.

### R14 - Cross-Device Verifiable Sync

**Category:** Core Runtime  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** Runtime and UX Foundation

**Problem statement.** Let multiple devices maintain a consistent SafeDrop namespace with deterministic conflict handling and signed state continuity.

**Methods path.** Use CRDT-inspired state exchange, signed operation logs, and checkpoint hashes bound to device identities.

**Key evaluation metrics.** Replication lag, divergence incidents, merge determinism, and number of conflict-free updates.

**First implementation wedge.** Sync manifests first; defer full folder mirroring until state propagation is stable.

### R15 - Regulatory-Compliant Export

**Category:** Privacy & Policy  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Private Edge Sharing

**Problem statement.** Create export bundles that combine files, access histories, and policy declarations in a way that is readable to legal or compliance reviewers.

**Methods path.** Emit structured JSON and human-readable PDF or DOCX summaries containing hashes, timestamps, consent notes, and provenance pointers.

**Key evaluation metrics.** Export completeness, validator success rate, and manual review time.

**First implementation wedge.** Support one structured evidence bundle format before adding jurisdiction-specific variants.

### R16 - Oblivious Access Modes

**Category:** Privacy & Policy  
**Priority:** P2  
**Difficulty:** Frontier  
**Bundle:** Private Edge Sharing

**Problem statement.** Offer a high-sensitivity mode that reduces leakage from storage-access patterns, particularly for repeated reads of valuable shares.

**Methods path.** Adapt Path ORAM-style techniques and block reshuffling to flash-friendly storage layouts with bounded overhead.

**Key evaluation metrics.** Bandwidth overhead, latency inflation, stash size, and access-pattern indistinguishability.

**First implementation wedge.** Keep this strictly optional and reserve it for small, highly sensitive datasets.

### R17 - Verifiable Multi-Party File Sharing

**Category:** Verifiability & Security  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** Proof-Carrying Sharing

**Problem statement.** Support one-to-many delivery where each recipient yields a distinct receipt while the sender still obtains a compact view of campaign completion.

**Methods path.** Issue recipient-scoped capability tokens, aggregate receipts, and optionally use threshold acknowledgments for group completion.

**Key evaluation metrics.** Receipt aggregation efficiency, recipient isolation, and completion visibility.

**First implementation wedge.** Implement independent recipient receipts before any threshold cryptography.

### R18 - Learned False-Positive Filtering for Access Logs

**Category:** Intelligence & Operations  
**Priority:** P2  
**Difficulty:** Medium  
**Bundle:** Runtime and UX Foundation

**Problem statement.** Reduce operator fatigue by suppressing repetitive benign alerts while keeping potentially meaningful anomalies visible.

**Methods path.** Use on-device causal or probabilistic features over session history, link age, device fingerprints, and failure patterns.

**Key evaluation metrics.** Alert precision, recall on malicious simulations, and reviewer time saved.

**First implementation wedge.** Keep conservative defaults and make the filter explain why an alert was suppressed.

### R19 - Energy-Proportional Cryptographic Profiles

**Category:** Intelligence & Operations  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Private Edge Sharing

**Problem statement.** Adjust proof batching, chunk sizes, and hybrid-crypto policy selections to preserve battery life without dropping below a secure baseline.

**Methods path.** Use an explicit policy floor, device power telemetry, and a profile selector that chooses among approved crypto and proof pipelines.

**Key evaluation metrics.** Energy cost, proof latency, battery impact, and percentage of sessions using each profile.

**First implementation wedge.** Never weaken authenticated encryption below the minimum supported policy; prefer deferral and batching over weaker protection.

### R20 - Wasm-Based Custom Sharing Rules

**Category:** Privacy & Policy  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** Policy and Explainable Trust

**Problem statement.** Allow users to define advanced access policies without embedding unsafe scripting runtimes into the single-binary server.

**Methods path.** Run deterministic WebAssembly policy modules in a sandbox with capability-scoped host calls and execution tracing.

**Key evaluation metrics.** Policy execution latency, memory limits, determinism, and rule-debugging effort.

**First implementation wedge.** Start with a fixed rule DSL and promote it to Wasm once the host interface is stable.

### R21 - Post-Quantum Key Rotation

**Category:** Verifiability & Security  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** PQ and Supply Chain Trust

**Problem statement.** Rotate long-lived keys on a schedule that preserves continuity for existing links and evidence bundles.

**Methods path.** Maintain signed key-transition chains, hybrid rekey ceremonies, and rotation checkpoints inside the audit log.

**Key evaluation metrics.** Rekey duration, failure recovery rate, and continuity verification cost.

**First implementation wedge.** Implement explicit versioned keys before any automatic rekey daemon.

### R22 - Verifiable Relay with Honesty Proofs

**Category:** Reachability  
**Priority:** P2  
**Difficulty:** Frontier  
**Bundle:** Relay Trust and Fallbacks

**Problem statement.** Move beyond encrypted forwarding and ask the relay to produce evidence that it forwarded the right transcript without tampering or selective omission.

**Methods path.** Explore transcript commitments, chunk-level signed counters, and eventually succinct proof systems over relay forwarding traces.

**Key evaluation metrics.** Proof size, forwarding slowdown, and robustness against omission or replay.

**First implementation wedge.** Treat this as a frontier track layered on top of the simpler encrypted relay from R07.

### R23 - Cross-Platform Binary Optimization

**Category:** Intelligence & Operations  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Runtime and UX Foundation

**Problem statement.** Keep the one-binary promise while tuning performance for x86_64, ARM64, and lightweight edge devices.

**Methods path.** Use architecture-specific code paths, feature detection, and benchmark-guided build profiles.

**Key evaluation metrics.** Binary size, startup time, memory footprint, and throughput across platforms.

**First implementation wedge.** Prioritize x86_64 and ARM64 first; treat other targets as opportunistic.

### R24 - On-Device Vector Search over Shared Files

**Category:** Privacy & Policy  
**Priority:** P2  
**Difficulty:** High  
**Bundle:** Private Edge Sharing

**Problem statement.** Enable semantic retrieval over local shared content without shipping raw documents to an external indexing service.

**Methods path.** Use local embeddings, a compact approximate nearest-neighbor index, and privacy-preserving query logging or noise injection.

**Key evaluation metrics.** Search latency, embedding storage cost, relevance, and metadata leakage risk.

**First implementation wedge.** Limit the first release to user-opted local search and avoid remote model dependencies.

### R25 - Long-Term Evolutionary Sharing Policy Improvement

**Category:** Intelligence & Operations  
**Priority:** P2  
**Difficulty:** Frontier  
**Bundle:** Policy and Explainable Trust

**Problem statement.** Explore whether SafeDrop can locally evolve better default policies over time while keeping the resulting policy history auditable.

**Methods path.** Use bounded search over expiry windows, throttling, relay preferences, and notification policies with explainable fitness functions.

**Key evaluation metrics.** Policy quality improvement, user override rate, and stability of learned policies.

**First implementation wedge.** Treat this as an optional research playground, never as an autonomous unreviewed control path for security-critical settings.

### R26 - Proof-Carrying Deduplication with Content-Defined Chunking

**Category:** Core Runtime  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Runtime and UX Foundation

**Problem statement.** Deduplicate repeated content without breaking the ability to prove exactly which bytes were shared and delivered.

**Methods path.** Use content-defined chunking, chunk manifests, and witness paths that rebind deduplicated chunks to each file root.

**Key evaluation metrics.** Dedup ratio, chunking overhead, proof reconstruction cost, and storage savings.

**First implementation wedge.** Implement fixed-size chunking first and only later switch to content-defined chunking for better dedup effectiveness.

### R27 - Forward-Secure Ephemeral Capability Links

**Category:** Verifiability & Security  
**Priority:** P0  
**Difficulty:** Medium  
**Bundle:** Proof-Carrying Sharing

**Problem statement.** Reduce damage from leaked links by using one-time or rolling link secrets that cannot expose earlier or later sessions.

**Methods path.** Derive capability URLs from ratcheting secrets and bind them to explicit epochs and recipient scopes.

**Key evaluation metrics.** Link-rotation cost, compromise window, and access failure rate under rotation.

**First implementation wedge.** Ship expiring single-use links before more advanced ratcheting.

### R28 - Resumable Verifiable Transfer Receipts

**Category:** Core Runtime  
**Priority:** P0  
**Difficulty:** Medium  
**Bundle:** Proof-Carrying Sharing

**Problem statement.** Support interrupted uploads or downloads without sacrificing strong delivery evidence.

**Methods path.** Issue segment-level receipts, checkpoint byte ranges, and aggregate the final receipt over the full manifest once all segments arrive.

**Key evaluation metrics.** Resume success rate, additional metadata per segment, and user-visible recovery speed.

**First implementation wedge.** Make segmented transfer a foundational primitive before adding ZK wrapping.

### R29 - Local-First Secret Splitting and Recovery

**Category:** Verifiability & Security  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** PQ and Supply Chain Trust

**Problem statement.** Provide disaster recovery without handing master recovery secrets to a centralized operator.

**Methods path.** Use threshold secret splitting across trusted devices, printable recovery codes, or offline hardware tokens.

**Key evaluation metrics.** Recovery success, setup burden, and probability of lockout under realistic failure scenarios.

**First implementation wedge.** Start with two-of-three recovery between desktop, phone, and printed emergency code.

### R30 - Namespace Isolation and Accountable Delegation

**Category:** Privacy & Policy  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Policy and Explainable Trust

**Problem statement.** Allow the owner to delegate limited management rights or team spaces without creating a heavyweight multi-user admin system.

**Methods path.** Define namespace-scoped capabilities, delegation chains, and revocation points logged in the audit tree.

**Key evaluation metrics.** Delegation clarity, revocation speed, and accidental privilege escalation rate.

**First implementation wedge.** Keep delegation explicit and shallow before supporting nested teams.

### R31 - Private Recipient Authentication Tokens

**Category:** Privacy & Policy  
**Priority:** P2  
**Difficulty:** High  
**Bundle:** Private Edge Sharing

**Problem statement.** Authenticate recipients to a link without forcing account creation or exposing more identity metadata than necessary.

**Methods path.** Use one-time secrets, blinded tokens, or anonymous capability upgrades that bind only to the minimum required claims.

**Key evaluation metrics.** Metadata minimization, authentication failure rate, and friction compared with ordinary links.

**First implementation wedge.** Single-use shared secrets are sufficient for the first iteration; anonymity-preserving token systems come later.

### R32 - Geo- and Latency-Aware Relay Selection Under Privacy Constraints

**Category:** Reachability  
**Priority:** P2  
**Difficulty:** Medium  
**Bundle:** Relay Trust and Fallbacks

**Problem statement.** Choose a relay path that is fast enough without overexposing user geography or network identity.

**Methods path.** Score relays using coarse regions, latency buckets, and privacy-weighted routing policies.

**Key evaluation metrics.** Median latency, relay churn, and amount of geographic leakage.

**First implementation wedge.** Use coarse regional hints instead of exact location data.

### R33 - Multi-Relay Route Diversity with Availability Proofs

**Category:** Reachability  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** Relay Trust and Fallbacks

**Problem statement.** Increase link resilience by using multiple relays or relay failover paths while recording which route preserved availability.

**Methods path.** Use parallel standby circuits, route health checks, and signed continuity records for path switches.

**Key evaluation metrics.** Downtime, switchover time, and incremental relay cost.

**First implementation wedge.** Implement cold-standby failover first; only later explore simultaneous multipath forwarding.

### R34 - Transport Agility for Restricted Networks

**Category:** Reachability  
**Priority:** P2  
**Difficulty:** High  
**Bundle:** Relay Trust and Fallbacks

**Problem statement.** Maintain connectivity across enterprise, campus, or hostile middleboxes by switching among approved transport encodings.

**Methods path.** Offer QUIC first, then HTTPS or WebSocket tunneling and conservative keep-alive strategies when networks are restrictive.

**Key evaluation metrics.** Connection success under constrained networks, handshake delay, and detection of blocked paths.

**First implementation wedge.** Keep the transport matrix small and auditable rather than turning SafeDrop into a general tunneling tool.

### R35 - Reachability Forecasting for Sleeping or Intermittent Hosts

**Category:** Intelligence & Operations  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Verifiable Home Reachability

**Problem statement.** Predict when a home PC is likely to be online and expose that expectation to the sender and recipient.

**Methods path.** Use local uptime history, power state transitions, and coarse recurrence patterns to estimate future availability windows.

**Key evaluation metrics.** Forecast accuracy, missed-online events, and user trust in availability indicators.

**First implementation wedge.** Rule-based forecasting based on recent uptime windows is enough to start.

### R36 - Secure Wake Assist with User-Mediated Confirmation

**Category:** Reachability  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Verifiable Home Reachability

**Problem statement.** When the host is sleeping rather than fully offline, offer a safe way to wake it with explicit user control.

**Methods path.** Use authenticated local wake support, trusted companion devices, and user-confirmed wake requests instead of silent remote wake behavior.

**Key evaluation metrics.** Wake success rate, false wakes, and time-to-available after wake.

**First implementation wedge.** Treat wake assist as opt-in and require explicit enrollment of the waking device.

### R37 - Delay-Tolerant Drop Queues with Eventual Verifiable Delivery

**Category:** Reachability  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Verifiable Home Reachability

**Problem statement.** Allow recipients to request a file while the host is offline and receive it later once the host returns online.

**Methods path.** Use queued delivery intents, signed request records, and eventual completion receipts anchored in the audit log.

**Key evaluation metrics.** Queue retention reliability, delivery completion after reconnection, and user satisfaction.

**First implementation wedge.** Start with offline request capture and notification; actual queued payload relay can come later.

### R38 - Cross-NAT Device Assist for Pairwise Access

**Category:** Reachability  
**Priority:** P2  
**Difficulty:** High  
**Bundle:** Relay Trust and Fallbacks

**Problem statement.** Use a secondary personal device such as a phone or Raspberry Pi as a lightweight introducer when direct home-host reachability is poor.

**Methods path.** Bind auxiliary devices to the owner identity and let them act as rendezvous mailboxes or assistive introducers.

**Key evaluation metrics.** Setup complexity, improved connection rate, and auxiliary-device resource use.

**First implementation wedge.** Require explicit enrollment and keep the assist device unable to decrypt payloads.

### R39 - Proof-Carrying Dynamic DNS Failover and Key Continuity

**Category:** Reachability  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Verifiable Home Reachability

**Problem statement.** When address changes or a relay takeover occurs, prove that the new endpoint remains the same logical SafeDrop host.

**Methods path.** Use signed continuity descriptors, rotating reachability manifests, and audit-linked binding changes.

**Key evaluation metrics.** Failover time, stale link rate, and continuity verification success.

**First implementation wedge.** Treat key continuity as the primary invariant, not stable IP addresses.

### R40 - Autonomous Link Survival Analytics and Self-Healing Network Policy

**Category:** Intelligence & Operations  
**Priority:** P2  
**Difficulty:** Medium  
**Bundle:** Verifiable Home Reachability

**Problem statement.** Let SafeDrop learn which network strategies tend to work best for a given host and adapt accordingly.

**Methods path.** Observe repeated path failures, relay performance, router behavior, and availability history to reorder future reachability attempts.

**Key evaluation metrics.** Reduction in failed link setups, mean time to successful path selection, and number of user interventions avoided.

**First implementation wedge.** Keep the policy engine explainable and reversible; learning must never obscure why a path was chosen.

### R41 - Transparent Remote Attestation of Single-Binary Builds

**Category:** Verifiability & Security  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** PQ and Supply Chain Trust

**Problem statement.** Give users confidence that the downloaded executable corresponds to a known build process and expected source state.

**Methods path.** Use signed build manifests, runtime self-reporting of version measurements, and optional external attestation checks.

**Key evaluation metrics.** Attestation verification cost, release reproducibility, and user trust indicators.

**First implementation wedge.** Begin with signed manifests and hash publication before pursuing hardware-backed runtime attestation.

### R42 - Reproducible Builds and Proof-Carrying Release Artifacts

**Category:** Verifiability & Security  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** PQ and Supply Chain Trust

**Problem statement.** Make release binaries reproducible so the community can independently verify that a published release matches the tagged source.

**Methods path.** Pin toolchains, capture build manifests, compare deterministic artifacts, and publish verification instructions with every release.

**Key evaluation metrics.** Bit-for-bit reproducibility rate, build variance, and time to verify a release artifact.

**First implementation wedge.** Integrate reproducible packaging into CI before the first public release.

### R43 - Side-Channel-Aware Storage Layout for Home Devices

**Category:** Verifiability & Security  
**Priority:** P2  
**Difficulty:** High  
**Bundle:** Private Edge Sharing

**Problem statement.** Reduce leakage from filesystem metadata, cache timing, and repeated block placement on commodity systems.

**Methods path.** Use padded writes, metadata normalization, and careful background maintenance strategies.

**Key evaluation metrics.** Residual side-channel leakage, write amplification, and latency overhead.

**First implementation wedge.** Document the leakage surfaces early and prioritize the ones that matter most in real home-hosted deployments.

### R44 - Memory-Safe Zero-Copy Encrypted I/O Pipeline

**Category:** Core Runtime  
**Priority:** P0  
**Difficulty:** Medium  
**Bundle:** Runtime and UX Foundation

**Problem statement.** Keep the server efficient enough for laptops and small boards while preserving Rust’s memory-safety advantages.

**Methods path.** Use streaming chunk pipelines, bounded buffers, and zero-copy handoff between storage, hashing, and transport stages where feasible.

**Key evaluation metrics.** Peak RSS, CPU load, throughput, and copy count per gigabyte transferred.

**First implementation wedge.** Instrument the data path before micro-optimizing it.

### R45 - PQ Migration Policy Synthesis and Crypto Agility Proofs

**Category:** Verifiability & Security  
**Priority:** P2  
**Difficulty:** High  
**Bundle:** PQ and Supply Chain Trust

**Problem statement.** Model how SafeDrop should migrate from classical to hybrid and eventually post-quantum-first modes without breaking stored evidence or old links.

**Methods path.** Represent crypto policy as signed versioned state and prove each migration path preserves verification semantics.

**Key evaluation metrics.** Migration downtime, verification compatibility, and policy rollback safety.

**First implementation wedge.** Solve version negotiation and evidence-format compatibility before optimizing crypto selection.

### R46 - On-Device AI for Link Expiry, Abuse Detection, and Policy Suggestions

**Category:** Intelligence & Operations  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Policy and Explainable Trust

**Problem statement.** Use local models to suggest practical defaults such as shorter expiry for sensitive shares or relay fallback for troublesome recipients.

**Methods path.** Train small local models over past transfer behavior and expose only advisory suggestions to the operator.

**Key evaluation metrics.** User acceptance rate, reduced manual tuning, and false-alarm rate on abuse suggestions.

**First implementation wedge.** All AI outputs remain advisory until the operator approves them.

### R47 - Human-Understandable Proof Summaries and Explainable Cryptographic UX

**Category:** Privacy & Policy  
**Priority:** P1  
**Difficulty:** Medium  
**Bundle:** Policy and Explainable Trust

**Problem statement.** Translate hashes, receipts, and proofs into plain-language evidence that non-cryptographers can still use during disputes or audits.

**Methods path.** Map protocol state into concise evidence cards, timelines, and machine-checked natural-language summaries.

**Key evaluation metrics.** Comprehension in user testing, time to find evidence, and reduction in operator confusion.

**First implementation wedge.** Design the explanation layer in parallel with the audit model rather than as an afterthought.

### R48 - Carbon- and Energy-Aware Sharing Optimization

**Category:** Intelligence & Operations  
**Priority:** P2  
**Difficulty:** Medium  
**Bundle:** Runtime and UX Foundation

**Problem statement.** Schedule large transfers to align with battery health, network cost, and sustainability goals where the user cares about them.

**Methods path.** Combine local power state, optional carbon-intensity estimates, and deadline-aware transfer scheduling.

**Key evaluation metrics.** Energy saved, completion delay, and user satisfaction with green scheduling choices.

**First implementation wedge.** Treat carbon data as optional and keep scheduling overrides simple.

### R49 - Federated Reliability Telemetry Without Raw Data Leakage

**Category:** Intelligence & Operations  
**Priority:** P2  
**Difficulty:** High  
**Bundle:** Private Edge Sharing

**Problem statement.** Improve heuristics across many SafeDrop deployments without centralizing raw transfer logs or sensitive metadata.

**Methods path.** Use local summary extraction, secure aggregation, and model updates that never upload raw evidence.

**Key evaluation metrics.** Model improvement per round, communication overhead, and privacy guarantees for participants.

**First implementation wedge.** Do not collect any telemetry by default; federation is opt-in research infrastructure.

### R50 - Formal Specification and Model Checking of Share and Relay Protocols

**Category:** Verifiability & Security  
**Priority:** P1  
**Difficulty:** High  
**Bundle:** PQ and Supply Chain Trust

**Problem statement.** Model the protocol states rigorously so security and lifecycle invariants can be checked before code complexity grows.

**Methods path.** Write TLA+ or PlusCal specifications for share creation, path selection, receipt finalization, expiry, and relay failover.

**Key evaluation metrics.** Invariant coverage, counterexamples found, and protocol bugs removed before implementation.

**First implementation wedge.** Model the smallest share lifecycle first and treat the specification as a release artifact.

## Recommended first ten implementation-and-paper steps

1. Ship the signed audit model, share links, resumable transfers, and evidence export (R10, R11, R27, R28).
2. Add the zero-copy streaming path and battery-aware scheduling (R05, R44).
3. Make home-hosted reachability work with UPnP/NAT-PMP and signed external verification (R06).
4. Add relay fallback without giving the relay access to plaintext (R07).
5. Publish continuity-aware address descriptors (R09, R39).
6. Introduce hybrid PQ link protection and versioned key rotation (R12, R21).
7. Improve human-facing evidence cards and operator explanation flows (R47).
8. Add a formal state-machine specification for the minimal share lifecycle (R50).
9. Expand to cross-device sync and namespace delegation (R14, R30).
10. Only then begin frontier work on relay honesty proofs and ORAM modes (R16, R22).

## What makes the agenda differentiated

SafeDrop is not just another secure sharing system. Its differentiation comes from the combination of **home-hosted reachability**, **proof-carrying evidence**, **energy-aware behavior**, **privacy-aware exports**, and **plain-language trust explanations** inside one coherent local-first product. That combination makes the research agenda suitable for both real-world deployment and journal publication.
