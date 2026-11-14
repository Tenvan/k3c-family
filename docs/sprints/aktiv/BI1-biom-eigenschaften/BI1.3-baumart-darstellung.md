# BI1.3 · Baumart im nativen Spiel und Folge-Tickets der Darstellung

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** bi1/3-baumart-darstellung
- **Abhängig von:** BI1.1
- **Tickets:** B-891
- **Kriterien:** AC-06

## Ziel

Das native Spiel zeichnet im Wald die Baumart aus dem Level; für Kristallfarben und die reinen Darstellungs-Eigenschaften gibt es eingeplante Folge-Tickets.

## Kontext

Level-Entitäten und Sim-Knoten tragen seit BI1.1 `Variant` (nur im Prozess, nicht im Zustand). Bild-Auswahl: `native/k3c-game/props/nature.go` (treeKinds tanne, pappel, birke), Zeichnen `props/plan.go` › nodes.

## Erlaubte Dateien

- `native/k3c-game/props/nature.go`
- `native/k3c-game/props/nature_test.go`
- `native/k3c-game/props/plan.go`

## Nicht-Ziele

Neue Grafik zeichnen (PAP-Ticket); Tasten, HUD, Cheat-Menü (Sprint CH1).

## Schritte

1. `variantLook` wählt im Wald das Asset der Baumart, sonst bleibt das Bild aus der Position.
2. Test.
3. Folge-Tickets: Kristallfarben-Assets (PAP), Darstellung Deckenhöhe, Tropfsteine, Stützbalken und Verfall, Hitze und Glut, Leuchtstärke.

## Fertig, wenn

- [ ] AC-06: `TestVariantLookBaumart` grün.
- [ ] Folge-Tickets mit Projekt NAT und Sprint angelegt.
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
