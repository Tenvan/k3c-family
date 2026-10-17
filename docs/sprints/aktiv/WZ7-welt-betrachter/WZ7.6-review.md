# WZ7.6 · Review WZ7

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz7/6-review
- **Abhängig von:** WZ7.5
- **Tickets:** B-460
- **Kriterien:** alle

## Ziel

Der Sprint ist unabhängig geprüft: `task check:all` grün, Diff der Domänen SIM, SRV und DEV durchgesehen, nur schwere Befunde behoben, PR des Sprints geöffnet; Abnahme-Liste für WZ7.7 geschrieben.

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- Ablauf der Review-Session: `docs/arbeitsweise.md` › Review-Session (nie im selben Lauf wie eine Umsetzung, auf dem Sprint-Branch). Sie trägt die Domäne der letzten Umsetzung (DEV) und prüft den Diff aller drei Domänen.
- Der PR gegen `develop` wird hier geöffnet (ein PR je Sprint), Auto-Merge nach Arbeitsweise.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- Planungsdateien von WZ7, B-460, B-472 und neue Tickets, `docs/sprints/README.md` (Fahrplan), alle über k3c-dev
- Für schwere Befunde alle Dateien, die WZ7.8 (`engine/sim/preview*`), WZ7.1 (`engine/room/`, `engine/net/`) und WZ7.2–WZ7.5 (`tools/k3c-studio/**`, `src/tools/**`) erlauben; Befunde außerhalb → Ticket

## Nicht-Ziele

Neue Funktionen oder Umbauten über schwere Befunde hinaus; Stil, Doku, Benennung und Vereinfachungen als Befund behandeln; Befunde außerhalb von SIM/SRV/DEV beheben (→ Ticket); Kriterien umformulieren oder die Spec freigeben; manuelle Prüfungen im Fenster oder Browser und ein Urteil anstelle von 🧑 (WZ7.7); eine Version setzen (nur vorschlagen).

## Schritte

1. `task check`, `task check:go`, `task check:dev` und `task check:all` grün; bei laufender Workbench über `check_run`.
2. Diff gegen `origin/develop` für SIM (`engine/sim/preview*`), SRV (`engine/room/`, `engine/net/`) und DEV (`tools/k3c-studio/**`, `src/tools/**`) lesen; Kriterien AC-01 bis AC-08 mit Nachweis abgleichen.
3. Schwere Befunde beheben, übrige als Ticket.
4. Abnahme-Liste für WZ7.7 unter `### Abnahme-Liste` schreiben, Abschnitt `Abnahme` der Sprint-README ausfüllen.
5. PR des Sprints öffnen (siehe Arbeitsweise), Auto-Merge nach Regel.

## Fertig, wenn

- [ ] Alle Kriterien AC-01 bis AC-08 mit Nachweis im Ergebnis (`umgesetzt`/`geprüft`/`verschoben`/`blockiert`).
- [ ] `task check:all` grün, PR geöffnet.
- [ ] Abnahme-Liste für WZ7.7 geschrieben.

## Prüfen

`task check:all`; über k3c-dev `check_run` (mit `checkout`).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
