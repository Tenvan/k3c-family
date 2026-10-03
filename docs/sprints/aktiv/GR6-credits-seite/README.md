# GR6 · PLAT · Credits-Seite

- **Status:** aktiv
- **Domäne:** PLAT
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-165
- **Start-Commit:** 605f467
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; umfasst B-165; mit Änderungen aus dem Spec-Review (CREDITS-Dateien als einzige Quelle, Nicht-Ziel Übersetzung nach Q05)

## Ausgangslage

`lizenzen.html` zeigt die Danksagung, der Test `src/tools/grafikPacks.test.ts` prüft nur die zwölf Umgebungs-Packs. Figuren und künftiger Sound haben keinen Vollständigkeits-Test. Details in B-165.

## Ziel

Die Credits-Seite entsteht aus den CREDITS-Dateien, ein Test sichert Vollständigkeit. Am Ende sichtbar: Seite mit allen Einträgen auf der Landingpage, roter Test bei fehlendem Eintrag.

## Beteiligte und Zielgruppen

Spieler und Urheber; Entwickler und Agenten; 🧑 prüft die Seite am TV.

## Anforderungen

B-165 › Anforderungen.

## Nicht-Ziele

Neue Assets, Übersetzung der Seitentexte (Q05: Deutsch und Englisch, zentrale Texte über B-172 in S5), Projekt-`LICENSE`.

## Regeln und Einschränkungen

Seiten-Regeln aus `CLAUDE.md`: `installPageChrome()`, Eintrag in `src/landing/pages.ts`, `openPage()` und `goHome()`; `tests/projectRules.test.ts` grün.

## Beispiele

Neues Pack ohne CREDITS-Eintrag → Test rot.

## Ausnahme- und Fehlerfälle

Eintrag ohne Verzeichnis oder ohne Lizenz → Test rot.

## Akzeptanzkriterien

- **AC-01** Jedes Verzeichnis unter `public/grafik/` und `public/sprites/` hat einen Credit-Eintrag, ein fehlender Eintrag macht den Test rot (B-165/AC-01).
- **AC-02** Die Seite zeigt alle Einträge der CREDITS-Dateien, erzeugt aus den CREDITS-Dateien als einziger Quelle, ohne zweite gepflegte Kopie (B-165/AC-02).
- **AC-03** CC-BY-Einträge zeigen Urheber, Lizenz und Quelle (B-165/AC-03).
- **AC-04** Die Seite ist über die Landingpage erreichbar, `task check` ist grün (B-165/AC-04).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR6.1 | `GR6.1-vollstaendigkeit-datenquelle.md` | Umsetzung | autonom | fertig |
| GR6.2 | `GR6.2-cc-by-seite.md` | Umsetzung | autonom | in Arbeit |
| GR6.3 | `GR6.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
