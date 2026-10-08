# TR1 · SRV · Testläufe über `sim_test` in der Workbench

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-348
- **Start-Commit:** edb2a8f
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1 („annehmen und sofort als Sprint anlegen und starten“, ein Tool `sim_*` mit Parametern)

## Ausgangslage

Balancing (`task balance`), Lasttest (`task load`) und Einzelläufe (`sim_run`) sind getrennte Werkzeuge mit langer Ausgabe; Agenten starten sie in der Shell und überwachen sie nicht. Ein Lauf mit laufenden Clients, deren Monarchen Bots steuern, fehlt (B-348).

## Ziel

Jeder Testlauf (Balancing, Performance von Server und Client, Stabilität) startet und läuft über ein MCP-Tool `sim_test` der Workbench, in den Modi offline/online und headless/mit 1–4 Clients.

Am Ende sichtbar: `sim_test start` → ID, `sim_test status` → höchstens 10 Zeilen, Bericht unter `reports/simtest-*`.

## Beteiligte und Zielgruppen

Agenten und 🧑 in der Workbench; REG (BR1, BR2) und LT1 nutzen die Läufe. 🧑 gibt frei.

## Anforderungen

B-348 › Anforderungen. Der Client-Anteil (Bot-Eingabe) ist B-349 im Sprint TR2 (PLAT); TR1.3 braucht ihn für den Nachweis im Browser.

## Nicht-Ziele

Bot-Strategien (B-347, BAL5), Wertänderungen (BR1, BR2), Client-Code (TR2), Hardware-Messläufe (LT1.3).

## Regeln und Einschränkungen

Nur `tools/k3c-dev/` (eigenes Go-Modul, importiert `k3c` per `replace`) und Planung/Doku. Fester Katalog wie `check_run` (B-046): keine Shell, keine freien Befehle, Positivlisten für Parameter. Keine neue Abhängigkeit (Browser aus der Installation). Läufe arbeiten im Checkout der Session (`X-K3C-Root`, Ports je Worktree). Datei ≤ 400 Zeilen, Funktion ≤ 60, Logging mit Emojis (🚀 🛑 ✅ ❌ 💥 🐢).

## Beispiele

B-348 › Beispiele.

## Ausnahme- und Fehlerfälle

B-348 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Lauf-Register und Aktionen `start`, `status`, `stop`, `list` (B-348/AC-01).
- **AC-02** Offline headless: Balancing über viele Seeds mit Bewertung der Ziele (B-348/AC-02).
- **AC-03** Online headless: Bot-Geräte gegen den Spielserver, Performance und Stabilität im Status (B-348/AC-03).
- **AC-04** Online mit 1–4 Clients: Clients starten oder anhängen, Bots steuern ihre Monarchen, Client-Kennzahlen im Status (B-348/AC-04).
- **AC-05** Ungültige Parameter und Kombinationen abgelehnt (B-348/AC-05).
- **AC-06** Regel „jeder autonome Testlauf über `sim_test`“ steht in `CLAUDE.md` und `docs/arbeitsweise.md` (B-348/AC-06).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| TR1.1 | `TR1.1-register-offline.md` | Umsetzung | autonom | fertig |
| TR1.2 | `TR1.2-online-headless.md` | Umsetzung | autonom | fertig |
| TR1.3 | `TR1.3-online-clients.md` | Umsetzung | autonom | fertig |
| TR1.4 | `TR1.4-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-07: Kriterien siehe Ergebnisse TR1.1–TR1.3; AC-04 Browser-Nachweis verschoben (B-349/TR2.1, B-351).
Keine schweren Befunde offen (zwei in TR1.3 vor dem Merge behoben). Neue Tickets: B-351, B-352.
Version: v0.15.0 vorgeschlagen (neues Werkzeug `sim_test`), nicht gesetzt (Entscheidung 🧑, 2026-10-07).
