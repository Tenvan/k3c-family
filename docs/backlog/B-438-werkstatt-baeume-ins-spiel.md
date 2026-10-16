# B-438 · Werkstatt und Bäume aus tools/gebaeude erscheinen im Spiel

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Sprint GEB1 hat Werkstatt, Tanne, Pappel und Birke mit `tools/gebaeude` erzeugt (`public/grafik/gebaeude/`, Regeln `docs/standards/gebaeude-grafik.md`). Ralf hat sie am 2026-10-16 in 32 und 16 px abgenommen und als Spiel-Dichte 32 px je Kachel ×1 festgelegt. Bisher erscheinen sie nur in der Asset-Vorschau; Slot `workshop` und `node:tree` in `data/assets.json` zeigen weiter die alten Packs.

## Ziel

Im Spiel zeigen die Werkstatt (`workshop`) und die Bäume (`node:tree`, als überlappende Reihe) die neuen Grafiken in 32 px je Kachel ×1; ganzzahlige Skalierung nach GR7.4 (B-251).

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

2026-10-16 (Chat, Ralf): Die Werkstatt (`workshop`) wandert mit den vier Lücken-Gebäuden in Sprint GEB2 (Session GEB2.3, Maßstab 1 im Spiel). Dieses Ticket behält nur die Bäume (`node:tree`) und nutzt den Maßstab-Weg aus GEB2.3.
