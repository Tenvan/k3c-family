# SO3 · PLAT · Hörprobenseite `soundtest.html`

- **Status:** aktiv
- **Domäne:** PLAT
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-169
- **Start-Commit:** cfdba1e
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑; umfasst B-169 und die Domänen-Ausnahme `public/audio/`; mit Änderungen aus dem Spec-Review (SO1 als Voraussetzung, AC-07 Tastatur/Touch, AC-08 Credits)

## Ausgangslage

Für Musik und Effekte wählt 🧑 aus Kandidaten; eine Seite zum Anhören am TV fehlt. Vorbild ist `grafiken.html`. Details in B-169.

## Ziel

Eine Seite spielt Kandidaten ab, bedienbar mit Controller. Am Ende sichtbar: `soundtest.html` auf der Landingpage, auf der Xbox hörbar.

## Beteiligte und Zielgruppen

🧑 hört und wählt (Q16); Agent baut die Seite.

## Anforderungen

B-169 › Anforderungen, darunter die Seiten-Regeln aus `CLAUDE.md`. Voraussetzung: SO1 (Audio-Kern) für SO3.2.

## Nicht-Ziele

Audio-Kern (SO1), Katalog (SO2), Musik-Einbau (SO4).

## Regeln und Einschränkungen

`installPageChrome()`, Eintrag in `src/landing/pages.ts`, `toggleFullscreen()`, `openPage()` und `goHome()`, B nicht belegen, View + Menu reserviert; `tests/projectRules.test.ts` grün; Datei ≤ 400 Zeilen.

**Domänen-Ausnahme (Freigabe dieser Spec erlaubt sie, wie bei X1):** SO3.2 darf die Kandidatenliste und selbst erzeugte Probetöne unter `public/audio/` (CLI) anlegen.

## Beispiele

Seite öffnen, A entsperrt Audio, Kandidat wählen, A spielt ihn ab.

## Ausnahme- und Fehlerfälle

Audio gesperrt → Hinweis zum Entsperren; Kandidat lädt nicht → Eintrag grau.

## Akzeptanzkriterien

- **AC-01** `soundtest.html` ruft `installPageChrome()` auf und steht in `src/landing/pages.ts`, `tests/projectRules.test.ts` ist grün (B-169/AC-01).
- **AC-02** Die Seite listet Kandidaten nach Zustand und Ereignis mit Quelle und Lizenz und spielt sie ab (B-169/AC-02).
- **AC-03** Die Seite ist mit dem Controller bedienbar, B ist nicht belegt, View + Menu führt zur Landingpage (B-169/AC-03).
- **AC-04** Die Crossfade-Probe zwischen zwei Stücken ist ohne Knacken hörbar (B-169/AC-04).
- **AC-05** Vollbild nur über `toggleFullscreen()`, Seitenwechsel nur über `openPage()` und `goHome()` (B-169/AC-05).
- **AC-06** `task check` ist grün.
- **AC-07** Die Seite ist auch mit Tastatur und Touch (`?touch=1`) bedienbar (B-169 › Anforderungen).
- **AC-08** Jede fremde Audiodatei hat Quelle und Lizenz in den Credits, ohne Credit keine fremde Datei (B-165).

## Offene Fragen

- Erste Kandidatenauswahl: Entscheidet 🧑 (`docs/fragenkatalog.md` Q16).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SO3.1 | `SO3.1-seite-rahmen.md` | Umsetzung | autonom | fertig |
| SO3.2 | `SO3.2-kandidaten-abspielen.md` | Umsetzung | autonom | in Arbeit |
| SO3.3 | `SO3.3-abnahme-tv.md` | Workshop | Mensch | offen |
| SO3.4 | `SO3.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
