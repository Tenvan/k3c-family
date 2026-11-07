# PV1.6 · Fixture-SVGs aus dem Prototyp exportieren

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** pv1/6-fixture-svg-export
- **Abhängig von:** PV1.2
- **Tickets:** B-699
- **Kriterien:** AC-05

## Ziel

Unter `tools/paper/fixtures/svg/` liegen die Quell-SVGs, aus denen PV1.3 alle Fixture-PNGs rendert.

## Kontext

- Der Prototyp `docs/decisions/005/prototyp-monarch-rig.html` braucht einen echten Browser (`getCTM`, `DOMPoint`). Lokal ausliefern: `python -m http.server 5197 --bind 127.0.0.1` im Ordner `docs/decisions/005/` (nur Loopback, Firewall), öffnen im Browser-Pane.
- `window.rig` bietet `apply(pose, zeit)`, `cloth.step(dt, zustand)`, `cloth.draw()`; Export wie `snapshot()` im Prototyp (Klon von `#svg`, `#ground`, `#shadow`, `.pv` entfernen, `XMLSerializer`). Zum Einschwingen des Umhangs vor jedem Export 240 Schritte zu 1/120 s.

## Erlaubte Dateien

- `tools/paper/fixtures/svg/`

## Nicht-Ziele

Kein Rendern (PV1.3), keine Änderung am Prototyp.

## Schritte

1. Prototyp ausliefern und im Browser-Pane öffnen.
2. Exportieren: `monarch-idle-0.svg` (Idle, t = 0), `monarch-galopp-0.svg` (Galopp, Phase 0), `monarch-galopp-0-purpur.svg` (wie zuvor, Umhang `#7c3a8a/#5a2466` statt Rot – Fixture für „Umhang-Material geändert“), `pferd-galopp-0.svg` (Galopp, Gruppe `#mon` und Zügel ausgeblendet).
3. `mauer-1.svg` und `mauer-2.svg` von Hand im Papierschnitt-Stil schreiben (Steinquader, Stufe 2 mit Zinnen), Filter `ds`, `ds2`, `paper` aus dem Prototyp übernehmen, `viewBox` wie der Prototyp.
4. Jede Datei im Browser öffnen und als Screenshot prüfen; Server beenden.
5. Commit `test(pap): Fixture-SVGs aus dem Prototyp`.

## Fertig, wenn

- [ ] Sechs SVG-Dateien vorhanden, jede ohne `class="pv"`, ohne `#ground`, ohne `#shadow`.
- [ ] Screenshots im Ergebnis verlinkt.

## Prüfen

```bash
task check:diff
```

## Ergebnis

- AC-05 (Teil Quell-SVGs) geprüft: sechs Dateien unter `tools/paper/fixtures/svg/` (`monarch-idle-0`, `monarch-galopp-0`, `monarch-galopp-0-purpur`, `pferd-galopp-0`, `mauer-1`, `mauer-2`), keine enthält `class="pv"`, `#ground` oder `#shadow`; viewBox `-120 -205 240 220`, 240×220.
- Export im Browser-Pane aus dem Prototyp (Loopback 5197): je Datei 240 Schritte zu 1/120 s Einschwingen bei Zeit 0, dann `cloth.draw()` und Klon wie `snapshot()`; ausgeblendete Gruppen (`display: none`) entfernt. Gespeichert über einen Loopback-Empfänger (5198, nur `*.svg`-Namen), danach alle Server beendet.
- Screenshots (Browser-Pane, je Datei geprüft): Notizordner der Sitzung, `pv1.6-screenshots/` (`monarch-idle-0.jpg`, `monarch-galopp-0.jpg`, `monarch-galopp-0-purpur.png`, `pferd-galopp-0.png`, `mauer-1.png`, `mauer-2.png`). Sichtprüfung durch 🧑 folgt in PS1.
- Abweichungen: `pferd-galopp-0` ohne `#mon`, Zügel **und** Umhang (`#capeG`, gehört zum Reiter); Purpur-Variante setzt auch die Lichtkante `#capeHi` auf `#a865b8`. Mauern per kleinem Generator im Stil des Prototyps (Quader mit Schatten-Lage und Licht-Falz, Stufe 2 mit Zinnen), Töne `stein` = `#c2bdb1/#9a9386/#6b6559` (gehen in PV1.3 in die Stil-Datei, B-701 D-07).
