# WZ14 · REG, SIM, SRV, CLI, DEV · Geplante Chunks tragen Ruinen und Gegnerlager

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** REG, SIM, SRV, CLI, DEV
- **Reife:** bereit
- **Tickets:** B-468
- **Start-Commit:** 37327754
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-21, Ralf im Chat (Revision 2)

## Ausgangslage

`B-468 › Ausgangslage`: Geplante Chunks (B-461, WZ8) tragen feste Objekte, aber keine vorgebauten Gebäude. Gebäude entstehen nur auf Hub-Plätzen (`worldSiteSpecs`, `islandSites`); das Feld `buildings` des geplanten Chunks (`engine/level/planned.go`) ist reserviert und wird abgelehnt. **Ab Revision 2 liefert WZ14 nur die Ruinen**; die Gegnerlager folgen in einem eigenen Sprint (B-517, Beschluss WZ14.1).

## Ziel

`B-468 › Ziel`: Ein geplanter Chunk kann eine Ruine tragen (gebautes Nutzgebäude mit 25 % HP an fester Lage, auch als Vorposten außerhalb der Mauern); sie wird beim Laden geprüft, wirkt im Spiel wie ein gebautes Gebäude ihrer Art, ist Hub-neutral, wird repariert (außerhalb der Mauern nur ohne Gefahr) und kehrt nach einem Burgfall zurück; Editor und Vorschau des Studios setzen und zeigen sie.

**Am Ende sichtbar:** im Studio eine Sondergegend mit Ruine in Chunk- und Live-Vorschau und im Spiel, inklusive Rückkehr nach dem Dev-Befehl „Burgfall“, von 🧑 am PC abgenommen.

## Beteiligte und Zielgruppen

`B-468 › Beteiligte und Zielgruppen`: 🧑 entscheidet die Regeln im Workshop, plant im Editor, nimmt ab und gibt die Spec frei; Spielende; Agenten je Domäne, getrennter Reviewer.

## Anforderungen

`B-468 › Anforderungen` (Revision 2, nur Ruinen, inklusive Nachtrag nach dem Spec-Review: Vorposten, Wiederaufbau ab Hub-Stufe der Art, Dev-Befehl „Burgfall“). Sprint-eigen: Domänen nacheinander SIM → SIM → SIM → SRV → CLI → DEV, je Session eine; Testplanung mit Ruine nur unter `testdata/planned/forest-ruin.json`; Sessions, die vom Studio genutzte Funktionen berühren, fahren `task check:studio`.

## Nicht-Ziele

`B-468 › Nicht-Ziele`; insbesondere Gegnerlager (B-517), neue Sprites je Art, Reparatur gegen Bezahlung, neuer Site-Zustand, neue Spielstand-Version, Protokollwechsel.

## Regeln und Einschränkungen

`B-468 › Regeln und Einschränkungen`: Zufall nur über `engine/rng` mit eigenem Strom je geplantem Chunk; Golden ohne Planung byte-gleich; `engine/level` importiert nichts aus `sim`, `room`, `net`; Spielstand über `SiteSave`/`kind@x` ohne neue Version; Protokoll 7 bleibt (nur additive Felder, `docs/protocol.md` und `testdata/protocol` nachziehen); Studio schreibt REG-Daten nur atomar mit Stand-Prüfung; Datei ≤ 400, Funktion ≤ 60 Zeilen; mit 2 Spielern; Taste B unbelegt.

## Beispiele

`B-468 › Beispiele`: Schmiede auf Platz +4 mit 63 HP wirkt sofort und wird repariert; Kaserne auf Platz −3 erhöht das Truppenlimit, nicht die Hub-Stufe; Burgfall → Ruine steht wieder; ohne Ruine Golden byte-gleich.

## Ausnahme- und Fehlerfälle

`B-468 › Ausnahme- und Fehlerfälle`: unbekannte oder nicht erlaubte Art, mehr als eine Ruine, ungültige Lage oder Abstand, doppelter Schlüssel `kind@x`, Eintrag in `enemies` → beim Laden abgelehnt mit Insel, Tiefe, Platz und Grund; Spielstand mit unbekannter Ruine → Platz verworfen.

## Akzeptanzkriterien

Aus `B-468` (Revision 2):

- **AC-01** `B-468/AC-01` Regeltext, Glossar, Werte in `data/ruins.json` – WZ14.1, WZ14.2
- **AC-02** `B-468/AC-02` Format `buildings` lesen und prüfen, je Fehlerfall ein Go-Test – WZ14.2
- **AC-03** `B-468/AC-03` Ruine als Platz, Vorposten-Schutz, Wiederaufbau, Burgfall, Dev-Befehl, Spielstand, 2 Spieler – WZ14.3, WZ14.4
- **AC-04** `B-468/AC-04` Golden byte-gleich, Benchmark ≤ +10 % – WZ14.2, WZ14.3
- **AC-05** `B-468/AC-05` Protokoll, Vorschau, Grafik, Editor, Dev-Befehl im Studio – WZ14.5, WZ14.6, WZ14.7
- **AC-06** `B-468/AC-06` Review, alle Prüfungen grün – WZ14.8
- **AC-07** `B-468/AC-07` Abnahme durch 🧑 am PC – WZ14.9

## Offene Fragen

Entschieden im Regel-Workshop WZ14.1 (2026-10-21, Ralf im Chat) samt Nachtrag nach dem Spec-Review, siehe `B-468 › Offene Fragen` 1–8: Ruine = gebautes Nutzgebäude mit 25 % HP, höchstens eine je Chunk, Hub-neutral und funktional, Reparatur kostenlos durch Bauern, Burgfall → zurück zur Ruine; Vorposten auch außerhalb der Mauern (Bauern dort nur ohne Gefahr); Wiederaufbau ab Hub-Stufe der Art; Dev-Befehl „Burgfall“; Abstände, Kennung `ruin`, Darstellung und Lager-Kapazität bestätigt; Gegnerlager als eigener Sprint (B-517). Offen ist nur die Freigabe der Spec (Revision 2) durch 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ14.1 | `WZ14.1-workshop-regeln.md` | Workshop | Mensch | fertig |
| WZ14.2 | `WZ14.2-werte-format-pruefung.md` | Umsetzung | autonom | fertig |
| WZ14.3 | `WZ14.3-ruine-als-platz.md` | Umsetzung | autonom | fertig |
| WZ14.4 | `WZ14.4-vorposten-dev-burgfall.md` | Umsetzung | autonom | offen |
| WZ14.5 | `WZ14.5-protokoll-vorschau-ruine.md` | Umsetzung | autonom | offen |
| WZ14.6 | `WZ14.6-ruine-zeichnen.md` | Umsetzung | autonom | offen |
| WZ14.7 | `WZ14.7-editor-ruine.md` | Umsetzung | autonom | offen |
| WZ14.8 | `WZ14.8-review.md` | Review | autonom | offen |
| WZ14.9 | `WZ14.9-abnahme.md` | Review | Mensch | offen |

- WZ14.1 (REG) Workshop mit 🧑: Beschlüsse, Regeltext § 8, Glossar (AC-01).
- WZ14.2 (SIM) `data/ruins.json`, Feld `buildings` lesen und prüfen, Golden und Benchmark (AC-01, AC-02, AC-04).
- WZ14.3 (SIM) Ruine als Platz mit Kennung, Wiederaufbau ab Hub-Stufe der Art, Burgfall, Spielstand, 2 Spieler (AC-03, AC-04).
- WZ14.4 (SIM) Vorposten-Schutz der Bürger, Dev-Funktion „Burgfall“, Golden byte-gleich (AC-03, AC-04).
- WZ14.5 (SRV) Protokoll-Doku `sites[].ruin`, Vorschau, Dev-Befehl über Raum und Protokoll (AC-05).
- WZ14.6 (CLI) Schadens-Darstellung, Trümmer, Debug-Knopf „Burgfall“ (AC-05).
- WZ14.7 (DEV) Editor und Vorschau des Studios (AC-05).
- WZ14.8 (DEV) Review: alle Prüfungen, Diff aller Domänen, PR (AC-01–AC-06).
- WZ14.9 (DEV) Abnahme durch 🧑 am PC, Agent Mensch (AC-07).

Budget: 9 Sessions inklusive Workshop, Review und Abnahme liegt über den üblichen 3–6; begründet durch die Domänenfolge SIM → SRV → CLI → DEV (wie WZ8 mit 7).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
