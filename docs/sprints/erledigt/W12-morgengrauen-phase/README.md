# W12 · SIM, SRV, REG · Morgengrauen als eigene Phase in Simulation, Protokoll und Regeln

- **Status:** erledigt
- **Projekt:** WRT
- **Domäne:** SIM, SRV, REG
- **Reife:** bereit
- **Tickets:** B-470
- **Start-Commit:** 80f2d3e9
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt; Defaults: dawn als kompatible Erweiterung ohne Versionssprung, Helligkeit linear, vor BR1, weil BAL ruht; gestapelt auf sprint/geb16 wegen der Golden-Regel)

## Ausgangslage

Q65 verlangt vier Phasen Tag, Dämmerung, Nacht, Morgengrauen (3:1:2:1, Start 6/2/4/2 min); gebaut sind drei Phasen mit 10/1/5 min, `dawn` ist nur ein Ereignis beim Übergang Nacht → Tag (`engine/sim/world_cycle.go`, `data/biomes/*.json` › `cycle`, B-470). Client und Welt-Betrachter folgen in W13.

## Ziel

Simulation, Protokoll und Regeltexte kennen das Morgengrauen als eigene Phase nach Q65. Am Ende sichtbar: `task check:go` grün mit vier Phasen, Golden mit Begründung verschoben, Protokoll-Beispiel mit `dawn`.

## Beteiligte und Zielgruppen

Spielende (längere Nacht-Übergänge); Agent in SIM, SRV und REG; 🧑 gibt die Spec frei.

## Anforderungen

B-470 › Anforderungen ohne Client und Welt-Betrachter (die folgen in W13). Sprint-eigen: Licht-Felder für Fackeln und Lagerfeuer (B-472) legt W12.1 mit vorläufigen Werten in `data/` an, damit W13 nur zeichnet.

## Nicht-Ziele

Zeichnen im Client und Welt-Betrachter (W13); Werte-Runde zu Gold, Wellen und Zielkorridoren (BR1, B-155).

## Regeln und Einschränkungen

Feature-Kette SIM → SRV (Protokoll als eigene Session für beide Enden) → REG (Doku). Golden-Änderung in eigenem Commit mit Begründung; nur ein aktiver Sprint ändert Golden (auch GD1). Deterministisch, 2 Spieler.

## Beispiele

Wald bei 480 s → Phase `night`, bei 720 s → `dawn`, Gold und Autosave fallen jetzt; bei 840 s → `day` ohne Ereignis.

## Ausnahme- und Fehlerfälle

Biom ohne Dauer des Morgengrauens oder mit nicht positiver Dauer → Laden abgelehnt. Alter Client kennt `dawn` nicht → zeichnet sie dunkel.

## Akzeptanzkriterien

- **AC-01** Die Simulation kennt vier Phasen mit Dauern aus `data/biomes/*.json`, das Ereignis `dawn` fällt bei Beginn des Morgengrauens (B-470/AC-01, B-470/AC-02, B-470/AC-03).
- **AC-02** Die Golden-Daten sind in eigenem Commit mit Begründung verschoben (B-470/AC-06).
- **AC-03** Das Protokoll beschreibt die Phase `dawn` mit Beispiel unter `testdata/protocol/` (B-470 › Offene Fragen 1).
- **AC-04** Regeltexte, Glossar und Game-Design nennen die vier Phasen (B-470/AC-07).
- **AC-05** Licht-Felder für Fackeln und Lagerfeuer stehen mit vorläufigen Werten in `data/` (B-472 › Anforderungen).

## Offene Fragen

🧑: Protokoll-Version oder kompatible Erweiterung (B-470 › Offene Fragen 1)? Vor oder nach BR1 (B-470 › 3)? Gibt es Fackeln schon als Spielobjekt (B-472 › 2)?

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W12.1 | `W12.1-vier-phasen-sim.md` | Umsetzung | autonom | fertig |
| W12.2 | `W12.2-protokoll-dawn.md` | Umsetzung | autonom | fertig |
| W12.3 | `W12.3-regeltexte.md` | Umsetzung | autonom | fertig |
| W12.4 | `W12.4-review.md` | Review | autonom | fertig |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W12.1 (SIM) Vier Phasen, Dauern und Licht-Felder in `data/`, Ereignis `dawn`, Golden in eigenem Commit (AC-01, AC-02, AC-05).
- W12.2 (SRV) Protokoll: Phase `dawn`, `docs/protocol.md`, Beispiel (AC-03).
- W12.3 (REG) Regeltexte, Glossar, Game-Design angleichen (AC-04).
- W12.4 (REG) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

2026-10-25, Claude (W12.4): `task check`, `task check:go` und `check:studio` grün. `dev:test` war einmal rot, ein Flake ohne Bezug zu W12 (B-621, mit -count=5 grün). `check:race` lokal übersprungen (kein C-Compiler).
AC-01, AC-02, AC-05 erfüllt durch W12.1 (vier Phasen, `data/light.json`), AC-03 durch W12.2 (Protokoll `dawn`), AC-04 durch W12.3 (Regeltexte). Golden ist im eigenen Commit verschoben.
Review (Opus) ohne schwere Befunde in Sim/Server; die zwei Client-Befunde sind behoben, dabei B-470/AC-04 und AC-05 vorgezogen.
Gestapelt auf `sprint/geb16`; die dort roten Studio-Tests (Fixture, Zählung) sind auf beiden Branches behoben. Neues Ticket B-621.
