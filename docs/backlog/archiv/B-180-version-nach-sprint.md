# B-180 · Nach jedem fertigen Sprint wird eine neue Version vorgeschlagen und bei Bestätigung gesetzt

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** –
- **Erstellt:** 2026-10-03
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** – (rückwirkend: aus der Umsetzung vom 2026-10-03 abgeleitet, ohne Freigabe)

## Ausgangslage

`docs/arbeitsweise.md` nannte nur „Nach jeder abgeschlossenen Feature-Kette oder jedem Spieleabend ein Release-Tag“. Neue Stände kommen auf den Pi nur über einen Tag `v*` (`.github/workflows/release.yml`); der letzte war `v0.3.0`, obwohl danach viele Sprints abgeschlossen wurden.

## Ziel

Jeder abgeschlossene Sprint schlägt eine neue Version vor, und 🧑 bestätigt sie; erst dann wird sie gesetzt. Nutzen: Der Pi und alle Geräte bekommen regelmäßig einen nachvollziehbaren Stand, ohne dass ein Agent eigenmächtig Versionen setzt.

## Beteiligte und Zielgruppen

🧑 bestätigt; Agenten (auch auf dem zweiten Account) schlagen vor und setzen den Tag nach der Bestätigung.

## Anforderungen

- Die Abnahme jedes Sprints enthält eine Zeile `Version: vX.Y.Z vorgeschlagen (Grund)`; Minor bei Wirkung im Spiel, Server oder Werkzeug, Patch bei Doku, Planung oder Korrektur.
- Der Tag wird nur nach ausdrücklicher Bestätigung gesetzt (`task check:all` grün, `git tag`, `git push origin <Version>`).
- Danach steht `Version: vX.Y.Z gesetzt` oder `nicht gesetzt (Grund)` in der Abnahme.

## Nicht-Ziele

Automatisches Taggen, Änderungen am Release-Workflow, Versionsnummer in einer Datei.

## Regeln und Einschränkungen

Domäne INF (`docs/arbeitsweise.md`). Gilt zusätzlich zum Tag nach jedem Spieleabend (B-170/RL1).

## Beispiele

Sprint H1 abgeschlossen → Abnahme: „Version: v0.4.0 vorgeschlagen (neuer Startvorrat im Spiel)“ → 🧑 bestätigt → Tag `v0.4.0` gesetzt.

## Ausnahme- und Fehlerfälle

Keine Bestätigung → kein Tag, `nicht gesetzt (Grund)`. Der Release-Workflow scheitert → Meldung an 🧑, der Tag bleibt bestehen, die Korrektur kommt als Patch-Version.

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` beschreibt den Versionsvorschlag nach jedem Sprint, die Bestätigung durch 🧑 und das Setzen des Tags (Sichtprüfung).

## Offene Fragen

keine

## Notizen

Auf Anweisung von 🧑 am 2026-10-03 direkt in der Arbeitsweise eingetragen.
