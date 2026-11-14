# B-872 · Der Web-Client der Workbench ist entfernt, die Workbench läuft nur als EXE

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ57
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ralf, 2026-11-14: Die Web-Clients werden mit der aktuellen Umsetzung (Entscheidung 009, nur natives Spiel) nicht mehr gebraucht. Bisher diente der Web-Client als Ausweich-MCP auf Port 5185, wenn die alte EXE nicht archivieren konnte. Deshalb erst entfernen, wenn die EXE neu gebaut ist (WZ47-Index-Format).

## Ziel

Die Workbench gibt es nur noch als EXE (`bin/k3c-workbench.exe`); Web-Client, `task k3c-workbench:web`, `web:build` und `bin/k3c-workbench-web` sind entfernt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Klasse A (DEV, Schuld, entfernt nur). `-headless` und `-dev` bleiben (Dev-Supervisor, Vite).

- **Bereich:** `tools/k3c-workbench/**`, `Taskfile.yml`, `CLAUDE.md`, `docs/arbeitsweise.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task k3c-workbench:web` und `web:build` sind aus dem Taskfile entfernt, ebenso die Erwähnungen in CLAUDE.md, docs/arbeitsweise.md und der MCP-Anleitung.
- **AC-02** `task buildall` baut keine Web-EXE mehr.
- **AC-03** Kein Code ist nur für den Web-Modus da (z. B. `-open`, Auslieferung der gebauten Oberfläche ohne Fenster); `check:dev` ist grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
