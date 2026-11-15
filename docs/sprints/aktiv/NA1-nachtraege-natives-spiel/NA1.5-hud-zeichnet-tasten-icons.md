# NA1.5 · HUD zeichnet die Tasten-Icons, gemalte Glyphe als Rückfall

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/na1
- **Abhängig von:** NA1.4
- **Tickets:** B-911
- **Kriterien:** AC-05, AC-06

## Ziel

Das HUD zeichnet Tasten-Hinweise mit den Papierschnitt-Icons aus NA1.4; ohne Grafik bleibt die gemalte Glyphe (B-911, Teil Spiel).

## Kontext

`native/k3c-game/hud/glyphs.go`: `glyphOf(lang, action, profile)` liefert Schlüssel (`pad:A`, `key:PAGEDOWN`) und Beschriftung; `IconAsset(key)` (Zeile 147) bildet heute `pad:<name>` → `icons/xbox-<name>` und `key:<name>` → `icons/taste-<name>` ab. Neu (B-911 Revision 2): Tasten nutzen `icons/taste-kappe` (ein Zeichen) oder `icons/taste-kappe-breit` (längere Beschriftung), die Beschriftung aus `keyLabel(lang, name)` setzt das HUD mit der Spielschrift darauf. `hud/drawmenu.go` › `drawGlyph` (Zeile 136) malt Kreis bzw. Kappe selbst und bleibt Rückfall. Icons laden: `hud/icons.go` › `hudAssets` und `Assets` (view.AssetSource), Zeichnen wie die Rohstoff-Icons dort. Tests: `native/k3c-game/e2e/glyphs_test.go` (`TestGlyphSchluessel`, `TestGlyphBeschriftung`), Tests in `hud/`. Zwei Spieler haben je ein eigenes Eingabe-Profil.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/e2e/glyphs_test.go`

## Nicht-Ziele

Neue Icons oder Änderungen an Specs (NA1.4); neue Belegungen; Menü-Paket `menu/`.

## Schritte

1. Roten Test: Für jeden Schlüssel aus `glyphOf` über alle Aktionen und Profile nennt `IconAsset` ein Icon, das in `data/paper.json` steht.
2. `IconAsset` anpassen (Kappe schmal/breit), Icons in `hudAssets` aufnehmen.
3. `drawGlyph`: Icon zeichnen, Beschriftung der Kappe darüber (Schrift verkleinern statt abschneiden); ohne Bild der bisherige Weg.
4. Test für den Rückfall ohne Grafik; Allokationen je Frame prüfen.
5. Agent `k3c-hud` prüfen lassen; `go test ./hud/ ./e2e/ -short`; `task check:diff`.

## Fertig, wenn

- [ ] AC-05: Test „jeder Glyph-Schlüssel hat ein Icon in `data/paper.json`“ grün (B-911/AC-01).
- [ ] AC-06: Test „ohne Icon gemalte Glyphe“ grün (B-911/AC-02).
- [ ] `task check:diff` grün; `k3c-hud` ohne schweren Befund.

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
