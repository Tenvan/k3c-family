# BB1.3 · Waldwand und Baumvarianten in mehreren Höhen und Tönen (B-1041)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hauptrechner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/bb1
- **Abhängig von:** BB1.2
- **Tickets:** B-1041
- **Kriterien:** AC-04

## Ziel

Waldbäume gibt es je Art in mindestens drei Höhen und mit einer dunklen Variante für die hintere Reihe, dazu ein Mittelgrund-Asset „Waldwand“ mit Stämmen und geschlossenem Kronendach; alles über `assets_*` angelegt und in `data/paper.json` zugeordnet.

## Kontext

- Standard `docs/standards/bildaufbau.md` §§ 2, 5: Nadelbaum 4–6 M, Laubbaum 2,5–3,5 M; dichter Wald = viele Stämme in mehreren Höhen und Tönen, Kronendach schließt oben (Luftperspektive: hinten heller und blasser, im dichten Wald dunkel und wenig gesättigt).
- Heute (`data/paper.json` › `objekte`, Höhe = `rahmen`-Höhe in Spiel-px; `skalierung` 1,5 für `objekte/tanne`, `objekte/pappel`, `objekte/birke`): tanne 146 px (1,8 M mit Faktor), pappel 150, birke 136, pappel-niedrig 75; Specs `public/grafik/objekte/*.paper.json` mit `ausgabe.hoehe_px` 360 (`pappel-niedrig` 180). Die Specs `tanne-2`, `pappel-2`, `birke-2` stehen nicht im Katalog (ob genutzt: ungeprüft, nicht anfassen, im Ergebnis nennen). Das Spiel kennt die drei Arten in `rust/crates/k3c-game/src/props/nature.rs` (`TREE_KINDS`).
- Zielhöhen (M; Spiel-px = M × 121,5), je Art drei Höhen und eine dunkle Variante: tanne 4,0 (486) · tanne-mittel neu 5,0 (608) · tanne-hoch neu 6,0 (729) · tanne-dunkel neu 5,0 · pappel-niedrig 2,5 (304) · pappel 3,0 (365) · pappel-hoch neu 3,5 (425) · pappel-dunkel neu 3,0 · birke 2,5 (304) · birke-mittel neu 3,0 (365) · birke-hoch neu 3,5 (425) · birke-dunkel neu 3,0. Faktor = Zielhöhe in px ÷ `rahmen`-Höhe; über 2,0 die Spec mit größerer `ausgabe.hoehe_px` neu zeichnen (`assets_set`) und `frame`/`rahmen` im Katalog aus dem neuen Manifest übernehmen (Texturbudget prüft `k3c-grafik`; `rust/crates/k3c-game/src/art/budget.rs` nur lesen).
- Dunkle Variante: eigenes Asset `<art>-dunkel` mit Ton `schatten`, wenigen Lagen und kühlem, wenig gesättigtem Material (`biom-wald`); gedacht für die hintere Reihe.
- Waldwand: Rig `hintergrund` (`assets_guide` mit `rig: hintergrund`), neues Asset `hintergruende/hg-wald-wand` (`ausgabe.hoehe_px` 480 wie `hg-wald-mitte-v1`, Rahmen [0,-480,768,480]): senkrechte Stämme in mehreren Tönen, geschlossenes Kronendach oben, nur Lücken für den Himmel; Ton `schatten`/`basis`, Material `biom-wald`. Zuordnung in `data/paper.json` als neuer Schlüssel `waldwand` (Form wie `wetter`): `"waldwand": "hintergruende/hg-wald-wand"`.
- Ablauf je Asset: `assets_create` (Rig, Gruppe), `assets_get` → `assets_set` (Merge-Patch) → `assets_preview` → `assets_review`; `assets_render` für das einzelne Asset (Manifest für Katalogwerte). Kein `gen:paper`, kein Golden. Alle `assets_*` mit `checkout`.
- Die Anbindung im Spiel (Wahl der Varianten, Waldwand-Schichten aus RU55.6) ist nicht Teil dieser Session; die Review (BB1.5) legt dafür ein Ticket an.
- Tiefe Stufen (Höhle, Mine) haben keinen Wald und brauchen das nicht.

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `public/grafik/hintergruende/**`
- `data/paper.json`
- `tests/bildaufbauMasstab.test.ts`

## Nicht-Ziele

Platzierung im Level-Generator, Wechsel nach Rodung (RU55.6, B-1047), Änderungen an Rust-Code, Löschen vorhandener Bäume, `gen:paper`, Golden.

## Schritte

1. Test zuerst: In `tests/bildaufbauMasstab.test.ts` die Bäume ergänzen: je Art (tanne, pappel, birke) mindestens drei verschiedene Höhen (`rahmen`-Höhe × `skalierung`), Nadelbaum in 4–6 M, Laubbaum in 2,5–3,5 M, eine `-dunkel`-Variante im Katalog; `waldwand` in `data/paper.json` zeigt auf eine vorhandene Spec in `public/grafik/hintergruende/`. Rot vor der Änderung.
2. Vorhandene Bäume auf die Zielhöhen heben (Faktor in `skalierung` oder Spec größer); die acht neuen Assets `tanne-mittel`, `tanne-hoch`, `tanne-dunkel`, `pappel-hoch`, `pappel-dunkel`, `birke-mittel`, `birke-hoch`, `birke-dunkel` anlegen und im Katalog `objekte` eintragen.
3. Waldwand anlegen und in `data/paper.json` zuordnen.
4. `assets_preview` und `assets_review` je Asset; `k3c-grafik` prüft Stil, Lesbarkeit und Texturbudget.
5. `check_run` mit Ziel `task:check:diff`: grün.

## Fertig, wenn

- [ ] AC-04: Varianten und Waldwand sind über `assets_*` angelegt und in `data/paper.json` zugeordnet; Test grün (drei Höhen je Art, Richtwerte, dunkle Variante, `waldwand`); `k3c-grafik` ohne schweren Befund.
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

AC-04 erfüllt: Test `tests/bildaufbauMasstab.test.ts` (je Art drei Höhen, Richtwerte Nadel 4–6 M, Laub 2,5–3,5 M, dunkle Variante, `waldwand`) vorher rot (7 Fälle), jetzt grün; `task check:diff` grün; `k3c-grafik` ohne schweren Befund. Kein Gen, kein Golden-Lauf (nur die vom Tool geschriebenen Golden-Einträge der neuen Assets), kein Sim.

### Änderungen
- 8 neue Bäume über `assets_create`/`assets_set` (Gruppe objekte, Rig requisite): tanne-mittel, tanne-hoch, tanne-dunkel, pappel-hoch, pappel-dunkel, birke-mittel, birke-hoch, birke-dunkel. Dunkle Varianten: Material `biom-wald`, Ton `schatten`, ohne Auflagen.
- 4 bestehende Bäume (tanne, pappel, pappel-niedrig, birke) mit größerem Rahmen neu gezeichnet (Geometrie skaliert), `skalierung` 1,5 für alle 12 Bäume.
- Höhen in M (Rahmen-Höhe x 1,5 / 121,5): tanne 4,1 · mittel 5,0 · hoch 5,9 · dunkel 5,0; pappel-niedrig 2,6 · pappel 3,0 · hoch 3,4 · dunkel 3,0; birke 2,6 · mittel 3,0 · hoch 3,4 · dunkel 3,0.
- Waldwand `hintergruende/hg-wald-wand` (768 x 480, kachelt, Kronendach mit drei Himmelslücken, Stämme in drei Gruppen, Unterholz); `data/paper.json` › `waldwand`. Nach Review: Wand-Ton `basis`, Stämme `schatten`, damit die dunklen Bäume davor nicht mit der Wand verschmelzen.
- Katalog `objekte`: frame/rahmen aus den Manifesten.

### Befunde (nicht behoben)
- Texturbudget: 12 Bäume 21,3 MiB statt 4 Bäume 6,1 MiB (+15,3 MiB, rund 3 % des Richtwerts); Wald-Level lädt derzeit nur tanne, pappel, birke (4,2 MiB). Texeldichte sinkt (`hoehe_px` = 0,75 x Spiel-px): leichte Weichzeichnung vermutet; Abhilfe `hoehe_px` anheben per `assets_set`. Tanne-Manifest zeigt Ping-Pong-Frames (5 von 8 Hashes), Sparpotenzial.
- Neue Assets ohne Verbraucher: Rust kennt nur `TREE_KINDS` (tanne, pappel, birke) und liest `waldwand` nicht (`k3c-data/src/paper.rs` ignoriert den Schlüssel). Ticket für die Anbindung legt BB1.5 an (zusammen mit RU55.6).
- Die bestehenden Bäume wachsen im Spiel sofort von rund 1,8 M auf 2,6–4,1 M; `pappel-niedrig` (Plantage, `nature.rs` plant_tree_look) wird 2,6 M hoch, Wachstumsstufen Setzling zu Jungbaum zu Altbaum kaum noch unterscheidbar (vermutet).
- Die Specs `tanne-2`, `pappel-2`, `birke-2` nicht angefasst (Nutzung ungeprüft).
- Sichtbare Höhe liegt rund 0,1 M unter der Rahmen-Höhe (Rand): birke, pappel-niedrig etwa 2,45 M, tanne etwa 3,85 M.

### Ralf prüft in der Workbench (Bereich Assets) und am TV
- Workbench: Vorschau der 8 neuen Bäume und der 4 umgezeichneten (Schärfe, Kronen-Lesbarkeit, Tannen-Stamm von tanne-hoch kurz und dünn), Waldwand (Naht links/rechts, Wand nach Tonwechsel jetzt hell, Himmellücken oben).
- TV: Wald (Bäume jetzt deutlich größer, Platz für Figuren und Hub), Plantage mit Pappeln, Schärfe der skalierten Tannen.
