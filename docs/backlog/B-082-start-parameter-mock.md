# B-082 · `game.html` startet per Parameter ohne Auswahl und mit Mock-Slots

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP08
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (SP08 Revision 4)

## Ausgangslage

Ab SP08 startet `game.html` immer in der Lobby. Die Testseite (B-081, PLAT) soll Szenarien mit 1–4 Spielern per Link starten und
darf dafür keine Szenen-Interna kennen. Das Spiel gehört der Domäne CLI, die Testseite PLAT.

## Ziel

`game.html` versteht Start-Parameter, über die eine Testseite oder ein Entwickler eine Session ohne Auswahl und mit Mock-Spielern
startet. Nutzen: Layout und Raumverhalten sind mit einem Gerät prüfbar.

## Beteiligte und Zielgruppen

Entwickler und Agenten; die Testseite (B-081); 🧑 prüft am TV.

## Anforderungen

- `?autostart=1` überspringt die Lobby-Auswahl und erstellt direkt einen Raum mit dem Spielstand aus `?save=NAME` (Vorgabe `familie`).
- `?fresh=1` erstellt den Spielstand neu (`fresh: true`), sonst wird er geöffnet bzw. bei `save_not_found` neu erstellt.
- `?mock=N` (0–3) legt N Mock-Slots am selben Gerät an: eigene Slots mit eigenem Monarchen, die keine Eingabe haben und stehen
  (`moveX: 0`). Der echte Spieler bleibt Slot 0.
- Ungültige Werte werden ignoriert, nie weitergereicht (`save` nur `^[a-z0-9-]{1,32}$`).

## Nicht-Ziele

Verhalten der Mock-Spieler über „stehen“ hinaus; Mock-Spieler als eigene Geräte; Protokolländerungen.

## Regeln und Einschränkungen

Domäne CLI; `src/scenes` zeichnet nur, die Parameter-Auswertung ist eine reine Funktion mit Test; Protokoll v2 unverändert
(Mock-Slots sind gewöhnliche Slots 1–3, Grenze 4 lokale Spieler pro Gerät).

## Beispiele

`game.html?autostart=1&fresh=1&save=test-ab12&mock=3` → neuer Raum, 4 Monarchen am Gerät, 2×2-Raster, die Mock-Monarchen stehen.

## Ausnahme- und Fehlerfälle

`mock=7` oder `save=Ab C` → Parameter ignoriert (Vorgabe). Raum nicht erstellbar (`too_many_rooms`, `save_exists`) → Hinweis wie in der Lobby.

## Akzeptanzkriterien

- **AC-01** Die Parameter-Auswertung liefert für gültige und ungültige Eingaben das beschriebene Ergebnis (Test).
- **AC-02** `autostart` mit `mock=0` bis `mock=3` erzeugt `create` mit 1 bis 4 Slots (Test).

## Offene Fragen

keine

## Notizen

Umgesetzt in SP08.3. Entstanden beim Planen von T1 (Testseite).
