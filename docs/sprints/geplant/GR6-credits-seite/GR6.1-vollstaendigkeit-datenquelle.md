# GR6.1 · Vollständigkeits-Test und gemeinsame Datenquelle der Credits

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr6/1-credits-daten
- **Abhängig von:** –
- **Tickets:** B-165
- **Kriterien:** AC-01, AC-02

## Ziel

Es gibt genau eine Datenquelle für alle Credits (Grafik und Figuren); ein Test macht jedes Verzeichnis unter `public/grafik/` und `public/sprites/` ohne Eintrag rot, und die Seite `lizenzen.html` zeigt ihre Grafik-Einträge aus dieser Quelle statt aus Handtext.

## Kontext

- CREDITS-Dateien: `public/grafik/CREDITS.md` (eine Tabelle, Spalten Ordner, Pack, Urheber, Lizenz, Quelle; 12 Packs, Warped Caves CC BY 3.0) und `public/sprites/CREDITS.md` (drei Tabellen: LuizMelo und Gothicvania mit Ordner, Pack, Quelle, Urheber und Lizenz stehen im Absatz davor; Reittiere mit Ordner, Werk, Autor:innen, Lizenz, Quelle). Eine Zeile kann mehrere Ordner nennen (`goblin, skeleton, …`).
- Verzeichnisse heute: 12 unter `public/grafik/` (dazu `index.json`, `CREDITS.md`), 41 unter `public/sprites/`. `public/audio-test/` ist selbst erzeugt (README dort) und gehört nicht zu diesem Sprint.
- Seite: `lizenzen.html` (211 Zeilen, Abschnitt „Grafiken“ ab Zeile 66 als Handtext), Script `src/tools/lizenzen.ts` (nur `installPageChrome()` und `installPadScroll()`).
- Bestehender Test: `src/tools/grafikPacks.test.ts` prüft für die 12 Umgebungs-Packs aus `src/tools/grafikPacks.ts`, dass `CREDITS.md` und `lizenzen.html` Quelle, Urheber und Lizenz nennen. Er bleibt grün oder wird durch den neuen Test ersetzt (dann im Ergebnis nennen).
- Datenquelle (Entscheidung dieser Session laut B-165): Entweder liest die Seite die CREDITS-Dateien (`?raw`-Import, Tabellen parsen) oder eine TS-Datei (z. B. `src/tools/credits.ts`, Muster `grafikPacks.ts`) ist die Quelle und der Test prüft CREDITS-Dateien und Verzeichnisse gegen sie. Kriterium: keine doppelt gepflegten Texte, kleiner Code. Wahl und Grund ins Ergebnis.
- Tests laufen mit Vitest (`task test`), Verzeichnisse lassen sich im Test mit `node:fs` lesen (Muster in `tests/planning.test.ts`).

## Erlaubte Dateien

- `src/tools/` (neue Datei für die Datenquelle mit Test; `lizenzen.ts`, `grafikPacks.ts`, `grafikPacks.test.ts`)
- `lizenzen.html` (Abschnitt „Grafiken“ wird aus der Quelle gefüllt)
- `public/grafik/CREDITS.md`, `public/sprites/CREDITS.md` (nur Form, damit sie maschinenlesbar werden; keine Inhalte streichen)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Neue Assets, Sound-Credits (B-167, B-168), Software-Abschnitt der Seite, Englisch, Projekt-`LICENSE`, Darstellung und Landingpage (GR6.2).

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Datenquelle wählen und anlegen: je Eintrag Ordner (einer oder mehrere), Werk, Urheber, Lizenz, Quelle.
3. Test: (a) jedes Verzeichnis unter `public/grafik/` und `public/sprites/` hat einen Eintrag; (b) jeder Eintrag zeigt auf ein vorhandenes Verzeichnis; (c) jeder Eintrag hat Lizenz und Quelle; (d) Datenquelle und CREDITS-Dateien stimmen überein. Einmal mit einem gelöschten Eintrag rot sehen (nicht einchecken).
4. `lizenzen.ts` füllt den Grafik-Abschnitt aus der Quelle; der Handtext für Figuren und Packs entfällt, Dank-Texte bleiben.
5. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: Test belegt Eintrag je Verzeichnis unter `public/grafik/` und `public/sprites/`; ein fehlender Eintrag macht ihn rot.
- [ ] AC-02: Test belegt, dass die Seite alle Einträge aus der gemeinsamen Quelle zeigt und die CREDITS-Dateien übereinstimmen.
- [ ] `task check` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check
```

## Ergebnis

–
