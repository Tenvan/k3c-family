# SO2.2 · Sounds beschaffen, nach public/audio/ legen, Credits

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** so2/2-sounds-beschaffen
- **Abhängig von:** SO2.1
- **Tickets:** B-167
- **Kriterien:** AC-01, AC-02

## Ziel

Die in SO2.1 gewählten Sounds liegen unter `public/audio/` mit Credit-Eintrag, Lücken der Pflicht-Ereignisse sind durch selbst erzeugte Retro-SFX gefüllt, nicht gewählte Kandidaten sind entfernt, und ein Test prüft Katalog und Credits.

## Kontext

- **Grundlage:** die Entscheidungen von 🧑 in `docs/assets/sounds.md` (SO2.1). Nur CC0 oder CC-BY (Q15); nichts hinzufügen, was nicht gewählt wurde.
- **Ordnung unter `public/audio/`:** je Quelle ein Unterordner (z. B. `public/audio/<pack>/…`), dazu `public/audio/CREDITS.md`. Das Credit-Format folgt dem der Grafik-Credits (`public/grafik/CREDITS.md`: Tabelle mit Kopf `| Ordner | … | Urheber | Lizenz | Quelle |`, vom Parser `src/tools/credits.ts` gelesen), damit die Credits-Seite sie später übernehmen kann. CC-BY-Dateien brauchen Urheber, Titel, Lizenz und Quelle wörtlich. Der Sound-Atlas und seine Beschreibung stammen aus SO1.2 (`public/audio/`, Ergebnis von SO1.2 lesen: Dateiname, Format, Beschreibungsfeld je Sound); `public/audio-test/` (selbst erzeugt, kein Credit nötig) bleibt unberührt.
- **Format und Atlas:** Format laut SO1.2 (Annahme mp3 mit Fallback, `docs/plan-weiterentwicklung.md` § 11.6, B-166). Die Sounds gehören in den Sound-Atlas, damit die Zahl der Audio-Requests klein bleibt (SO1 AC-05). Fehlt ein Werkzeug zum Zusammenfügen, Einzeldateien nutzen, die Request-Zahl im Ergebnis nennen und ein Ticket (INF) für das Atlas-Werkzeug anlegen; kein Werkzeug in dieser Session bauen.
- **Retro-Füller per Web Audio (Q15):** Pflicht-Ereignisse ohne Treffer bekommen einen selbst erzeugten Ton (kurzer Chiptune-Blip, Rauschen). Erzeugung als kleine reine Funktion in `src/audio/` (Parameter → Samples/Oszillator-Kette) mit Test; „selbst erzeugt, kein Credit nötig“ im Katalog vermerken. Kein `Math.random()` für Spiel-Logik (Rauschen als Klangquelle ist Audio, beeinflusst keinen Spielzustand; im Code so kommentieren oder festen Seed nutzen).
- **Test:** `src/audio/soundKatalog.test.ts` (neu): liest `docs/assets/sounds.md` und `public/audio/CREDITS.md` (`?raw`-Import wie `src/tools/credits.ts`) und prüft: jedes Ereignis der Liste aus B-167 steht im Katalog, jede Zeile hat Quelle und Lizenz oder ist fett als Lücke mit Ticket markiert, jede Sound-Datei unter `public/audio/*/` (Vorbild `import.meta.glob` in `src/tools/credits.test.ts`) hat einen Credit-Eintrag, Lizenz nur CC0 oder CC-BY.
- **Credits-Seite (B-165):** Der Parser und der Vollständigkeits-Test in `src/tools/credits.ts` und `src/tools/credits.test.ts` kennen heute nur `public/grafik/` und `public/sprites/`; `src/tools/` und `lizenzen.html` gehören der Domäne PLAT und werden **hier nicht** geändert. Ein Ticket (PLAT, Typ Idee): Credits-Seite und Test um `public/audio/` erweitern; wird in SO4.3 (CC-BY-Stücke auf der Credits-Seite) gebraucht. Existiert es schon, nicht doppelt anlegen.

## Erlaubte Dateien

- `public/audio/` (Sounds, Atlas-Ergänzung, `CREDITS.md`, `kandidaten.json`), `docs/assets/sounds.md`
- `src/audio/` (Füller-Funktion und Katalog-Test, mit Tests)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Einbinden ins Spiel (SO2.3), Musik (SO4), Änderungen an `src/tools/credits.ts`, `lizenzen.html` und am Audio-Kern, Atlas-Werkzeug.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Katalog aus SO2.1 lesen; prüfen, dass SO1 (Audio-Kern, Atlas-Format) auf `develop` liegt, sonst `Status: blockiert`.
2. Gewählte Dateien nach `public/audio/<quelle>/` legen (nur diese, Format laut SO1.2), nicht gewählte Kandidaten und deren Einträge in `kandidaten.json` entfernen.
3. Füller für Pflicht-Ereignisse ohne Treffer erzeugen (`src/audio/`), Katalog aktualisieren (Status `zugeordnet`, Quelle „selbst erzeugt“ bzw. Lücke mit Ticket).
4. `public/audio/CREDITS.md` schreiben; Ticket für die Credits-Seite (PLAT) anlegen (Vorlage `docs/vorlagen/ticket.md`, Index in `docs/backlog/README.md`).
5. `src/audio/soundKatalog.test.ts` schreiben, `task check`. Ergebnis (Zahl Dateien, Zahl Lücken, Audio-Requests), `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: Der Test prüft, dass `docs/assets/sounds.md` jedes Ereignis der Liste mit Sound, Quelle und Lizenz oder als Lücke mit Ticket enthält.
- [ ] AC-02: Der Test prüft, dass jede Sound-Datei unter `public/audio/` einen Credit-Eintrag in `public/audio/CREDITS.md` hat; CC-BY mit Urheber und Quelle.
- [ ] Ticket für die Credits-Seite (PLAT) liegt vor oder existierte schon.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
