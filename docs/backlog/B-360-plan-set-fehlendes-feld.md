# B-360 · plan_set lehnt ein Feld ab, das in der Datei fehlt, statt es still zu übergehen

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ2
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-11, 🧑 im Chat (Sprint WZ2, Revision 2)

## Ausgangslage

`setField` in `tools/k3c-dev/internal/planning/edit.go` ersetzt nur vorhandene Feldzeilen. Fehlt das Feld in einer Datei (z. B. eine ältere Datei ohne `Projekt`), meldet `plan_set` Erfolg und ändert nichts. Aufgefallen in PJ2.1 an einer Test-Datei; die echten Dateien tragen seit PJ1.2 alle Felder.

## Ziel

Eine Änderung, die nicht geschrieben werden kann, wird nie als Erfolg gemeldet.

## Beteiligte und Zielgruppen

Agenten beim Planen über `plan_set`.

## Anforderungen

- Ein Feld der Vorlage, das in der Datei fehlt, wird **abgelehnt**: `plan_set` meldet einen Fehler mit Grund (Feldname und Datei), ändert nichts und meldet nie Erfolg (🧑 2026-10-11).
- Es wird kein Feld eingefügt: die echten Dateien tragen seit PJ1.2 alle Felder, ein fehlendes Feld ist ein Datenfehler und wird nicht still repariert.

## Nicht-Ziele

Dateien migrieren (PJ1.2 erledigt).

## Regeln und Einschränkungen

Domäne SRV (`tools/k3c-dev/`), Komplexitäts-Budget.

## Beispiele

`plan_set B-001 {"Projekt": "GRA"}` auf einer Datei ohne Zeile `Projekt` → Fehler mit Grund oder Feld eingefügt, nie stiller Erfolg.

## Ausnahme- und Fehlerfälle

nicht relevant (das Ticket beschreibt selbst einen Fehlerfall).

## Akzeptanzkriterien

- **AC-01** Go-Test: `Set` auf einer Datei ohne das Feld liefert einen Fehler mit Feldname und Datei, die Datei bleibt unverändert, es gibt keinen stillen Erfolg (`dev:test` grün).

## Offene Fragen

–  (Einfügen oder ablehnen? Entschieden von 🧑 am 2026-10-11: ablehnen.)

## Notizen

–
