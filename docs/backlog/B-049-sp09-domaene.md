# B-049 · SP09 bleibt in einer Domäne oder hat einen erlaubten Grenzfall

- **Domäne:** INF
- **Typ:** Frage
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`docs/arbeitsweise.md` › Domänen: „Jeder Sprint gehört zu genau einer Domäne und ändert nur deren Dateien.“ SP09 (INF)
löscht `src/world/` (SIM), `server/*.mjs`, `src/online/room.ts`, `src/online/wsServer.ts` (SRV) und passt mit B-032
GitHub Pages bzw. die Landingpage an (PLAT). Einen Grenzfall für das Löschen der Alt-Engine nennt `arbeitsweise.md`
nicht. Ähnlich, aber kleiner: SP03 (SRV) entfernt mit B-020 den Smoke-Test aus `.github/workflows/ci.yml` (INF).
Gefunden im Review SP00.5.

## Ziel

Die Domänen-Regel und der Plan für SP09 (und SP03) widersprechen sich nicht.

## Beteiligte und Zielgruppen

🧑 entscheidet; die Session, die SP09 bereit macht, setzt es um.

## Anforderungen

- Entweder nennt `arbeitsweise.md` einen Grenzfall „Alt-Engine löschen“ (und CI-Anpassung durch SRV), oder SP09 wird
  nach Domänen aufgeteilt.

## Nicht-Ziele

SP09 inhaltlich ausarbeiten (geschieht beim Bereitmachen).

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; Spec-Änderungen nur mit neuer Freigabe durch 🧑.

## Beispiele

Review von SP09 prüft „Datei gehört zur Domäne des Sprints“ → jede gelöschte Datei ist Domäne oder erlaubter Grenzfall.

## Ausnahme- und Fehlerfälle

nicht relevant – Planungsfrage.

## Akzeptanzkriterien

- **AC-01** Für jede Datei, die SP09 ändert oder löscht, nennt `arbeitsweise.md` die Domäne INF oder einen Grenzfall.

## Offene Fragen

Grenzfall in `arbeitsweise.md` oder SP09 aufteilen? (🧑)

## Notizen

–
