# BI1.3 · Baumart im nativen Spiel und Folge-Tickets der Darstellung

- **Status:** fertig
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

- AC-06 geprüft: `variantLook` wählt im Wald das Asset der Baumart (tanne, pappel, birke); `TestVariantLookBaumart` grün. Plantage-Bäume (ohne Variante) und andere Biome bleiben beim Bild aus der Position. Sichtprüfung im Spiel durch Ralf offen.
- Folge-Tickets im Sprint BI2 (Projekt NAT, geplant): B-900 Kristall-Adern in drei Farben (PAP), B-901 Kristallfarbe im Spiel, B-902 Deckenhöhe, B-903 Tropfsteine, B-904 Stützbalken und Verfall, B-905 Hitze und Glut, B-906 Leuchtstärke.
- Baumarten brauchen kein neues Asset (vorhandene Papierschnitt-Bäume).
- `task check:diff:go` grün; `check:diff:js` rot nur durch fremde Tickets (B-887, B-892 bis B-894).
