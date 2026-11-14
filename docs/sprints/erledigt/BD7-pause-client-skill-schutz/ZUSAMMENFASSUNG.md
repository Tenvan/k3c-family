# BD7 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Pause-Client, Skill-Reset beim Tod, Schutz wartender Monarchen
- **Projekt:** BED

## Ziel

Das Pause-Menü pausiert das Gerät wirklich am Server, getrennte Monarchen sind geschützt und ein Skill-Druck nach dem Respawn meldet wieder `castFailed`. Am Ende sichtbar: `task check` und `task check:go` grün, Server-Log zeigt beim Öffnen des Menüs „🚦 Gerät pausiert“.

## Ergebnis

2026-10-22 (BD7.4, autonom): AC-01 bis AC-05 umgesetzt und geprüft (Ergebnisse BD7.1 bis BD7.3); Pause-Ablauf zusätzlich im Browser-Pane belegt (BD7.4). `task check` und `task check:go` grün, `task check:race` lokal übersprungen (kein C-Compiler, prüft die CI). Sprint-Review (Sonnet): abgenommen, keine schweren Befunde. Neues Ticket: B-535 (Entscheidung zur Prüfung durch 🧑: erneutes `pause` nach Wiederverbinden). Version: v0.21.0 gemeinsam mit BD3 als ein Release, kein eigenes v0.21.1 (Entscheidung 🧑 2026-10-23, B-536/B-567); der Tag wird erst beim Release durch 🧑 gesetzt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Sim-Test: halten → `castFailed`, sterben, wiederbeleben, neu drücken → zweites `castFailed … |
| AC-02 | – | Go-Test: Ein wartender Monarch verliert während der Frist keine HP (B-530/AC-01). |
| AC-03 | – | Go-Test: Ein freier Monarch ist kein Ziel der Gegner und nimmt keinen Flächenschaden; bese … |
| AC-04 | – | Vitest: Die reine Funktion liefert beim Öffnen genau ein `pause`, beim Schließen genau ein … |
| AC-05 | – | `GameScene` meldet den Zustand der Optionen-Szene je Frame an den Client; `tests/noSim.tes … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-524 | erledigt | AC-01 – | skillHeld beim Tod zurücksetzen, damit der erste Druck nach dem Respawn castFail … |
| B-530 | erledigt | AC-02 –, AC-03 – | Wartende Monarchen nach Verbindungsverlust sind in der Simulation nicht unverwun … |
