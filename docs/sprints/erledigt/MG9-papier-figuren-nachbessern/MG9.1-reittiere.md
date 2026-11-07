# MG9.1 · Reittiere: einheitlicher Maßstab, Wolfssilhouette, Füße

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** reittiere
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/mg9
- **Abhängig von:** –
- **Tickets:** B-716
- **Kriterien:** AC-01

## Ziel

Alle Reittiere der Gruppe `reittiere` haben denselben Maßstab (px je Einheit); Wolf, Weißer Wolf, Höllenhund und Elefant haben eigene Silhouette, Fußform und Gangarten über `proportionen` aus B-715.

## Kontext

Ticket B-716. Specs `public/grafik/reittiere/*.paper.json` (Rig `vierbeiner`). Heute haben Pferd, Rappe, Schimmel, Albtraum, Pegasus, Elefant den Rahmen `[-78,-122,155,126]`, Einhorn, Hirsch, Wölfe, Höllenhund `[-78,-128,155,132]` bei jeweils `hoehe_px` 192: px je Einheit = `hoehe_px / rahmen[3]` ist damit ~4 % verschieden (D-MG2-06). RD2.4 liefert `proportionen` (`fuss`: huf/pfote/elefantenfuss, `gangarten`), Standard `docs/standards/paper-spec.md`. Golden: `bun tools/paper/golden/schreibe.ts` nur bei Fixture-Änderung, Gruppen-Golden über `task gen:paper`.

## Erlaubte Dateien

- `public/grafik/reittiere/**`, `public/grafik/figuren/**` (nur neu gerenderte Ausgabe)
- `tools/paper/golden/**` (neuer Gruppen-Test zum Maßstab)
- `tools/paper/rig/reiten.ts` (Variante färbt auch die Satteldecke des Reittiers, Vorgriff auf MG9.2/AC-02)
- `docs/sprints/aktiv/MG9-papier-figuren-nachbessern/**`

## Nicht-Ziele

Neue Reittiere; Änderungen am Rig-Code (RD2.4 ist fertig); Einbau ins Spiel (US3).

## Schritte

1. Test zuerst: alle Reittier-Specs haben dasselbe Verhältnis `hoehe_px / rahmen[3]`.
2. Rahmen aller Reittiere auf `[-78,-128,155,132]` vereinheitlichen (Rahmen wächst, kein Skalieren).
3. Wolf, Weißer Wolf, Höllenhund: `proportionen.fuss: pfote`, eigene Kopf-/Rumpfform (Schnauze, Spitzohren, buschige Rute) statt Pferdekopf; Elefant: `elefantenfuss`, `gangarten: [schritt, trab]`, Animation `galopp` entfernen.
4. `task gen:paper` für die Gruppe, Bilder mit `assets_preview` ansehen, Golden schreiben.

## Fertig, wenn

- [ ] AC-01: Test in `tools/paper`: gleiches `hoehe_px / rahmen[3]` in der ganzen Gruppe (B-716/AC-01).
- [ ] AC-01: Wolf, Höllenhund, Elefant mit eigener Silhouette und Fußform, Vorschau angesehen (B-716/AC-02).
- [ ] AC-01: Elefant ohne `galopp`, Spec gültig (B-716/AC-03).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-11-07, Claude.

- AC-01 / B-716/AC-01 umgesetzt, geprüft (`tools/paper/golden/reittiere.test.ts`): alle elf Reittiere haben Rahmenhöhe 132 bei `hoehe_px` 192, also denselben Maßstab. Die drei Wölfe haben den Rahmen `[-94,-128,184,132]` (breiter, gleiche Höhe), weil Schnauze und Rute breiter sind.
- AC-01 / B-716/AC-02 umgesetzt, Sichtprüfung per `assets_preview` (Claude): Wolf, Weißer Wolf, Höllenhund mit Hundekopf (lange Schnauze, Spitzohren, Nase), buschiger Rute und Pfoten (`proportionen.fuss: pfote`); Elefant mit `elefantenfuss` und Zehennägeln. Manuelle Abnahme der Optik bleibt bei Ralf.
- AC-01 / B-716/AC-03 umgesetzt, geprüft: Elefant mit `gangarten: [schritt, trab]`, Animation `galopp` und PNG entfernt.
- Abweichung: `tools/paper/rig/reiten.ts` angefasst (Variante tauscht auch das Reittier, damit die Satteldecke der Rockfarbe folgt, B-711), dadurch neue Ausgabe der Monarch-Varianten spieler2–4; Farbe von spieler4 folgt in MG9.2.
