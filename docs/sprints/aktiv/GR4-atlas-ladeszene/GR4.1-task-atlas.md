# GR4.1 · `task atlas` mit deterministischer Ausgabe, eingebunden in `task build`

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr4/1-task-atlas
- **Abhängig von:** –
- **Tickets:** B-163
- **Kriterien:** AC-01

## Ziel

`task atlas` packt die im Spiel genutzten Grafiken zu Atlas-Bild(ern) und -Beschreibung; zwei Läufe liefern byte-gleiche Dateien, und `task build` ruft den Schritt auf.

## Kontext

- Anforderungen: B-163 › Anforderungen und Ausnahme- und Fehlerfälle (fehlendes Quell-Bild → Abbruch mit Dateinamen; zu groß für die Textur-Grenze → mehrere Atlanten, kein Absturz).
- **Welche Grafiken:** heute die Figuren mit Rolle im Spiel: `src/scenes/sprites.ts` lädt nur Sheets aus `data/sprites.json` (`players`, `troops`, `enemies`; Konstante `USED`), je Sheet `public/sprites/<sheet>/<anim>.png` mit `idle`, `run`, `attack` als Streifen (Frame-Größe in `sheets`). Reittiere (`mounts`, S7) zählen dazu, sobald `sprites.ts` sie lädt. Umgebungs-Grafiken kommen erst mit GR3 (B-010) ins Spiel; die Quelle dafür ist dann die Zuordnung aus GR1 (`docs/assets/zuordnung.md`). Das Werkzeug liest seine Liste aus denselben Daten, keine zweite Liste.
- **Werkzeug (Vorschlag, Wahl im Ergebnis begründen):** Go mit der Standardbibliothek (`image`, `image/png`) ist deterministisch und braucht keine neue Abhängigkeit; `package.json` hat keine Bildbibliothek, eine neue npm-Abhängigkeit nur mit Begründung (B-163 › Regeln). Unter Windows EXE mit festem Pfad bauen (`bin/…`), nicht `go run` (Firewall-Abfrage).
- Determinismus: feste Sortierung der Eingaben (Name), festes Pack-Verfahren, PNG ohne Zeitstempel/Metadaten; Beschreibung als JSON im Phaser-Atlas-Format (`frames` mit `frame`, `sourceSize`), sortierte Schlüssel.
- Textur-Grenze: Startwert 4096 px Kantenlänge (WebGL-Minimum auf Konsolen; angenommen, bis GR4.3 misst), Überlauf → zweiter Atlas.
- Ausgabe nach `public/atlas/` (wird von Vite nach `dist/` kopiert) oder ins Build-Verzeichnis; ob die Atlas-Dateien eingecheckt werden, im Ergebnis begründen (ohne Einchecken braucht `task dev` den Schritt ebenfalls).
- Lizenzen bleiben je Quell-Pack erhalten (`public/sprites/CREDITS.md`); der Atlas ist eine Bearbeitung, keine neue Quelle.

## Erlaubte Dateien

- `tools/atlas/` (neu, Go) oder `src/tools/` bei TS-Lösung, jeweils mit Test
- `Taskfile.yml` (Task `atlas`, Aufruf in `build`, ggf. `dev`)
- `public/atlas/` (neu, falls eingecheckt), `.gitignore` (falls nicht eingecheckt)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Laden aus dem Atlas im Spiel und Lade-Szene (GR4.2), Messung auf der Xbox (GR4.3), Umgebungs-Grafiken vor GR3, neue Grafiken (GR2).

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Werkzeug: Liste aus `data/sprites.json` (genutzte Sheets), Bilder lesen, packen, Atlas-PNG und -JSON schreiben; fehlendes Bild → Abbruch mit Dateinamen; Überlauf → weiterer Atlas.
3. Test: zweimaliger Lauf byte-gleich (Hash); fehlendes Bild bricht mit Namen ab; kleine Grenze erzwingt zwei Atlanten.
4. `task atlas` anlegen, `task build` ruft ihn vor `vite build`.
5. `task check`, `task build` (und `task check:go`, falls Go). Ergebnis mit Zahl der Atlanten und Größe, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-01: `task atlas` erzeugt Atlas-Bild und -Beschreibung; Test belegt byte-gleiche Ausgabe bei zweimaligem Lauf.
- [x] `task build` ruft `task atlas`; `task check` und `task build` grün.

## Prüfen

```bash
task atlas
task check
task build
```

## Ergebnis

Nachweis je Kriterium:

- **AC-01** umgesetzt und geprüft: `task atlas` (Go-Werkzeug `tools/atlas`, nur Standardbibliothek, EXE `bin/atlas`) schreibt `public/atlas/atlas-N.png` und `atlas.json` (Phaser-Multiatlas, Frames `<sheet>-<anim>/<i>`, sortiert). `tools/atlas/atlas_test.go`: zweimaliger Lauf byte-gleich, fehlendes Bild bricht mit Dateinamen ab, kleine Grenze erzwingt mehrere Atlanten (zu großer Frame → Fehler statt Absturz). Zusätzlich zwei echte Läufe per `md5sum` verglichen: gleich.
- **`task build` ruft den Atlas:** `build` und `dev` haben `deps: [atlas]`; `task check`, `task check:go` (0 issues), `task build` grün, `dist/atlas/` enthalten.

Zahlen: 1 Atlas, 4067 x 1004 px, 262 Frames, PNG 232 KB, JSON 118 KB (Grenze 4096, GR4.3 misst die echte).

Entscheidungen:

- **Werkzeug Go statt TS:** deterministisch, keine neue Abhängigkeit (`package.json` hat keine Bildbibliothek).
- **Nicht eingecheckt** (`/public/atlas/` in `.gitignore`): Ausgabe ist ableitbar, Binärdiffs bei jeder Sprite-Änderung vermieden; `task dev` und `task build` erzeugen sie (Task `sources`/`generates` überspringt unveränderte Läufe).
- **Packer:** Regalpacker, Frames nach Höhe absteigend, dann Name; 2 px Abstand, kein Trimmen (`sourceSize` = Frame-Größe, damit `generateFrameNames` in GR4.2 dieselben Frames wie `generateFrameNumbers` liefert).
- Reittiere (`mounts`) sind noch nicht dabei: `sprites.ts` lädt sie nicht; Umgebungs-Grafiken folgen mit GR3.
- Grenze: die PNG-Bytes sind nur je Go-Version stabil (Encoder); innerhalb des Repos mit der Version aus `go.mod` reicht das.

Neues Ticket: B-196 (Pages-/CI-Workflows brauchen `setup-go`, weil `task pages` jetzt Go braucht).
