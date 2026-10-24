# SO6.1 · Credits-Seite mit Sounds und Vollständigkeits-Test

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** so6/1-credits-seite-sounds
- **Abhängig von:** –
- **Tickets:** B-493
- **Kriterien:** AC-01

## Ziel

`lizenzen.html` nennt die Sounds aus `public/audio/README.md`, CC-BY hervorgehoben; ein Test prüft, dass jede fremde Datei unter `public/audio/` einen Eintrag hat.

## Kontext

B-493. Parser `src/tools/credits.ts` liest Tabellen mit Kopf `| Ordner |`; die Sound-Importe stehen in `public/audio/README.md` › Importe (eine Zeile je Datei, `kandidaten/<datei>`). Selbst erzeugt und ohne Credit: `atlas.*`, `probe-*`. Credit-Datei bleibt README.md (Default, B-556).

## Erlaubte Dateien

- `src/tools/credits.ts`, `src/tools/credits.test.ts`, `src/tools/texts.de.ts`, `src/tools/texts.en.ts` (nur Hinweistext der Seite)
- `docs/sprints/` (Status, Ergebnis), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Format der Credit-Tabellen, Lizenz-Bestätigung (🧑), Sound-Auswahl.

## Schritte

1. Parser um die Wurzel `audio` erweitern.
2. Test: Vollständigkeit und Verweise für `public/audio/`, Seite zeigt die Einträge, CC-BY hervorgehoben.
3. Hinweistext der Seite nachziehen, `task check:diff`.

## Fertig, wenn

- [x] AC-01 (B-493/AC-01): Seite zeigt jede Sound-Credit-Zeile, CC-BY hervorgehoben (Test).
- [x] AC-01 (B-493/AC-02): Test scheitert bei fremder Datei ohne Eintrag.

## Prüfen

```bash
task test -- credits
task check:diff
```

## Ergebnis

2026-10-22, autonom (Claude).

- **AC-01 (B-493/AC-01) umgesetzt, geprüft:** `CREDITS` enthält die Wurzel `audio` aus `public/audio/README.md`; Test „die Seite zeigt die Sounds, CC-BY-Stücke hervorgehoben“ (`a-big-adventure.ogg`, `summer-adventure-background-music.mp3` als `class="by"`).
- **AC-01 (B-493/AC-02) umgesetzt, geprüft:** Test listet jede Datei unter `public/audio/**` außer `atlas.*`/`probe-*` ohne Eintrag und jeden Eintrag ohne Datei.
- Hinweistext auf `lizenzen.html` nennt die Sound-Datei. `task check:diff` grün. Seite nicht im Browser angesehen (offline-Session, Test prüft das HTML).
