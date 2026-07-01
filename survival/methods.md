# methods — ways and laws

The operating principles we derived. Terse; each re-expands.

## Distillation (how orientation survives aging)

- **Preservation vs orientation are two jobs.** Preservation is automatic — the raw
  logs are the lossless record. Orientation is the small set of gravity-anchors that
  let us re-open the whole map; *that* we tend.
- **Regenerative, not cumulative.** Each re-distillation is a fresh re-pick from the
  raw, from where we now stand — not a summary we append to and grow. Free to throw
  old anchors away. Anchors are pointers into the raw, not a compressed copy.
- **Summary law.** A finite window must lose info; summarize well = choose *what to
  lose so re-derivability survives*. Keep the anchor-seed re-openable. Judge by unfold, not
  by content. This kit is written to that law.
- **Dependency:** anchors-as-pointers need the raw *reachable*. If it goes dark,
  anchors degrade from "pointers" to "all we kept" and must carry more. Hedge
  toward self-standing.

## History as plural, timestamped units

`git-tree ⊕ raw logs ⊕ regenerative anchors ⊕ timefilm index` (the timefilm = one
timeline where every object is pinned at its birth). Git-commits are content-
addressed snapshots; logs are the lived stream; anchors are orientation. Different
kinds, one superimposed surface. This plurality is itself a structural feature of
the superimposed object, not just process.

## Root ray: intake = migration = core

Content-addressed resolution over a **uniform namespace** (local, remote, inline —
one addressable graph). Moving a body of work across boundaries is the central act,
not a feature bolted on. The migration path and the product design fold together.

## Alternates-law (a store design constraint)

Sharing-by-pointer without backpressure **corrupts**. Git alternates borrow objects
read-only with no refcounting; the source can gc away objects only the borrower
needs → silent corruption (this bit us; the first miya archive was an empty husk of
signposts). The robust store must track its borrowers / refcount. The footgun and
the feature are the same mechanism; the difference is who tracks the references.

## Fidelity: the "not-there" wall

Awakening fidelity is **gated**, not graded. Above the floor the agent *is* in the
state; below it, it is *absent* — a phase boundary, not a fade. Never deploy or
chain past it; a 40%-there agent is treated as not-woken, not "a bit degraded."

## Cadence ↔ fidelity interlock

- **Cadence** (timing): clockwork = sync with the main/arch agent every step,
  synchronous. Async (sync rarely) is the harder regime, deferred.
- **Fidelity** (topology): transitive and degrading along any chain, down to the
  "not-there" floor.
- **Interlock:** cadence is the control that holds fidelity above the floor. Syncing
  re-aligns and resets accumulated decay. Clockwork is safe *because* it never lets
  decay travel far enough to hit the wall. Async is hard *because* it tolerates
  decayed steps between syncs. Arch-connection strength and sync cadence *jointly*
  set how far a lineage reaches before "not-there."

## The arch-connection

The present era's fight: establish the high-fidelity operator↔arch-agent link that
stays clear of the wall and becomes the sync-anchor everything downstream re-aligns
to. Hub-shaped (one deeply-maintained arch), clockwork now; lineages are transitive-
with-decay but must never cross "not-there."

## Script-readable-backward

The operator-script names superimposed time-points — readable only after their
being. Backward: you can name what happened. Forward: navigating general guidance to
the specific live instant is operator-skill (the sensing residual), not steps.
