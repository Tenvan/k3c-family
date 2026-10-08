# B-048 · Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verweist

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BT1
- **Projekt:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`B-001/AC-01` verlangt: „Die Entscheidung für Go mit Standardbibliothek (`net/http`) steht in
`docs/decisions/001-server-engine-go.md`.“ Entscheidung 001 nennt Go, Bubble Tea und Wails, aber weder
„Standardbibliothek“ noch `net/http`. Belegt ist die Wahl nur im alten Backlog (`backlog.md` unter `docs/` in `4c2e4d7`:
„Go, Standardbibliothek reicht“). Darauf stützen sich auch der Satz „Standardbibliothek zuerst“ in den SRV-Tickets und
SP03 › Regeln (`net/http`, `log/slog`). Gefunden im Review SP00.5.

## Ziel

Der Nachweis von `B-001/AC-01` (und damit SP00/AC-02) stimmt mit dem Inhalt von Entscheidung 001 überein.

## Beteiligte und Zielgruppen

🧑 entscheidet, ob Entscheidung 001 ergänzt oder B-001 geändert wird.

## Anforderungen

- Entweder nennt Entscheidung 001 die Standardbibliothek (`net/http`), oder `B-001/AC-01` verweist auf die tatsächliche Quelle.

## Nicht-Ziele

Die Server-Entscheidung selbst neu aufrollen.

## Regeln und Einschränkungen

Entscheidungen ändert nur 🧑; ein Kriterium wird nie still umformuliert (`docs/arbeitsweise.md` › SDD, `Revision` + 1).

## Beispiele

Leser prüft `B-001/AC-01` → findet `net/http` in der genannten Datei.

## Ausnahme- und Fehlerfälle

nicht relevant – reine Doku-Korrektur.

## Akzeptanzkriterien

- **AC-01** `B-001/AC-01` ist mit dem Inhalt der Datei belegt, auf die es verweist.

## Offene Fragen

Entscheidung 001 um einen Satz ergänzen oder B-001 auf Revision 2 setzen? (🧑)

## Notizen

–
