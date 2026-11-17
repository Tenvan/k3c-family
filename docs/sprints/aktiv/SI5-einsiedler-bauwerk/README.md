# SI5 · REG, SIM, PAP, CLI · Besondere Figur der Welt mit einmaligem Bauwerk: Regel, Engine, Assets, Anzeige

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** REG, SIM, PAP, CLI
- **Reife:** bereit
- **Tickets:** B-972, B-1001
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), von 🧑 ausgewählt. Es gibt den Händler als Besucher im Hub (`docs/rules/buerger.md` § 1, `engine/sim/merchant.go`); eine Figur mit Eigenart der Welt gibt es nicht. Feste Hub-Plätze stehen in `data/hub.json` (`sites`, `islandSites`), geprüft von `engine/sim/hub_validate*.go`; Gebäude in `data/buildings.json` und `engine/sim/building_*.go`.

## Ziel

In Sengoku und Nil wohnt auf jeder Insel abseits in Stufe 2 eine besondere Figur; gegen Münzen zieht sie in den Hub und errichtet ein einmaliges Bauwerk mit Dauerbonus (Dojo, Schrein). Regel, Engine, Assets und Anzeige liegen auf `main`.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions, Fachprüfung durch `k3c-welten` und `k3c-grafik`. Freigabe der Spec und Abnahme am TV: 🧑.

## Anforderungen

Siehe `B-972 › Anforderungen` (Regel und Engine, mit den Entscheidungen von 🧑 vom 2026-11-15) und `B-1001 › Anforderungen` (Assets und Anzeige). Die Auswahl im Welten-Editor ist B-1003 (Sprint SI7).

## Nicht-Ziele

Segen für eine Nacht, besondere Truppenart, Ausbau des Bauwerks in Stufen, frei gewählter Bauplatz, Besucher im Hub wie der Händler (B-972). Walfänger, Tranhütte und Polar-Welt. Welten-Editor (SI7). Simulation und Balancing (gesperrt).

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/buerger.md`, `docs/rules/materialien-gebaeude.md`, `docs/rules/welten.md`, `engine/sim/**`, `data/hermits.json`, `data/embed.go`, `data/hub.json`, `data/buildings.json`, `data/worlds/sengoku.json`, `data/worlds/nil.json`, `public/grafik/gebaeude-paper/**`, `public/grafik/figuren/**`, `data/paper.json`, `native/k3c-game/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Sperre:** –
- Kette REG → SIM → PAP → CLI → Review; die Sessions laufen nacheinander auf `sprint/si5`.
- Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`). Kein Eingriff in `engine/level/**`, kein neues Feld im Spielstand (bezahlt = Bau begonnen); sonst anhalten und melden.
- Das Format der Welt-Datei bekommt das optionale Feld `hermit`. Werte vorläufig in `data/hermits.json`.
- Assets nur über `assets_*`. Die Abnahme am TV (AC-05) sammelt die Ralf-Queue; sie hält den Sprint nicht auf.

## Beispiele

Sengoku, Insel 1: In Stufe 2 steht abseits die Hütte des Schwertschmieds. Monarch 2 wirft Münzen ein, der Schmied wandert in den Hub und baut dort den Dojo; ab dann sind die Kämpfer dieser Insel stärker.

## Ausnahme- und Fehlerfälle

Siehe `B-972 › Ausnahme- und Fehlerfälle` (Insel mit einer Stufe, zweite Zahlung, zerstörtes Bauwerk, verlorene Stufe) und `B-1001 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Regeltext in `buerger.md` und `materialien-gebaeude.md` steht (B-972/AC-01).
- **AC-02** Katalog, Feld `hermit` der Welt-Datei, Hütte, Zahlung, Bauplatz und die Boni von Dojo und Schrein sind in der Engine umgesetzt und getestet (B-972/AC-02 bis AC-09).
- **AC-03** Die Assets Hütte, Dojo, Schrein, Schwertschmied und Priesterin sind angelegt und geprüft (B-1001/AC-01).
- **AC-04** Das Spiel zeichnet Hütte, Figur und Bauwerk (B-1001/AC-02, AC-04).
- **AC-05** Abnahme am TV durch 🧑 (B-1001/AC-03).
- **AC-06** Review: `check_run task:check` und `task:check:go` grün, schwere Befunde behoben, Abnahme-Ticket für Sim, Balance, Gen und Race angelegt (B-742).

## Offene Fragen

Keine blockierenden. Defaults in `B-972 › Offene Fragen` und `B-1001 › Offene Fragen` (vorläufig, 🧑 bestätigt), vor allem: Die Zahlung startet den Bau sofort, der Weg der Figur ist nur Anzeige; Insel mit einer Stufe hat keine Figur; Tranhütte erst mit der Polar-Welt.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SI5.1 | `SI5.1-regeltext-einsiedler.md` | Umsetzung | autonom | fertig |
| SI5.2 | `SI5.2-engine-einsiedler.md` | Umsetzung | autonom | verworfen |
| SI5.3 | `SI5.3-assets-einsiedler.md` | Umsetzung | autonom | fertig |
| SI5.4 | `SI5.4-anzeige-einsiedler.md` | Umsetzung | autonom | verworfen |
| SI5.5 | `SI5.5-review.md` | Review | autonom | blockiert |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

–
