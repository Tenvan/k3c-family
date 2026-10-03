# GR6 · PLAT · Credits-Seite

- **Status:** geplant
- **Domäne:** PLAT
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-165
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`lizenzen.html` zeigt die Danksagung, der Test `src/tools/grafikPacks.test.ts` prüft nur die zwölf Umgebungs-Packs. Figuren und künftiger Sound haben keinen Vollständigkeits-Test. Details in B-165.

## Ziel

Die Credits-Seite entsteht aus den CREDITS-Dateien, ein Test sichert Vollständigkeit. Am Ende sichtbar: Seite mit allen Einträgen auf der Landingpage, roter Test bei fehlendem Eintrag.

## Beteiligte und Zielgruppen

Spieler und Urheber; Entwickler und Agenten; 🧑 prüft die Seite am TV.

## Anforderungen

B-165 › Anforderungen.

## Nicht-Ziele

Neue Assets, Englisch, Projekt-`LICENSE`.

## Regeln und Einschränkungen

Seiten-Regeln aus `CLAUDE.md`: `installPageChrome()`, Eintrag in `src/landing/pages.ts`, `openPage()` und `goHome()`; `tests/projectRules.test.ts` grün.

## Beispiele

Neues Pack ohne CREDITS-Eintrag → Test rot.

## Ausnahme- und Fehlerfälle

Eintrag ohne Verzeichnis oder ohne Lizenz → Test rot.

## Akzeptanzkriterien

- **AC-01** Jedes Verzeichnis unter `public/grafik/` und `public/sprites/` hat einen Credit-Eintrag, ein fehlender Eintrag macht den Test rot (B-165/AC-01).
- **AC-02** Die Seite zeigt alle Einträge der CREDITS-Dateien, erzeugt aus den Dateien oder einer gemeinsamen Quelle (B-165/AC-02).
- **AC-03** CC-BY-Einträge zeigen Urheber, Lizenz und Quelle (B-165/AC-03).
- **AC-04** Die Seite ist über die Landingpage erreichbar, `task check` ist grün (B-165/AC-04).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GR6.1 Vollständigkeits-Test und gemeinsame Datenquelle (AC-01, AC-02).
- GR6.2 Seite mit CC-BY-Einträgen, Landingpage-Eintrag (AC-03, AC-04).
- GR6.3 Review.

## Abnahme

–
