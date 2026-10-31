# B-682 · Der Welten-Editor bearbeitet Werte-Overrides einer Welt und zeigt Basiswert gegen Override

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

Der Welten-Editor des Studios (`tools/k3c-studio/frontend/src/world/`, Entwurf in `worldDraft.ts`) bearbeitet nur Inseln, Tiefen, Größe und Planung; Ressourcen- und Kostenwerte sind nicht pro Welt einstellbar (geprüft 2026-10-31).

## Ziel

Im Editor lässt sich pro Welt jeder Basiswert übersteuern oder auf „geerbt“ zurücksetzen; die Oberfläche zeigt, was geerbt und was überschrieben ist.

## Beteiligte und Zielgruppen

🧑 Ralf entwirft Welten und balanciert sie; Sessions der Domänen REG, SIM, DEV setzen um.

## Anforderungen

- Ansicht „Werte“ je Welt: Basiswert, Override, Status (geerbt/überschrieben), Zurücksetzen je Feld.
- Speichern schreibt nur geänderte Felder nach `overrides` (B-680); Prüfung wie im Server.
- Radix nur über `@webkit`, Atomic Design.
- Änderungen wirken auf Kind-Welten (`extends`, B-680); der Editor listet sie vor dem Speichern.

## Nicht-Ziele

Chat-Anbindung (B-683).

## Regeln und Einschränkungen

Determinismus (`engine/rng`), 2 Spieler gleichzeitig, `docs/rules/welten.md`, Datei ≤ 400 Zeilen, Funktion ≤ 60. Zusammenhang: B-680 → B-681 → B-682 → B-683.

## Beispiele

Welt „Nordmeer“ setzt nur `overrides` für die Kosten einer Brücke; alle anderen Werte kommen aus `data/`, die Standard-Welt bleibt unverändert.

## Ausnahme- und Fehlerfälle

Ungültiger Override → Welt wird nicht geladen, Meldung nennt Welt, Datei und Pfad.

## Akzeptanzkriterien

- **AC-01** Vitest: Override setzen, zurücksetzen, speichern schreibt nur Abweichungen.
- **AC-02** Vitest: geerbt/überschrieben wird korrekt angezeigt.
- **AC-03** Abnahme durch 🧑 im Studio.

## Offene Fragen

keine

## Notizen

–
