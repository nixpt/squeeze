# State

Updated: 2026-07-16T11:00:00-05:00

M0 shipped and pushed (nixpt/squeeze public, 2026-07-14). Planning refresh landed on feature branch `planning/m1-m6-roadmap-refresh` (worktree at `../squeeze--planning-m1-m6`, unmerged): ROADMAP.md now defines M0–M6 with explicit exit criteria; TASKS.md buckets sub-tasks by milestone; SQUEEZE-1 technical approach updated to **full cutover** (cmd_run + cmd_default both go through `crush_pkg::runners::handle_run()`; cmd_build/cmd_check stay on PackageBuilder for crush and error out explicitly for non-crush). Five new tickets authored: SQUEEZE-2 (comprehensive tests), SQUEEZE-3 (CI + crates.io), SQUEEZE-4 (wasm/crush-web verification), SQUEEZE-5 (distribution wrappers), SQUEEZE-6 (v1.0 CLI stability contract). Order ratified: M1 before M2.
