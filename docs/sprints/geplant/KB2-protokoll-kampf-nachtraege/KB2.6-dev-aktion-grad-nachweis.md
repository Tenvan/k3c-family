# KB2.6 · Nachweis Dev-Aktionen: Zustand im nächsten delta, Gradwechsel ab nächster Welle

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb2/6-dev-aktion-grad-nachweis
- **Abhängig von:** –
- **Tickets:** B-080
- **Kriterien:** AC-05

## Ziel

B-080 ist nachgewiesen: Go-Tests belegen, dass eine Dev-Aktion den Raum ändert und im nächsten `delta` sichtbar ist und dass der Gradwechsel ab der nächsten Welle wirkt und ohne Dev-Mode abgelehnt wird; Lücken sind geschlossen.

## Kontext

Dev-Aktionen gibt es seit DBG1 (`engine/net/dev.go`, `engine/net/dispatch.go`, Tests `devpage_test.go`, `options_test.go`); `dev.go` kennt bereits ein Feld `grade` (ungeprüft, ob Gradwechsel samt Zustandsfeld „aktueller Grad“ und Ablehnung ohne Dev-Mode vollständig sind). Grad wirkt über `SetGrade` in `engine/sim/island_options.go`. Spec: B-080 › Anforderungen, Akzeptanzkriterien; Protokoll: `docs/protocol.md` › Dev-Aktionen. K5.3 (Debug-Panel) hängt von dieser Session ab.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `engine/net/`
- `engine/room/`
- `docs/protocol.md` (nur Abschnitt Dev-Aktionen)
- `docs/backlog/B-080-dev-tasten-server.md` (über `plan_set`/`plan_section`)

## Nicht-Ziele

Debug-Panel im Client (K5.3); Tasten im Browser zurückbringen; Neustart und Stufenwechsel, falls nicht schon vorhanden (→ Ticket).

## Schritte

1. B-080/AC-01 und AC-02 gegen Code und Tests abgleichen, Befund im Ergebnis festhalten.
2. Fehlende Tests ergänzen: Dev-Aktion → Änderung im nächsten `delta`; Gradwechsel ab nächster Welle; ohne Dev-Mode `rejected`, unbekannter Grad `bad_request`.
3. Fehlt das Zustandsfeld „aktueller Grad“ oder die Ablehnung, minimal ergänzen und `docs/protocol.md` › Dev-Aktionen nachziehen.
4. B-080 auf `erledigt` setzen, wenn beide Kriterien belegt sind.

## Fertig, wenn

- [ ] AC-05: Go-Tests belegen B-080/AC-01 und B-080/AC-02 (`check_run` go grün).
- [ ] `task check:go` grün.

## Prüfen

```bash
task check:go
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
