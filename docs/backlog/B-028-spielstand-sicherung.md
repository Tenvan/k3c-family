# B-028 · Spielstände werden rotierend gesichert

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP03
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (Pauschalauftrag „beide komplett autonom fertig stellen“; N = 5, /api/health neu, ohne Token Diagnose aus)

## Ausgangslage

Spielstände liegen als einzelne Dateien in `saves/` (`server/saves.mjs`), ohne Sicherung.

## Ziel

Spielstände werden rotierend gesichert. Nutzen: Ein kaputter oder überschriebener Spielstand darf keinen Abend kosten.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Rotierende Sicherungen: die letzten N = 5 überschriebenen Stände je Spielstand bleiben erhalten.
- Im Docker liegen sie in einem Volume.
- Wiederherstellen per TUI oder API, ohne Dateien umzubenennen.

## Nicht-Ziele

Sicherung außerhalb des Servers (Cloud).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

Ein Spielstand wird überschrieben → der vorige Stand wird per API wiederhergestellt.

## Ausnahme- und Fehlerfälle

Schreiben wird unterbrochen → vorhandene Stände bleiben unbeschädigt.

## Akzeptanzkriterien

- **AC-01** Nach N+1 Speichervorgängen liegen genau die letzten N Stände vor (Test).
- **AC-02** Wiederherstellen per API funktioniert (Test); per TUI ab SP10.
- **AC-03** Im Docker liegen die Stände in einem Volume.

## Offene Fragen

keine (N = 5, 🧑 2026-09-30).

## Notizen

Grundlage in SP03, Pi-Betrieb in SP11.
