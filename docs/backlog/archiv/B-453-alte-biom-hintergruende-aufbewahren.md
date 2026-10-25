# B-453 · Alte Biom-Hintergründe (Packs) als Grafik aufbewahren

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** GRJ1
- **Projekt:** GRJ
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB7 (PR #324) löschte die Pack-Hintergründe `forest-background`, `blue-cave-background`, `warped-super-grotto-escape-pack`, `warped-caves-pixel-art-pack` und `sunnyland-tall-forest-environment/ebenen` samt Lizenz, Credits und Texten (Aufräum-Regel). 🧑 fand sie „recht gut“ und will sie als Grafik aufbewahren. Stand heute nur in der Git-Historie: `git show e0d26d0e^:public/grafik/<pack>/…`.

## Ziel

Die alten Hintergründe liegen mit Lizenztext und Quelle in einem Archiv-Ordner (Vorschlag `docs/funde/` oder `public/grafik-archiv/`, ohne Verweis aus Spiel, Register und Credits-Tests), damit GRJ sie neben den neuen Schichten vergleichen kann.

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

Ort des Archivs (im Repo, Lizenz CC-BY/CC0 der Packs beachten, `credits.test.ts` verlangt Einträge nur unter `public/grafik/`) oder nur ein Git-Tag auf `e0d26d0e^`? Entscheidet 🧑.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
