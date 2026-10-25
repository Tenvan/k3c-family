# B-620 · Golden-Grenze 20 MB nach dem dichteren Wald bestätigen

- **Domäne:** INF
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** KRT
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB16.5 (B-585) macht den Wald dichter (`forest.tree` [22, 30]). Dadurch wachsen die Golden-Daten von 16,0 auf 18,1 MB und liegen über der Grenze von 16 MB (B-417, Ralf 2026-10-13). Claude hat die Grenze autonom auf 20 MB angehoben, im Test `tests/projectRules.test.ts`.

## Ziel

Ralf bestätigt die Grenze von 20 MB oder wählt einen anderen Weg.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

- Entscheidung: (A) 20 MB bestätigen, Verkleinern später in GD1; (B) zuerst GD1 (Golden kleiner bei gleicher Prüfschärfe), dann die Grenze zurück auf 16 MB.

## Nicht-Ziele

Golden-Dateien jetzt verkleinern.

## Regeln und Einschränkungen

Prüfschärfe der Golden-Läufe bleibt (B-417).

## Beispiele

Default (autonom, Claude): Variante A.

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** 🧑 hat die Grenze bestätigt oder geändert, der Test spiegelt die Entscheidung.

## Offene Fragen

- Bleibt die Grenze bei 20 MB (Variante A)? Entscheidet 🧑.

## Notizen

Herkunft: GEB16.5, Sprint GEB16 (2026-10-25).

Nachtrag (GD1, 2026-10-25): Die Golden-Daten liegen jetzt gzip-komprimiert bei 0,73 MiB, die Grenze steht wieder auf 16 MB. Die Frage nach 20 MB ist damit hinfällig; offen bleibt nur die Bestätigung des Wegs (B-624).
