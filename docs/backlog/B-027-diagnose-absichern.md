# B-027 · Diagnose-Schnittstelle ist abgesichert

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP03
- **Erstellt:** 2026-09-29

## Beschreibung

`/api/status` und die TUI dürfen nicht offen im Netz hängen.

## Warum

Auch im Heimnetz soll nicht jedes Gerät Spieler trennen können.

## Akzeptanz

Zugriff nur mit Token aus einer Umgebungsvariable, Test für 401 ohne Token.

## Notizen

–
