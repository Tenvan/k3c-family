# ECO1 · PAP, CLI, SIM · Eco-Plan: Schwierigkeits-Siegel, Werkzeuggürtel und Handwerker, tiefe Ebenen und Minen

- **Status:** aktiv
- **Projekt:** WIR
- **Domäne:** PAP, CLI, SIM
- **Reife:** bereit
- **Tickets:** B-1191, B-1192, B-1193, B-1194, B-1195, B-1196, B-1197, B-1198
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21

## Ausgangslage

🧑-Ideen 2026-11-21, in einem Feature-Sprint gebündelt, damit er mit einem parallel laufenden Sprint ähnlicher Ideen zusammengeführt werden kann (zuerst die Tickets und Offenen Fragen abgleichen, dann die Sessions). Heute: Schwierigkeit nur als Text; Handwerker ist Bauer + Hammer (B-950), die Werkstatt macht Hämmer und Schwerter, Reparatur ist kostenlos; jede Ebene darf alles bauen, was die Hub-Stufe freigibt; Erzadern können hinter Hindernissen liegen, ohne dass Freiräumen nötig ist.

## Ziel

(1) Jeder Schwierigkeitsgrad hat ein Wachssiegel, sichtbar in Auswahl, Spielständen und HUD. (2) Der Werkzeugmacher stellt einen Werkzeuggürtel (Hammer, Spitzhacke, Axt) her; ein Bürger mit Gürtel ist Handwerker und macht alle Bauarbeiten; Reparatur kostet 1 Grundmaterial der Ausbaustufe und 2 Gold. (3) Untere Ebenen haben nur Verteidigung, Produktion nur ganz oben, Ausrüstung kommt von oben oder vom Boden, Erzadern sind erst nach dem Freiräumen abbaubar.

## Beteiligte und Zielgruppen

🧑 gibt die Spec frei (Spielregeln, Level-Layout) und nimmt am PC mit Pad ab; Hauptrechner setzt in Rust um, der Second Worker die Assets (Lane zweit).

## Anforderungen

Block A Siegel: `B-1191` (Assets, Lane zweit), `B-1192` (Anzeige). Block B Handwerker: `B-1193` Gürtel und Werkzeugmacher, `B-1194` Handwerker mit Gürtel macht alle Bauarbeiten, `B-1195` Reparaturkosten. Block C Tiefe: `B-1196` Gebäude nach Tiefe, `B-1197` Ausrüstung von oben, `B-1198` Erzadern freiräumen. Sessions (bei Aktivierung als Dateien): ECO1.1 Siegel-Assets (Lane zweit), ECO1.2 Siegel-Anzeige, ECO1.3 Gürtel, ECO1.4 Handwerker, ECO1.5 Reparaturkosten, ECO1.6 Gebäude nach Tiefe, ECO1.7 Ausrüstung von oben, ECO1.8 Erzadern, ECO1.9 Review. Abhängigkeiten: B-1194 braucht B-1193, B-1195 braucht B-1194, B-1197 braucht B-1196, B-1198 braucht B-1194. Block A ist unabhängig.

## Nicht-Ziele

Neue Schwierigkeitsgrade, Grafik des Gürtels (eigenes Ticket, Lane zweit), Balancing (Abnahme-Ticket), neue Welten.

## Regeln und Einschränkungen

Neue Mechanik nur in Rust (Entscheidung 013), Go-Engine eingefroren, Go-Golden unverändert; deterministisch (`k3c-rng`), 2 Spieler; Spielstand-Version erhöhen, alte Stände egal (🧑 2026-11-21); Regeltexte (`buerger.md`, `materialien-gebaeude.md`, `stufen.md`, `welten.md`) und Glossar ziehen mit; Assets nur über `assets_*` (Entscheidung 005); Datei ≤ 400 Zeilen; Sim und Balancing gesperrt.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-sim/src/**`, `rust/crates/k3c-level/src/**`, `rust/crates/k3c-data/src/**`, `rust/crates/k3c-game/src/menu/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/actors/**`, `public/grafik/icons/**`, `data/paper.json`, `data/buildings.json`, `data/economy.json`, `data/hub.json`
- **Abhängig von:** LA3
- **Braucht 🧑:** Freigabe | Abnahme
- **Sperre:** golden | spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** (B-1191) Je Grad ein Siegel-Asset, in `data/paper.json` zugeordnet.
- **AC-02** (B-1192) Auswahl, Spielstand-Liste und HUD zeigen das Siegel; alter Spielstand ohne Grad ohne Fehler.
- **AC-03** (B-1193) Die Werkstatt stellt Gürtel her; Spielstand-Roundtrip grün.
- **AC-04** (B-1194) Ein Bürger mit Gürtel macht Bau, Reparatur, Rodung und Fels; ohne Gürtel nichts davon.
- **AC-05** (B-1195) Reparatur bucht 1 Grundmaterial der Stufe und 2 Gold.
- **AC-06** (B-1196) Unten nur erlaubte Arten, oben alle; Zuordnung in den Daten.
- **AC-07** (B-1197) Unten ohne Regal holt ein Bürger liegende Ausrüstung oder wandert nach oben.
- **AC-08** (B-1198) Jede Ader hat freiräumbare Hindernisse; Abbau erst nach dem Freiräumen.
- **AC-09** Abnahme 🧑 am PC (Siegel, Handwerker mit Werkzeug, Mine).

## Offene Fragen

🧑 muss die Spielregeln freigeben. Defaults stehen in den Tickets: Werkzeugmacher = Werkstatt, Gürtel ersetzt den Einzelhammer (30 Holz, 10 Stein, 3 Gold), Bergmann und Baumeister gehen im Handwerker auf, Reparatur je Vorgang bis volle HP, Kampfgebäude (Mauer, Turm, Tor, Kaserne, Bogner, Heilplatz) überall und Produktion nur oben, ein bis drei Hindernisse je Ader, Siegel links neben dem Weltnamen. Beim Zusammenführen mit dem parallelen Sprint: doppelte Tickets über `Duplikat von:` zusammenlegen und Regeln gegeneinander prüfen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| ECO1.1 | `ECO1.1-siegel-assets.md` | Umsetzung | autonom | fertig |
| ECO1.2 | `ECO1.2-siegel-anzeige.md` | Umsetzung | autonom | offen |
| ECO1.3 | `ECO1.3-werkzeuggurtel.md` | Umsetzung | autonom | offen |
| ECO1.4 | `ECO1.4-handwerker-bauarbeiten.md` | Umsetzung | autonom | offen |
| ECO1.5 | `ECO1.5-reparaturkosten.md` | Umsetzung | autonom | offen |
| ECO1.6 | `ECO1.6-gebaeude-nach-tiefe.md` | Umsetzung | autonom | offen |
| ECO1.7 | `ECO1.7-ausruestung-von-oben.md` | Umsetzung | autonom | offen |
| ECO1.8 | `ECO1.8-erzadern-freiraeumen.md` | Umsetzung | autonom | offen |
| ECO1.9 | `ECO1.9-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
