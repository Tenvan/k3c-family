# BS1 · DEV · Schneller Balancing-Lauf mit Seed k3c-dev

- **Status:** erledigt
- **Projekt:** BAS
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-634, B-639
- **Start-Commit:** 3e95ba48
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑, Revision 1, mit den Defaults der offenen Fragen

## Ausgangslage

Balancing-Läufe rechnen ohne Seed-Angabe alle 100 Seeds aus `data/balance-targets.json` › `seeds` (`tools/k3c-workbench/internal/balance/targets.go`, `cmd/main.go` › `explicitSeeds`). Für die Werte-Runden in BAS dauert das zu lange (B-634).

## Ziel

Ein Balancing-Lauf `coop2` standard startet sauber als MCP-Tool (`sim_test`) und rechnet bis zur Abnahme der neuen Werte nur den Seed `k3c-dev`, in wenigen Minuten; die Prompt-Buttons der Workbench schicken kurze Prompts.

**Am Ende sichtbar:** `sim_test start bots=coop2 world=standard` (offline) → Bericht „Seed-Modus: dev (1 Seed: k3c-dev)“; Laufzeit vorher/nachher in B-634; Button an einer Session schickt `Session BS1.1`.

## Beteiligte und Zielgruppen

Claude (alle Werte-Runden in BS3 und BS5), 🧑 gibt frei und entscheidet das Zurückschalten.

## Anforderungen

B-634 › Anforderungen, B-639 › Anforderungen.

## Nicht-Ziele

Neue Werte (BS3), neue Bot-Profile, Simulationsänderungen.

## Regeln und Einschränkungen

Domäne DEV; `data/balance-targets.json` gehört hier zum Werkzeug (Seeds und Schalter, keine Zielwerte). Läufe nur über `sim_test`. Keine Abhängigkeit, läuft parallel zu BS2.

**⚠️ Workbench-Neustart:** BS1.1 ändert Go-Code und `instructions.md` der Workbench. Danach muss 🧑 die laufende Workbench neu starten (EXE neu bauen mit `task k3c-workbench:build` bzw. `k3c-workbench:web` neu starten), sonst kennt `sim_test` die neuen Parameter nicht. BS1.1 meldet das im Ergebnis (erste Zeile), per `notify_ui` (warn) und im Chat; BS1.2 prüft zuerst mit `workbench_status`/Tool-Schema, dass die neue Version läuft, sonst `Status: blockiert`.

## Beispiele

B-634 › Beispiele.

## Ausnahme- und Fehlerfälle

B-634 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** `seeds[0]` ist `k3c-dev`, ein Test prüft das (B-634/AC-01).
- **AC-02** `seedMode: dev` → `sim_test`-Lauf rechnet genau einen Seed, Bericht nennt den Modus (B-634/AC-02).
- **AC-03** Laufzeit `coop2` standard vorher/nachher notiert, nachher deutlich kürzer (B-634/AC-03).
- **AC-04** `seedMode: all` verhält sich wie bisher, Tests grün (B-634/AC-04).
- **AC-05** `sim_test` nimmt offline `bots`, `seedList`, `world`; Tool-Beschreibung und `instructions.md` nennen den BAS-Standardlauf (B-634/AC-05).
- **AC-06** Workbench-Neustart deutlich gemeldet, Messlauf erst danach über das neue Tool (B-634/AC-06).
- **AC-07** Prompt-Buttons schicken eine Zeile ohne Anweisungen, Tests prüfen die Beispiele (B-639/AC-01).
- **AC-08** Skill `k3c-session` erkennt die Kurzform; Neustart gemeldet (B-639/AC-02, AC-03).

## Offene Fragen

- Zurückschalten auf `all`: Default Ende BAS (BS6), 🧑 kann früher entscheiden. Nicht blockierend.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS1.1 | `BS1.1-seed-modus.md` | Umsetzung | autonom | fertig |
| BS1.2 | `BS1.2-laufzeit.md` | Umsetzung | autonom | fertig |
| BS1.3 | `BS1.3-prompt-buttons.md` | Umsetzung | autonom | fertig |
| BS1.4 | `BS1.4-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-26, Claude (Review BS1.4, auf `feature/bas-remake`, kein PR). AC-01–AC-06 siehe BS1.1/BS1.2 (Laufzeit k3c-dev 14 min → 4 min, parallele Läufe), AC-07/AC-08 siehe BS1.3. `world` für `sim_test` → BS11 (B-642). `check:race` lokal übersprungen. Behoben: flackernder Test `TestSimTestOfflineBewertetZiele`. Neue Tickets: B-642.
