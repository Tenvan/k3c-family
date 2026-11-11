# NG5.1 · Belegung: Tasten zu Befehlen und Menü-Aktionen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng5/1-belegung
- **Abhängig von:** NG1
- **Tickets:** B-798
- **Kriterien:** AC-01

## Ziel

Paket `input` übersetzt Tasten- und Achsenzustände nach der heutigen Belegung in Befehle je Spieler und Menü-Aktionen.

## Kontext

Spec: B-798. Modul `native/k3c-game` (NG1): Eingabe-Typen in `view/`, Platzhalter in `input/` (aus NG1.4: direkte Pads) ersetzen, `game/` und `view/` unverändert; Importregel nur `view`, `art`, `k3c/engine/sim`, `k3c/data`. Belegung: `docs/rules/bedienung.md`, `src/scenes/keyTable.ts`, `src/input/playerInput.ts`, `slotBindings.ts`, `keyboardLayouts.ts` samt Tests; Befehl: `sim.PlayerCommand`. Nur Xbox-Belegung. Eingabe-Code: Reviewer (Sonnet) parallel zu `task check:diff`.

## Erlaubte Dateien

- `native/k3c-game/input/**`
- Planung NG5, B-798 (über MCP)

## Nicht-Ziele

Pad-Verwaltung und Hot-Plug (NG5.2); Belegung ändern; Touch.

## Schritte

1. Tabelle aus `docs/rules/bedienung.md` als Go-Testfälle (rot).
2. Reine Funktion Tastenzustand → Befehl bzw. Menü-Aktion (Pad und Tastatur).
3. Tastatur-Layouts wie `keyboardLayouts.ts`.

## Fertig, wenn

- [ ] AC-01: Tests grün über alle Zeilen der Belegung.
- [ ] `task check:diff` grün, Review ohne schwere Befunde.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
