# BB1.2 · Maßstab Hub-Gebäude: Burg 3,5–4,5 M, Türme, Mauern, Werkstätten (B-1040)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hauptrechner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/bb1
- **Abhängig von:** BB1.1
- **Tickets:** B-1040
- **Kriterien:** AC-03

## Ziel

Die Burg ist mit 3,5–4,5 M das größte Gebäude des Hubs, Türme, Mauern und Werkstätten liegen im Richtwert; die Messtabelle aller Hub-Gebäude steht im Ergebnis.

## Kontext

- Richtwerte `docs/standards/bildaufbau.md` § 2: Burg 3,5–4,5 M, Turm 2–2,5 M, Mauer und Palisade 0,8–1,2 M, Zelt, Stand und Werkstatt 1–1,4 M; Abweichung über 20 % wird angeglichen (AC-02 des Tickets).
- M = 121,5 Spiel-px. Gebäude zeichnen 1 : 1 in Spiel-px (Spec-Höhe `ausgabe.hoehe_px` ÷ 121,5, so rechnet auch Standard § 7). Anzeige-Maßstab je Asset: `data/paper.json` › `skalierung` (`gebaeude-paper/<id>` → Faktor, Fußpunkt bleibt; `rust/crates/k3c-game/src/art/scales.rs`); Default: skalieren statt neu zeichnen.
- Vormessung (Spec-Höhe in px, in M; nicht gerendert, Stufen teilen den Rahmen): burg 296 (2,44), burg-ausbau 292 (2,40), turm 298 (2,45), pagode 298 (2,45), mauer 191 (1,57), tor 155 (1,28), schmiede 147 (1,21), taverne 168 (1,38), werkstatt 162 (1,33), kaserne 161 (1,33), bogner 162 (1,33), barke 164 (1,35), lager 168 (1,38), ruestkammer 151 (1,24), heilplatz 142 (1,17), dojo 145 (1,19), schrein 124 (1,02), einsiedler-huette 135 (1,11), farm 112 (0,92), rundhaus 112 (0,92), rekrutierungslager 68 (0,56), hafen 132 (1,09).
- Klassen für den Test: Burg (burg, burg-ausbau) 3,5–4,5; Turm (turm, pagode) 2–2,5; Mauer (mauer, tor) 0,8–1,2; Werkstatt (alle übrigen oben, auch Dojo und Schrein als Bauwerke der Welt) 1–1,4; Toleranz 20 %. Nicht gemessen: `geruest-*`, `hub-geruest`, `treppe-*`, `hintergrundmauer`, `lagerfeuer`, `schiff` (BB1.1).
- Heute klar abweichend (Default-Faktoren): burg und burg-ausbau ×1,64 (≈ 4,0 M), mauer ×0,70 (≈ 1,1 M), rekrutierungslager ×1,8 (≈ 1,0 M) oder als Ausnahme im Ergebnis begründen (Lager am Boden, in der Tabelle des Tests unter `AUSNAHMEN`). Türme und Werkstätten liegen schon im Richtwert.
- Gerüste: `hub-geruest` (292 px) und `geruest-3` (260 px) gehören vermutlich zu Burg und Burg-Ausbau (ungeprüft, Suche `hub-geruest` und `geruest-` in `rust/crates/k3c-game/src/props/` und `data/`); ein Gerüst eines skalierten Gebäudes bekommt denselben Faktor, sonst Ticket-Vorschlag.
- Bauplatz-Breiten stehen in den Spielregeln (`data/buildings.json`), nicht im Sprite; nur lesen: Überlappung benachbarter Bauplätze durch die breitere Burg (264 px × 1,64 ≈ 433 px) als Befund ins Ergebnis.

## Erlaubte Dateien

- `data/paper.json`
- `public/grafik/gebaeude-paper/**`
- `tests/bildaufbauMasstab.test.ts`

## Nicht-Ziele

Neue Gebäude oder Ausbau-Stufen, Hintergrundmauer-Logik, Bauplatz-Breiten ändern, `gen:paper`, Golden, Sichtprüfung am TV (🧑).

## Schritte

1. Test zuerst: In `tests/bildaufbauMasstab.test.ts` (aus BB1.1) die Gebäude-Klassen ergänzen (Höhe = `ausgabe.hoehe_px` × `skalierung` ÷ M, je Klasse mit 20 % Toleranz, `AUSNAHMEN` mit Begründung); rot vor der Änderung.
2. Messtabelle (Asset, Klasse, Höhe in px und M vorher/nachher) aus dem Test erzeugen und ins Ergebnis schreiben (AC-01).
3. `data/paper.json` › `skalierung`: Faktoren für Burg, Burg-Ausbau, Mauer, Rekrutierungslager und zugehörige Gerüste setzen; Specs nur anfassen, wenn Skalieren die Detaildichte sichtbar verschlechtert (`assets_preview` bei Zielgröße, `k3c-grafik` prüft).
4. Überlappungen (Bauplätze, Gerüst, Treppen) als Befund im Ergebnis, nicht beheben.
5. `check_run` mit Ziel `task:check:diff`: grün.

## Fertig, wenn

- [ ] AC-03: Messtabelle aller Hub-Gebäude in M im Ergebnis (vorher und nachher).
- [ ] AC-03: Test grün: Abweichungen über 20 % vom Richtwert angeglichen oder in `AUSNAHMEN` begründet; `k3c-grafik` ohne schweren Befund.
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
