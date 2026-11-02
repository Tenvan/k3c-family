# B-683 · Der Studio-Chat kann Welt-Overrides lesen, setzen und zurücksetzen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ37
- **Projekt:** WZG
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31 autonom, Claude (Wunsch 🧑 im Chat)

## Ausgangslage

Der Chat soll jede Bearbeitung des Studios auslösen können (B-679). Welt-Overrides gibt es noch nicht (B-680); sie müssen danach über den Chat bedienbar sein.

## Ziel

Der Chat setzt, ändert und entfernt Overrides einer Welt als Aktionskarte und erklärt Basis gegen Override.

## Beteiligte und Zielgruppen

🧑 Ralf entwirft Welten und balanciert sie; Sessions der Domänen REG, SIM, DEV setzen um.

## Anforderungen

- Aktionskarten für Override setzen/zurücksetzen mit Vorschau Basiswert → neuer Wert.
- Gleiche Prüfung wie Editor und Server.
- Abhängig von B-679 und B-682.

## Nicht-Ziele

Neue Override-Arten (B-680).

## Regeln und Einschränkungen

Determinismus (`engine/rng`), 2 Spieler gleichzeitig, `docs/rules/welten.md`, Datei ≤ 400 Zeilen, Funktion ≤ 60. Zusammenhang: B-680 → B-681 → B-682 → B-683.

## Beispiele

Welt „Nordmeer“ setzt nur `overrides` für die Kosten einer Brücke; alle anderen Werte kommen aus `data/`, die Standard-Welt bleibt unverändert.

## Ausnahme- und Fehlerfälle

Ungültiger Override → Welt wird nicht geladen, Meldung nennt Welt, Datei und Pfad.

## Akzeptanzkriterien

- **AC-01** Go-/Vitest: Chat-Aktion ändert nur das genannte Override-Feld der genannten Welt.
- **AC-02** Ungültiger Wert wird mit Meldung abgelehnt, nichts geschrieben.

## Offene Fragen

keine

## Notizen

–
