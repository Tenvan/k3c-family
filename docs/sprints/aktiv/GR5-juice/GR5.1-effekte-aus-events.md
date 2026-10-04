# GR5.1 · Effekte für Treffer, Kill, Münze und Bauen, nur aus Events

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr5/1-effekte-aus-events
- **Abhängig von:** –
- **Tickets:** B-164
- **Kriterien:** AC-01, AC-03

## Ziel

Die Feedback-Events `hit`, `kill`, `coinPickup`/`coinGive`, `buildProgress`/`built` (sowie Tod `playerDown`) lösen je einen sichtbaren Effekt aus (Treffer-Blitz, Münz-Partikel, Todes- und Bau-Effekt); die Auslösung liest nur Events und ändert keinen Spielzustand.

## Kontext

- **Events (F3 und F4 sind erledigt):** Der Client-Typ `GameEvent` steht in `src/model/types.ts`; Felder und Bedeutung je Typ in `docs/protocol.md` › Ereignisse: `hit` (`x`, `target`, `id`, `damage`), `kill` (`kind`, `x`, `gold`), `coinPickup` (`player`, `x`), `coinGive` (`player`, `x`, `to`), `buildProgress` (`site`, `kind`, `x`, `percent` 25/50/75), `built`, `revive`, `playerDown`. Orte `x` in Units (× `UNIT_PX` = 32 für Pixel, `src/core/constants.ts`). Ein Client übergeht unbekannte Typen: unbekanntes Event → ignorieren, kein Absturz. Die Simulation liefert höchstens 32 Events je Tick und Stufe (`eventsDropped`); Effekte müssen damit auskommen und dürfen nichts zählen oder voraussetzen.
- **Fallstrick Event-Quelle:** `GameScene.pendingEvents` (`src/scenes/GameScene.ts`) wird von der HudScene abgeholt und geleert (`game.pendingEvents.splice(0)` in `HudScene.showBanner`). Effekte dürfen **nicht** daraus lesen, sondern brauchen eine eigene Warteschlange, die `GameScene` dort mitfüllt, wo `frame.state.events` ankommen (derselbe Ort, an dem `pendingEvents` gefüllt wird, `GameScene.ts` Zeile um 193).
- **Zeichnen:** Effekte entstehen in der Ebene der Stufe (`StageView.layer`, `src/scenes/stageView.ts`), damit jede Zelle nur die Effekte ihrer Stufe sieht (`showOnly`). Treffer-Blitz am Ziel z. B. als kurzer Tint-Flash des Sprites (`WorldRenderer` hält Views je ID, `playerView(index)`) oder als kleiner Ring an `x`; Partikel mit Phaser-Emitter oder wenigen Sprites. Rein optische Streuung darf zufällig sein (B-164 › Anforderungen), aber nie Spielzustand beeinflussen; kein `Math.random()` für Spiel-Logik.
- **Dateiaufteilung:** `worldRenderer.ts` hat 395 Zeilen (Grenze 400) und wächst nicht; neue Dateien in `src/scenes/`, z. B. `effects.ts` (Zuordnung Event → Effekt als reine Funktion mit Test, ohne Phaser) und `effectsView.ts` (Zeichnen). `src/scenes/noSim.test.ts` prüft, dass `src/scenes` nichts aus `world/` importiert; die Zuordnung Event → Effekt-Art bleibt davon unberührt und ist reine Tabelle.
- **Blitzgrenze und Schalter** gehören zu GR5.2; hier schon eine Konfiguration (Dauer, Anzahl Partikel, Mindestabstand) an **einer** Stelle halten, damit GR5.2 dort die Grenze einhängt.
- **Regeln:** `src/scenes` rechnet nichts; 2 Spieler im Split-Screen (Effekt erscheint in der Zelle, deren Stufe das Event trägt); B-Taste nicht belegen; Schrift-/Kontrastregeln nur falls Text erscheint (`src/scenes/fontRules.ts`).

## Erlaubte Dateien

- `src/scenes/GameScene.ts` (nur: eigene Event-Warteschlange für Effekte), `src/scenes/stageView.ts`, `src/scenes/worldRenderer.ts` (nur: Zugriff auf Views, nicht länger machen)
- neue Dateien in `src/scenes/` (`effects*.ts`) mit Tests daneben
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Abschalten und Optionen (GR5.2), Split-Screen-Kamera-Schütteln, Blitzgrenze, Vibration (GR5.2), Feedback-Events und Protokoll (F3, F4), Sound (SO2), neue Spielregeln.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `docs/protocol.md` › Ereignisse und `src/model/types.ts` › `GameEvent` lesen; `git log origin/develop -- src/scenes/` auf parallele Änderungen ansehen.
2. Reine Zuordnung Event → Effekt (Art, Ort in Pixeln, Stufe) mit Test: je Event der Liste genau ein Effekt, unbekannter Typ → kein Effekt.
3. Eigene Event-Warteschlange in `GameScene`, Effekte in der Stufen-Ebene zeichnen; Aufräumen abgelaufener Effekte (kein Wachstum der Objektzahl).
4. Im Browser-Pane mit Events auslösen (Spiel mit Gegnern, Münzen, Bauplatz; oder Events als Mock in die Warteschlange legen) und je Effekt einen Screenshot; Hinweise unter „Im Browser-Pane testen“ in `CLAUDE.md`.
5. `task check` (inklusive `noSim.test.ts`). Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-01: Test der Zuordnung und Beobachtung im Browser-Pane: `hit`, `kill`, `coinPickup` und `built` (bzw. `buildProgress`) lösen je ihren Effekt aus.
- [x] AC-03: Die Effekt-Auslösung liest nur Events und ändert keinen Spielzustand (Test der reinen Funktion, `src/scenes/noSim.test.ts` grün).
- [x] Unbekannter Event-Typ → kein Fehler (Test).

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Umgesetzt: `src/scenes/effects.ts` (reine Zuordnung Event → Effekt, `EFFECT_CONFIG` als eine Stelle für Dauer, Partikel, Grenze `MAX_LIVE_EFFECTS`), `effectsView.ts` (Ring und Partikel in der Stufen-Ebene, Tween, Aufräumen), `stageView.ts` (hält `effects`), `GameScene.ts` (`spawnEffects` aus `frame.state.events`, nicht aus `pendingEvents`).

- AC-01: `effects.test.ts` prüft `hit`, `kill`, `coinPickup`, `coinGive`, `buildProgress`, `built`, `playerDown` je ein Effekt mit Ort in Pixeln. Beobachtung im Browser-Pane nicht gemacht (laut Auftrag keine Browser-Prüfung): 🧑 bitte am Gerät ansehen.
- AC-03: `effectFor` liest nur das Ereignis, Test „verändert das Ereignis nicht“; `noSim.test.ts` grün.
- Unbekannter Typ und unbekannter Spieler: `null`, kein Fehler (Test).
- `task check` grün (49 Testdateien, 892 Tests).
- Abweichung: `built` und `playerDown` tragen kein `x`; Behelf im Client (letztes `buildProgress`, Spieler-Snapshot), Ticket B-216.
- Effekte erscheinen in der Stufe des aktuellen `client.level`; Zuordnung per Ereignis-`stage` entfällt, bis B-176 mehrere Stufen im Client liefert.
