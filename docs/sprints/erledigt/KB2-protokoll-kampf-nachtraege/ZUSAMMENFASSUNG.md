# KB2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Protokoll-Nachträge Kampf: Inselwechsel, Game Over, Händler, Client-Typen
- **Projekt:** KMP

## Ziel

Server, Protokoll und Client-Typen des Kampfs sind konsistent: Inselwechsel kommt beim Client an, Game Over zerstört keinen Spielstand, das Protokoll beschreibt den Händler-Überfall, Client-Typen passen zu den Daten. Am Ende sichtbar: `task check` und `task check:go` grün, Protokoll-Beispiele ergänzt.

## Ergebnis

2026-10-25, Claude (KB2.5): `task check` und `task check:go` sind grün. Beim ersten Lauf hat gocyclo einmal angeschlagen, in `TestWirtschaftZustand` (wegen KB2.3). Claude hat das behoben. `task check:race` wurde lokal übersprungen, weil kein C-Compiler da ist; `-race` prüft die CI. AC-01, AC-02 und AC-05 sind laut KB2.1, KB2.2 und KB2.6 erfüllt (schon auf develop, #458). AC-03 ist mit KB2.3 erfüllt (Review ohne Befund), AC-04 mit KB2.4. Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `islandSwitch` erreicht beim Inselwechsel jeden Client, mit Test in `engine/room/` (B-385) … |
| AC-02 | geprüft | Der Raum speichert nach „Komplett verloren“ nicht mehr (B-344). |
| AC-03 | geprüft | `docs/protocol.md`, `src/model/types.ts` und das Beispiel beschreiben `merchant.raid` und … |
| AC-04 | geprüft | `src/model/data.ts` kennt alle Felder aus `enemies.json` und `waves.json` ohne `as unknown … |
| AC-05 | geprüft | Go-Tests belegen: Eine Dev-Aktion ist im nächsten `delta` sichtbar, der Gradwechsel wirkt … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-385 | erledigt | AC-01 geprüft | Das Ereignis `islandSwitch` erreicht beim Inselwechsel keinen Client |
| B-344 | erledigt | AC-02 geprüft | Der Raum speichert nach „Komplett verloren“ nicht mehr |
| B-381 | erledigt | AC-03 geprüft | Das Protokoll beschreibt Überfall und HP des Händlers |
| B-386 | erledigt | AC-03 geprüft | Netz-Budget des Kampf-Zustands: Q08 gilt für alles oder nur für Ereignisse? |
| B-324 | erledigt | AC-04 geprüft | Die Client-Typen der Gegner- und Wellendaten passen zu den JSON-Dateien |
| B-080 | erledigt | AC-05 geprüft | Dev-Tasten (Gold, Stufe, Neustart) wirken über den Server |
