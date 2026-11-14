# W8 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Bauplätze mit Grund und alle Rohstoffe im Client
- **Projekt:** WRT

## Ziel

Neue Platz-Arten, Platz-Felder und Rohstoffe fallen beim Typecheck oder in Vitest auf statt am TV. Am Ende sichtbar: Ein Test meldet jede unbekannte `kind` aus `data/hub.json`, ein einziger Rohstoff-Typ im Client, `DevResource` ist weg.

## Ergebnis

- 2026-10-14, W8.3 (leichter Review, Diff `origin/develop...sprint/w8`): keine schweren Befunde. - AC-01 Nachweis W8.1 (`src/model/data.test.ts`, Gegenprobe `market` rot); AC-02 Nachweis W8.2 (`DevResource` entfernt, `ResourceKind` mit `iron`/`crystal`); AC-03 `task check` grün. - Hinweis: `src/model/types.ts` hat 412 Zeilen (vorher 411), Oxlint meldet nichts; bei der nächsten Änderung dort teilen. - Neues Ticket: B-422 (B-207 in der Spec von W11 nachtragen). - Version: v0.16.2 vorgeschlagen (Patch: nur Client-Typen, kein Verhalten); nicht gesetzt (wartet auf Bestätigung 🧑).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Test: Jede `kind` aus `data/hub.json` (`sites`, `islandSites`) ist im Client bekannt; der … |
| AC-02 | geprüft | `ResourceKind` enthält `iron` und `crystal`, `DevResource` ist entfernt, `Stock` und `mate … |
| AC-03 | geprüft | `task check` grün (B-209/AC-02, B-188/AC-01). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-209 | erledigt | AC-01 geprüft, AC-03 geprüft | `src/model/data.ts` kennt alle Platz-Arten aus `hub.json` |
| B-188 | erledigt | AC-02 geprüft, AC-03 geprüft | Der Client kennt alle fünf Rohstoffe des Servers |
