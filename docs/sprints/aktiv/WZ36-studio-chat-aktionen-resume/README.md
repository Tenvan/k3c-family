# WZ36 · DEV · Studio-Chat: Aktionskarten für jede Bearbeitung, Resume, Welt-Prüfung

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-679, B-687, B-686
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 1)

## Ausgangslage

Der Chat des Asset-Studios läuft mit Aktionskarten für einige Bearbeitungen (B-679 will alle), schickt bei jeder Nachricht den Verlauf neu (B-687) und liefert nur Welten, die die Prüfung bestehen (B-686, am 2026-10-31 umgesetzt, Abnahme offen).

## Ziel

Jede Bearbeitung des Studios lässt sich im Chat anstößen; der Chat setzt die Claude-Sitzung per Resume fort. Am Ende sichtbar: Chat schlägt eine Bearbeitung als Karte vor, Studio prüft und führt sie aus; lange Chats antworten schneller.

## Beteiligte und Zielgruppen

Wer im Studio arbeitet; Agent DEV; 🧑 nimmt B-686 und die Karten ab.

## Anforderungen

`B-679 › Anforderungen`, `B-687 › Anforderungen`, `B-686 › Anforderungen`.

## Nicht-Ziele

Welt-Overrides im Chat (B-683, Sprint WZ37).

## Regeln und Einschränkungen

Studio schreibt `data/**` und `public/` (Entscheidung 004); Aktionen nur nach Prüfung durch das Studio; Port 5196 für Tests, nie 5192/8091.

## Beispiele

Chat: „Gib dem Gebaeude X Stufe 3 diese Grafik“ → Aktionskarte → Studio prüft → Ausführen.

## Ausnahme- und Fehlerfälle

Resume schlägt fehl → Rückfall auf den Verlauf als Text (`B-687`). Ungültige Welt → keine ausführbare Karte (`B-686`).

## Akzeptanzkriterien

- **AC-01** Jede Bearbeitung aus dem Inventar lässt sich als Aktionskarte auslösen (`B-679/AC-01` bis `AC-07`).
- **AC-02** Der Chat setzt die Sitzung per Resume fort, mit Rückfall auf den Verlauf (`B-687/AC-01` bis `AC-05`).
- **AC-03** Kein Weltvorschlag ist ausführbar, wenn die Welt die Prüfung nicht besteht; Abnahme der schon umgesetzten Teile durch 🧑 (`B-686/AC-01` bis `AC-07`).

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ36.1 | `WZ36.1-aktionskarten-alle.md` | Umsetzung | autonom | fertig |
| WZ36.2 | `WZ36.2-resume-rueckfall.md` | Umsetzung | autonom | fertig |
| WZ36.3 | `WZ36.3-review.md` | Review | autonom | fertig |
| WZ36.4 | `WZ36.4-abnahme-pc.md` | Umsetzung | Mensch | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ36.1 (DEV) Aktionskarten für alle Bearbeitungen (AC-01).
- WZ36.2 (DEV) Resume mit Rückfall (AC-02).
- WZ36.3 Review, alle Kriterien, schließt ab.
- WZ36.4 (🧑) Abnahme am PC im Studio (AC-01 bis AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
