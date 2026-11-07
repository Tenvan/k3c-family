# MG9.1 · Reittiere: einheitlicher Maßstab, Wolfssilhouette, Füße

- **Status:** offen
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

- `public/grafik/reittiere/**`
- `tools/paper/golden/**` (neuer Gruppen-Test zum Maßstab)
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
