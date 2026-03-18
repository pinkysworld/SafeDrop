
# Threat Model

## Assets

- file contents
- metadata exposed through links and logs
- capability URLs and recipient secrets
- long-lived identity keys
- exported evidence bundles

## Adversaries

1. passive network observer
2. active network manipulator
3. curious or semi-trusted relay operator
4. malicious recipient who redistributes a link
5. local malware or compromised endpoint
6. future adversary with quantum capability against long-lived public-key material

## Security objectives

- confidentiality of file contents in transit and at rest
- integrity of manifests, receipts, and exports
- narrow capability scope and bounded link lifetime
- auditable lifecycle events
- recoverable failure handling without hidden silent corruption

## Honest limitations

- A compromised owner device can still exfiltrate plaintext before encryption.
- Consumer hardware cannot usually provide perfect physical deletion proof.
- Some ISPs block inbound traffic, so a relay remains operationally necessary in part of the deployment population.
- Metadata minimization and usability are in tension; SafeDrop should expose the trade-off explicitly.
