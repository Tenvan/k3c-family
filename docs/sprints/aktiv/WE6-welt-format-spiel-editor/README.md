# WE6 · CLI, DEV · Welt-Format im Spiel und im Welten-Editor: Endboss-Bau je Biom, Skins, neue Felder

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** CLI, DEV
- **Reife:** bereit
- **Tickets:** B-987, B-993, B-994
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

WE5 erweitert das Welt-Format (Endboss je Insel B-981, Event- und Platz-Overrides B-982/B-983, `skins` B-984). Das native Spiel zeichnet Bau und Endboss nur für die Kristallhöhle und kennt keine Skins; Welten-Editor und Welten-MCP kennen die neuen Felder nicht.

## Ziel

Die neuen Felder sind im Spiel sichtbar und im Editor bedienbar. Am Ende sichtbar: Eine Insel, die im Eisenstollen endet, zeigt dort Bau und Boss; eine Welt mit Skins zeigt andere Gegner; der Editor hat Endboss-Auswahl und Skin-Tabelle.

## Beteiligte und Zielgruppen

Spieler (2 Spieler), 🧑 im Welten-Editor, Claude über `world_*`. 🧑 gibt frei und nimmt am PC und im Editor ab.

## Anforderungen

Siehe `B-993 › Anforderungen`, `B-994 › Anforderungen`, `B-987 › Anforderungen`.

## Nicht-Ziele

Assets (WE7), Events und Zuordnung in den Welten (WE8), Engine-Änderungen (Fehler dort → Ticket).

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game` (Entscheidung 013). Die Go-Sessions WE6.1 und WE6.2 sind verworfen; WE6.6 ersetzt sie. Der Sprint wartet auf RU37 und RU60.

CLI-Session (WE6.6) und DEV-Sessions (WE6.3, WE6.4) laufen parallel zueinander, innerhalb der Domäne nacheinander. Das Spiel zeichnet nur. Wails-Oberfläche nach Atomic Design (`@webkit`).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-game/src/**`, `data/paper.json`, `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/internal/mcpsrv/**`, `tools/k3c-workbench/frontend/src/world/**`
- **Abhängig von:** WE5, RU37, RU60
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

`world_set sengoku {skins: {"gegner/wolf": "gegner/ninja"}}` → Editor zeigt die Zeile, das Spiel den Ninja (sobald das Asset aus WE7 da ist, sonst den Wolf).

## Ausnahme- und Fehlerfälle

Ersatz-Asset fehlt → Basis-Asset und Log. Workbench-Neubau nötig → erst am Sprint-Ende (Sperre `exe` gilt nur dann).

## Akzeptanzkriterien

- **AC-01** Bau und Endboss in jedem Biom, Ersatz-Asset der Basisart (B-993/AC-01, AC-02).
- **AC-02** Skins der Welt im Spiel, Rückfall auf das Basis-Asset, Namen aus dem Wertesatz (B-994/AC-01 bis AC-03).
- **AC-03** Welten-MCP setzt und liest `endBoss`, `skins`, `events`, `hub › sites`; Rundlauf verliert nichts (B-987/AC-01, AC-02).
- **AC-04** Welten-Editor mit Endboss-Auswahl, Events, Plätzen und Skin-Tabelle; `task check:dev` grün (B-987/AC-03, AC-04).

## Offene Fragen

Keine blockierende. Defaults in den Tickets (Größe des Ersatz-Bosses B-993, Umfang der Skins B-994, Skin-Vorschau B-987). Sichtprüfungen 🧑: B-993/AC-03, B-987/AC-04.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WE6.1 | `WE6.1-spiel-endboss-bau.md` | Umsetzung | autonom | verworfen |
| WE6.2 | `WE6.2-spiel-skins.md` | Umsetzung | autonom | verworfen |
| WE6.3 | `WE6.3-welten-mcp-neue-felder.md` | Umsetzung | autonom | fertig |
| WE6.4 | `WE6.4-welten-editor-neue-felder.md` | Umsetzung | autonom | fertig |
| WE6.5 | `WE6.5-review.md` | Review | autonom | offen |
| WE6.6 | `WE6.6-spiel-endboss-bau-skins-rust.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
