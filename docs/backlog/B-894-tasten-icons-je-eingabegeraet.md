# B-894 · Die Oberfläche zeigt Tasten-Hinweise als Icon des Eingabegeräts des jeweiligen Spielers (Tastenkappe oder Xbox-Taste)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 (Defaults der offenen Fragen)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Tasten-Hinweise im Spiel (Menü, Hinweise, F1-Übersicht aus B-888) zeigen heute Text oder nur Xbox-Symbole. Mit Eingabe-Profilen je Spieler (B-893) muss jeder Hinweis zum Gerät des Spielers passen.

## Ziel

Jeder Tasten-Hinweis zeigt das Icon der Taste im Gerät des jeweiligen Spielers: Tastenkappe (z. B. [E], [Leer], [←]) für Tastatur, Xbox-Taste (A/B/X/Y, LB/RB, LT/RT, Steuerkreuz) für Gamepad.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Eine zentrale Funktion Aktion + Profil → Icon; alle Hinweise gehen darüber.
- Icons als Papierschnitt-Assets (Gruppe icons) im gleichen Stil; Tastenkappen mit Beschriftung generiert, Xbox-Tasten in Xbox-Farben (nur Xbox-Symbole für jedes Pad, Plattform-Regel).
- Im Split-Screen zeigt jede Hälfte die Icons ihres Spielers; die F1-Übersicht (UI-Tasten) zeigt Tastenkappen.
- Fehlende Icons als PAP-Ticket.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

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

- AC-01: Für jede Aktion und jedes Profil gibt es ein Icon (Test).
- AC-02: Ein Wechsel des Profils ändert die Hinweise sofort.
- AC-03: Sichtprüfung durch 🧑 (TV und PC).

## Offene Fragen

- Abhängig von B-893 (Profile) und B-888 (F1-Übersicht).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
