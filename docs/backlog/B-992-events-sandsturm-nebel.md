# B-992 · Die Events Sandsturm und Nebel verändern für eine Nacht Tempo und Fernkampf-Reichweite an der Oberfläche

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE8
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Es gibt kein Wetter mit Spielwirkung. B-969 (Sprint SI1, Regeltext) plant „Wetter je Welt“ mit Taifun, Flut und Schneesturm: kurz, angekündigt, für eine Nacht. Events stehen in `data/events.json`, Regeln in `docs/rules/bosse.md` § 2, Code in `engine/sim/events.go`.

## Ziel

Nil hat den Sandsturm, Britannien den Nebel: zwei Wetter-Events, die für eine Nacht die Verteidigung an der Oberfläche erschweren.

## Beteiligte und Zielgruppen

Spieler (2 Spieler) müssen in diesen Nächten anders verteidigen. 🧑 gibt frei (neue Spielregel).

## Anforderungen

- Zwei Events in `data/events.json`, in der Basis aus (`everyNights: 0`), je Welt über `overrides › events` einschaltbar (B-982):
  - `sandstorm`: in der Event-Nacht an der Oberfläche (Tiefe 0) Tempo aller Truppen und Gegner × `speedFactor`, Fernkampf-Reichweite aller × `rangeFactor`.
  - `fog`: in der Event-Nacht an der Oberfläche Fernkampf-Reichweite der Verteidiger (Türme, Fernkämpfer) × `rangeFactor`; Gegner unverändert.
- Wirkung nur in der Nacht des Events und nur in Tiefe 0; tiefere Stufen bleiben unberührt.
- Die Regeln folgen dem Wetter-Rahmen aus B-969 (Ankündigung, Dauer); weicht B-969 ab, gilt B-969.
- Deterministisch, gleiche Wirkung für alle Spieler der Insel; Zustand nennt das aktive Event (`docs/protocol.md`).
- Standard-Welt unverändert, `testdata/golden/` bytegleich. Regeltext in `bosse.md` § 2, Begriffe im Glossar.

## Nicht-Ziele

Taifun, Flut, Schneesturm (B-969), Wetter am Tag, Anzeige im Spiel (B-995), Einschalten in den Welten (B-996), Balancing.

## Regeln und Einschränkungen

Sim-Code im eigenen Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). Werte in `data/events.json`, vorläufig. Keine Simulation.

- **Bereich:** `engine/sim/**`, `engine/room/**`, `data/events.json`, `docs/rules/bosse.md`, `docs/protocol.md`
- **Abhängig von:** B-991, B-969
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** protokoll

## Beispiele

Nil, Sandsturm jede 6. Nacht mit `speedFactor` 0,7 und `rangeFactor` 0,6 → Bogner treffen erst auf kürzere Distanz, alle laufen langsamer. Britannien, Nebel jede 4. Nacht mit `rangeFactor` 0,5 → Türme schießen halb so weit, Gegner kommen näher heran.

## Ausnahme- und Fehlerfälle

Faktor ≤ 0 → Daten bzw. Welt laden nicht. Event-Nacht fällt mit Vollmond, Blutmond oder Überfall zusammen → alle gelten, Faktoren multiplizieren sich. Spieler wechselt in der Event-Nacht in eine tiefere Stufe → dort keine Wirkung.

## Akzeptanzkriterien

- **AC-01** Test `sandstorm`: Tempo und Reichweite in der Event-Nacht in Tiefe 0 geändert, am Tag und in Tiefe 1 nicht.
- **AC-02** Test `fog`: Reichweite nur der Verteidiger geändert.
- **AC-03** Test: Überlagerung mit einem anderen Event; gleicher Seed → gleicher Ablauf; `testdata/golden/` unverändert.
- **AC-04** `bosse.md` § 2 und `docs/protocol.md` nachgezogen.

## Offene Fragen

- Wirkung → Default: Sandsturm bremst alle und kürzt jede Fernkampf-Reichweite, Nebel kürzt nur die der Verteidiger (vorläufig, 🧑 bestätigt).
- Verhältnis zu B-969 → Default: Sandsturm und Nebel sind die Wetter-Events von Nil und Britannien im Rahmen von B-969; dieses Ticket startet erst nach B-969 (vorläufig, 🧑 bestätigt).
- Werte → Default: Sandsturm `everyNights` 6, `speedFactor` 0,7, `rangeFactor` 0,6; Nebel `everyNights` 4, `rangeFactor` 0,5 (vorläufig, Balancing später, 🧑 bestätigt).

## Notizen

Zerlegt aus B-960 (Events Sandsturm und Nebel). B-969 plant Flut für Nil; ob Nil Flut und Sandsturm bekommt, entscheidet 🧑 mit B-969.
