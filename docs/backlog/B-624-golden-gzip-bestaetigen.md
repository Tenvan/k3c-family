# B-624 · Golden-Daten als gzip mit lesbaren Diffs bestätigen

- **Domäne:** INF
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** KRT
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GD1.1 (Workshop „Wege wählen“, B-419) war Ralf zugedacht. Laut Ralfs Ziel hat Claude am 2026-10-25 vorentschieden, damit GD1.2 laufen kann.

## Ziel

Ralf bestätigt den Weg oder wählt einen anderen.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

Vorentscheidung (autonom, Claude):
- Die großen Golden-Dateien (`sim-*`, `campaign-*`) liegen gzip-komprimiert (`.json.gz`), Inhalt und `snapshotEvery` bleiben gleich. Damit bleibt die Prüfschärfe voll erhalten.
- Lesbare Diffs über `.gitattributes` mit `diff=gzip` und einem `textconv` (`gzip -dc`); eine Anleitung steht in `docs/arbeitsweise.md` › Golden.
- Die Grenze fällt danach wieder auf 16 MB (oder darunter); B-620 ist damit erledigt.

## Nicht-Ziele

Seltenere Snapshots (weniger Prüfschärfe), Delta-Format.

## Regeln und Einschränkungen

Prüfschärfe unverändert (B-419).

## Beispiele

`git diff` einer Golden-Datei zeigt JSON-Zeilen, sofern `git config diff.gzip.textconv "gzip -dc"` gesetzt ist.

## Ausnahme- und Fehlerfälle

Ohne textconv-Konfiguration zeigt git „Binary files differ“; `task go:golden` gibt dann einen Hinweis.

## Akzeptanzkriterien

- **AC-01** 🧑 hat den Weg bestätigt oder geändert.

## Offene Fragen

- Passt gzip mit textconv? Entscheidet 🧑.

## Notizen

Herkunft: GD1.1, Sprint GD1 (2026-10-25).
