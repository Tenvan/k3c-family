# BB1.3 · Waldwand und Baumvarianten in mehreren Höhen und Tönen (B-1041)

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
