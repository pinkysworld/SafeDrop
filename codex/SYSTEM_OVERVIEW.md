
# System Overview

## Core entities

- **Chunk** - immutable binary segment addressed by content hash
- **Manifest** - ordered list of chunk references, metadata, and file root hash
- **Share** - capability-scoped object that binds a manifest to policy, recipients, and expiry
- **Receipt** - signed event proving chunk or manifest delivery progress
- **Audit event** - append-only event inserted into the Merkle audit log
- **Reachability descriptor** - signed statement that tells the recipient which path to use
- **Export bundle** - human-readable and machine-readable evidence package

## Minimal successful session

1. file ingested into chunks
2. manifest committed
3. share capability issued
4. reachability path selected
5. recipient downloads or resumes transfer
6. final receipt committed
7. export bundle available in admin console

## Non-goals for version 1

- no social feed or chat
- no permanent account system for recipients
- no distributed consensus cluster
- no mandatory relay dependence
- no hard dependency on zero-knowledge proving systems
