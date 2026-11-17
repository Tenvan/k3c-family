# B-1033 · Abnahme-Läufe Sprint SI4 Reittiere je Welt sind grün

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** SI4
- **Projekt:** KMP
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Was ist heute so? Problem oder Idee in 1–5 Sätzen, mit Dateipfaden, wenn es um Code geht.

## Ziel

Die Sim-, Balance-, Gen- und Race-Läufe zu Sprint SI4 sind gefahren (B-742).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

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

- **AC-01** Sim: entfällt, solange Sim gesperrt ist (Beschluss 2026-11-11) und SI4 keine Engine-Änderung trägt.
- **AC-02** Balance: entfällt aus demselben Grund.
- **AC-03** Gen: `task gen:paper` grün, Grafik-Golden für `reittiere/kamel`, `objekte/gatter` und `figuren/monarch` (neue Animationen `*-kamel`) bestätigt.
- **AC-04** Race: `check_run task:check:race` grün oder mangels gcc als übersprungen genannt.
- **AC-05** `task:check:go` grün nach Merge von #673 (B-1030); weicht der Detvec-Baum dann wegen `objekte.gatter` ab, mit `-write` neu schreiben und begründen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
