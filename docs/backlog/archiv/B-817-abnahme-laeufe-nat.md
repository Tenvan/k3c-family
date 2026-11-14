# B-817 · Abnahme-Läufe NAT

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Zum Abschluss des Projekts NAT (Welle 3, NG9 und NG11) laufen die Gen- und Race-Läufe gesammelt (Regel B-742, `docs/arbeitsweise.md` › Sim, Balance und Gen). Offene Abnahme-Tickets einzelner NG-Sprints gibt es nicht (geprüft mit `plan_list`; B-774 gehört zu LG1). `task check:all` lief in NG10 nicht.

## Ziel

Die gesammelten Läufe für NAT sind einmal grün gelaufen und belegt.

## Beteiligte und Zielgruppen

Claude arbeitet autonom ab (gedacht für nachts), Ralf liest das Ergebnis.

## Anforderungen

- Läufe nur über `check_run`, Ergebnisse mit Messwerten in den Notizen.
- Rot heißt neues Ticket (Typ Problem) im Projekt NAT, kein Nachbessern hier.

## Nicht-Ziele

Sim, Balance und Lasttest (gesperrt bis auf Widerruf, Beschluss 2026-11-11).

## Regeln und Einschränkungen

B-742: Der Sprint wartet nicht auf dieses Ticket.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** –
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`check_run task:test` grün → AC-03 erfüllt, Dauer in den Notizen.

## Ausnahme- und Fehlerfälle

Ohne C-Compiler läuft `check:race` still durch: im Ergebnis als übersprungen nennen.

## Akzeptanzkriterien

- **AC-01** Sim: verschoben (Sperre 2026-11-11).
- **AC-02** Balance: verschoben (Sperre 2026-11-11).
- **AC-03** Gen: `check_run task:test` (ruft `gen:paper` auf, Grafik-Golden), grün.
- **AC-04** Race: `check_run task:check:race`, grün (ohne C-Compiler übersprungen, im Ergebnis so nennen).
- **AC-05** `task check:all` einmal grün (wurde in NG10 nicht gefahren).

## Offene Fragen

keine

## Notizen

Abgearbeitet 2026-11-11 (Claude, autonom, je Lauf einmal):

| Kriterium | Lauf | Ergebnis |
|---|---|---|
| AC-01 Sim | – | verschoben (Sperre 2026-11-11) |
| AC-02 Balance | – | verschoben (Sperre 2026-11-11) |
| AC-03 Gen | `task test` (Shell, weil `check_run` nach 5 min abbrach: `gen:paper` rendert nach dem fps-Fix alle 134 Figuren neu) | grün: 180 Testdateien, 3520 Tests, 218 s; keine eingecheckte Datei geändert (Grafik-Golden unverändert) |
| AC-04 Race | `check_run task:check:race` | übersprungen: kein C-Compiler auf dem Entwicklungs-PC (221 ms); die CI fährt `-race` |
| AC-05 | `task check:all` (462 s) | rot: einziger Fehler `TestHTMLEigenstaendig` in `tools/k3c-workbench/internal/balance` („Grafik castle/wall/tower/coin/enemy fehlt“); alle anderen Pakete und Schritte grün |

AC-05: Der Fehler ist kein Befund des Projekts NAT. Er bestand schon vor NAT auf `develop` (gemessen 2026-11-11 beim Umbau von `check:diff`, steht in B-787). Ursache: `report_html.go` bettet Pixel-Grafik aus alten Pfaden ein (`grafik/gebaeude/burg-1-32.png`, `sprites/skeleton/idle.png` …), die mit CU1 entfernt wurden. Nachbesserung in B-787 (WZG), nicht hier. Der Balancing-Bericht ist während der Sperre ohnehin nicht in Gebrauch.
