# BB1.4 · Leitmotiv je Welt in der Ferne des Hubs (B-1042)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hauptrechner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/bb1
- **Abhängig von:** BB1.3
- **Tickets:** B-1042
- **Kriterien:** AC-05

## Ziel

Jede der vier Welten (standard, sengoku, nil, britannien) hat ein Leitmotiv als blasse Fern-Ebene, 3–5 M hoch, angelegt über `assets_*` und in `data/paper.json` zugeordnet.

## Kontext

- Standard `docs/standards/bildaufbau.md` §§ 3, 5: Im gerodeten Hub steht das Leitmotiv der Welt groß in der Ferne (Luftperspektive: hell, blass, wenig Kontrast, nur Ton `licht` und wenige Lagen), darf oben anschneiden.
- Motive (autonome Entscheidung mit Default, Ralf ändert): standard = Burgberg; sengoku = Vulkanberg mit Torii auf dem Grat (Vorlage `objekte/torii`); nil = Pyramide und Sphinx; britannien = Steinkreis auf einem Hügel (Vorlage `objekte/steinkreis`).
- Höhe 3–5 M = 365–608 Spiel-px (M = 121,5); Vorschlag Spec `ausgabe.hoehe_px` 480 (wie die Kulissen `hg-wald-*`), Rahmen [0,-480,768,480], Motiv-Größe im Spiel steht in `data/look.json` › `worlds.<level>.motif` (RU55.6, `size` in M, 3 bis 5).
- Neue Assets: `hintergruende/leitmotiv-standard`, `leitmotiv-sengoku`, `leitmotiv-nil`, `leitmotiv-britannien` (Rig `hintergrund`, `assets_guide` mit `rig: hintergrund`). Zuordnung in `data/paper.json` als neuer Schlüssel `leitmotive` je Welt-ID (Form wie `weltbilder`): `"leitmotive": { "standard": "hintergruende/leitmotiv-standard", … }` (Welt-Asset-Paket, Entscheidung 010 steht noch im Entwurf; die Zuordnung hier ist die vorläufige Quelle).
- Ablauf je Asset: `assets_create`, `assets_get` → `assets_set` → `assets_preview` → `assets_review`; `assets_render` für das einzelne Asset. Kein `gen:paper`, kein Golden. Alle `assets_*` mit `checkout`.
- Tiefe Stufen zeigen das Leitmotiv nicht; die Anbindung (Platz, Ebene, Tönung im Spiel) ist nicht Teil dieser Session, die Review (BB1.5) legt dafür ein Ticket an.

## Erlaubte Dateien

- `public/grafik/hintergruende/**`
- `data/paper.json`
- `tests/bildaufbauMasstab.test.ts`

## Nicht-Ziele

Zeichnen und Parallaxe im Spiel (RU55, B-1047), Polar-Welt, Änderungen an Rust-Code oder `data/look.json`, Zeichnen von Weltbildern (`weltbild-*`), `gen:paper`, Golden.

## Schritte

1. Test zuerst: In `tests/bildaufbauMasstab.test.ts`: `data/paper.json` › `leitmotive` hat genau die Welten standard, sengoku, nil, britannien, jeder Wert zeigt auf eine Spec in `public/grafik/hintergruende/`, deren `ausgabe.hoehe_px` ÷ 121,5 mindestens 3 ist (Anschnitt oben erlaubt). Rot vor der Änderung.
2. Vier Assets anlegen und zeichnen (Motive oben), in `data/paper.json` zuordnen.
3. `assets_preview` im Spielmaßstab (neben dem Monarchen, 3–5 M) und `assets_review`; `k3c-grafik` und `k3c-welten` prüfen Stil und Lesbarkeit der Motive.
4. `check_run` mit Ziel `task:check:diff`: grün.

## Fertig, wenn

- [ ] AC-05: Vier Leitmotive angelegt und zugeordnet (`leitmotive` in `data/paper.json`), Test grün; `k3c-grafik` ohne schweren Befund.
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
