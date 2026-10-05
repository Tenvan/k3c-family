# MON1 · SRV · Metrik-Sammler und /api/metrics

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-281
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑 (umfasst B-281 Revision 1)

## Ausgangslage

`/api/status` liefert nur Momentaufnahmen (Tick last/p99, `failures`, CPU, Heap). Verläufe und eine Fehler-Zeitleiste fehlen. Details: B-281.

## Ziel

Der Server sammelt Tick-Dauer, Latenz je Gerät, Warteschlangen, Laufzeitwerte und Ereignisse als Verlauf und gibt sie über `/api/metrics` aus, ohne den Tick spürbar zu belasten. Am Ende sichtbar: `curl -H "Authorization: Bearer …" /api/metrics?since=…` zeigt Deltas, der Benchmark zeigt höchstens 1 % Mehrkosten.

## Beteiligte und Zielgruppen

Autonome Umsetzung; 🧑 entscheidet das Zeitfenster (Offene Fragen) und gibt die Spec frei. Nutzer der Daten: MON2 (Seite), TUI, k3c-dev.

## Anforderungen

`B-281 › Anforderungen`.

## Nicht-Ziele

`B-281 › Nicht-Ziele`; die Seite kommt in MON2 (B-282).

## Regeln und Einschränkungen

`B-281 › Regeln und Einschränkungen`. Nur `engine/room/`, `engine/net/` (ohne `protocol*.go`, kein Protokoll-Wechsel nötig) und Doku. Läuft nach S2, weil S2 dieselben Dateien in `engine/net/` und `engine/room/` ändert.

## Beispiele

`B-281 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-281 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Ring-Puffer fest begrenzt (`B-281/AC-01`).
- **AC-02** `/api/metrics` mit Delta und Token-Schutz (`B-281/AC-02`).
- **AC-03** Benchmark ≤ 1 % bzw. 20 µs (`B-281/AC-03`).
- **AC-04** Reihen je Gerät getrennt (`B-281/AC-04`).
- **AC-05** Ereignis-Ring (`B-281/AC-05`).
- **AC-06** Endpunkt dokumentiert (`B-281/AC-06`).
- **AC-07** `task check:go` grün.

## Offene Fragen

`B-281 › Offene Fragen` (Zeitfenster entschieden: 1 h bei 1 s; RTT-Quelle klärt MON1.1).

## Sessions

Reife Entwurf, Stichpunkte:

- **MON1.1** Sammler: Ring-Puffer, 1-s-Ticker, Zähler im Tick, RTT- und Warteschlangen-Reihen je Gerät, Benchmark (AC-01, AC-03, AC-04).
- **MON1.2** Ereignis-Ring und `GET /api/metrics?since=` mit Token, Doku (AC-02, AC-05, AC-06).
- **MON1.3** Review (AC-07, alle Kriterien gegenprüfen).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
