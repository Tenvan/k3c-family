# B-107 · Ein Debug-Panel im Dev-Mode wechselt den Schwierigkeitsgrad und weitere Optionen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** K5
- **Projekt:** KMP
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint K5

## Ausgangslage

Das Debug-Overlay (B-093) zeigt nur Werte; Dev-Aktionen laufen nicht über den Server (B-080).

## Ziel

Im Dev-Mode öffnet ein Panel mit Aktionen: Schwierigkeitsgrad (Dev, Leicht, Normal, Hart, Ultra) wechseln, auch im laufenden Raum (wirkt ab der nächsten Welle), später Gold, Stufe, Neustart (B-080). Nutzen: Entwickeln und Testen ohne Neustart.

## Beteiligte und Zielgruppen

Entwickler und 🧑 beim Test.

## Anforderungen

- Panel nur im Dev-Mode (heute Standard, B-098 nimmt das vor dem Release zurück).
- Aktionen laufen über den Server (B-080), das Panel sendet nur.
- Belegung: nicht Taste B, nicht View + Menu, nicht X; im Overlay-Toggle Ö bleibt erhalten.

## Nicht-Ziele

Aufzeichnen, Export der Werte.

## Regeln und Einschränkungen

`CLAUDE.md` (Tasten), `docs/rules/wirtschaft.md` § 4. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Panel öffnen, „Hart“ wählen → ab der nächsten Welle gelten die Hart-Faktoren.

## Ausnahme- und Fehlerfälle

Wechsel ohne Dev-Mode → vom Server abgelehnt; im laufenden Raum mit Dev-Mode erlaubt, wirkt ab der nächsten Welle.

## Akzeptanzkriterien

- **AC-01** Das Panel zeigt und wechselt den Grad über eine Server-Aktion (B-080).
- **AC-02** Das Panel ist ohne Dev-Mode nicht verfügbar (Test).
- **AC-03** 🧑 hat das Panel am Gerät abgenommen.

## Offene Fragen

Tastenbelegung und Gestaltung (🧑).

## Notizen

Aus R1.2. Abhängig von B-080 und B-101.
