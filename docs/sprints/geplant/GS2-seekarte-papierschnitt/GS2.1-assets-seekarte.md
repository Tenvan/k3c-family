# GS2.1 · Assets: Meer, Insel-Miniaturen, Schiff, Aufsätze, Zeichen (B-964)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/gs2
- **Abhängig von:** –
- **Tickets:** B-964
- **Kriterien:** AC-01

## Ziel

Die Papierschnitt-Assets der Seekarte liegen als Specs vor, sind in `data/paper.json` › `seekarte` zugeordnet und im Review der Workbench sichtbar.

## Kontext

Spec: `docs/backlog/B-964-seekarte-spiel-grafik.md` (R1, R2, AC-01). Entscheidung 005: Assets entstehen **nur über die MCP-Tools `assets_*`** (`assets_guide` zuerst lesen, dann `assets_create`, `assets_set`, `assets_render`, `assets_preview`, `assets_review`); braucht die laufende Workbench. Eingecheckt werden Specs (`*.paper.json`), `index.paper.json` und `golden.json` der Gruppe; PNG und `manifest.json` nicht (B-737). Vorlagen: Weltbilder `public/grafik/hintergruende/weltbild-<welt>` und `-klein` (Zuordnung `data/paper.json` › `weltbilder`), Stil `public/grafik/stil.paper.json`, Standards `docs/standards/paper-*.md`. Welten: `standard`, `sengoku`, `nil`, `britannien`. Die Miniatur je Welt greift Motiv und Farben des Weltbilds auf; Größe, Schichten und Aufsatz setzt später der Code (GS2.2), deshalb ein Bild je Welt ohne Varianten.

## Erlaubte Dateien

- `public/grafik/hintergruende/**`
- `public/grafik/objekte/**`
- `data/paper.json`

## Nicht-Ziele

Code im Spiel (GS2.2), Trophäen (GS3.2), Miniatur für „Polar“, Animationen im Asset.

## Schritte

1. `assets_guide`, `assets_style_get`, `assets_list` (Gruppen `hintergruende`, `objekte`).
2. Anlegen: Meer hinten, Mitte, vorn (Wellenkante); Insel-Miniatur je Welt (4); Schiff klein; Aufsätze Turm, Baum, Fels; Zeichen Fahne und Nebel.
3. Rendern und Vorschau prüfen (`assets_render`, `assets_preview`), Fachprüfung `k3c-grafik` (Stil, Texturbudget).
4. `data/paper.json` › `seekarte`: Welt-ID → Miniatur, dazu `meer`, `schiff`, `aufsaetze`, `zeichen`.
5. `gen:paper` samt Grafik-Golden der betroffenen Gruppen (am Feature erlaubt), `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-01: Assets über `assets_*` angelegt, `assets_list` ohne Fehler, Zuordnung in `data/paper.json` › `seekarte` (B-964/AC-01).
- [ ] Grafik-Golden der Gruppen aktualisiert, `check_run task:check:diff` grün.

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
