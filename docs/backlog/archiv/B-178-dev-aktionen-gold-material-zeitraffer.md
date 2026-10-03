# B-178 · Im Dev-Mode lassen sich Gold und Material droppen und die Zeit beschleunigen

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** DBG1
- **Erstellt:** 2026-10-03
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint DBG1

## Ausgangslage

Der Dev-Mode (`K3C_DEV`, `Room.Dev`, Grad `dev`, SP14) kennt keine Aktionen im laufenden Raum. B-080 (Dev-Tasten über den Server) und B-107 (Debug-Panel) sind offen. Zum Testen von Bau, Wirtschaft und Nächten fehlen Gold und Material, und die Simulation läuft fest mit 30 Hz in Echtzeit (`engine/room/run.go`, `StepIsland` mit 1/`TickHz`); ein Tag dauert real mehrere Minuten. 🧑 wünscht am 2026-10-03: Gold und Material per Debug-Overlay droppen, dazu eine Zeitraffer-Funktion, vorgezogen für die Tests.

## Ziel

Ein Gerät in einem Dev-Mode-Raum kann (1) Gold für einen Spieler droppen, (2) Material in den Insel-Vorrat legen und (3) die Zeit des Raums beschleunigen. Nutzen: Mechaniken, Wirtschaft und Nächte lassen sich in Sekunden statt Minuten ausprobieren.

## Beteiligte und Zielgruppen

Entwickler und 🧑 beim Testen am PC, am Handy und am TV; Domäne SRV (Raum, Protokoll). Client-Bedienung: B-179.

## Anforderungen

- **Protokoll:** neue Client-Nachricht `dev` (`docs/protocol.md`, `engine/net/`, `src/online/protocol.ts` bei beiden Enden in einer eigenen Session): `{ "t": "dev", "action": "gold" | "material" | "timescale", … }`. Der Server nimmt sie **nur an, wenn der Raum im Dev-Mode läuft** (`Room.Dev`); sonst Fehlercode `forbidden` (neu) und eine Warnung im Log. Älteren Servern bleibt sie unbekannt (`bad_request`), eine Protokollversion ändert sich nicht.
- **Gold droppen:** Aktion `gold` mit `slot` (lokaler Spieler) und `amount` (Vorschlag 10, 50, 100): Münzen fallen am Spieler zu Boden und werden wie normale Münzen aufgehoben oder bezahlt (prüft Aufheben und Bezahlen mit).
- **Material:** Aktion `material` mit `resource` (wood, stone, copper, iron, crystal) und `amount`: legt Material in den Vorrat der Insel des Spielers, begrenzt durch das Lager-Maximum (`addStockCapped`).
- **Zeitraffer:** Aktion `timescale` mit `factor` 1, 2, 4 oder 8 (Standard 1): Der Raum rechnet je Tick `factor` Simulationsschritte (je 1/30 s), bleibt dabei deterministisch (gleicher Zustand wie `factor` normale Ticks) und gilt für den ganzen Raum; der Faktor ist im Zustand sichtbar (Feld im Snapshot, nur im Dev-Mode) und im Server-Log. Tick-Dauer wächst mit dem Faktor; Obergrenze 8 schützt den Server.
- Jede Dev-Aktion steht im Server-Log (Info, Gerät, Aktion, Werte, Raum).
- Der Zeitraffer endet, wenn das letzte Gerät den Raum verlässt oder der Raum pausiert.

## Nicht-Ziele

Bedienung im Client (B-179), Stufenwechsel und Raum-Neustart (B-080), Wechsel des Schwierigkeitsgrads (B-107), Dev-Aktionen außerhalb des Dev-Mode, neue Spielregeln.

## Regeln und Einschränkungen

B-098: Vor dem Release ist der Dev-Mode wieder aus (`K3C_DEV=0`); die Nachricht ist dann nicht nutzbar. Deterministisch, 2+ Spieler, Datei ≤ 400 Zeilen, Funktion ≤ 60. Protokolländerungen sind eine eigene Session (`docs/arbeitsweise.md`).

## Beispiele

Dev-Raum, Spieler 1 sendet `gold` 50 → 50 Gold fallen als Münzen vor seine Füße und er hebt sie auf. `timescale` 8 → eine Nacht vergeht in etwa einem Achtel der Zeit.

## Ausnahme- und Fehlerfälle

Raum ohne Dev-Mode → `forbidden`. Unbekannter Slot, Material oder Faktor → `bad_request`. Material über dem Lager-Maximum → der Rest wird verworfen (kein Fehler). Zeitraffer 8 mit vielen Gegnern überschreitet das Tick-Budget → der Server verlangsamt (Ticks laufen später), rechnet aber weiter korrekt; das steht im Log.

## Akzeptanzkriterien

- **AC-01** Test: `dev` ohne Dev-Mode wird mit `forbidden` abgelehnt und im Log gewarnt.
- **AC-02** Test: `gold` lässt Münzen am Spieler fallen, die aufgehoben werden können; ungültiger Slot ergibt `bad_request`.
- **AC-03** Test: `material` erhöht den Insel-Vorrat, begrenzt durch das Lager-Maximum.
- **AC-04** Test: `timescale` mit Faktor 4 ergibt nach einem Tick denselben Zustand wie 4 normale Ticks (gleiches Seed, gleiche Eingaben); Faktor 1 ist der Normalfall.
- **AC-05** `docs/protocol.md` beschreibt die Nachricht, `testdata/protocol/` hat Beispiele, beide Enden parsen sie.
- **AC-06** Dev-Aktionen stehen im Server-Log; `task check` und `task check:go` sind grün.

## Offene Fragen

Vorschlag in diesem Ticket: Dev-Aktionen als WebSocket-Nachricht, nur im Dev-Mode des Raums (keine Anmeldung nötig, da der Dev-Mode vor dem Release aus ist). Alternative aus B-080: geschützter HTTP-Aufruf mit Status-Token. 🧑 bestätigt den Vorschlag mit der Freigabe.

## Notizen

Anlass: Wunsch von 🧑 am 2026-10-03. Ergänzt B-080 (dort bleiben Stufenwechsel und Neustart).
