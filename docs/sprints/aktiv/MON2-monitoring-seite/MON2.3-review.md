# MON2.3 · Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** mon2/3-review
- **Abhängig von:** MON2.2
- **Tickets:** B-282
- **Kriterien:** alle

## Ziel

Sprint MON2 ist nach `docs/arbeitsweise.md` › Review-Session abgenommen, der eine PR des Sprints ist offen.

## Kontext

Diff `origin/develop...origin/sprint/mon2`. AC-05 ist eine Hardware-Abnahme (MON2.4) und keine Abhängigkeit des Reviews:
als `angenommen, Validierung offen (MON2.4)` führen. Ist AC-04 aus MON2.2 noch ohne Nachweis, mit Freigabe 🧑 im Browser-Pane
nachholen oder als offen führen.

## Erlaubte Dateien

- Dateien der Domäne PLAT aus MON2.1 und MON2.2 (nur schwere Befunde)
- Planungsdateien

## Nicht-Ziele

Stil, Benennung, Vereinfachungen; Änderungen am Server.

## Schritte

1. `task check` grün.
2. Diff lesen, nur schwere Befunde nach `docs/arbeitsweise.md` beheben oder als Ticket anlegen.
3. Abnahme in der Sprint-README, Ordner nach `sprints/erledigt/`, Fahrplan (MON2.4 unter „Offen am Gerät“), Versionsvorschlag.
4. Commit, `git merge origin/develop`, push, PR des Sprints öffnen.

## Fertig, wenn

- [x] Alle Kriterien mit Nachweis oder `angenommen`/`verschoben` in der Abnahme.
- [x] `task check` grün, PR offen.

## Prüfen

```bash
task check
```

## Ergebnis

- `task check` und `task check:go` grün (2026-10-05, Agent, Shell im Worktree).
- Diff `origin/develop...origin/sprint/mon2` gelesen: keine schweren Befunde. Server-Daten (Raum-Codes, Ereignistexte) nur per `textContent`, kein `innerHTML`; Token nur in `localStorage` (Passwortfeld, nie URL oder Anzeige) und Authorization-Header; `installPageChrome()` und `pages.ts` vorhanden, kein `requestFullscreen`, kein `location`-Wechsel, B und View + Menu frei, kein `Math.random`. Keine Ticket-Anlage nötig.
- AC-01 bis AC-03 laut MON2.1/MON2.2. AC-04 im Browser-Pane (Hauptagent, von 🧑 freigegeben, 2026-10-05): Build `sprint/mon2`, Server mit Token auf 127.0.0.1:8090, `k3c-load` mit 2 Räumen × 3 Bots. Token-Eingabe funktioniert; Übersicht zeigt beide Räume mit Ampel (rot, Tick p99 ca. 44 bis 47 ms am Dev-PC unter Last), RTT, Fehlerzahl, Server-Heap; Ereignisse listet 🐢-langsame Ticks je Raum; Verlauf zeigt p50/p95/p99/Max für Tick und RTT, „Budget überschritten“, Verläufe Tick-Dauer und RTT je Gerät mit Ausreißer-Punkten. Keine Konsolenfehler; bei 375 px kein waagerechtes Scrollen (scrollWidth = 375). Main-Thread-Last je 30 s (Timer-Verzögerung, Grundverzögerung abgezogen): 2,35 % bei Fenster 5 min, 2,37 % bei 1 h, also unter 5 %; Einschränkung: das 1-h-Fenster enthielt nur etwa 3 min Daten. Nebenbefund (kein Fehler der Seite): bei 280 px Breite scrollt die Perzentil-Tabelle waagerecht.
- AC-05 (Handy, MON2.4, Agent Mensch): angenommen, Validierung offen; Fahrplan „Offen am Gerät“.
- Version: v0.13.0 vorgeschlagen (DBG3 und MON1 schlagen ebenfalls die nächste Minor vor).
