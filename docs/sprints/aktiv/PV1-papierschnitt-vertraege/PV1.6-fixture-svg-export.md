# PV1.6 · Fixture-SVGs aus dem Prototyp exportieren

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
