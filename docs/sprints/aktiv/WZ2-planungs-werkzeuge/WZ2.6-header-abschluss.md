# WZ2.6 · B-341 Header-Ursache gegen die Notizen prüfen und abschließen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** sprint/wz2
- **Abhängig von:** WZ2.1
- **Tickets:** B-341
- **Kriterien:** AC-07

## Ziel

B-341 ist abgeschlossen: Header-Ursache (V1) und Abhilfe (B-388) stehen belegt unter Notizen, die Kriterien AC-01 und AC-02 sind nachgewiesen.

## Kontext

- Spec: `docs/backlog/B-341-header-ursache-live-messen.md` (AC-01, AC-02).
- Die Notizen enthalten bereits die Offline-Messung (M9.1, 2026-10-07) und die Log-Messung vom 2026-10-10: 356 `ns=mcp`-Zeilen mit `header=true` und `checkout=Repo-Wurzel`, Schluss **V1** (Client führt den Helper mit der Wurzel als cwd aus), Abhilfe Ticket B-388 (Argument `checkout`).
- Offen ist nur der Nachweis je Kriterium: AC-01 verlangt je eine Log-Zeile aus einer Session in der Wurzel und in einem Worktree; die Notizen belegen bisher nur die Wurzel-Seite aus dem Log.
- B-388 existiert; `checkout` ist seit B-388 in den Tools vorhanden (siehe k3c-dev-Server-Anweisungen › Worktrees).

## Erlaubte Dateien

- `docs/backlog/B-341-header-ursache-live-messen.md` (Notizen, Abschnitt Akzeptanzkriterien nur wenn ein Kriterium neu formuliert werden muss, dann Revision + 1)
- `logs/k3c-dev.jsonl` nur lesen

## Nicht-Ziele

Den Client oder `.mcp.json` ändern; Ablehnen ohne Header (Beschluss 🧑 2026-10-06, B-275); neue MCP-Tools.

## Schritte

1. `logs_query` (`ns=mcp`, Felder `header`, `checkout`) lesen: je eine Zeile aus Wurzel-Session und Worktree-Session heraussuchen.
2. Notizen von B-341 gegen AC-01 und AC-02 abgleichen; fehlt die Worktree-Zeile, nennen und als `blockiert` oder `verschoben` eintragen (nie still abhaken).
3. Ergebnis (V1, Abhilfe B-388, Beleg) unter Notizen zusammenfassen.
4. `task test -- planning` ausführen.

## Fertig, wenn

- [ ] AC-07: Header-Ursache (V1) und Abhilfe (B-388) stehen in B-341 › Notizen, je Kriterium `umgesetzt`, `geprüft` oder `verschoben` mit Grund (`B-341/AC-01`, `AC-02`).
- [ ] `task test -- planning` grün.

## Prüfen

```bash
task test -- planning
```

Lesen der Logs über `logs_query`. Manuelle Prüfungen: keine.

## Ergebnis

- AC-07 geprüft: B-341/AC-01 mit je einer Log-Zeile aus der Wurzel (ohne Argument) und aus einem Worktree (mit Argument `checkout`) unter Notizen belegt, Vermutung **V1** bestätigt (1044 Zeilen, alle `header=true`; 33 Worktree-Zeilen, alle mit `checkout_arg=true`). B-341/AC-02: Abhilfe benannt, B-388 (erledigt, Argument `checkout`); im Client nicht änderbar.
- Geprüft mit: `grep` auf `logs/k3c-dev.jsonl` (Zählungen und Zeilen stehen in B-341 › Notizen), `task test -- planning` grün.
- Einschränkung: Das Log trägt den Header-Wert nicht; die Zuordnung zum Worktree folgt aus `checkout_arg`. Abweichungen vom Plan: keine. Neue Tickets: keine.
