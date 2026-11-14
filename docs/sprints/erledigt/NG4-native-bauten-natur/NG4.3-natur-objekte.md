# NG4.3 · Natur und Objekte

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** bauten
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng4/3-natur-objekte
- **Abhängig von:** NG4.2
- **Tickets:** B-797
- **Kriterien:** AC-03

## Ziel

Natur (Bäume, Steine, Adern, Plantagen) und Objekte (Münzen, Truhen, Fackeln und was `objectView.ts` kennt) erscheinen wie im Browser.

## Kontext

Spec: B-797. Vorbild: `src/scenes/render/natureSprites.ts`, `objectView.ts`, `plantation.ts`; Natur, Objekte und Gerüste sind bis MG8 Pixel-Grafik. Typen in `data/` und `engine/sim/world_vein.go`, `torches.go`, `building_plantation.go`. Vertrag und Importregel wie NG4.1.

## Erlaubte Dateien

- `native/k3c-game/props/**`
- Planung NG4, B-797 (über MCP)

## Nicht-Ziele

Umstellung auf Papier-Grafik (MG8); Licht der Fackeln (NG2.3).

## Schritte

1. Test zuerst: jeder Natur- und Objekt-Typ aus `data/` hat eine Grafik.
2. Natur zeichnen.
3. Objekte zeichnen.

## Fertig, wenn

- [ ] AC-03: Test über alle Typen grün; Screenshot im Ergebnis.
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

AC-03 umgesetzt: Bäume, Wurzeln, Felsen, Erz, Büsche, Adern, Plantage, Portal, Ausgang, Hafen-Steg, Rekrutierungslager, Truhe, Stern, Münzen, Fackeln; Natur wiegt je Platz in eigener Phase. Geprüft mit `TestJederTypAusDenDatenHatEineGrafik` und `TestJedeGewaehlteGrafikLiegtVorUndIstGemeldet` sowie an den Bildern `ng4/rand`, `ng4/hoehle`, `ng4/hub` (Agent).
Abweichungen: alles auf einer Ebene (ein Drawer je Fachpaket); Text ist die einzige Allokation im Zeichenpfad; Schrift fehlt (Debug-Schrift). Offen für die Zusammenführung: zwei Tests in `game/` erwarten noch Platzhalter-Rollen.
