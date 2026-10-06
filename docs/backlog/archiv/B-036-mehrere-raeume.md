# B-036 · Mehrere Spiele laufen gleichzeitig

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** SP07
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (mit SP02); 2026-10-01 🧑 Chat (SP07 Rev. 1: Protokoll v2 vollständig, coder/websocket, B-047 nach M6, B-030 Rev. 3)

## Ausgangslage

Mehrere gleichzeitige Spiele auf einem Server sind Teil von Entscheidung 001; der Go-Server hat noch keine Räume.

## Ziel

Mehrere Spiele laufen gleichzeitig. Nutzen: Die ganze Familie soll parallel spielen können.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Mehrere Räume laufen gleichzeitig auf einem Server.
- Die Räume beeinflussen sich nicht.

## Nicht-Ziele

Bedienung (Lobby, B-037).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Im Kern von Anfang an (Entscheidung 001).

## Beispiele

Ein 2er-Spiel auf der Xbox und ein 3er-Spiel per Handy laufen gleichzeitig.

## Ausnahme- und Fehlerfälle

Ein Raum stürzt ab → die anderen laufen weiter.

## Akzeptanzkriterien

- **AC-01** Ein Test lässt 3 Räume parallel laufen.
- **AC-02** Die Räume beeinflussen sich im Test nicht.

## Offene Fragen

keine

## Notizen

Im Kern von Anfang an (Entscheidung 001). Entwurf SP02, Umsetzung SP07.

Erledigt in SP07 (2026-10-01).
