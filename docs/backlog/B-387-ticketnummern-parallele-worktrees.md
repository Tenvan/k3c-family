# B-387 · Ticket-Nummern bleiben zwischen parallelen Worktrees eindeutig

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-10
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`plan_create` vergibt die nächste Ticket-Nummer aus dem eigenen Checkout (größte vorhandene + 1). Mehrere Worktrees und die Wurzel zählen unabhängig und vergeben dieselbe Nummer. Beobachtet am 2026-10-10: B-383 existiert doppelt, als `B-383-sim-zustand-boss-event-wechsel` (Worktree `backlog-tickets-sprint-dbg3-3c91cd`, Branch `sprint/k4`) und als `B-383-touch-hud-verdeckt-bei-aufloesung` (Worktree `sprint-br1-fortsetzung-1b4aa6`, ungetrackt); B-382 und B-384 stehen nur im zweiten, B-385 nur im ersten. Beim Zusammenführen auf `develop` bricht `tests/planning.test.ts` oder ein Ticket muss von Hand umnummeriert werden, samt Verweisen in Sprints und Sessions.

## Ziel

Jede neue Ticket-Nummer ist im ganzen Repo eindeutig, auch wenn mehrere Worktrees gleichzeitig Tickets anlegen.

## Beteiligte und Zielgruppen

Agenten-Sessions in Worktrees, 🧑 (löst Kollisionen beim Zusammenführen).

## Anforderungen

- `plan_create` für Tickets bestimmt die nächste Nummer über alle Checkouts des Repos: Wurzel, alle Git-Worktrees (`git worktree list`) und die Archive darin; größte Nummer + 1.
- Optional auch Branches auf `origin` (`git ls-tree` auf `origin/develop`, `origin/sprint/*`), damit Nummern aus Branches ohne Worktree nicht wiederkehren.
- Nummern, die in keinem Checkout mehr vorkommen, werden nicht wiederverwendet, solange ein Worktree sie noch kennt.
- Die Antwort nennt die vergebene Nummer und, wenn ein anderer Checkout eine höhere hatte, aus welchem.

## Nicht-Ziele

Sperren oder Reservierungsdateien zwischen Läufen; Umnummerieren bestehender Tickets (dafür gilt: Nummern werden nie umnummeriert, außer bei einer Kollision von Hand); Sprint- und Session-IDs (haben Namen, kein Zähler).

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-dev/internal/planning/`). Nur lesender Zugriff auf andere Worktrees, geschrieben wird nur im Ziel-Checkout. Firewall-Regel: Proben nur auf Loopback.

## Beispiele

Wurzel hat B-381 als größtes Ticket, ein Worktree hat B-385 ungetrackt → `plan_create` aus der Wurzel legt B-388 an, nicht B-382.

## Ausnahme- und Fehlerfälle

Ein Worktree ist nicht lesbar (gelöscht, gesperrt) → er wird übersprungen und in der Antwort genannt; die Anlage scheitert nicht. `git` fehlt im Pfad → Nummer nur aus dem eigenen Checkout, mit Warnzeile.

## Akzeptanzkriterien

- **AC-01** Hat ein anderer Worktree eine höhere Ticket-Nummer, vergibt `plan_create` eine größere (Test mit zwei Worktrees in einem Temp-Repo).
- **AC-02** Nicht lesbare Worktrees werden übersprungen und in der Antwort genannt (Test).
- **AC-03** Die Antwort nennt die vergebene Nummer und den Checkout, der sie bestimmt hat (Test).

## Offene Fragen

Sollen auch Branches auf `origin` ohne Worktree zählen? Das braucht `git fetch` und macht `plan_create` langsamer. Vorschlag: ja, aus dem lokalen Stand der Remote-Refs ohne eigenen Fetch.

## Notizen

Entstanden aus der Rückfrage zu B-275 am 2026-10-10 (Wahl 🧑 im Chat). Verwandt mit B-388: Solange Worktree-Sessions in die Wurzel schreiben, entstehen Nummern dort, während der Worktree seine eigenen zählt. Eigene Ticket-Anlage in der Wurzel am selben Tag lieferte B-382; weil B-382 bis B-385 in zwei Worktrees schon vergeben waren, trägt B-388 seine Nummer von Hand.
