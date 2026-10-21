# W12 · SIM · Morgengrauen als eigene Phase in Simulation, Protokoll und Regeln

- **Status:** geplant
- **Projekt:** WRT
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-470
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W12.1 (SIM) Vier Phasen, Dauern und Licht-Felder in `data/`, Ereignis `dawn`, Golden in eigenem Commit (AC-01, AC-02, AC-05).
- W12.2 (SRV) Protokoll: Phase `dawn`, `docs/protocol.md`, Beispiel (AC-03).
- W12.3 (REG) Regeltexte, Glossar, Game-Design angleichen (AC-04).
- W12.4 (REG) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
