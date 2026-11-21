# KS1 · REG · Kampfregeln K3C festschreiben (Portale, Angriff, Ausrüstung, Krone)

- **Status:** erledigt
- **Projekt:** KSY
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Kampfsystem nach K2C, Vorgaben und Interview 🧑 2026-11-15 in B-926 bis B-933.

## Ziel

Verbindliche Kampfregeln in `docs/rules/kampf.md`, übrige Regeltexte angeglichen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `pfad/**` (Vereinigung der Erlaubten Dateien der Sessions ohne Sammeldateien; ein Sprint = eine Dateimenge)
- **Abhängig von:** – (nur IDs: SP00, B-000, SP00.1; erst starten, wenn erledigt und gemergt)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; Abnahmen am Gerät sind keine Abhängigkeit)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `docs/rules/kampf.md` vollständig, übrige Regeln angeglichen (`B-926/AC-01`, `AC-02`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KS1.1 | `KS1.1-kampfregeln.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-21 (Claude, Klasse A, nachträgliche Prüfung): AC-01 geprüft: `docs/rules/kampf.md` vollständig (§ 1 bis § 9, 126 Zeilen), 33 Verweise aus den übrigen Regeln; B-926 bis B-933 archiviert.
Keine Sim, Balance, Gen (Doku-Sprint), daher kein Abnahme-Ticket. Die Session-Datei KS1.1 enthält keinen Ergebnis-Text, der Nachweis stammt aus dem Repo.
