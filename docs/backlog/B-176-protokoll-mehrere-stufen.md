# B-176 · Das Protokoll liefert Level und Zustand jeder Stufe, in der ein lokaler Spieler steht

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** S2
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Protokoll v3 liefert einem Gerät nur Level und Zustand der Stufe seines ersten Monarchen (`engine/room/stages.go` › `deviceStage`, `pushState`; Entscheidung in `docs/sprints/erledigt/SP14-raum-auf-insel/README.md` › Offene Fragen: „mehrere Stufen gleichzeitig folgen mit B-106“). Zwei Spieler am selben Gerät in verschiedenen Stufen (Split-Screen, B-106) können deshalb nicht beide ihre Stufe sehen.

## Ziel

Ein Gerät bekommt für jede Stufe, in der einer seiner lokalen Spieler steht, Level, vollen Zustand und Deltas. Nutzen: B-106 (Kamera je Stufe) ist abnehmbar, jeder Spieler sieht seine Stufe.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy (1–4 lokale Spieler), Domäne SRV (`docs/protocol.md`, `engine/net/protocol*.go`, `src/online/protocol.ts` bleibt bei PLAT/CLI: Änderung des Protokolls bekommt laut Arbeitsweise eine eigene Session, die beide Enden anpasst).

## Anforderungen

- Nachrichten `level`, `snap` und `delta` tragen die Stufe (`stage`); je Stufe gibt es einen eigenen Delta-Strom mit eigenem `prev` (`engine/net/ws.go`, `delta.go`).
- Der Server schickt nur Stufen, in denen mindestens ein lokaler Slot des Geräts steht; verlässt der letzte Spieler eine Stufe, endet der Strom dieser Stufe.
- Wechselt ein Spieler die Stufe, bekommt das Gerät `level` und `snap` der neuen Stufe, bevor Deltas folgen.
- `docs/protocol.md` und `testdata/protocol/` beschreiben und belegen die Änderung; Server und Client parsen sie.
- Bandbreite: Der Benchmark aus F4 (B-140) misst Bytes je Tick bei 2 Spielern in verschiedenen Stufen; das Budget aus Q08 gilt je Gerät.
- Protokollversion: Vorschlag Erhöhung auf 4, ältere Clients erhalten `version`.

## Nicht-Ziele

Zeichnen und Kamera (B-106, S4), Zustand von Stufen ohne lokalen Spieler, neue Spielregeln.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Domänen: Protokolländerung als eigene Session, die nur das Protokoll und beide Enden anpasst. Deterministisch, 2+ Spieler, Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Zwei Spieler auf einem Gerät, Spieler 1 in Stufe 0, Spieler 2 in Stufe 1 → das Gerät bekommt `level`/`snap` beider Stufen und danach je Tick `delta` mit `stage`.

## Ausnahme- und Fehlerfälle

Ein Spieler fällt und wartet → seine Stufe bleibt im Strom, solange der Slot belegt ist. Gerät trennt → alle Ströme enden, beim Wiederverbinden kommen alle Stufen als `snap`. Sehr viele Stufen (n Stufen je Insel) → nur die der lokalen Spieler.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt `stage` in `level`, `snap` und `delta` und die Regel, welche Stufen ein Gerät bekommt; `testdata/protocol/` hat Beispiele, beide Enden parsen sie.
- **AC-02** Test (`engine/room`/`engine/net`): Ein Gerät mit zwei Spielern in verschiedenen Stufen bekommt `level`, `snap` und `delta` beider Stufen; ein Gerät mit einem Spieler nur seine Stufe.
- **AC-03** Test: Stufenwechsel eines Spielers beendet den alten und startet den neuen Strom mit `level` und `snap`; Wiederverbinden liefert `snap` aller Stufen.
- **AC-04** Der Benchmark (B-140) nennt Bytes je Tick für zwei Spieler in verschiedenen Stufen und bleibt im Budget aus Q08, oder die Abweichung steht als Ticket.
- **AC-05** `task check` und `task check:go` sind grün; ältere Clients erhalten `version`.

## Offene Fragen

Versionssprung 4 oder rückwärtskompatibel (optionales Feld): Vorschlag Sprung auf 4, 🧑 bestätigt mit der Freigabe.

## Notizen

Anlass: Hinweis aus der S4-Vorbereitung (Protokolllücke, 2026-10-03). Voraussetzung für S4.3 (Abnahme).
