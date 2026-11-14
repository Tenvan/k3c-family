# MON1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Metrik-Sammler und /api/metrics
- **Projekt:** –

## Ziel

Der Server sammelt Tick-Dauer, Latenz je Gerät, Warteschlangen, Laufzeitwerte und Ereignisse als Verlauf und gibt sie über `/api/metrics` aus, ohne den Tick spürbar zu belasten. Am Ende sichtbar: `curl -H "Authorization: Bearer …" /api/metrics?since=…` zeigt Deltas, der Benchmark zeigt höchstens 1 % Mehrkosten.

## Ergebnis

2026-10-05 · AC-01 bis AC-07 laut MON1.1 und MON1.2 geprüft (Nachweise dort); `task check` und `task check:go` grün, `-race` prüft die CI. Keine schweren Befunde (MON1.3): Token-Schutz von `/api/metrics` (404/401/405), Ring-Puffer und Ping-Goroutine (endet mit der Verbindung) ohne Leck, Sperrreihenfolge wie bisher (`m.mu` vor `r.mu`, `Monitor.mu` Blatt). Keine neuen Tickets. Version: v0.11.0 vorgeschlagen (Minor: neue Server-API `/api/metrics`; wird DBG3 zuerst als v0.11.0 getaggt, dann v0.12.0).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Ring-Puffer fest begrenzt (`B-281/AC-01`). |
| AC-02 | – | `/api/metrics` mit Delta und Token-Schutz (`B-281/AC-02`). |
| AC-03 | – | Benchmark ≤ 1 % bzw. 20 µs (`B-281/AC-03`). |
| AC-04 | – | Reihen je Gerät getrennt (`B-281/AC-04`). |
| AC-05 | – | Ereignis-Ring (`B-281/AC-05`). |
| AC-06 | – | Endpunkt dokumentiert (`B-281/AC-06`). |
| AC-07 | geprüft | `task check:go` grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-281 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 –, AC-06 – | Der Server sammelt Latenzen, Tick-Dauer und Fehler als Verlauf und liefert sie ü … |
