# B-354 · Die Bot-Eingabe verliert keine kurzen Drücke und hält bei stummem Feed an

- **Domäne:** PLAT
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** TST
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`src/input/botInput.ts` (TR2) hält je Slot nur das letzte Feed-Kommando und liest es einmal pro Frame. Kommen zwischen zwei Frames `attack` oder ein Skill an und gleich wieder aus, sieht das Spiel den Druck nie (ruckelnder Headless-Browser). Bleibt der Feed offen, schickt aber nichts mehr, läuft der Monarch mit dem letzten `moveX` weiter. Fehlt der Feed ganz, loggt jeder Versuch einmal pro Sekunde `👋` als `warn`. Befunde aus dem Review TR2.2.

## Ziel

Bot-Testläufe spielen jeden Druck der Workbench ab, ein hängender Feed hält die Monarchen an, und das Client-Log bleibt ohne Feed ruhig.

## Beteiligte und Zielgruppen

Workbench (`sim_test`), Agenten und 🧑 bei Testläufen.

## Anforderungen

- Ein Druck (`pay`, `attack`, Skill), der zwischen zwei `update()` an- und wieder ausgeht, zählt im nächsten Frame als gedrückt.
- Kommt für einen Slot länger als eine Frist (z. B. 1 s) keine Nachricht, liefert er keine Eingabe mehr.
- Fehlt der Feed dauerhaft, wird nur der erste Abbruch geloggt (oder die Pause wächst).

## Nicht-Ziele

Einbindung ins Spiel (B-353), Bot-Entscheidung.

## Regeln und Einschränkungen

Domäne PLAT (`src/input/`). Regeln aus `CLAUDE.md` (Logging mit Emoji, kein `Math.random()`).

## Beispiele

Feed schickt `attack: true` und 5 ms später `attack: false`, der nächste Frame kommt nach 30 ms → `justPressed('attack')` ist in diesem Frame wahr.

## Ausnahme- und Fehlerfälle

Workbench hängt, Socket bleibt offen → nach der Frist stehen die Monarchen.

## Akzeptanzkriterien

- **AC-01** Test: kurzer Druck zwischen zwei `update()` wird einmal als gedrückt gemeldet.
- **AC-02** Test: ohne Nachricht länger als die Frist liefert der Slot keine Eingabe.
- **AC-03** Test: dauerhaft fehlender Feed loggt `👋` nicht bei jedem Versuch.

## Offene Fragen

keine

## Notizen

Review TR2.2, Befunde zu `botInput.ts` (Impulse, stummer Feed, Log-Takt).
