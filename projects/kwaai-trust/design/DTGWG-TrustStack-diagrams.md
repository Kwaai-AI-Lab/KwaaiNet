# DTGWG trust stack — design diagrams

Diagrams for [`plans/DTGWG-TrustStack-plan.md`](../plans/DTGWG-TrustStack-plan.md). They
show the **target state after Phase 3 (Q1 2027)**. Names marked *(proposed)* do not exist
yet in any codebase; everything else exists today in KwaaiNet `main` or in upstream
`trustoverip/dtgwg-trust-tasks-tf` at `trust-tasks-rs` 0.26.0.

Ownership is the point of the design, so every diagram keeps it visible:

| Colour | Owner | Changes when Trust Tasks changes? |
|---|---|---|
| Blue | KwaaiNet tree | **No** — only the `(PeerId, stream)` seam lives here |
| Green | `trust-tasks-libp2p` — contributed by Kwaai, in the upstream workspace, co-maintained | Yes, carried by upstream's lockstep releases |
| Grey | Upstream core (`trust-tasks-rs`) and third-party crates (OpenVTC, Affinidi) | Their releases |

## 1. Data flow — one Trust Task between two nodes

```mermaid
flowchart TB
    classDef kwaai fill:#dbeafe,stroke:#1d4ed8,color:#0f172a
    classDef crate fill:#dcfce7,stroke:#15803d,color:#0f172a
    classDef ext fill:#f1f5f9,stroke:#64748b,color:#0f172a
    classDef store fill:#fff7ed,stroke:#c2410c,color:#0f172a

    subgraph A["Node A — producer"]
        direction LR
        AH["Task handler<br/>kwaai-cli now, kwaai-twin later"]:::kwaai
        AW[("Credential wallet")]:::store
        AC["dtg-credentials<br/>sign eddsa-jcs-2022"]:::ext
        AV["VID helper (proposed)<br/>PeerId to did:key"]:::crate
        AS["send() (proposed)<br/>trust-tasks-libp2p client"]:::crate
        AF["Frame codec (proposed)<br/>unsigned-varint + JSON envelope"]:::crate
        AP["kwaai-p2p seam<br/>open_raw_stream"]:::kwaai
        AW --> AC
        AH -- "TrustTask" --> AS
        AC -- "signed credential" --> AS
        AV -- "issuer VID" --> AS
        AS --> AF -- "framed bytes" --> AP
    end

    subgraph NET["libp2p fabric — Noise, yamux"]
        direction LR
        D["Direct connection"]:::ext
        R["Circuit relay<br/>still end-to-end Noise"]:::ext
    end

    subgraph B["Node B — consumer"]
        direction LR
        BP["kwaai-p2p seam<br/>accept_streams<br/>+ relayed flag (proposed)"]:::kwaai
        BF["Frame codec<br/>over 1 MiB: reset"]:::crate
        BR["resolve_parties<br/>SPEC 4.8.1 cross-check"]:::crate
        BC["consume_inbound<br/>replay guard, freshness"]:::ext
        BH["Task handler"]:::kwaai
        BX["dtg-credentials<br/>verify"]:::ext
        BS[("TrustScore<br/>VC weight alpha = 0")]:::store
        BE["reject(): identity_mismatch<br/>back to A, same stream"]:::crate
        BO["response frame<br/>back to A, same stream"]:::crate
        BP -- "stream, PeerId, mode" --> BF -- "envelope" --> BR
        BR -- "parties agree" --> BC -- "fresh" --> BH
        BR -. "mismatch" .-> BE
        BH --> BO
        BH -- "attached credentials" --> BX -- "verified only" --> BS
    end

    A -- "/trust-tasks/0.1 stream" --> NET
    NET -- "authenticated PeerId + stream" --> B
```

What crosses the KwaaiNet seam is only a stream, a `PeerId` and a direct/relayed flag.
The Trust Task document, its parties, framing, the replay guard and the VID mapping all
live in the contributed crate (green) or upstream (grey). That is why an upstream release
does not touch our code.

## 2. Entity relationships

```mermaid
erDiagram
    NODE ||--|| VID : "derives deterministically"
    NODE ||--o| ROUTING_DID : "optionally publishes"
    NODE ||--o{ SIGNED_PEER_RECORD : "announces"
    NODE ||--|| WALLET : "keeps"
    NODE ||--o{ PEER_REPUTATION : "observes peers"
    VID ||--o{ TRUST_TASK : "issues"
    VID ||--o{ TRUST_TASK : "receives"
    TRUST_TASK }o--|| THREAD : "belongs to"
    TRUST_TASK ||--|| ENVELOPE : "carried in"
    ENVELOPE }o--|| BINDING : "framed by"
    TRUST_TASK |o--o{ DTG_CREDENTIAL : "binds via taskContext"
    WALLET ||--o{ DTG_CREDENTIAL : "holds"
    WALLET ||--o{ LEGACY_VC : "keeps read-only"
    VID ||--o{ DTG_CREDENTIAL : "issues"
    VID ||--o{ DTG_CREDENTIAL : "is subject of"
    DTG_CREDENTIAL ||--|| DATA_INTEGRITY_PROOF : "signed by"
    DTG_CREDENTIAL }o--o| VSC_PREDICATE : "states, if VSC"
    DTG_CREDENTIAL }o--o{ TRUST_SCORE : "counts toward once verified"

    NODE {
        string peer_id PK "libp2p PeerId, Noise-authenticated"
        bytes public_key "identity multihash: Ed25519, secp256k1 or ECDSA"
    }
    VID {
        string did_key PK "did:key, compared by exact string"
    }
    ROUTING_DID {
        string did_peer_2 PK "did:peer:2, keys plus service endpoint"
        string multiaddrs "for the TSP bridge"
    }
    SIGNED_PEER_RECORD {
        bytes envelope "libp2p signed peer record"
        string addrs "at most 4 dialable"
    }
    TRUST_TASK {
        string id PK
        string type_uri "trusttasks.org spec URI"
        string issuer FK "VID"
        string recipient FK "VID"
        string thread_id FK
        json payload
        json proof "optional"
    }
    THREAD {
        string thread_id PK "correlates follow-up streams"
        string state "lifecycle per SPEC 4.12 and 11"
    }
    ENVELOPE {
        string type "binding envelope type URI"
        json document "the Trust Task"
        int max_bytes "1 MiB frame limit"
    }
    BINDING {
        string binding_uri PK "trusttasks.org binding libp2p 0.1"
        string protocol_id "/trust-tasks/0.1 (proposed)"
        string mode "direct or relayed"
    }
    DTG_CREDENTIAL {
        string id PK
        string subtype "VMC VRC VDC VIC VPC VSC VAC"
        string issuer FK "VID"
        string subject FK "VID"
        string issuer_scope "pairwise directed or public"
        datetime valid_from
        string task_digest_multibase "optional"
    }
    DATA_INTEGRITY_PROOF {
        string cryptosuite "eddsa-jcs-2022"
        string verification_method
        string proof_value
    }
    VSC_PREDICATE {
        string iri PK "endorses/1 or Kwaai-namespaced"
        bool on_accept_list
    }
    LEGACY_VC {
        string type "pre-DTG Kwaai VC types"
        string note "not reliably verifiable, never scored"
    }
    WALLET {
        string path "under KWAAINET_HOME"
    }
    TRUST_SCORE {
        float score
        string tier "Unknown Known Verified Trusted"
    }
    PEER_REPUTATION {
        string peer_id FK
        string observations "availability, throughput, latency"
    }
```

Mapping of today's credential types to DTG subtypes (from the plan):

| Today | DTG credential | Notes |
|---|---|---|
| PeerEndorsementVC | VSC `endorses/1` | |
| UptimeVC, ThroughputVC | VSC, Kwaai-namespaced predicate | `witnessed/1` needs a `taskContext`; ours are self-measured |
| FiduciaryPledgeVC | VSC | |
| SummitAttendeeVC | VMC, `issuerScope: public` | |
| VerifiedNodeVC | VMC | |
| BindingVC | VDC grant/accept | Re-issue depends on summit-server's removal |

## 3. Sequence — request, response, and a rejected sender

```mermaid
sequenceDiagram
    autonumber
    box rgb(219,234,254) KwaaiNet, node A
    participant HA as Node A handler
    end
    box rgb(220,252,231) trust-tasks-libp2p, node A
    participant LA as client (A)
    end
    box rgb(219,234,254) KwaaiNet seam
    participant PA as kwaai-p2p (A)
    participant PB as kwaai-p2p (B)
    end
    box rgb(220,252,231) trust-tasks-libp2p, node B
    participant LB as server (B)
    end
    box rgb(241,245,249) Upstream core
    participant CB as consume_inbound (B)
    end
    box rgb(219,234,254) KwaaiNet, node B
    participant HB as Node B handler
    end

    HA->>LA: send(peer B, TrustTask)
    LA->>LA: issuer = did:key from own PeerId<br/>prepare_outbound, attach proof if required
    LA->>PA: open stream on /trust-tasks/0.1
    PA->>PB: Noise-authenticated libp2p stream, direct or relayed
    PB->>LB: serve(stream, remote PeerId, mode)
    LA->>LB: frame 1: envelope with the TrustTask
    LB->>LB: frame length under 1 MiB, else reset the stream
    LB->>LB: resolve_parties: in-band issuer vs did:key of remote PeerId
    alt issuer matches, or omitted and filled from transport
        LB->>CB: document with resolved parties
        CB->>CB: replay guard and freshness check
        CB->>HB: accepted task
        HB-->>LB: response document
        LB-->>LA: response frame, same stream
        LA-->>HA: response
    else in-band issuer does not match the transport
        LB-->>LA: error identity_mismatch, built by reject()
        LA-->>HA: Err(identity_mismatch)
    end
    LA->>LB: half-close
    Note over LA,LB: half-close, reset and connection drop map to NO lifecycle state
```

## 4. Sequence — late response on a fresh stream

```mermaid
sequenceDiagram
    autonumber
    participant A as Node A, producer
    participant B as Node B, consumer

    A->>B: stream 1: TrustTask, id t1, thread th1
    B-->>A: stream 1: status executing
    A-xB: stream 1 closes
    Note over A,B: the task continues, nothing on stream 1 is waited for
    B->>A: stream 2, opened by B: response for th1
    Note left of A: correlate by id and threadId<br/>libp2p has no correlation of its own
    opt producer sends control later
        A->>B: stream 3, opened by A: trust-task-control suspend or cancel for th1
    end
```

## 5. Sequence — credential issued and verified

```mermaid
sequenceDiagram
    autonumber
    participant I as Issuer node
    participant DI as dtg-credentials (issuer)
    participant S as Subject node
    participant DV as dtg-credentials (subject)
    participant TS as TrustScore (subject)

    I->>DI: build VSC, issuer and subject are did:key VIDs
    DI->>DI: JCS-canonicalise, sign eddsa-jcs-2022
    DI-->>I: DTGCredential with DataIntegrityProof
    I->>S: deliver as a Trust Task payload over /trust-tasks/0.1
    S->>DV: verify proof, issuerScope present, predicate on accept-list
    alt valid
        DV-->>S: verified credential
        S->>S: store in wallet
        S->>TS: count it, with VC weight alpha = 0 until issuer policy exists
    else invalid
        DV-->>S: rejected, never stored or scored
    end
```
