# B-512 · k3c-dev beendet einen übernommenen Spielserver auch nach dessen Selbst-Neustart (neue PID am Port, Supervisor-Prozess)

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ25
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit TS1.4 (B-507) läuft `k3c-server` als Supervisor mit einem Kindprozess, der am Port lauscht; bei einem Neustart wegen geänderter Dateien bekommt das Kind eine neue PID. `stopAdopted` in `tools/k3c-dev/internal/services/adopt.go` beendet nur die PID, die bei der Übernahme am Port lauschte: Nach einem Neustart meldet es „am Port lauscht jetzt PID X statt Y; nichts beendet“. Ohne Neustart beendet es das Kind (Kill nach längerer Laufzeit), der Supervisor reicht den Code dann nur weiter und endet (geprüft in TS1.4). Gestartete Dienste (Job-Objekt) sind nicht betroffen. Idee: beim Stoppen den Prozessbaum ab dem Elternprozess des Lauschenden beenden, wenn dieser `k3c-server` ist, oder die PID am Port neu übernehmen.

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
