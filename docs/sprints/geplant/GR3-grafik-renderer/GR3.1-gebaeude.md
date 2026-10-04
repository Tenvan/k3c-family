# GR3.1 · Gebäude, Bauplätze, Hub- und Materialstufen als Sprites

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr3/1-gebaeude
- **Abhängig von:** GR1.3, GR2.3
- **Tickets:** B-010
- **Kriterien:** AC-01, AC-03, AC-04

## Ziel

Burg, Bauplätze und gebaute Gebäude (inklusive Hub-Stufen 1–5 und Mauer-/Turm-Materialstufen, soweit der Snapshot sie liefert) werden mit den zugeordneten Sprites gezeichnet; fehlt eine Grafik oder lädt sie nicht, bleibt die bisherige Platzhalter-Form sichtbar.

## Kontext

- **Zuordnung:** `docs/assets/zuordnung.md` (entsteht in GR1.1 bis GR1.3, Lücken schließt GR2.3) sagt je Objekt-ID Pack, Datei und Status (`zugeordnet` oder fett markierte Lücke). Gebaut wird nur, was dort `zugeordnet` ist; Lücken (laut B-010 › Notizen u. a. Werkstatt, Farm, Kaserne, Treppen, Rekrutierungslager) behalten die Platzhalter-Form. Stil nach Q13: Grundraster 16/32 px, Skalierung ×2 bis ×3, `UNIT_PX` = 32 (`src/core/constants.ts`).
- **Heute:** `src/scenes/worldRenderer.ts` (395 Zeilen, Grenze 400) zeichnet die Burg in `drawStatic()` und Bauplätze in `createSite()`/`updateSite()` als Rechtecke (`SITE_SIZE`, `SITE_COLOR` je `SiteKind`: `wall`, `tower`, `workshop`, `storage`, `stairsUp`, `stairsDown`); der Zustand `unpaid` → `waitingMaterial` → `waitingWorker` → `built` steuert Umriss, Baufortschritt und Preisschild (`drawPrice`). **Die Datei ist voll:** neue Zeichenlogik gehört in neue Dateien in `src/scenes/` (z. B. `siteView.ts`, `buildingSprites.ts`), `worldRenderer.ts` ruft sie nur auf und wird dabei kürzer, nicht länger.
- **Laden:** Der Figuren-Atlas kommt aus `data/sprites.json` über `task atlas` (GR4; `tools/atlas/`), geladen in `LoadScene.preload()` (`preloadSprites` in `src/scenes/sprites.ts`). Die Umgebungs-Packs unter `public/grafik/` sind **nicht** im Atlas. Umgebungsbilder in `LoadScene.preload()` einzeln mit `load.image` laden (Fortschrittsbalken und Fehlermeldung `loaderror` der Szene gelten dann mit); nur die in der Zuordnung genannten Dateien laden, nicht alle. Eine Atlas-Erweiterung wäre ein eigenes Ticket (INF), nicht Teil dieser Session. Pixel-Art mit `NEAREST` filtern wie in `createSpriteAnims`.
- **Stufen:** `Site` (`src/model/types.ts`) hat noch kein Stufen-Feld; Hub-Stufen und Mauer-/Turm-Materialstufen liefert die Simulation erst mit W1 (B-112, Regeln `docs/rules/materialien-gebaeude.md` § 2 und § 3.1: Stufe 1 bis 5, Turm-Stufe 5 = Zaubertum). Ist W1 gelaufen, den Zustandswert aus dem Snapshot lesen und je Stufe das Sprite wechseln; **sonst nur Stufe 1** zeichnen und die Auswahl nach Stufe als reine Funktion (Stufe → Sprite-Schlüssel) mit Test bauen, damit W1 nur das Feld anbindet. Welcher Sprite zu welcher Stufe gehört, steht in der Zuordnung; fehlt es dort: Lücke und Platzhalter, nichts erfinden.
- **Rückfall (AC-03):** Die Auswahlfunktion liefert „kein Sprite“, wenn der Schlüssel in der Zuordnung fehlt oder die Textur nicht existiert (`scene.textures.exists`); dann zeichnet der bestehende Platzhalter-Code. Test der reinen Funktion plus Beobachtung mit absichtlich entfernter Textur.
- **Regeln:** `src/scenes` rechnet nichts (`src/scenes/noSim.test.ts`), keine Spiel-Logik; B-Taste nicht belegen; 2 Spieler im Split-Screen (jede Kamera zeigt dieselben Sprites, `showOnly` in `src/scenes/stageView.ts`); Preisschilder und Münz-Slots bleiben lesbar (Schrift ≥ Q03, `src/scenes/fontRules.ts`).
- **Reihenfolge:** GR3 läuft nie gleichzeitig mit S4, S7, W6 oder K5 (gemeinsame Datei `worldRenderer.ts`, `docs/plan-weiterentwicklung.md` § 11); vor dem Start `git log origin/develop -- src/scenes/worldRenderer.ts` ansehen.

## Erlaubte Dateien

- `src/scenes/worldRenderer.ts`, `src/scenes/stageView.ts`, `src/scenes/LoadScene.ts`, neue Dateien in `src/scenes/` mit Tests daneben
- `public/` (nur Credits-Ergänzung, falls ein genutztes Asset ihn noch nicht hat)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Ressourcen, Adern, Plantage und Parallax (GR3.2), neue Grafiken (GR2), Reittiere (B-152), Atlas-Werkzeug (GR4), Effekte (GR5), Spiel-Logik für Stufen (W1).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Zuordnung und `git log` der Datei `worldRenderer.ts` lesen.
2. Reine Auswahlfunktion (Objekt, Zustand, Stufe → Sprite-Schlüssel oder „keiner“) mit Test; Texturen in `LoadScene` laden.
3. Neue Zeichen-Datei für Burg und Bauplätze; `worldRenderer.ts` ruft sie auf, der Platzhalter-Code bleibt als Rückfall.
4. Zustände `unpaid`, `waitingMaterial`, `waitingWorker`, `built` ansehen (Preisschild, Baufortschritt, Münz-Slots bleiben lesbar); mit 2 Spielern im Split-Screen im Browser-Pane prüfen (Hinweise unter „Im Browser-Pane testen“ in `CLAUDE.md`), Screenshot je Stufe 1 und, falls vorhanden, höhere Stufen.
5. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: Jedes Gebäude (Burg, Mauer, Turm, Tor, Lager usw.), das in der Zuordnung `zugeordnet` ist, erscheint im Browser-Pane als Sprite (Screenshot).
- [ ] AC-03: Test der Auswahlfunktion und Beobachtung: Textur fehlt oder Gebäude ist Lücke → Platzhalter-Form, keine leere Stelle.
- [ ] AC-04: Stufen unterscheiden sich im Bild (Screenshot Stufe 1 und höhere Stufen); ohne W1 steht nur Stufe 1 plus der Test der Stufen-Zuordnung im Ergebnis. Die Sicht am TV ist Abnahme durch 🧑 und wird im Review als „angenommen, Validierung offen“ geführt.
- [ ] `worldRenderer.ts` ≤ 400 Zeilen, jede neue Funktion ≤ 60 Zeilen, `noSim.test.ts` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
