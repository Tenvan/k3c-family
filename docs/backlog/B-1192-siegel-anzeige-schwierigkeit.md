# B-1192 · Das Schwierigkeits-Siegel steht in der Auswahl, bei den Spielständen und im HUD bei der Welt

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** DS1
- **Projekt:** PAP
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
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

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

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
