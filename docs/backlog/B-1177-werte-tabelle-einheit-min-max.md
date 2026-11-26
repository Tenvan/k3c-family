# B-1177 · Das Backend liefert je Welt-Wert Einheit und Min/Max, damit die Werte-Tabelle Regler und Bereichsprüfung beim Tippen bietet

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WN4
- **Projekt:** WZN
- **Erstellt:** 2026-11-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Werte-Tabelle im Welten-Editor (`frontend/src/world/WorldValues.tsx`, WZ62.7) prüft Eingaben nach Typ (Zahl, Ja/Nein, Text, Liste). Einheit und zulässiger Bereich je Feld kennt sie nicht, weil `worldValues` (`internal/world/values.go`) nur Datei, Pfad, Basiswert und Override liefert. Den Bereich prüft erst der Server beim Schreiben (`docs/rules/welten.md` § 1).

## Ziel

Jedes Feld der Werte-Tabelle trägt Einheit (z. B. Gold, Units, Sekunden) und, wo bekannt, Min und Max. Die Tabelle zeigt die Einheit, bietet Regler plus Zahl und meldet Bereichsverletzungen schon beim Tippen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Metadaten kommen aus einer Quelle in `data/` oder der Engine, nicht aus Heuristik im Frontend.
- Felder ohne Metadaten bleiben wie heute bedienbar.
- Die Prüfung im Backend bleibt maßgeblich.

## Nicht-Ziele

Neue Werte oder Regeln, Balancing.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
