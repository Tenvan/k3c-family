# GR6.2 · CC-BY-Einträge sichtbar, Seite über die Landingpage

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr6/2-cc-by-seite
- **Abhängig von:** GR6.1
- **Tickets:** B-165
- **Kriterien:** AC-03, AC-04

## Ziel

Jeder CC-BY-Eintrag zeigt auf der Credits-Seite Urheber, Lizenz und Quelle als Link; die Seite ist über die Landingpage erreichbar und hält die Seiten-Regeln aus `CLAUDE.md` ein.

## Kontext

- CC-BY-Einträge heute: Warped Caves (`public/grafik/`, CC BY 3.0) und die LPC-Reittiere (`public/sprites/`: horse, horse-walk, white-horse, unicorn, pegasus, elephant, stag, lpc-wolf, lpc-hellhound; CC-BY 3.0). Die Namensnennung ist dort Pflicht.
- Datenquelle und Grafik-Abschnitt stammen aus GR6.1 (siehe deren Ergebnis).
- Seiten-Regeln (`CLAUDE.md` › Regel: Seiten & Navigation): `installPageChrome()` (steht schon in `src/tools/lizenzen.ts`), Eintrag in `src/landing/pages.ts` (steht schon: `href: 'lizenzen.html'`, Abschnitt `about`), oben ca. 70 px frei (`main` hat `padding-top: max(4.75rem, 70px)`), B nicht belegen, Links nach außen nur mit `target="_blank" rel="noopener"`. `tests/projectRules.test.ts` prüft das automatisch; `src/landing/serverCheck.test.ts` erwartet `lizenzen.html` in den Kacheln.
- Am TV prüft 🧑 die Seite nur, wenn er es für diesen Lauf freigibt; das ist kein Kriterium.

## Erlaubte Dateien

- `src/tools/` (Datenquelle aus GR6.1, `lizenzen.ts`, Tests)
- `lizenzen.html`
- `src/landing/pages.ts` (nur Beschreibung der Kachel, falls nötig)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Neue Assets, Sound-Credits, neue Seite neben `lizenzen.html`, Übersetzung (B-172), Umbau der Shell.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. CC-BY-Einträge auf der Seite hervorheben: Werk, alle Urheber, Lizenz, Quelle als Link.
3. Test: Für jeden Eintrag mit CC-BY-Lizenz enthält die erzeugte Seite Urheber, Lizenz und Quelle (aus der Datenquelle, nicht abgeschrieben).
4. Prüfen, dass `projectRules.test.ts` und `serverCheck.test.ts` grün sind; Kachel-Text bei Bedarf anpassen.
5. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-03: Test belegt Urheber, Lizenz und Quelle für jeden CC-BY-Eintrag auf der Seite.
- [ ] AC-04: Seite in `src/landing/pages.ts`, `installPageChrome()` aktiv, `task check` grün.
- [ ] Keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, TV) nur, wenn 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
