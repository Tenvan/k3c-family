# BS1.1 · Seed k3c-dev zuerst und Seed-Modus dev/all

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bs1/1-seed-modus
- **Abhängig von:** –
- **Tickets:** B-634
- **Kriterien:** AC-01, AC-02, AC-04, AC-05, AC-06

## Ziel

`k3c-dev` ist der erste Balancing-Seed, ein Seed-Modus `dev`/`all` steuert die Läufe, und `sim_test` startet offline einen Lauf mit Bot-Profil, Seed-Liste und Welt sauber als MCP-Tool.

## Kontext

- Seeds: `data/balance-targets.json` › `seeds` ("1"…"100"), gelesen in `tools/k3c-workbench/internal/balance/targets.go`; ohne Angabe nimmt `cmd/main.go` › `explicitSeeds` alle.
- MCP-Tool: `tools/k3c-workbench/internal/mcpsrv/simtest.go` (`sim_test`: `bots` nur online, `seeds` nur Anzahl, keine Welt); Instructions `tools/k3c-workbench/internal/mcpsrv/instructions.md`.
- Bot-Profile: `tools/k3c-workbench/internal/balance/bots.go` (`coop2`).
- Alle Testläufe nur über `sim_test` (B-348).
- ⚠️ Die laufende Workbench lädt neuen Go-Code und Instructions erst nach einem Neustart (siehe Sprint › Regeln).

## Erlaubte Dateien

- `data/balance-targets.json`
- `tools/k3c-workbench/internal/balance/**`
- `tools/k3c-workbench/internal/simrun/**`
- `tools/k3c-workbench/internal/mcpsrv/simtest*.go`, `tools/k3c-workbench/internal/mcpsrv/instructions.md`
- `docs/backlog/B-634-*.md` (nur Notizen, über `plan_section`)

## Nicht-Ziele

Laufzeitmessung und weitere Tempo-Hebel (BS1.2); Zielwerte.

## Schritte

1. Vorher-Laufzeit messen: `sim_test start` offline, `bots`-Standard, alle Seeds (noch alte Version) → Dauer in B-634 › Notizen.
2. `k3c-dev` an `seeds[0]`, Feld `seedMode` (`dev`|`all`, Standard `dev`) mit Schema-Prüfung und Test.
3. `balance`/`simrun`: Seed-Modus anwenden; ausdrücklich übergebene Seeds gehen vor; Bericht-Kopf nennt Modus und Seeds.
4. `sim_test`: offline `bots` (z. B. `coop2`), `seedList` (Komma-Liste), `world` (Standard `standard`) durchreichen; Beschreibung erweitern; Test in `mcpsrv`.
5. `instructions.md`: Abschnitt BAS-Standardlauf (`sim_test start bots=coop2 world=standard`, Seed-Modus, „nie Shell“).
6. `check_run task:check:diff`, Commit.
7. **⚠️ Neustart melden:** Ergebnis-Zeile 1 „⚠️ Workbench-Neustart nötig“, `notify_ui` level `warn`, deutlicher Hinweis im Chat an 🧑 mit Befehl.

## Fertig, wenn

- [ ] AC-01: Test prüft `seeds[0] == "k3c-dev"`.
- [ ] AC-02: Test: `seedMode: dev` → genau ein Seed; Bericht-Kopf nennt Modus.
- [ ] AC-04: Test: `seedMode: all` → alle Seeds wie vorher.
- [ ] AC-05: `mcpsrv`-Test für `bots`/`seedList`/`world` offline; `instructions.md` ergänzt.
- [ ] AC-06: Neustart-Hinweis im Ergebnis, per `notify_ui` und im Chat.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Über `check_run task:check:diff`; gezielt `check_run dev:test` mit Muster `balance|simtest`.

## Ergebnis

⚠️ **Workbench-Neustart nötig:** `sim_test`, Instructions und Balance-CLI geändert; die laufende Workbench kennt `seedList` und `bots` offline erst nach dem Neustart.
- AC-01 umgesetzt, geprüft: `k3c-dev` an `seeds[0]`, `TestDevSeedStehtVorn`.
- AC-02 umgesetzt, geprüft: `seedMode: dev` rechnet ohne Angabe einen Seed, Bericht-Kopf „1 Seed, Seed-Modus dev (k3c-dev)“ (`TestSeedModus`, `TestBerichtNenntSeedModus`); Sensitivität und Kurven rechnen weiter alle Seeds.
- AC-04 umgesetzt, geprüft: `seedMode: all` bzw. leer wie bisher; `--seed-list` und `seedList` gehen vor (`TestWithSeeds`), `check:diff` grün.
- AC-05 teilweise: `sim_test` offline nimmt `bots` und `seedList` (Test in `TestSimTestLehntAb`), Beschreibung und `instructions.md` nennen den BAS-Standardlauf. **`world` verschoben (B-642):** Der Tester spielt nur eine Stufe, Meilenstein-Definition klärt 🧑.
- AC-06: Neustart oben, per `notify_ui` und im Chat gemeldet.
- Abweichungen: Gold-Korridor in `balance-targets.json` auf 5–20 nachgezogen (BS2.1 hatte nur die Regel geändert, `TestGrenzenWieTabelle` war rot); `docs/data/balance-targets.md` um `seedMode` ergänzt. Vorher-Laufzeit nicht gemessen (100 Seeds ≈ 35–40 min laut README), kommt in BS1.2 über `seedList`.
- Review (Sonnet): kein Blocker; behoben: Modus „auswahl“ bei Seed-Liste, Sensitivität ohne dev-Modus, leere/doppelte `seedList` abgelehnt. Offen (niedrig): Folgebefehle der Auswertung nennen `--seeds N` statt der Seed-Liste; Baseline-Vergleich bei anderem Seed-Modus nur als Hinweis (BS7).
