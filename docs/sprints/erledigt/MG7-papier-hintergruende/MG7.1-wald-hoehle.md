# MG7.1 · Hintergründe Wald und Höhle

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hintergruende
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg7/1-wald-hoehle
- **Abhängig von:** –
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Hintergründe Wald (Tannenhang) und Höhle (Tropfsteinhalle) als Rig `hintergrund`: `hg-wald-<fern|mitte|nah>-v1/v2`, `hg-hoehle-<fern|mitte|nah>-v1/v2`, gerendert, Golden eingetragen.

## Kontext

Sprint-README (Anforderungen, D-MG7-01 bis D-MG7-04); Motive aus `public/grafik/gebaeude/hg-wald-*.json` und `hg-hoehle-*.json` (Feld `vorlage`). Wald: `smaragd`, `holz`; Höhle: `stein`, Moos `smaragd`.

## Erlaubte Dateien

- `public/grafik/hintergruende/**`
- `docs/sprints/*/MG7-papier-hintergruende/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine anderen Biome, kein Laden im Spiel.

## Schritte

1. `assets_guide {rig: hintergrund}`, Entwurf mit Generator im Scratchpad, Probe-Render mit der CLI.
2. `assets_create` und `assets_set` je Asset.
3. `assets_preview` je Asset, `task gen:paper`, fremde Gruppen zurücksetzen, `assets_review`.

## Fertig, wenn

- [ ] 12 Assets gültig und gerendert, jedes per Preview geprüft.
- [ ] Golden und Index der Gruppe aktuell.
- [ ] Ergebnis mit Nachweis je Kriterium.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- 12 Assets in `public/grafik/hintergruende/` (Rig `hintergrund`, 768 × 480, Animation `stand`): `hg-wald-<fern|mitte|nah>-v1/v2` (Tannenhang: Bergkette und Tannen-Silhouetten, Bäume und Büsche, dicke Stämme mit Laub, Gras, Blumen und Pilzen) und `hg-hoehle-<fern|mitte|nah>-v1/v2` (Tropfsteinhalle: Decke und Wand mit Zapfen, Säulen mit Zapfen und Kegeln, Felsblöcke mit Moos, Pfütze und Kieseln).
- Entstanden aus einem Generator im Scratchpad (periodische Profile, Zufall je ID fest); gleichartige Flächen sind je Teil zu einer Lage mit Teilpfaden zusammengefasst, Polylinien relativ (`l`).
- **AC-01 geprüft:** `assets_list {gruppe: hintergruende}` 12/12 gültig.
- **AC-02 geprüft:** `task gen:paper` grün, `golden.json` und `index.paper.json` der Gruppe neu; fremde Gruppen zurückgesetzt; `task test -- paper` 187/187 grün.
- **AC-03 geprüft:** `assets_preview` je Asset (12 Previews, dazu ein Kontaktbogen mit den drei Ebenen übereinander, je zweimal nebeneinander: Profile kacheln ohne Sprung); `assets_review` 12× neu, kein roter Eintrag.
- Befund behoben: Schatten der Felsblöcke in `hg-hoehle-nah-*` war eine dunkle Halbkuppel (wirkte wie Höhleneingang), jetzt Schattenkante links.
- Abweichung: `animationen.stand` ergänzt, weil `assets_preview` statische Specs sonst ablehnt (D-MG7-05). Laub in `hg-wald-nah-*` reicht oben bis knapp über den Rahmen (Krone geht oben weiter, gewollt).
- `task check:diff`: 1 Test unter Parallellast rot; `task test` einzeln 3173/3173 grün (Wackel-Test wie D-MG2-07).
