# B-961 · Inseln einer Etappe sind frei wählbar, die nächste Etappe öffnet nach allen Inseln der vorigen

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WE4
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15 (Mock „So umsetzen“)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Heute fährt der Inselwechsel immer zur nächsten Insel der Liste (`docs/rules/welten.md` › Inselwechsel, `engine/sim/island_switch.go`). 🧑 2026-11-15: Inseln einer **Etappe** sollen in beliebiger Reihenfolge anfahrbar sein; die nächste Etappe öffnet erst, wenn alle Inseln der laufenden Etappe abgeschlossen sind (Mock Seekarte, Freigabe „So umsetzen“).

## Ziel

Welt-Datei kennt je Insel eine Etappe; der Hafen bietet als Ziele die offenen Inseln der laufenden Etappe, danach die der nächsten; der Spielstand merkt sich abgeschlossene Inseln und die gewählte Zielinsel.

## Beteiligte und Zielgruppen

Spieler (Koop 1–2), Welten-Autor (Editor B-963), Anzeige im Spiel (B-962). Entscheidet: 🧑.

## Anforderungen

- Neues optionales Feld `etappe` (Ganzzahl ≥ 1) je Insel in `data/worlds/*.json`; fehlt es, gilt Listenplatz + 1 (heutiges lineares Verhalten, Golden unverändert).
- Prüfung beim Laden: Etappen lückenlos ab 1, in der Liste nicht fallend; Fehlermeldung nennt Insel und Feld.
- Abgeschlossen ist eine Insel mit besiegtem Endboss beim Ablegen.
- Ziele am Hafen: nicht abgeschlossene Inseln der Etappe der aktuellen Insel; sind alle abgeschlossen, die Inseln der nächsten Etappe. Keine Ziele = letzte Insel der Welt (Siegvariante wie heute).
- Wahl des Ziels: Aktion eines gekrönten Spielers am geöffneten Wechselpunkt, nur aus den Zielen; ohne Wahl gilt das erste Ziel in Listenreihenfolge.
- `NextIsland` baut das gewählte Ziel; Seed weiter aus Spiel-ID und **Listenplatz** des Ziels.
- Spielstand: abgeschlossene Insel-IDs und gewähltes Ziel; Zustand/Ereignisse nennen Ziele und Wahl (`docs/protocol.md`).
- Deterministisch, mit 2 Spielern gleichzeitig.

## Nicht-Ziele

Seekarte im Spiel (B-962), Editor (B-963), Rückkehr auf abgeschlossene Inseln, Startinsel wählbar (Start bleibt die erste Insel der Liste).

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `engine/room/**`, `docs/rules/welten.md`, `docs/rules/stufen.md`, `docs/protocol.md`, `data/worlds/sengoku.json`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** spielstand
- Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`).
- Keine Rücksicht auf alte Spielstände.

## Beispiele

- Sengoku mit Etappen 1, 2, 2: Nach Insel 1 bietet der Hafen Insel 2 und 3; Wahl Insel 3 → danach bietet der Hafen nur Insel 2; nach Insel 2 keine Ziele mehr, Insel 2 ist die letzte (Sieg möglich).
- Welt ohne `etappe`: Verhalten wie heute.

## Ausnahme- und Fehlerfälle

- Wahl einer Insel außerhalb der Ziele → abgelehnt, Ziel unverändert.
- Etappe 3 ohne Etappe 2 → Ladefehler.
- Spieler wählen gleichzeitig: die letzte Wahl im Takt gilt (deterministisch nach Spieler-Index).

## Akzeptanzkriterien

- **AC-01** Etappen-Feld geladen, Standardwert und Prüfungen getestet (`engine/sim/island_worlds*_test.go`).
- **AC-02** Ziele und Wahl am Hafen getestet, auch mit 2 Spielern (`engine/sim/island_switch*_test.go`).
- **AC-03** Spielstand speichert und lädt abgeschlossene Inseln und Ziel (`engine/room` Test); Golden unverändert.
- **AC-04** `welten.md`, `stufen.md`, `protocol.md` und Glossar-Begriff „Etappe“ nachgezogen; `sengoku.json` mit Etappen 1, 2, 2.

## Offene Fragen

keine (Defaults: Start auf erster Insel, ohne Wahl erstes Ziel)

## Notizen

Mock: Seekarte links, Querschnitt rechts (Session 2026-11-15).
