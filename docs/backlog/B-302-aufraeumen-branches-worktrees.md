# B-302 · Lokale Branches und Worktrees werden an festen Meilensteinen aufgeräumt

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Am 2026-10-06 lagen 41 lokale Branches herum, dazu ein Worktree unter `.claude/worktrees/` und auf `origin` die erledigten Branches `sprint/s6` und `wip/w4.3a-krieger`. Alle waren inhaltlich schon in `develop`. `git branch --merged` erkennt das nicht, weil per Squash gemergt wird und `develop` einmal neu geschrieben wurde. Auf GitHub ist `delete_branch_on_merge` an, gemergte PR-Branches verschwinden auf `origin` also von selbst. Lokale Branches, Worktrees und nie gemergte `wip/…`-Branches bleiben liegen. `docs/arbeitsweise.md` sagt nirgends, wann aufgeräumt wird. `git branch -D` von Hand lehnt der Auto-Modus von Claude Code als unumkehrbar ab.

## Ziel

Lokal und auf `origin` liegen nur `develop`, `main`, die Branches aktiver Sprints und deren Worktrees. Das Aufräumen hängt an Schritten, die es im Ablauf schon gibt, und braucht einen Befehl statt Handarbeit.

## Beteiligte und Zielgruppen

Agenten (lokal und Cloud), die Sessions abarbeiten; 🧑 als einziger, der auf `origin` löscht und Releases macht.

## Anforderungen

- **Meilenstein Session-Start** (Autonomer Ablauf, Schritt 1): `git fetch --prune`, dann `task aufraeumen`.
- **Meilenstein Sprint erledigt** (Review Schritt 5 bzw. letzte Session): Die `wip/<präfix>…`-Branches des Sprints werden geprüft. Was inhaltlich im Sprint-Branch steckt, steht im PR-Text unter „Zum Löschen“.
- **Meilenstein Sprint-PR gemergt:** 🧑 löscht die dort genannten `origin/wip/…`-Branches. Löschen auf `origin` bleibt beim Menschen.
- **Meilenstein Release:** neue Zeile „Aufgeräumt“ in der Release-Checkliste. Sie ist ein Hinweis und blockiert den Tag nicht.
- **`task aufraeumen`** (Taskfile): löscht nur sichere Fälle und listet den Rest mit Grund auf:
  - lokale Branches, deren Upstream `[gone]` ist **und** deren PR laut `gh` gemergt ist;
  - lokale Branches ohne Upstream, deren Commits laut `git cherry` alle in `origin/develop` stecken;
  - Worktrees ohne uncommittete Änderungen, deren Branch gelöscht ist oder die detached auf einem Stand von `develop` stehen.
- Nie gelöscht werden `develop`, `main`, der aktuell ausgecheckte Branch und Branches aktiver Sprints (`sprint/*` mit Ordner in `docs/sprints/aktiv/`).
- `task aufraeumen -- --dry` zeigt nur an.

## Nicht-Ziele

Kein automatisches Löschen auf `origin`. Kein Aufräumen von Tags. Keine Änderung an k3c-dev (Worktree-Ports: B-275).

## Regeln und Einschränkungen

Domäne INF (`docs/arbeitsweise.md`, `Taskfile.yml`). Prozess steht nur in `docs/arbeitsweise.md`. Befehle nur über `task`. Muss unter Windows (Git Bash) und in der Cloud (Linux) laufen. `gh` kann fehlen, dann werden `[gone]`-Branches nur aufgelistet.

## Beispiele

- `sprint/so1` lokal, PR #159 gemergt, Upstream `[gone]` → gelöscht.
- `u3/2-seite` ohne Upstream, alle Commits per `git cherry` in `develop` → gelöscht.
- `claude/resource-manager-…`: Squash mit geänderten Tickets, ein Commit nicht wörtlich in `develop`, PR #163 gemergt, Upstream `[gone]` → gelöscht (der PR-Status entscheidet).
- `sprint/w4` mit offenem PR #164 → bleibt.

## Ausnahme- und Fehlerfälle

- Branch ohne Upstream mit Commits, die nicht in `develop` stecken → nur auflisten („nicht gemergt, X Commits“).
- Worktree mit uncommitteten Änderungen → nur auflisten.
- Kein Netz oder `gh` fehlt → nur auflisten, nichts löschen, Exit 0.

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` nennt die vier Meilensteine, jeweils an der Stelle des bestehenden Schritts (Autonomer Ablauf 1, Review 5, Branches, Release-Checkliste).
- **AC-02** `task aufraeumen` löscht in einem Test-Repo die sicheren Fälle aus „Anforderungen“ und lässt `develop`, `main`, den ausgecheckten Branch, aktive `sprint/*`, ungemergte Branches und schmutzige Worktrees stehen (Testskript oder Go-/Vitest-Test).
- **AC-03** `task aufraeumen -- --dry` ändert nichts und listet jeden Kandidaten mit Grund.
- **AC-04** Ohne `gh` oder ohne Netz endet der Task mit Exit 0 und löscht nichts außer Fällen, die allein per `git cherry` sicher sind.
- **AC-05** Die PR-Vorlage `.github/pull_request_template.md` hat einen Abschnitt „Zum Löschen“ für `origin/wip/…`.

## Offene Fragen

- Soll der Release bei Resten rot werden statt nur zu warnen? Vorschlag: nur Hinweis. Entscheidet 🧑.
- Soll der Agent `task aufraeumen` am Session-Start selbst ausführen dürfen (Permission-Regel für den Auto-Modus) oder nur `--dry` und dir das Ergebnis melden? Entscheidet 🧑.

## Notizen

Aufräumaktion 2026-10-06: 41 lokale Branches per Hand gelöscht (Prüfung über `gh pr list` + `git cherry`), außerdem `origin/sprint/s6` (PR #156) und `origin/wip/w4.3a-krieger` (W4.3a neu in `sprint/w4`, PR #164).
