# B-743 · Online-Bots über k3c-sim bot erholen sich nicht von einem toten Bot-Prozess

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** RST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Review RS2.6: `tools/k3c-workbench/internal/simexe/bot.go` (`Bot.fail`) setzt `b.err` dauerhaft, `simexe/pool.go` (`Pool.Decide`) legt den Bot danach wieder in den Pool, ohne Neustart. Stirbt oder hängt ein `k3c-sim bot`-Prozess einmal (Zeitlimit 30 s, dann Kill), scheitert jede weitere Anfrage auf diesem Platz; Geräte stehen teils still, der Fehler zählt nur in `Errors`. Dazu ein seltenes Rennen: feuert der `AfterFunc`-Timer nach erfolgreichem `Scan`, wird ein gesunder Bot getötet. `decide` läuft synchron in der Lese-Schleife des Geräts (`botdev/device.go` `play`) und blockiert bei mehr Geräten als Kernen (vermutet, nicht gemessen).

## Ziel

Ein einzelner Bot-Ausfall verfälscht einen `sim_test`-Lauf mit Clients nicht stumm: der Bot startet neu oder der Lauf wird rot.

## Beteiligte und Zielgruppen

Agenten und Ralf (🧑) bei `sim_test` online mit Clients.

## Anforderungen

- Fehlerhafter Bot wird aus dem Pool genommen und neu gestartet (oder Lauf `fehlgeschlagen` mit Grund).
- Timer-Rennen in `Decide` beseitigt (Ergebnis nach erfolgreichem Scan zählt).
- `Errors > 0` sichtbar im Status/Bericht.

## Nicht-Ziele

Bot-Profile, Protokoll.

## Regeln und Einschränkungen

B-726, Entscheidung 006; Funktion ≤ 60 Zeilen.

## Beispiele

Bot-Prozess wird während des Laufs beendet → nächste Anfrage geht an einen neu gestarteten Prozess, Status nennt einen Neustart.

## Ausnahme- und Fehlerfälle

Neustart scheitert → Lauf `fehlgeschlagen` mit stderr-Zeilen.

## Akzeptanzkriterien

- **AC-01** Test in `internal/simexe`: Bot-Prozess stirbt, `Pool.Decide` liefert danach wieder Antworten (Neustart) oder einen eindeutigen Lauf-Fehler.
- **AC-02** Test: Antwort kurz vor Ablauf des Zeitlimits lässt den Bot gesund.

## Offene Fragen

1. Neustart oder Lauf abbrechen? Default: Neustart, höchstens 3 je Platz, danach Lauf rot.

## Notizen

Aus Review RS2.6 (Sonnet-Reviewer), Befunde mittel 1, 2, 4.
