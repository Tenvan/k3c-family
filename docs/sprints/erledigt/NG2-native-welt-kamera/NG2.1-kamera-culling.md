# NG2.1 · Kamera und Culling je Zelle

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** welt
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng2/1-kamera-culling
- **Abhängig von:** NG1
- **Tickets:** B-795
- **Kriterien:** AC-01

## Ziel

Paket `world` führt die Kamera je Zelle wie das heutige Spiel und wählt nur Sichtbares zum Zeichnen.

## Kontext

Spec: B-795. Modul `native/k3c-game` (NG1): Vertrag in `view/` (Frame mit `*sim.World` nur lesend, Cell, Camera, Drawer mit Ebene); `world/` hat einen Platzhalter, den `game/` registriert – nur dessen Inhalt ersetzen, `game/` und `view/` bleiben unverändert (fehlt etwas: Ticket). Importregel: nur `view`, `art`, `k3c/engine/sim`, `k3c/data`. Vorbild: `src/scenes/render/cameraFollow.ts`, `cameraCull.ts`, `viewRules.ts` samt Tests. Welt-Koordinaten in Units (1 Unit = 32 px). Rechnung als reine Funktionen mit Tests, Zeichnen dünn. Superpowers TDD; keine Sim- und Balancing-Läufe.

## Erlaubte Dateien

- `native/k3c-game/world/**`
- Planung NG2, B-795 (über MCP)

## Nicht-Ziele

Schichten, Fluss, Licht (NG2.2, NG2.3); `view` oder `game` ändern.

## Schritte

1. Fälle aus den TS-Tests zu `cameraFollow` und `cameraCull` als Go-Tests übertragen (rot).
2. Kamera je Zelle (Totzone, Grenzen der Stufe, zwei Zellen unabhängig).
3. Culling: Auswahl sichtbarer Objekte für alle Ebenen als Hilfsfunktion in `world` (andere Pakete nutzen sie nicht direkt; der Ausschnitt geht über `view.Camera`).

## Fertig, wenn

- [ ] AC-01: Go-Tests zu Kamera und Culling grün (`go test ./world/`).
- [ ] `task check:diff` grün.

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

AC-01 umgesetzt: Kamera-Regel (`view.CameraRule`) in `world`: folgt dem Monarchen mit Totzone ±1 Unit und Nachführung, Grenzen der Stufe, Sprung bei Stufen- und Inselwechsel; Culling mit Rand. Geprüft mit `camera_test.go` und `cull_test.go` (Fälle aus dem Browser-Client), Lint 0 Befunde (Agent, Opus).
