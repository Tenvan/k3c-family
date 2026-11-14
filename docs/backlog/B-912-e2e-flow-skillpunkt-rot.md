# B-912 · Der Ende-zu-Ende-Lauf TestFlow scheitert, weil Spieler 2 bis Takt 600 keinen Skill-Punkt findet

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** EP1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`native/k3c-game/e2e` › `TestFlow` (nur ohne `-short`) scheitert mit „Spieler 2 findet einen Skill-Punkt: bis Takt 600 nicht erreicht“. Der Stand `main` 6802a69d9 scheitert genauso (Review EP1, per `git archive` geprüft); keine Folge von EP1.

## Ziel

`TestFlow` ist wieder grün: Ursache gefunden (vermutet: geänderte Welt- oder Biom-Daten, BI1) und Skript bzw. Erwartung angepasst.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Ursache bestimmen (wo liegt der Skill-Punkt, läuft Spieler 2 hin).
- Skript `flowScript` oder Erwartung anpassen, ohne die Stationen zu schwächen.

## Nicht-Ziele

Spielregeln ändern.

## Regeln und Einschränkungen

- **Bereich:** `native/k3c-game/e2e/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`go test ./e2e/ -run TestFlow` im Modul `native/k3c-game` → grün.

## Ausnahme- und Fehlerfälle

nicht relevant: Testkorrektur.

## Akzeptanzkriterien

- **AC-01** `TestFlow` ist grün und zweimal hintereinander gleich (Test).

## Offene Fragen

keine

## Notizen

Gefunden in EP1.1; Spieler 2 ist seit B-893 Pad 1 statt der rechten Tastatur-Hälfte, das ändert die Simulation nicht.
