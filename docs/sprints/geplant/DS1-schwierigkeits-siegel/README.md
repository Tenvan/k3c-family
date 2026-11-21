# DS1 · CLI · Schwierigkeits-Siegel: Wachssiegel je Schwierigkeitsgrad in Auswahl, Spielständen und HUD

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-1191, B-1192
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Schwierigkeit zeigt das Spiel nur als Text. 🧑-Idee 2026-11-21: ein Wachssiegel je Grad.

## Ziel

Jeder Schwierigkeitsgrad hat ein Wachssiegel-Symbol; es steht in der Auswahl, bei den Spielständen und im HUD bei der Welt. Am Ende sichtbar am PC mit Pad, in einer und zwei Ansichten.

## Beteiligte und Zielgruppen

🧑 gibt die Spec frei und nimmt am PC ab; Second Worker (Assets, Lane zweit) und Hauptrechner (Anzeige).

## Anforderungen

`B-1191 › Anforderungen`, `B-1192 › Anforderungen`. Sessions (bei Aktivierung als Dateien): DS1.1 Siegel-Assets (Lane zweit, B-1191), DS1.2 Anzeige in Auswahl, Spielständen und HUD (B-1192), DS1.3 Review.

## Nicht-Ziele

Neue Schwierigkeitsgrade oder -Regeln.

## Regeln und Einschränkungen

Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; 2 Spieler; Frame-Budget; Assets nur über `assets_*` (Entscheidung 005); Gen und Bildvergleiche in der Lane zweit.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `public/grafik/icons/**`, `data/paper.json`, `rust/crates/k3c-game/src/menu/**`, `rust/crates/k3c-game/src/hud/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe | Abnahme
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** (B-1191/AC-01, AC-02) Je Grad ein Siegel-Asset, in `data/paper.json` zugeordnet.
- **AC-02** (B-1192/AC-01, AC-02) Auswahl, Spielstand-Liste und HUD zeigen das Siegel; alter Spielstand ohne Grad ohne Fehler.
- **AC-03** Abnahme 🧑 am PC (B-1192/AC-03).

## Offene Fragen

Keine offen. Defaults stehen in B-1191 und B-1192 (Prägung, Farbe, Lage im HUD); 🧑 bestätigt sie bei der Freigabe oder Abnahme.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
