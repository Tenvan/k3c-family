# B-369 · Die Landingpage zeigt ihre Texte in der gewählten Sprache

- **Domäne:** PLAT
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GL1
- **Projekt:** BED
- **Erstellt:** 2026-10-08
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-10, 🧑 im Chat (mit GL1 Revision 2)

## Ausgangslage

Bei der Abnahme PL2.5 (2026-10-08, 🧑 am PC) zeigten Spiel und Werkzeug-Seiten nach dem Sprachwechsel Englisch, die Landingpage blieb deutsch. Ihre Texte stehen als deutsche Literale im Code: `src/landing/pages.ts` (Kachel-Titel und -Beschreibungen, `play: 'Spielen'`), `src/landing/landing.ts` (Vollbild-Knopf und -Hinweis, Controller-Status, `frame.title`), `src/landing/serverCheck.ts` (`NO_SERVER_HINT`) und `index.html` (Tagline, Controller-Status, Vollbild, Hinweis „View + Menu …“). Weder PL2 (nur `src/tools/`) noch B-215 (Home-Button, Touch-Overlay) decken sie ab.

## Ziel

Die Landingpage folgt der gewählten Sprache mit Rückfall auf Deutsch, wie Spiel und Werkzeug-Seiten.

## Beteiligte und Zielgruppen

Alle, die an PC, Handy, TV oder Xbox spielen und die Landingpage als Einstieg sehen; Agent baut in `src/landing/` und `index.html`.

## Anforderungen

- Kachel-Texte, Statuszeilen, Vollbild-Knopf und Hinweise der Landingpage kommen über `t()` aus den Textdateien.
- Die Sprache kommt aus den Einstellungen des Geräts (`currentLanguage()`); fehlt ein englischer Text, gilt der deutsche.

## Nicht-Ziele

Weitere Sprachen; Home-Button und Touch-Overlay (B-215); Werkzeug-Seiten (erledigt mit PL2, B-322); Spielname „Family Three Crowns“ bleibt unübersetzt.

## Regeln und Einschränkungen

`CLAUDE.md` › Seiten & Navigation (Landingpage bleibt offen, iframe); Domäne PLAT; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; keine neue Abhängigkeit.

## Beispiele

Sprache English → Kachel „Continue“ statt „Weiterspielen“, Status „Controller: press a button“.

## Ausnahme- und Fehlerfälle

Kein Speicher bzw. unbekannte Sprache → Deutsch. Die Sprache wechselt im Spiel (iframe), die Landingpage übernimmt sie spätestens beim Neuladen.

## Akzeptanzkriterien

- **AC-01** `src/landing/*.ts` und `index.html` enthalten kein deutsches Text-Literal mehr (Regeltest wie `textRule.test.ts`); die Texte stehen in den Textdateien für de und en.
- **AC-02** Nach Sprachwechsel in den Optionen zeigt die Landingpage nach dem Neuladen die gewählte Sprache (🧑 am Gerät).

## Offene Fragen

Keine. Entschieden 2026-10-10 (🧑): Die Landingpage übernimmt die Sprache nur beim Neuladen, nicht schon beim Schließen der Spiel-Seite.

## Notizen

–
