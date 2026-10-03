# B-169 · Eine Hörprobenseite spielt Kandidaten für Musik und Effekte ab

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SO3
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Für Musik (B-168) und Effekte (B-167) wählt 🧑 aus Kandidaten. Dafür gibt es keine Seite; der Hörvergleich am TV und mit Controller ist nicht möglich. Vorbild für Auswahlseiten ist `grafiken.html` (Packs ansehen). Weitere Testseiten stehen in `src/landing/pages.ts`.

## Ziel

Die Seite `soundtest.html` spielt Kandidaten-Sounds und -Musik ab, gruppiert nach Zustand oder Ereignis, bedienbar mit Controller, Tastatur und Touch. Nutzen: 🧑 hört und wählt am TV, die Auswahl landet in der Tabelle von B-167 und B-168.

## Beteiligte und Zielgruppen

🧑 hört und wählt (Q16 in `docs/fragenkatalog.md`); Entwickler und Agenten bauen die Seite; Familie am TV.

## Anforderungen

- Neue Seite `soundtest.html` im Projektordner (wird automatisch gebaut) mit Script unter `src/tools/` (Vorbild `src/tools/grafiken.ts`).
- Listet Kandidaten nach Zustand (Tag, Abend, Nacht, Kampf, Tiefe, Boss, Lobby …) und Ereignis (Münze, Schlag, Bauen …) mit Quelle und Lizenz; Abspielen, Stoppen, Lautstärke, Crossfade-Probe zwischen zwei Stücken.
- Nutzt den Audio-Kern aus SO1; Entsperren per Geste (Autoplay-Ergebnis aus B-166).
- Bedienbar mit Gamepad (Stick/Steuerkreuz wählen, A abspielen), Tastatur und Touch; große Schrift für den TV.
- **Seiten-Regeln aus `CLAUDE.md`:**
  - Im Script `installPageChrome()` aus `src/core/shell.ts` aufrufen (Home-Button oben mittig, View + Menu gemeinsam oder Pos1 = zurück, Zurück-Falle für B); oben ca. 70 px frei lassen.
  - Eintrag in `src/landing/pages.ts`, sonst ist die Seite vom Controller aus nicht erreichbar.
  - Vollbild nur über `toggleFullscreen()` aus `src/core/fullscreen.ts`, nie `requestFullscreen()` direkt.
  - Seitenwechsel nur über `openPage()`, Rückweg über `goHome()` aus `src/core/shell.ts`, nie per Link oder `location`.
  - Controller-Taste B nicht belegen; View + Menu ist reserviert.
  - `tests/projectRules.test.ts` prüft die Regeln automatisch und muss grün bleiben.

## Nicht-Ziele

Audio-Kern (SO1), Katalog (B-167), Musik-Einbau (B-168), Bearbeiten oder Mischen von Sounds.

## Regeln und Einschränkungen

Kleine Seite, kein Framework (`CLAUDE.md`, Komplexitäts-Budget); Datei ≤ 400 Zeilen; Credits aller Kandidaten in den CREDITS-Dateien (B-165); keine Spielzustände.

## Beispiele

Seite öffnen, A drücken (Entsperren), Stück „Nacht 2“ wählen, A → Musik spielt; zweites Stück wählen, „Überblenden“ → Crossfade hörbar.

## Ausnahme- und Fehlerfälle

Audio ist gesperrt → Hinweis „Taste drücken zum Entsperren“. Kandidat lässt sich nicht laden → Eintrag grau mit Hinweis, Rest funktioniert.

## Akzeptanzkriterien

- **AC-01** `soundtest.html` existiert, ruft `installPageChrome()` auf und steht in `src/landing/pages.ts`; `tests/projectRules.test.ts` ist grün.
- **AC-02** Die Seite listet Kandidaten gruppiert nach Zustand und Ereignis mit Quelle und Lizenz und spielt sie ab (Beobachtung im Browser und am TV).
- **AC-03** Beobachtung am TV: Die Seite ist mit dem Controller bedienbar (wählen, abspielen, stoppen); B ist nicht belegt, View + Menu führt zur Landingpage.
- **AC-04** Die Crossfade-Probe zwischen zwei Stücken ist hörbar ohne Knacken.
- **AC-05** Vollbild läuft nur über `toggleFullscreen()`, Seitenwechsel nur über `openPage()` und `goHome()` (Test `tests/projectRules.test.ts`).

## Offene Fragen

- Welche Kandidaten sollen in die erste Auswahl? Entscheidet 🧑, `docs/fragenkatalog.md` Q16.

## Notizen

Quelle: `docs/plan-weiterentwicklung.md` Schiene A (Musik, Hörproben auf `soundtest.html`).
