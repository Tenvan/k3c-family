# B-350 · Die MCP-Seite zeigt alle Tools mit Aufruf-Statistik und filtert die Statistik nach Zeit

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** M11
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑 („Anzeige aller MCP-Tools … sollte wieder rein“, „Zeitfilter einbauen“)

## Ausgangslage

Die Übersicht der MCP-Seite der Workbench (`tools/k3c-dev/frontend/src/mcp/McpPage.tsx`) zeigt Server-Karten, Live-Monitore und das Aufruf-Protokoll, aber keine Liste aller Tools mehr. Die Statistik (`StatsView.tsx`) kennt nur „Sitzung“ und „All time“. Die Live-Monitore haben schon den Zeitfilter 15 min / 1 h / 24 h / 7 T (`series.ts › RANGES`, `windowOf`), dazu Minuten-Buckets je Tool (`McpUsage.minutes`, `UsageBucket`).

## Ziel

Auf einen Blick sehen, welche Tools es gibt und wie sie genutzt werden; die Statistik zeigt jeden Zeitraum, nicht nur Sitzung oder Gesamtzeit.

## Beteiligte und Zielgruppen

🧑 und Agenten in der Workbench.

## Anforderungen

- Übersicht: Liste **aller** registrierten Tools (auch ohne Aufruf) mit kleiner Statistik je Tool: Aufrufe, Fehler, Ø-Dauer, letzter Aufruf; Beschreibung als Tooltip.
- Statistik: Zeitfilter 15 min / 1 h / 24 h / 7 T (wie die Live-Monitore, gleiche Bedienung) zusätzlich zu Sitzung und All time; Kacheln und Tool-Tabelle rechnen für den gewählten Zeitraum aus den Minuten-Buckets.
- Die Auswahl wird gemerkt (`lib/prefs.ts`).

## Nicht-Ziele

Neue Messgrößen im Go-Server, Export.

## Regeln und Einschränkungen

Domäne SRV (`tools/k3c-dev/`), Frontend React + Radix wie bestehend; Mock-Daten (`api/mockMcp.ts`) für den Browser ohne Wails ergänzen. Datei ≤ 400 Zeilen.

## Beispiele

Filter „1 h“ → die Tabelle zeigt nur Aufrufe der letzten Stunde, z. B. `sim_test` mit 4 Aufrufen, Ø 12 ms.

## Ausnahme- und Fehlerfälle

Keine Aufrufe im Zeitraum → alle Tools mit 0, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Test: Die Übersicht listet alle Tools aus dem Katalog mit Aufrufen, Fehlern, Ø-Dauer und letztem Aufruf, auch Tools ohne Aufruf.
- **AC-02** Test: Der Zeitfilter der Statistik rechnet Kacheln und Tabelle aus den Minuten-Buckets des Zeitraums; Sitzung und All time bleiben.

## Offene Fragen

keine

## Notizen

Anstoß 🧑 2026-10-07 mit Bildschirmfoto des Filters der Live-Monitore.
