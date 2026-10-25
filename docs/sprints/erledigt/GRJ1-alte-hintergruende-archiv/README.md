# GRJ1 · CLI · Alte Biom-Hintergründe mit Lizenz im Archiv

- **Status:** erledigt
- **Projekt:** GRJ
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-453
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt; Ort: Git-Tag archiv/hintergruende-vor-geb7)

## Ausgangslage

GEB7 (PR #324) löschte fünf Pack-Hintergründe samt Lizenz und Credits; 🧑 fand sie „recht gut“ und will sie aufbewahren. Sie liegen nur noch in der Git-Historie (`git show e0d26d0e^:public/grafik/<pack>/…`, B-453).

## Ziel

Die alten Hintergründe liegen mit Lizenz und Quelle in einem Archiv, damit GRJ sie neben den neuen Schichten vergleichen kann. Am Ende sichtbar: Archiv-Ordner oder Git-Tag, `task check` grün.

## Beteiligte und Zielgruppen

🧑 bei der Grafik-Justierung; Agent in CLI.

## Anforderungen

B-453 › Ziel (Spec noch Entwurf).

## Nicht-Ziele

Rückkehr der alten Hintergründe ins Spiel; Vergleichsseite (später in GRJ).

## Regeln und Einschränkungen

Projekt GRJ ruht: der Sprint wird erst gewählt, wenn 🧑 GRJ aktiviert. Lizenzen CC-BY/CC0 der Packs beachten; kein Verweis aus Spiel, Register und Credits-Tests. Bewusst nur zwei Sessions (kleiner Umfang).

## Beispiele

`forest-background` liegt mit Lizenztext und Quelle im Archiv; `credits.test.ts` und `grafikPacks.test.ts` bleiben grün.

## Ausnahme- und Fehlerfälle

Pack ohne erhaltene Lizenz → nicht archivieren, im Ergebnis nennen.

## Akzeptanzkriterien

- **AC-01** Die fünf alten Hintergründe liegen mit Lizenztext und Quelle am von 🧑 gewählten Ort (oder als Git-Tag); `task check` grün (B-453).

## Offene Fragen

🧑: Archiv im Repo (`docs/funde/` oder `public/grafik-archiv/`) oder nur ein Git-Tag auf `e0d26d0e^`?

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GRJ1.1 | `GRJ1.1-archiv-tag.md` | Umsetzung | autonom | fertig |
| GRJ1.2 | `GRJ1.2-review.md` | Review | autonom | fertig |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GRJ1.1 (CLI) Alte Hintergründe mit Lizenz ins Archiv (AC-01).
- GRJ1.2 (CLI) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

2026-10-25, Claude (GRJ1.2): AC-01 erfüllt durch GRJ1.1 (Tag `archiv/hintergruende-vor-geb7` mit Bildern und Lizenzen auf origin). `task check` grün. Keine neuen Tickets.
