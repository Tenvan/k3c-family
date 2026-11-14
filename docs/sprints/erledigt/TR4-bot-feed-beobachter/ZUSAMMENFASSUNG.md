# TR4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Bot-Eingabe ohne verlorene Drücke, Beobachter ohne Platz im Raum
- **Projekt:** TST

## Ziel

Testläufe mit Bots verlieren keine Eingaben und lassen sich als Beobachter an echte Clients hängen. Am Ende sichtbar: Test der Bot-Eingabe grün, `sim_test` mit Clients hängt sich ohne Platz an.

## Ergebnis

2026-10-25, Claude (TR4.3): `task check`, `task check:go` und `dev:test` (Workbench) grün; `check:race` lokal übersprungen (kein C-Compiler), die CI prüft es. AC-01 erfüllt durch TR4.1 (Bot-Feed behält kurze Drücke, hält bei stummem Feed an), AC-02 durch TR4.2 (Beobachter ohne Platz, `sim_test` bis 4 Spieler). Review: Der Befund HOCH (Dev-Aktionen durch Beobachter) und der Befund NIEDRIG (Frist EmptyFor) sind behoben, mit Test. Offen: Bot-Feed im LAN (B-352, Frage an Ralf, kein Kriterium des Sprints). Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Die Bot-Eingabe liefert auch Drücke, die zwischen zwei Frames kommen und gehen, und hält b … |
| AC-02 | geprüft | Ein Beobachter-Gerät kann ohne Platz angehängt werden; `sim_test` mit Clients erlaubt `pla … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-354 | erledigt | AC-01 erfüllt | Die Bot-Eingabe verliert keine kurzen Drücke und hält bei stummem Feed an |
| B-352 | verworfen | AC-02 geprüft | `sim_test` hängt sich an Clients auf der Xbox an, ohne einen Platz im Raum zu be … |
