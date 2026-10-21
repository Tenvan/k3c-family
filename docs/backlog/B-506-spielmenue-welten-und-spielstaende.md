# B-506 · Spielmenü zeigt keine Welt und erlaubt kein Anlegen oder Löschen von Spielständen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** BD5
- **Projekt:** BED
- **Erstellt:** 2026-10-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Befund aus der Abnahme WZ18.8 (2026-10-20, 🧑 am PC): Welt anlegen, im Spiel starten und Inselwechsel funktionieren. Im Spielmenü fehlt aber:
- Die gewählte bzw. verfügbare Welt wird nicht angezeigt (kein Weltname, keine Auswahl).
- Spielstände lassen sich dort nicht neu anlegen und nicht löschen.
Ein weiterer Punkt des Berichts war leer; Rückfrage an 🧑 offen.

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

- Welcher vierte Punkt war gemeint (Eintrag in der Meldung leer)? Entscheidet 🧑.
- Welt-Auswahl beim Neuanlegen oder reine Anzeige der Welt je Spielstand? Entscheidet 🧑.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
