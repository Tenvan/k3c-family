# LP1.2 · Nachweis „Neues Spiel“ und Seiten im Browser-Pane

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** lp1/2-neues-spiel-nachweis
- **Abhängig von:** LP1.1
- **Tickets:** B-292, B-335
- **Kriterien:** AC-02, AC-05

## Ziel

Nachweis im Browser-Pane: „Neues Spiel“ startet bei vorhandenem Stand `familie` (B-292) und jede Kachel der Entwicklerseite öffnet ihre Seite.

## Kontext

B-292 ist im Code umgesetzt (Commit `f07d6fae`, PR #153): `newGameHref()` in `src/landing/pages.ts`, Test in `pages.test.ts` (B-292/AC-01, AC-03). Offen ist B-292/AC-02 im Browser: Server mit vorhandenem Spielstand `familie` (`saves/familie*`, sonst einmal über „Weiterspielen“ anlegen). Dienste über k3c-dev (`svc_start` Vite und Spielserver), Logs über `logs_query` (`🏰 Raum erstellt`, kein `save_exists`). Browser-Pane-Prüfungen sind für diesen Sprint von 🧑 freigegeben (Freigabe der Spec 2026-10-07).

## Erlaubte Dateien

- Planungsdateien (Ergebnis); bei Befund nur Ticket, kein Code

## Nicht-Ziele

Code ändern; Server sichert beim `fresh`-Start (B-292 › Nicht-Ziele).

## Schritte

1. Dienste starten, prüfen, dass ein Stand `familie` existiert (`saves_list`).
2. Browser-Pane: Landingpage → „Neues Spiel“ → Lobby ohne Fehlerhinweis → „Spielen“; Log `🏰 Raum erstellt`, kein `save_exists`; `familie` unverändert.
3. Landingpage → „Entwicklung“ → jede Kachel öffnen und zurück (Home); `dm.html` öffnet in neuem Tab; Balancing zeigt „noch keine Aufrufe“.
4. Ergebnis mit Nachweis je Kriterium eintragen.

## Fertig, wenn

- [ ] AC-05: B-292/AC-01 und AC-03 per `task test -- pages`, AC-02 im Browser-Pane mit Log-Zeile belegt.
- [ ] AC-02: Jede Kachel der Entwicklerseite im Browser-Pane geöffnet.

## Prüfen

```bash
task test -- pages
```

Browser-Pane laut Schritte (freigegeben für diesen Sprint).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
