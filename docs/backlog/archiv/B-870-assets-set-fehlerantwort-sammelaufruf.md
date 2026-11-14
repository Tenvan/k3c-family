# B-870 · assets_set nennt auch beim Abbruch des Sammelaufrufs die schon bekannten Felder und Pixel

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ56
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WZ55.2: Bricht der Sammelaufruf der Paper-CLI als Ganzes ab (Zeitlimit, bun fehlt, Exit 2), fehlen Felder und Pixel in der Fehlerantwort von `assets_set`; vorher waren sie bei einem render-Fehler teilweise gefüllt.

## Ziel

Die Fehlerantwort nennt, was vor dem Abbruch bekannt war (Validierung, Diff), und den Grund des Abbruchs.

## Beteiligte und Zielgruppen

Agenten bei Asset-Änderungen; Claude (Umsetzung).

## Anforderungen

- Teilergebnisse des Sammelaufrufs in die Fehlerantwort übernehmen.
- Fixture für den Fehlerpfad.

## Nicht-Ziele

Erfolgspfad.

## Regeln und Einschränkungen

Klasse A (Problem, DEV).

- **Bereich:** `tools/k3c-workbench/internal/paperassets/**`, `tools/paper/fixtures/mcp/**`
- **Abhängig von:** WZ55
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Render-Zeitlimit → Antwort nennt geänderte Felder und „Render abgebrochen: Zeitlimit“.

## Ausnahme- und Fehlerfälle

bun fehlt → Hinweis auf `requirements.md` wie bisher.

## Akzeptanzkriterien

- **AC-01** Test: Abbruch des Sammelaufrufs liefert Felder und Abbruchgrund.

## Offene Fragen

keine

## Notizen

Befund WZ55.2 (2026-11-14).
