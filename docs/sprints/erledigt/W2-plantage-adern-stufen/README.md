# W2 · SIM · Plantage, Adern, Stufenbreite und Mine

- **Status:** erledigt
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-114, B-115, B-012
- **Start-Commit:** 3215b03
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, Revision 1 (umfasst B-114, B-115, B-012 Revision 1)

## Ausgangslage

Holz wächst nicht nach, Stein bis Kristall sind endlich, Eisenstollen und Kristallhöhle sind nicht als Stufen angelegt, die Mine (Tiefe 2) ist unvollständig (B-114, B-115, B-012).

## Ziel

Plantage und Adern liefern Material über Zeit, die Insel führt fünf Stufen mit Eisenstollen und Kristallhöhle, die Mine ist spielbar.

Am Ende sichtbar: Tests für Generator, Adern, Plantage und Biome grün, aktualisierte Golden-Level.

## Beteiligte und Zielgruppen

Spieler und Bauern; Dichtewerte pflegt REG (Zielwerte aus B-099); 🧑 gibt die Spec frei.

## Anforderungen

B-114 › Anforderungen, B-115 › Anforderungen, B-012 › Anforderungen.

## Nicht-Ziele

Gegner der neuen Stufen (K1, B-129), Anzeige (W6), Protokoll (W5).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/sim/`; deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig. Golden-Daten nach dem Golden-Ablauf (B-137) aktualisieren, Spielstand-Änderungen nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Der Sprint bleibt in der Domäne SIM. Quellen: `docs/rules/materialien-gebaeude.md` § 1, `docs/rules/stufen.md` § 1.

## Beispiele

Mine-Stufe mit Seed 7 erzeugt → genau 2 Adern; 2 Bauern auf einer Ader liefern die Zielrate des Materials.

## Ausnahme- und Fehlerfälle

Dritter Bauer an einer Ader → wird nicht zugeteilt (höchstens 2). Baum-Limit der Plantage erreicht → kein weiterer Wuchs.

## Akzeptanzkriterien

- **AC-01** Der Generator erzeugt je Mine-Stufe genau 2 Adern, Seeds bleiben reproduzierbar (Test) (B-114/AC-01).
- **AC-02** Adern liefern mit 2 Bauern die Zielrate (±10 %), die Plantage lässt alle 30 s Bäume nachwachsen (höchstens 6), beides lässt den Startvorrat unberührt (Test) (B-114/AC-02, B-114/AC-03, B-114/AC-04).
- **AC-03** Eisenstollen und Kristallhöhle erzeugen mit 500 Seeds gültige Level, die Insel führt fünf Stufen mit Eingang und Treppen (Test) (B-115/AC-01, B-115/AC-03).
- **AC-04** Breiten von Höhle und Mine sind angepasst, die Ressourcendichte erfüllt die Zielwerte (Messung im Ticket) (B-115/AC-02).
- **AC-05** Die Mine ist spielbar mit eigenen Ressourcen und Gegnern, Go-Tests decken beides ab (B-012/AC-01, B-012/AC-02).
- **AC-06** Golden-Level aktualisiert, `task check` und `task check:go` grün (B-114/AC-05, B-115/AC-04).

## Offene Fragen

Zahlenwerte der Ressourcendichte: Zielwerte aus den Zielkorridoren (F1) bzw. B-099 müssen vorliegen, sonst Messung im Ticket (B-115/AC-02); 🧑 bestätigt.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W2.1 | `W2.1-adern-plantage.md` | Umsetzung | autonom | fertig |
| W2.2 | `W2.2-eisenstollen-kristallhoehle.md` | Umsetzung | autonom | fertig |
| W2.3 | `W2.3-mine-golden.md` | Umsetzung | autonom | fertig |
| W2.4 | `W2.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

- 2026-10-05 (W2.4, autonom): AC-01 (W2.1: `TestAdernJeStufe`), AC-02 (W2.1: `vein_test.go`, `plantation_test.go`), AC-03, AC-04 (W2.2: `TestValidate500Seeds`, `TestInselFuenfStufen`, `TestBreiteUndDichte`), AC-05 (W2.3: `mine_test.go`), AC-06 (Golden aktuell, `task check:go` und `task check` grün, `-race` übersprungen) mit Nachweis.
- Review des Diffs: keine schweren Befunde, keine Befunde behoben (Werte nur in `data/`, deterministisch, höchstens 2 Bauern je Ader, Startvorrat unberührt, `rng.json` und Spielstand unverändert); begründete Abweichungen in W2.2 erklärt.
- Neue Tickets aus dem Sprint: B-289 (Aggressionspool mit Adern), B-290 (Raum mit fünf Stufen), B-291 (Lava nicht auf Mauerlinien).
- Version: v0.9.0 vorgeschlagen (Minor: Adern, Plantage, Eisenstollen, Kristallhöhle und Lava wirken in der Simulation; letzter Tag v0.8.0).
