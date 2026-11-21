# B-1216 · Der Welten-Editor lädt Dateiänderungen nach, zeigt Geschichten, bietet mehr Prompts und zeigt Werte als Baum

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WL2
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ralf 2026-11-21: Der Welten-Editor soll die Welt automatisch neu laden, wenn eine Datei sich ändert (per Editor oder MCP). Geschichten (Start, Inseln, Ende) sollen im Editor anzeigbar und per Prompt änderbar sein, dazu weitere nützliche Prompts für Teile der Welt. Die Werte-Tabelle (1037 Felder in 73 Gruppen) soll als Baum mit Icons und Namen erscheinen, als eigener Dialog oder Tab neben dem Insel-Editor.

## Ziel

Editor bleibt ohne Neuladen aktuell, zeigt die Geschichte und bietet Prompts; Werte als Baum.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Änderung an `data/worlds/<id>.json` (oder den Werte-Dateien) lädt Liste, Detail, Werte und Level der gewählten Welt neu (Dateiwächter im Backend, Ereignis an die Oberfläche).
- **AC-02** Tab/Dialog Geschichte: Start, Insel-Ankunft, -Boss, -Sieg, Etappen, Ende, Flaschenpost, Wortschatz; Prompt-Button je Teil.
- **AC-03** Weitere Prompts (Insel hinzufügen oder ändern, Größe/Tiefen, Biome, geplante Chunks, Skalierung, Werte-Override).
- **AC-04** Werte als Baum mit Icons und Namen als eigener Tab neben dem Insel-Editor.

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

- **AC-01** Frontend-Test mit Mock-Ereignis lädt neu; Go-Test des Wächters. **AC-02** bis **AC-04** Frontend-Tests je Ansicht, `check_run` grün, Abnahme durch Ralf.

## Offene Fragen

keine (Standard: Tab im Welten-Editor, Prompts wie die bestehenden Prompt-Buttons).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
