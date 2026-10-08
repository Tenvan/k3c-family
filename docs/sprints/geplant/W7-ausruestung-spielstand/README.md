# W7 · SIM · Ausrüstung ohne Unverwundbarkeit, Spielstand vollständig

- **Status:** geplant
- **Projekt:** –
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-312, B-313, B-202, B-201, B-075
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Sofortiges Wiederaufheben macht die Burg bei passivem Spiel unverwundbar (B-312); W4.3b hängt an `sites_test.go` (B-313); Spielstand v3 ist zwischen S1 und W1 unklar (B-202) und stellt W2–W4 nicht vollständig her (B-201); der Golden-Spielstand hat keinen ausgebauten Hub (B-075).

Code-Stand 2026-10-06: `IslandSaveVersion = 4` (`engine/sim/island_save.go`, S2.2), Fixtures `testdata/saves/v1/` bis `v4/`; S1 und W1 liegen in `docs/sprints/erledigt/`. Das Ereignis `equipmentTaken` ist in `engine/sim/events.go` nur angelegt. W4 ist aktiv, W4.3b steht auf `blockiert` (B-313).

## Ziel

Ohne Handlung verliert man die Burg im Zielkorridor, und Speichern/Laden verliert nichts aus W2–W4. Am Ende sichtbar: Passive Burg kann fallen, Spielstand stellt W2–W4 wieder her, Golden-Hub geprüft.

## Beteiligte und Zielgruppen

🧑 hat B-312, B-313 und B-201 entschieden; Agent baut in `engine/sim/`, `data/` und `testdata/`; REG legt die vorläufigen Werte später fest (BR1).

## Anforderungen

B-312 › Anforderungen; B-313 › Anforderungen; B-202 › Anforderungen; B-201 › Anforderungen; B-075 › Anforderungen.

## Nicht-Ziele

Neue Gegner (K1), Bosse (K2), Migration alter Stände (Beschluss: keine Rücksicht), Balancing der neuen Werte (BR1), Protokoll (W5).

## Regeln und Einschränkungen

SIM; nach W4. Golden-Diffs nur mit Begründung in der Session.

- Beschluss 🧑 2026-10-06 (Chat): B-312 über Gegner, die fallengelassene Ausrüstung wegtragen (`equipmentTaken`); keine Wartezeit vor dem Wiederaufheben. Werte als neue Felder in `data/` mit vorläufigen Zahlen, REG legt sie später fest.
- Beschluss 🧑 2026-10-06 (Chat): B-313 Weg (a): `engine/sim/sites_test.go` in die Erlaubten Dateien, Vermerk-Prüfung raus, Testname ohne „WirkungOffen“. W7 ändert keine W4-Dateien; W7.1 prüft zuerst, ob W4.3b das schon erledigt hat.
- Beschluss 🧑 2026-10-06 (Chat): B-201 direkt nach W4 in W7, nicht mit W5 gebündelt. B-202: Beschluss Q42 gilt.
- Version 4 hat inzwischen S2.2 belegt; B-201 nimmt deshalb die nächste freie Version (heute 5, `testdata/saves/v5/`, Migration aus v4) statt der in Q42 genannten 4.
- Decision 001: Es gibt keine TS-Simulation mehr. B-075 wird als Go-Golden umgesetzt (`task golden:update`), der Vergleich „wie in TS“ entfällt; bestehende Golden-Dateien ändern sich nur mit Begründung.

## Beispiele

Welt mit 1 Spieler ohne Eingaben über 5 Nächte → Burg fällt in mindestens einem der Seeds 1–3. Stand mit 2 Kriegern, Elite-Bogenschütze, Bergmann, Rüstungsstufe 1, 4 Plantage-Bäumen speichern → laden → derselbe Stand.

## Ausnahme- und Fehlerfälle

Spielstand aus neuerer Version → Fehler 🚫 mit Version, kein stiller Verlust. Unbekannte Truppenart im Stand → Ladefehler statt Landstreicher (B-201/AC-03).

## Akzeptanzkriterien

- **AC-01** Gegner tragen fallengelassene Ausrüstung weg (Ereignis `equipmentTaken`, Werte in `data/`); bei passivem Spiel (1 Spieler, keine Eingaben) fällt die Burg in mindestens einem der Seeds 1–3 (Test), Golden aktualisiert (B-312/AC-01, B-312/AC-02).
- **AC-02** Der W3.2-Test heißt `TestSchmiedeUndRuestkammerZerstoerbar`, prüft weiter Bau und Zerstörung von Schmiede und Rüstkammer und verlangt den Vermerk „Wirkung offen“ nicht mehr (B-313/AC-01).
- **AC-03** S1 und W1 teilen sich die Spielstand-Version 3 eindeutig; ein Test lädt einen v3-Stand ohne Hub- und Platz-Stufe mit Stufe 1 (B-202/AC-01, B-202/AC-02, B-202/AC-03).
- **AC-04** Der Spielstand stellt Plantage, Berufe, Krieger, Elite, Rüstung, Schwerter und Händler nach dem Laden wieder her, in der nächsten freien Version mit Fixture (B-201/AC-01, B-201/AC-02, B-201/AC-03, B-201/AC-04).
- **AC-05** Ein Go-Golden-Spielstand mit gebautem und verändertem Hub (nicht leere `nodesGone`, `nodesMarked`, `pickupsTaken`, gebaute Plätze, Bogenschützen) wird geschrieben, geladen und weitergerechnet (B-075/AC-01).
- **AC-06** `task check` und `task check:go` grün.

## Offene Fragen

- Nicht blockierend: B-075/AC-01 nennt noch „in Go wie in TS“ und B-201 „Version 4“; Ticket-Revision durch 🧑 (W7 setzt Go-only und die nächste freie Version um).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W7.1 | `W7.1-ausruestung-wegtragen.md` | Umsetzung | autonom | offen |
| W7.2 | `W7.2-spielstand-wirtschaft.md` | Umsetzung | autonom | offen |
| W7.3 | `W7.3-golden-hub.md` | Umsetzung | autonom | offen |
| W7.4 | `W7.4-review.md` | Review | autonom | offen |

## Abnahme

–
