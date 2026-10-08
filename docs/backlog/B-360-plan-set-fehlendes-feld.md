# B-360 · plan_set lehnt ein Feld ab, das in der Datei fehlt, statt es still zu übergehen

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`setField` in `tools/k3c-dev/internal/planning/edit.go` ersetzt nur vorhandene Feldzeilen. Fehlt das Feld in einer Datei (z. B. eine ältere Datei ohne `Projekt`), meldet `plan_set` Erfolg und ändert nichts. Aufgefallen in PJ2.1 an einer Test-Datei; die echten Dateien tragen seit PJ1.2 alle Felder.

## Ziel

Eine Änderung, die nicht geschrieben werden kann, wird nie als Erfolg gemeldet.

## Beteiligte und Zielgruppen

Agenten beim Planen über `plan_set`.

## Anforderungen

- Ein Feld der Vorlage, das in der Datei fehlt, wird an der Stelle der Vorlage eingefügt oder mit Grund abgelehnt (Entscheidung beim Einplanen).

## Nicht-Ziele

Dateien migrieren (PJ1.2 erledigt).

## Regeln und Einschränkungen

Domäne SRV (`tools/k3c-dev/`), Komplexitäts-Budget.

## Beispiele

`plan_set B-001 {"Projekt": "GRA"}` auf einer Datei ohne Zeile `Projekt` → Fehler mit Grund oder Feld eingefügt, nie stiller Erfolg.

## Ausnahme- und Fehlerfälle

nicht relevant (das Ticket beschreibt selbst einen Fehlerfall).

## Akzeptanzkriterien

- **AC-01** Go-Test: `Set` auf einer Datei ohne das Feld meldet keinen stillen Erfolg (`dev:test` grün).

## Offene Fragen

Einfügen oder ablehnen? Entscheidet 🧑 beim Einplanen.

## Notizen

–
