# Transpiler Design Explorations — YAML to Rust
**Source**: 2 ChatGPT Chat History Conversations
**Relevance**: DSL design, transpiler architecture, and type system thinking that informed left-right's transpilation approach

---

## Conversation 1: YAML to Rust Transpiler

### DSL and Workflow Authoring

**Prompt to YAML Workflow Generation**
- User prompts should be converted into structured YAML workflows that represent how to solve a request
- Workflows can then be inspected, diffed, cached, versioned, re-run, or transpiled
- Workflow generation is a first-class step before execution
- Justifies having explicit planning nodes and synthesis nodes

**YAML Workflow Schema Requirements**
- YAML should describe agentic behavior at all abstraction levels
- Capable of representing simple direct-response flows and long-running background orchestration, review loops, file operations, multi-step planning
- Schema is contract between generation, validation, compilation, and runtime
- Supports linting, type checks, capability checks, deterministic codegen/interpreted execution, and easier visualization/debugging

**Structural Semantics**
- Explicit DAG/state-machine semantics with composable nodes
- Workflows should encode steps, dependencies, transitions, and well-defined effects
- Supports typed IR, deterministic compilation, graph visualization, reusable subflows, and deterministic rewrites
- Avoids "the model figures it out" loops in favor of explicit structure

**Workflow Archetypes and Execution Modes**
- Multiple workflow shapes: direct answer, plan-then-answer, plan-execute-verify, iterative refinement, background loops, recurring cron jobs
- Primary run categories: ASAP response, longtail response, and cron
- Execution shapes require mode-specific budgets, checkpoints, scheduling rules, persistence, and extensible mode handling

**Tool Integration Model**
- Tools explicitly represented in schema, including custom Rust-based tools
- Tool calls should have structured inputs, outputs, retries, errors, and permission boundaries
- Encourages typed tool contracts, plugin hooks, deterministic operations, and runtime adapters

**Artifact and Effect Declaration**
- Workflows should declare which tools, files, scopes, or models they expect to touch before major execution
- File actions especially should surface as proposed diffs or explicit effect previews
- Enables safer confirmations, audit trails, verifier checks, and user trust

### Transpilation and Compilation Architecture

**Core Transform: YAML → Rust AgentSDK**
- YAML workflow specs become executable Rust-oriented logic targeting AgentSDK or compatible runtime abstraction
- Rust provides strong typing, performance, and portable local deployment
- Transpiler is central to system's identity — compilation, inspectability, and replay are core

**Intermediate Representation**
- Requires typed intermediate representation (WorkflowIR) for validation, optimization, and compilation
- Enables deterministic compilation stages, graph visualization, and code generation

**Runtime Target**
- AgentSDK represents the Rust runtime target for compiled workflows
- Can be external library, internal runtime abstraction, or framework to be defined
- Requires stable runtime traits for models, tools, queues, artifacts, verifiers, and scheduler interactions

### Runtime Orchestration

**Scheduler Core**
- Queue semantics, worker pools, persistence, cancellation, retries, and result delivery
- Required because workflows can be queued, reprioritized, parallelized, and run on cron schedules
- Supports pause/resume, checkpointing, and worker capacity management, and tight coordination with UI queue

**Review Loop as Runtime Semantics**
- Generate → verify → repair cycle is core runtime pattern
- Outputs should be generated, checked by verifiers or tools, repaired when needed
- Only then returned or committed depending on policy
- Makes ReviewLoop and Verifier first-class runtime concepts tied directly to slider policy

**Long-running and Background Workflows**
- Background, semi-endless, endless workflows continue iterating based on chat logs and prior results
- Requires checkpoints, resumability, bounded/unbounded loop semantics, and queue coexistence with direct chats
- Important for drag/drop reprioritization, device failures, and endless workflows

### Type System and Schema Concepts

**Capability Profiles**
- Structured description of hardware/model/tool limits for a device or node
- Captures memory, model availability, max context practical limits, tool availability, concurrency headroom, device role
- Enables workflow adaptation based on device constraints and resources

**Context Engineering**
- Budgeting/caching/summarization/retrieval to keep workflows effective under memory/context constraints
- Context size, KV cache cost, memory pressure, and hardware topology influence model choice, chunk sizes, summarization frequency, concurrency, and offloading
- System must actively manage context by appending history, summarizing, retrieving artifacts, compressing logs, and passing forward the right state

**Schema-First Structured Outputs**
- Enforce structured outputs (JSON/schema-first) for artifacts, tool calls, plan files, and workflow nodes
- Strongly favors typed tool contracts, plugin hooks, and deterministic operations

### Filesystem and Safety Model

**Scoped Workspaces**
- Every chat associated with a scope root for bounded workspace
- Scope is core part of chat identity
- Scope must be attached to ChatSession and enforced at every file/tool access point
- Prevents unbounded file operations within invisible or unconfirmed contexts

**System-of-Record: `.glyphnova/` Folder**
- Per-context directory storing prompts, chat logs, summaries, graphs, reports, workflow specs, and related execution artifacts
- Drives provenance, replay, auditability, debugging, and safe confinement of runtime byproducts

**Safety Posture**
- Confirmation by default for high-impact operations (create/edit/delete)
- Requires diff proposal stage, verifier gates, and confirm/deny branching
- FileAction as first-class primitive with risk metadata, verifier hooks, and approval branches

---

## Conversation 2: Agentic YAML to Rust Transpiler

### Architecture Layers and System Design

**11-Layer Architecture Spine**

**Layer 1: Intent & Interaction Surface (CLI → Tauri/React UI)**
- Capture user intent via chat, commands, drag/drop reprioritization
- Present chat queue, titles, statuses, and "what will happen" previews
- Expose sliders: runtime expectation, review cycles, criticality, etc.
- Invariant: A request is never executed without explicit IntentPacket + resolved Scope
- UI must always show current scope and pending effects before committing file actions
- Failure modes: ambiguous intent or missing scope → prompt for disambiguation; user reorders queue mid-run → reschedule safely; stale context → trigger summary refresh workflow node

**Layer 2: Context & Scope Manager (chat contexts + folder scopes)**
- Maintain per-chat context state and enforce folder scopes
- Require confirmation on context switching by default
- Every tool/file action is scoped to an explicit scope root
- Cross-scope actions require explicit user-confirmed transition
- Invariant: Every tool/file action is scoped to an explicit scope root
- Failure modes: scope conflict → block and propose scope change request; stale context summaries out of sync → trigger summary refresh workflow node

**Layer 3: Slider Compiler (UX controls → policy + workflow knobs)**
- Convert slider vector into policy knobs: gating level, review depth, conservatism, concurrency, etc.
- Produce RunPolicy that influences workflow generation and runtime behavior
- Invariant: Higher criticality never reduces safety gates
- Policy conflicts (fast + high criticality) → bias to safety; degrade speed, not correctness
- Overly expensive policy on weak device → adapt via deeper decomposition/chunking rather than skipping checks

**Layer 4: Workflow Authoring (prompt → YAML)**
- Turn IntentPacket + RunPolicy into a WorkflowSpec in YAML
- Choose workflow templates: direct answer, plan/execute, plan/verify/repair, long-running iterative
- Invariant: Every node declares inputs/outputs and expected artifacts
- WorkflowSpec is serializable and replayable
- Failure modes: bad/invalid YAML → schema validation; auto-repair; fallback to "safe minimal workflow" template
- Missing tool requirements → insert "tool resolution" node (load specs on-demand)

**Layer 5: Compilation (YAML → Rust IR → Rust code)**
- Parse YAML into typed IR (intermediate representation)
- Compile IR into executable Rust (or Rust-configured runtime graph) targeting AgentSDK
- Invariant: Compilation is deterministic given same inputs
- Generated code/IR cannot bypass Scope/Policy enforcement
- Failure modes: compilation errors → emit structured error artifact; fall back to interpreter mode if available; version drift → lockfile-style pinning

**Layer 6: Runtime Orchestrator (queues, scheduler, worker pools)**
- Schedule workflows, manage concurrency, cancellations, prioritization
- Maintain chat/workflow queue semantics consistent with UI drag/drop
- Invariant: No file commits occur outside Commit subsystem and its gates
- Failure modes: deadlocks/runaway loops → policy-based watchdog; step budget; checkpoint + pause; resource contention → downshift concurrency; defer to other device in swarm

**Layer 7: Model & Tool Backends (llama.cpp / LM Studio / vLLM / tools)**
- Provide unified ModelProvider interface: load/unload, run inference, manage parallel instances
- Provide ToolProvider interface: deterministic actions (diff, parse, test, format, etc.)
- Invariant: Model calls that must be structured enforce schemas (or reject/repair)
- Tool calls are pure/deterministic where possible
- Failure modes: backend unavailable → fallback provider or remote/offline mode; model OOM/context overflow → chunking/summarization strategy; reroute to smaller context or different device

**Layer 8: File & Artifact System (safe ops + `.glyphnova/`)**
- Manage `.glyphnova/` as system-of-record: prompts, logs, summaries, graphs, hashes
- Perform safe file ops: create/edit/delete using staged diffs and commit gates
- Invariant: All file mutations are staged (diff) before commit
- Every commit is attributable to a workflow run + node id + policy snapshot
- Failure modes: conflicting edits → three-way merge attempt; unsafe delete/edit → block; require elevated confirmation; provide rollback plan

**Layer 9: Summaries, Topics, Graphs (multi-zoom knowledge surface)**
- Generate nested topics and nested summaries
- Build graph views: workflow graph + topic graph
- Provide zoom levels for navigation and context compression
- Invariant: Summaries are derived artifacts with provenance pointers back to source logs
- Topic graph never becomes authoritative over raw logs; it's an index
- Failure modes: summary drift/hallucinated summary → verifier checks against logs; regenerate from sources; graph bloat → pruning rules; keep only active/important edges

**Layer 10: Distributed Execution Fabric (device swarm)**
- Allocate tasks across devices: Android, desktop, Pi/OrangePi cluster, rack
- Maintain device capability registry and availability
- Invariant: Scope rules travel with tasks; remote devices cannot exceed granted scope
- Failure modes: device drops mid-run → checkpoint + reschedule on another device; network partition → local queue continues; reconcile artifacts later

**Layer 11: Observability, Correctness, Reproducibility (verifiers + metrics)**
- Enforce review cycles and criticality: verify outputs, run tests, lint, diff sanity checks
- Record reproducibility metadata: model id, quantization, prompt hashes, tool versions
- Invariant: Higher criticality means stricter acceptance criteria
- Failure modes: verifier uncertainty → escalate: more review cycles, request user confirmation, or route to stronger model/device; non-deterministic outcomes → lock versions, reduce stochasticity, increase tool-based checks

### Core Primitives

**Canonical Primitives from Requirements**
- ChatSession: User-facing unit tying together messages, queue state, scope root, summaries, and workflow runs
- Scope: Bounded filesystem context attached to chat or requested operation
- IntentPacket: Normalized form of user request plus slider settings and current context
- RunPolicy: Fully expanded execution policy derived from sliders and capability profile
- CapabilityProfile: Structured description of available compute for a device or node
- WorkflowSpec: YAML representation of planned agentic workflow
- WorkflowIR: Typed internal representation used for validation, visualization, and execution/codegen
- ExecutablePlan: Runnable form produced from workflow spec/IR
- QueueItem: Schedulable unit corresponding to whole workflows or subgraphs/nodes
- FileAction: Proposed create/edit/delete action with scope, risk, and diff metadata
- Artifact: Persisted output such as logs, summaries, diffs, graphs, or reports
- ReviewLoop: Bounded or policy-driven generate→verify→repair cycle
- Verifier: Check that determines whether proposed output or file action is acceptable
- ModelProvider: Adapter interface for local runners supporting inference and lifecycle ops
- ToolProvider: Adapter interface for deterministic operations and limited side-effect tools

### Design Doctrine

**Default Posture**
- Scoped-by-default, staged commits, schema-first outputs, verifier-gated side effects
- Higher criticality adds more checks and raises bar for auto-commit
- Safe file ops by default, confirmation gates, scoped folders and context switching confirmation
- Logging detail and `.glyphnova/` layout with structured organization

**"Time Doesn't Matter" Philosophy**
- Workflow depth compensates for model weakness
- Runtime becomes scheduler of iterative refinement rather than single-shot prompting
- Weak devices can still reach useful/similar outcomes via deeper, more structured workflows
- Observability and provenance matter because long-running systems must remain debuggable and auditable

**Determinism Over Emergence**
- Strongly favors typed schemas, explicit rewrites, and pure-function-like compilation steps
- Compilation must be deterministic given same inputs
- Avoids "and then the model figures it out" behavior
- State-machine workflows with named steps and dependencies over opaque agent loops

### GlyphNova-Style System Concepts

**GlyphNova**: Envisioned local agentic IDE/system where chats, workflows, files, logs, and model execution all unify around a structured orchestration core

**Router + Skills + Verifiers Architecture**
- Small model routes, skill bundles execute procedures, verifier stack performs cheap validation
- Two loops: workflow loop (execute tasks reliably) and optimization loop (cache, reduce context/tools, escalate model only when needed)

**Structured Outputs**
- Enforce JSON/schema-first outputs for all artifacts, tool calls, plan files, and workflow nodes

**Pushdown Operations**
- Heavy computation, parsing, diffing, and deterministic transformations occur in code/tools, not in model context

**Small-by-Default Model Strategy**
- Smaller models handle routine steps: routing, summarization, or schema filling
- Larger models reserved for complex synthesis or low-confidence actions
- Enables tiered model selection, escalation rules, cheaper workflows, and hardware-normalized outcomes

**Context Budget Contract**
- Context engineering to prevent token/latency blowup
- Caching, targeted retrieval and summarization strategies
- Context budget bias: compressive vs balanced vs expansive context modes
