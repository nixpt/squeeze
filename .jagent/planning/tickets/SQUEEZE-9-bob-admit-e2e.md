# SQUEEZE-9 — end-to-end test of `SQUEEZE_ADMIT` under the real `bob admit`

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-9 |
| **Priority** | P2 |
| **Status** | Blocked on buildsched BSC-30 (nixpt/buildsched#14) |
| **Assignee** | altair (Crush lane) |
| **Dependencies** | BSC-30 `bob admit -- <cmd>` merged and deployed |

## Problem

SQUEEZE-8 shipped the admission hook (`src/admit.rs`): with
`SQUEEZE_ADMIT="bob admit --"`, squeeze re-runs itself under bob. `tests/cli.rs`
(`admission_hook_*`) checks the squeeze side with a fake wrapper script, and
that contract is right. But `bob` has no `admit` subcommand (checked
2026-11-13: buildsched main has preflight / gc / clean / store; the deployed
`jokersquad/bin/bob` has only `clean`). The README's documented command
therefore fails, and no test would notice.

## Success criteria

- [ ] An opt-in test (skipped with a printed reason unless `SQUEEZE_BOB` names a
      `bob` binary whose `bob admit --help` succeeds) that runs, in a temp package:
  - `squeeze run` with `SQUEEZE_ADMIT="$SQUEEZE_BOB admit --min-free 0 --min-ram 0 --"`
    and gets `hello from Crush`;
  - a `sys.exit(3)` program gets exit status 3 through bob;
  - an impossible `--min-free` with `--no-wait` makes `squeeze build` exit 2 and write no `target/`.
- [ ] README "Build admission": the example's flags match the `bob admit` that
      actually shipped, and it notes the [zorro] `--slot-lock
      /workspace/scratch/tmp/heavy-slot.lock` form if BSC-30 keeps it.
- [ ] CI keeps skipping the test (no bob there). The skip is visible, not silent.

## Non-goals

- Changing the hook's contract (prefix split on whitespace, `SQUEEZE_ADMITTED=1`,
  the wrapper's status is squeeze's status).
- Implementing anything in bob: that is BSC-30.
