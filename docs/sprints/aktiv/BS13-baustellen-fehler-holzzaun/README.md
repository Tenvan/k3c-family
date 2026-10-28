# BS13 · CLI · Baustellen-Schilder, Bauoptionen und Holzzaun

- **Status:** aktiv
- **Projekt:** BAS
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-28, Chat, durch 🧑, Revision 1 (B-653)

## Ausgangslage

B-653: Schilder an unmarkierten Bäumen/Felsen, Bauoptionen für gesperrte Gebäude, Holzmauer ohne Zaun-Optik.

## Ziel

Schilder und Bauoptionen nur dort, wo gebaut oder abgebaut werden darf; Holzmauer sieht aus wie ein Holzzaun.

## Beteiligte und Zielgruppen

Claude setzt um, 🧑 nimmt die Optik im Browser ab.

## Anforderungen

B-653 › Anforderungen und AC-01 bis AC-03.

## Nicht-Ziele

Plätze und Regeln der Mauern/Türme (BS12).

## Regeln und Einschränkungen

Client zeichnet nur den Server-Zustand; Logik in `engine/sim` mit Test; Grafik über Asset-Studio/Gebäude-Planer; Dateien ≤ 400 Zeilen.

## Beispiele

B-653 › Beispiele.

## Ausnahme- und Fehlerfälle

B-653 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** B-653/AC-01 (Schilder nur nach Markierung).
- **AC-02** B-653/AC-02 (Bauoptionen nur baubar).
- **AC-03** B-653/AC-03 (Holzzaun).

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS13.1 | `BS13.1-schilder-bauoptionen.md` | Umsetzung | autonom | fertig |
| BS13.2 | `BS13.2-holzzaun.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
