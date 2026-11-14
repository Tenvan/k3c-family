# SP08 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Browser als reiner Client
- **Projekt:** –

## Ziel

Der Browser rechnet nichts mehr, er schickt Eingaben und zeichnet Snapshots, lokal wie online. Am Ende sichtbar: 2 Controller an der Xbox + 1 Handy im selben Raum.

## Ergebnis

Review 2026-10-01 (SP08.4): `task check` und `task check:go` grün, Diff `98a4907..main` geprüft, keine schweren Befunde; AC-01 bis AC-03 und AC-06 bis AC-14 mit Tests laut SP08.1 bis SP08.3 belegt (AC-09: je Fehler-Code ein Test; AC-13: `noSim.test.ts`). Browser-Lauf 2026-10-01 (Agent, Freigabe 🧑, `task serve:go`): Lobby, Raum erstellen, Slots, `?mock=N`, Wiederverbinden, `replaced`, gemockter Controller bestätigt; danach Testseite und alle Layouts 1–4 von 🧑 am PC abgenommen (T1). AC-04 entfällt. **AC-05 abgenommen 2026-10-01 durch 🧑 (Ralf)** an Xbox mit Controllern und Handy im selben Raum: „ …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Der Browser schickt nur Eingaben und zeichnet Snapshots nach Protokoll v2 mit Interpolatio … |
| AC-02 | geprüft | Mehrere lokale Spieler pro Gerät nach dem Layout aus B-016 (B-016/AC-02). |
| AC-03 | – | Raum erstellen und beitreten funktioniert (B-037/AC-01, B-037/AC-02). |
| AC-04 | entfällt | entfällt (Revision 2: B-018 verworfen, der Zielwert 300 Zeilen gilt nicht mehr). |
| AC-05 | – | Am TV: 2 Controller an der Xbox + 1 Handy spielen im selben Raum (Beobachtung durch 🧑). |
| AC-06 | geprüft | Handschlag: Geräte-ID dauerhaft im `localStorage` (ohne Speicher eine Zufalls-ID), `hello` … |
| AC-07 | geprüft | Eingabe-Takt: `input` bei Änderung, sonst spätestens alle 500 ms, höchstens eine je Tick, … |
| AC-08 | geprüft | Level aus `level` (Biom-Werte über die Biom-ID aus `data/`, kein Generator), `snap` ersetz … |
| AC-09 | geprüft | Jeder Fehler-Code aus `docs/protocol.md` › *Fehler-Codes* hat ein festes Verhalten: die Se … |
| AC-10 | geprüft | Wiederverbinden (Client-Teil von B-030): Bricht die Verbindung ab, zeigt der Client einen … |
| AC-11 | geprüft | Slots: A auf einem neuen Controller sendet `addSlot`, ein getrennter Controller `removeSlo … |
| AC-12 | geprüft | Layout nach B-016: 1 lokaler Spieler Vollbild (mit Mitspieler oben ⅓), 2 Streifen, 3 Spiel … |
| AC-13 | geprüft | Der Browser rechnet nichts: `src/scenes` importiert keine Simulationsfunktionen aus `src/w … |
| AC-14 | – | `game.html?autostart=1&fresh=1&save=NAME&mock=N` erstellt ohne Auswahl einen neuen Raum mi … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-016 | erledigt | AC-02 geprüft, AC-12 geprüft | Layout für mehr als zwei lokale Spieler ist entschieden |
| B-037 | erledigt | AC-03 – | Lobby zeigt Räume und startet Spiele |
| B-039 | erledigt | AC-01 geprüft | Bewegungen laufen trotz Snapshots flüssig |
| B-061 | erledigt | – | Die Spec von SP08 deckt alle Client-Regeln aus Protokoll v2 ab |
| B-082 | erledigt | AC-14 – | `game.html` startet per Parameter ohne Auswahl und mit Mock-Slots |
