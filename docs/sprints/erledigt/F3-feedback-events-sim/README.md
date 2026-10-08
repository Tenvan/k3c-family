# F3 · SIM · Feedback-Ereignisse in der Simulation

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-139
- **Start-Commit:** fab601a
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-139; bestätigt die Auslegung (Tod, Skill, Nacht naht, Portal auf bestehende Ereignisse) und die vorläufige Obergrenze K

## Ausgangslage

`World.Events` kennt nur grobe Ereignisse (`built`, `wave`, `chest`, `playerDown` …). Treffer, Kill, Münze, Pfeil und Schlag fehlen; ohne sie gibt es weder Ton noch Effekte aus dem Server. Die Liste und das Größenbudget sind Beschluss Q08 (`docs/fragenkatalog.md`).

## Ziel

Die Simulation erzeugt die beschlossenen Feedback-Ereignisse deterministisch und mit Obergrenze je Tick. Am Ende sichtbar: Go-Tests je Ereignistyp, `task check:go` grün, Ereignisse im Ergebnis von `sim_run` (k3c-dev).

## Beteiligte und Zielgruppen

Entwickler von F4, SO1, GR5; 🧑 beschließt Liste und Budget (Q08) vor der Freigabe.

## Anforderungen

B-139 › Anforderungen.

## Nicht-Ziele

Protokoll und Bandbreitenmessung (F4, B-140), Ton und Effekte im Client (B-167, B-164).

## Regeln und Einschränkungen

Domäne SIM (`engine/sim/`). F2 (Golden-Ablauf, `task golden:update`) ist vorher abgeschlossen. Deterministisch, nur `engine/rng`, mit 2+ Spielern und Insel mit mehreren Stufen.

## Beispiele

Gegner trifft Mauer → ein `hit`-Ereignis mit Ort und Ziel; zweimal derselbe Lauf → gleiche Folge.

## Ausnahme- und Fehlerfälle

Mehr als K Ereignisse in einem Tick → niedrigste Priorität fällt weg, Zähler zählt mit (B-139/AC-03).

## Akzeptanzkriterien

- **AC-01** Je Ereignistyp der beschlossenen Liste prüft ein Test Typ, Ort und Beteiligte (B-139/AC-01).
- **AC-02** Ereignisfolgen sind bei gleichem Seed und gleichen Eingaben identisch (B-139/AC-02).
- **AC-03** Die Obergrenze je Tick schützt Tod- und Bau-Ereignisse und zählt Verworfene (B-139/AC-03).
- **AC-04** Auf der Insel entsteht ein Ereignis nur in seiner Stufe (B-139/AC-04).
- **AC-05** Golden-Daten sind aktualisiert und `task check:go` grün (B-139/AC-05).

## Offene Fragen

Q08 ist entschieden (2026-10-03): 12 Ereignisse (Treffer, Kill, Münze auf/gegeben, Pfeil, Schlag, Bau-Fortschritt/fertig, Tod, Wiederbeleben, Skill, Nacht naht, Portal), Budget ≤ 200 Byte je Tick und Client im Mittel. Offen: Obergrenze K je Tick und Priorität bei Überlauf (Tod und Bau zuerst, Vorschlag), das bestätigt 🧑 mit der Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| F3.1 | `F3.1-ereignisse-kampf.md` | Umsetzung | autonom | fertig |
| F3.2 | `F3.2-ereignisse-rest-obergrenze.md` | Umsetzung | autonom | fertig |
| F3.3 | `F3.3-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-10-03, Review F3.3: AC-01 bis AC-05 belegt in den Ergebnissen F3.1, F3.2 und F3.3; `task check` und `task check:go` grün.
- Keine schweren Befunde; Golden-Diff betrifft nur `events`, `rng.json` unverändert. B-139 archiviert; offen bleiben B-189 (Flüchtender ohne `kill`) und B-190 (`eventsDropped` im Protokoll, F4).
- Obergrenze für F4: K = 32 je Tick und Stufe (vorläufig), größter gemessener Tick 19, Mittel ≤ 0,1 Ereignisse je Tick; Budget ≤ 200 Byte je Tick und Client misst F4.
- Version: v0.5.0 gesetzt (2026-10-03, Bestätigung 🧑; Minor: Feedback-Ereignisse in der Simulation; gemeinsam mit DBG1/DBG2, solange v0.5.0 nicht gesetzt ist).
