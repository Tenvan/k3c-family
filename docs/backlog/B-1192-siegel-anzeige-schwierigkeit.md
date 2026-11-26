# B-1192 · Das Schwierigkeits-Siegel steht in der Auswahl, bei den Spielständen und im HUD bei der Welt

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WK1
- **Projekt:** NAC
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Heute zeigen Auswahl, Spielstände und HUD die Schwierigkeit nur als Text oder gar nicht. Die Siegel-Assets kommen aus B-1191.

## Ziel

Das Siegel des gewählten Grades steht (1) in der Auswahl neben dem Grad, (2) an jedem Spielstand in der Liste und (3) im HUD bei der Welt-Anzeige; in 1 und 2 Ansichten, TV-tauglich.

## Beteiligte und Zielgruppen

🧑 nimmt am PC mit Pad ab.

## Anforderungen

- Anzeige in `menu`, Spielstand-Liste und `hud` (Welt-Zeile); Siegel nur Anzeige, keine neue Eingabe.
- Der Grad liegt schon im Spielstand; fehlt er (alter Stand), gilt der Standardgrad ohne Siegel-Fehler.
- Frame-Budget: Textur einmal laden, kein Allokieren je Bild.

## Nicht-Ziele

Die Siegel selbst (B-1191), Schwierigkeits-Regeln.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`; Frame-Budget.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-game/src/menu/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/local/**`
- **Abhängig von:** B-1191
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Auswahl, Spielstand-Liste und HUD zeigen das Siegel des Grades (Aufnahme je Ansicht).
- **AC-02** Ohne Grad im Spielstand kein Fehler (Test).
- **AC-03** Abnahme 🧑 am PC.

## Offene Fragen

Default: HUD-Siegel links neben dem Welt-Namen, Größe 1 M-Zeilenhöhe; 🧑 ändert bei der Abnahme.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
