# B-1225 · Abnahme Spieltest-Aufnahme (WZ64–WZ66): Sofa-Test mit zwei Pads und Frame-Messung bei laufender Aufnahme

- **Domäne:** DEV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** WN1
- **Projekt:** WZN
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Chat 2026-11-22
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WZ64 (Aufnahme), WZ66 (Spieltest-Modus im Spiel) und WZ65 (Auswertung) bauen die Spieltest-Kette. Einzeln sind die Teile per Unit-Test geprüft; ob sie am TV zusammen funktionieren und das Frame-Budget halten, zeigt nur ein echter Spieltest.

## Ziel

Die Kette Aufnahme → Transkript → Befunde → Tickets ist am Gerät belegt, und die Aufnahme kostet keine spürbaren Frames.

## Beteiligte und Zielgruppen

Ralf (🧑) mit einem Mitspieler am TV; Claude misst und wertet aus.

## Anforderungen

- Ein echter Spieltest von mindestens 15 min mit 2 Pads über den Knopf Spieltest starten.
- Frame-Messung `--measure-frames` einmal ohne und einmal mit laufender Aufnahme (gleiche Szene), Vergleich des schlechtesten Frames.
- Durchlauf Transkribieren → Auswerten → Haken → Tickets anlegen.

## Nicht-Ziele

- Sim, Balancing, Lasttest (gesperrt, Beschluss 🧑 2026-11-11).

## Regeln und Einschränkungen

Genau ein Abnahme-Ticket für WZ64–WZ66 (B-742).

- **Bereich:** `playtests/**`
- **Abhängig von:** WZ64, WZ66, B-1224
- **Braucht 🧑:** Abnahme, Gerät
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Abnahme-Ticket, die Fälle stehen in den Kriterien.

## Ausnahme- und Fehlerfälle

- Schlechtester Frame mit Aufnahme deutlich schlechter → neues Ticket in WZG (Encoder, Auflösung, fps), Abnahme bleibt offen.

## Akzeptanzkriterien

- **AC-01** Sim (`sim_test`): entfällt, Sim gesperrt und keine Wirkung auf Simulation.
- **AC-02** Balance: entfällt, keine Werte geändert.
- **AC-03** Gen: entfällt, keine Papierschnitt-Grafik geändert.
- **AC-04** Race: `check_run task:check:race` (ohne gcc als übersprungen nennen).
- **AC-05** 🧑 hat einen Spieltest ≥ 15 min mit 2 Pads aufgenommen: REC sichtbar, Marker beider Spieler in `markers.jsonl`, Fragebogen in `survey.json`, Video und Sprache abspielbar.
- **AC-06** Schlechtester Frame mit Aufnahme höchstens 2 ms schlechter als ohne (beide Werte im Ergebnis).
- **AC-07** 🧑 hat aus dem Spieltest per Knopf Befunde erzeugt, angehakt und mindestens ein Ticket anlegen lassen; Duplikat-Abgleich hat gegriffen oder ist begründet leer.

## Offene Fragen

keine

## Notizen

–
