# .jagent/planning — squeeze

Execution board. Pattern: squadron `.jagent/planning/` (same shape as
`crush-ast`'s).

## Directory map

```
planning/
├── README.md           # this file
├── STATE.md            # current project state + metrics (per-session updates)
├── ROADMAP.md          # milestones (M0-M...), phases, non-goals
├── TASKS.md            # kanban: P0-P5 priority levels
├── tickets/            # ticket files
└── templates/
    ├── ticket.md       # ticket template
    └── issue.md        # bug report template
```

## How to use

1. **Start of session:** Read `STATE.md` → `TASKS.md` → pick from P1/P3/P4
2. **Working:** Create/fill a ticket file in `tickets/`. Move checklist item to next status.
3. **End of session:** Update `STATE.md` metrics + test count. Move completed items.
4. **Roadmap changes:** Edit `ROADMAP.md` when milestones change.

## Ticket naming

```
SQUEEZE-NNN-{slug}.md
```

Where NNN = sequential number, slug = short name. Start from SQUEEZE-1. Never reuse IDs.
