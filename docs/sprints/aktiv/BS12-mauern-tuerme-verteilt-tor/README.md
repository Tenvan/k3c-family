# BS12 · SIM, DEV, REG · Mauern und Türme verteilt, Tor ist die äußerste Mauer

- **Status:** aktiv
- **Projekt:** BAS
- **Domäne:** SIM, DEV, REG
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-28, Chat, durch 🧑, Revision 1 (B-652, Defaults der offenen Fragen)

## Ausgangslage

Mauern und Türme stehen auf 5 festen Linien je Seite, das Tor ist ein eigenes Gebäude (B-652). Beschluss 🧑 2026-10-28: Das kommt vor weiteren Balancing-Läufen (B-651 pausiert).

## Ziel

Verteilte Mauer- und Turm-Plätze je Chunk (1–3, per Seed, Türme zum Teil nah an Mauern), Tor = äußerste gebaute Mauer, Bauplatz erst nach Freiraum von Bäumen und Felsen bezahlbar; Bots und Client laufen damit, der Meilenstein-Lauf `coop2` läuft durch (Ergebnis offen).

## Beteiligte und Zielgruppen

Claude setzt um, 🧑 nimmt ab und entscheidet Werte.

## Anforderungen

B-652 › Anforderungen und Akzeptanzkriterien AC-01 bis AC-07.

## Nicht-Ziele

Balancing-Justierung (BS3/B-651); Baustellen-Fehler und Holzzaun (BS13/B-653).

## Regeln und Einschränkungen

Werte nur in `data/`; deterministisch (`engine/rng`); keine Rücksicht auf alte Spielstände; Golden mit Begründung; Simulationen nur in BS12.4 (`task balance:milestone` oder `sim_test`).

## Beispiele

B-652 › Beispiele.

## Ausnahme- und Fehlerfälle

Stimmt ein Kriterium nicht mit dem Code überein: Session `blockiert`, Ticket vom Typ Frage, 🧑 fragen.

## Akzeptanzkriterien

- **AC-01** B-652/AC-01, AC-02 (kein Tor mehr, Tor-Regel).
- **AC-02** B-652/AC-03, AC-04 (Plätze je Chunk, Türme nah).
- **AC-03** B-652/AC-05 (Freiraum).
- **AC-04** B-652/AC-06 (Bots, Meilenstein-Lauf läuft durch).
- **AC-05** B-652/AC-07 (Regeln, Glossar, Protokoll, check:diff grün).

## Offene Fragen

keine (Defaults aus B-652).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS12.1 | `BS12.1-daten-regeln-generator.md` | Umsetzung | autonom | fertig |
| BS12.2 | `BS12.2-sim-tor-freiraum.md` | Umsetzung | autonom | fertig |
| BS12.3 | `BS12.3-bots-client-protokoll.md` | Umsetzung | autonom | fertig |
| BS12.4 | `BS12.4-simulationen.md` | Umsetzung | autonom | fertig |
| BS12.5 | `BS12.5-review.md` | Review | autonom | offen |
| BS12.6 | `BS12.6-aufraeumen-gate-linien.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
