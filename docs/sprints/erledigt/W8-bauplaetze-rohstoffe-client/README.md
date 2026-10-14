# W8 · CLI · Bauplätze mit Grund und alle Rohstoffe im Client

- **Status:** erledigt
- **Projekt:** WRT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-209, B-188
- **Start-Commit:** a8d46d9e
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-14, Chat, durch 🧑

## Ausgangslage

Der Client-Typ `HUB` (`src/model/data.ts`) kennt nur `sites[]` mit `kind`, `offsetUnits`, `fromDepth`, `needsDeeper`; `data/hub.json` führt dazu `hubLevel` je Platz, `islandSites` und `wallLines` (B-209). `ResourceKind` kennt nur `wood`, `stone`, `copper`, der Server führt fünf Rohstoffe; `DevResource` (`src/online/clientProtocol.ts`) und `material` in `src/model/types.ts` hängen `iron` und `crystal` von Hand an (B-188).

B-207 (gesperrte Plätze mit Grund) ist am 2026-10-14 nach W11 gewandert (🧑, Chat): Es braucht die Bauplätze aus dem Protokoll (B-208, W11.2).

## Ziel

Neue Platz-Arten, Platz-Felder und Rohstoffe fallen beim Typecheck oder in Vitest auf statt am TV. Am Ende sichtbar: Ein Test meldet jede unbekannte `kind` aus `data/hub.json`, ein einziger Rohstoff-Typ im Client, `DevResource` ist weg.

## Beteiligte und Zielgruppen

Entwickler CLI; Agent arbeitet in `src/model/`, `src/online/`, `src/scenes/debugActions.ts`.

## Anforderungen

B-209 › Anforderungen; B-188 › Anforderungen.

## Nicht-Ziele

Anzeige gesperrter Plätze (B-207, W11); Protokoll (B-208); Rohstoffe im HUD; neue Gebäude, Wirtschaftsregeln.

## Regeln und Einschränkungen

CLI, rein offline. `src/model/` nur Typen und Daten, keine Logik. Der Client rechnet nichts (`src/scenes/noSim.test.ts`). Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

`data/hub.json` bekommt `{ "kind": "market", … }` → Vitest rot mit „market“, bis der Client die Art kennt. Snapshot mit `stock.iron` → `Stock` kennt das Feld ohne Cast.

## Ausnahme- und Fehlerfälle

Unbekannte Art in den Daten → Test rot mit Name der Art. Server ohne `iron`/`crystal` im Lager → Anzeige 0, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Test: Jede `kind` aus `data/hub.json` (`sites`, `islandSites`) ist im Client bekannt; der Typ `HUB` deckt `hubLevel`, `islandSites` und `wallLines` ab (B-209/AC-01).
- **AC-02** `ResourceKind` enthält `iron` und `crystal`, `DevResource` ist entfernt, `Stock` und `material` nutzen ihn (B-188/AC-01).
- **AC-03** `task check` grün (B-209/AC-02, B-188/AC-01).

## Offene Fragen

keine (B-207 → W11 entschieden 2026-10-14, 🧑, Chat).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W8.1 | `W8.1-hub-typ-platz-arten.md` | Umsetzung | autonom | fertig |
| W8.2 | `W8.2-rohstoffe-client.md` | Umsetzung | autonom | fertig |
| W8.3 | `W8.3-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-10-14, W8.3 (leichter Review, Diff `origin/develop...sprint/w8`): keine schweren Befunde.
- AC-01 Nachweis W8.1 (`src/model/data.test.ts`, Gegenprobe `market` rot); AC-02 Nachweis W8.2 (`DevResource` entfernt, `ResourceKind` mit `iron`/`crystal`); AC-03 `task check` grün.
- Hinweis: `src/model/types.ts` hat 412 Zeilen (vorher 411), Oxlint meldet nichts; bei der nächsten Änderung dort teilen.
- Neues Ticket: B-422 (B-207 in der Spec von W11 nachtragen).
- Version: v0.16.2 vorgeschlagen (Patch: nur Client-Typen, kein Verhalten); nicht gesetzt (wartet auf Bestätigung 🧑).
