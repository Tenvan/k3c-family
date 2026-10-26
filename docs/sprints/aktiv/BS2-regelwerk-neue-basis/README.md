# BS2 · REG · Regelwerk der neuen Basis: Münzskala, Baumschule, Hafen und Schiff

- **Status:** aktiv
- **Projekt:** BAS
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-635, B-636, B-637, B-638
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑, Revision 1, mit den Defaults der offenen Fragen

## Ausgangslage

Regeln und Glossar kennen Farm und Farm-Weltplatz (±52), Goldpreise bis 300, Inselwechsel per „Boot oder Portal“ nach dem Endboss (`docs/rules/stufen.md` § 1). Die neue Basis (B-635 bis B-638) ist noch nicht beschrieben.

## Ziel

Die neue Basis steht verbindlich im Regelwerk und im Glossar, bevor Werte und Code folgen: Münzskala 2–30 mit Ausnahmen, Baumschule auf gerodeten Waldbauplätzen, Hafen-Chunk auf Inselstufe 1 und Schiff mit Bauabschnitten und Ressourcen.

**Am Ende sichtbar:** `docs/rules/wirtschaft.md`, `materialien-gebaeude.md`, `stufen.md`, `geplante-chunks.md`, `zielkorridore.md` und `docs/glossar.md` tragen die neuen Regeln; „Farm“ kommt in `docs/rules` nicht mehr vor.

## Beteiligte und Zielgruppen

Claude schreibt, 🧑 entscheidet die offenen Fragen der Tickets und gibt frei.

## Anforderungen

B-635 › Anforderungen (Regel-Teil), B-636 › Anforderungen (Begriff, Regel), B-637 › Anforderungen (Regel, Glossar), B-638 › Anforderungen (Schiffskosten als Regel, Zielkorridor).

## Nicht-Ziele

Werte in `data/` (BS3), Code (BS4–BS6).

## Regeln und Einschränkungen

Doku-Sprint, nur `docs/`; keine Review-Session, die letzte Session schließt ab. Neue Begriffe zuerst ins Glossar. Parallel zu BS1.

## Beispiele

Siehe Tickets B-635 bis B-638.

## Ausnahme- und Fehlerfälle

Eine offene Frage bleibt ohne Entscheidung → Default aus dem Ticket eintragen, als „angenommen (Default B-6xx)“ markieren.

## Akzeptanzkriterien

- **AC-01** `wirtschaft.md` beschreibt die Münzskala 2–30 mit Stufen und Ausnahmen; `zielkorridore.md` ist umgerechnet (B-635/AC-01).
- **AC-02** Glossar und `materialien-gebaeude.md` nennen Baumschule und Baumschul-Platz im Wald, freigeschaltet durch Roden; „Farm“ nicht mehr in `docs/rules` (B-636/AC-01).
- **AC-03** `stufen.md`, `geplante-chunks.md`, `welten.md` und Glossar beschreiben Hafen-Chunk, Schiff mit Abschnitten und die Wechselbedingung (B-637/AC-01).
- **AC-04** Schiffskosten je Abschnitt (Münzen und Material) und der Zielkorridor „Schiff fertig“ stehen im Regelwerk (B-638).

## Offene Fragen

Aus den Tickets, je mit Default (nicht blockierend):
- Stufen der Münzskala (B-635), Default 2, 3, 4, 6, 8, 10, 12, 15, 20, 25, 30.
- Bedeutung „durchlaufen“ für Meilenstein 1 (B-635), Default: Endboss der Insel 1 besiegt.
- Anzahl und Lage der Baumschul-Plätze (B-636), Default je Seite 1 hinter Linie 2.
- Hafen-Platz, Schiff-Abschnitte, Schaden durch Gegner, Portal entfällt (B-637), Default −7, 3 Abschnitte, kein Schaden, nur Schiff.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS2.1 | `BS2.1-muenzskala.md` | Umsetzung | autonom | offen |
| BS2.2 | `BS2.2-baumschule-hafen-schiff.md` | Umsetzung | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
