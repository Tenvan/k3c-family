# B-165 · Eine Credits-Seite entsteht aus den CREDITS-Dateien, ein Test prüft die Vollständigkeit

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** GR6
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; mit Sprint GR6; mit Änderungen aus dem Spec-Review (CREDITS-Dateien als einzige Quelle, Nicht-Ziel Übersetzung nach Q05)

## Ausgangslage

Lizenzen stehen in `public/grafik/CREDITS.md` und `public/sprites/CREDITS.md`; die Danksagung im Spiel ist `lizenzen.html` (Script `src/tools/lizenzen.ts`, Packs in `src/tools/grafikPacks.ts`) und muss mit den CREDITS-Dateien übereinstimmen; `src/tools/grafikPacks.test.ts` prüft das für die zwölf Umgebungs-Packs. Für Figuren und künftig Sound (B-167, B-168) gibt es keinen Vollständigkeits-Test. CC-BY-Werke (Warped Caves, LPC-Reittiere, Musik) verlangen sichtbare Namensnennung im Produkt.

## Ziel

Die Credits-Seite wird aus den CREDITS-Dateien erzeugt, und ein Test stellt sicher, dass jedes Asset-Verzeichnis einen Credit-Eintrag hat. Nutzen: Keine CC-BY-Pflicht wird vergessen, Seite und Dateien laufen nicht auseinander.

## Beteiligte und Zielgruppen

Spieler (sehen die Danksagung), Urheber der Assets, Entwickler und Agenten (Test); 🧑 prüft die Seite am TV.

## Anforderungen

- Die Credits-Seite (Erweiterung von `lizenzen.html` oder Ersatz, Entscheidung in der Session) erzeugt ihre Einträge aus den CREDITS-Dateien als einziger Quelle (`?raw`-Import), nicht aus Handtext und nicht aus einer zweiten Datei.
- Test: Jedes Verzeichnis unter `public/grafik/` und `public/sprites/` (später `public/audio/`) hat einen Eintrag in der zugehörigen CREDITS-Datei und auf der Seite.
- CC-BY-Einträge zeigen Urheber, Lizenz und Quelle sichtbar.
- Die Seite hält die Regeln aus `CLAUDE.md` ein (Home-Button, Eintrag in `src/landing/pages.ts`).

## Nicht-Ziele

Neue Assets, Lizenz des Projekts (`LICENSE`), Übersetzung der Seitentexte (Q05: Deutsch und Englisch, zentrale Texte über B-172 in S5).

## Regeln und Einschränkungen

`installPageChrome()` aus `src/core/shell.ts`, Eintrag in `src/landing/pages.ts`, Seitenwechsel nur über `openPage()` und `goHome()`; oben ca. 70 px frei; B nicht belegen; `tests/projectRules.test.ts` bleibt grün. Werte und Texte aus den CREDITS-Dateien, keine Duplikate pflegen.

## Beispiele

Neues Pack unter `public/grafik/` ohne CREDITS-Eintrag → Test rot. CREDITS-Eintrag ergänzt → Seite zeigt ihn nach dem Build.

## Ausnahme- und Fehlerfälle

CREDITS-Eintrag ohne Verzeichnis (Asset entfernt) → Test rot. Eintrag ohne Lizenz oder Quelle → Test rot.

## Akzeptanzkriterien

- **AC-01** Test: Jedes Verzeichnis unter `public/grafik/` und `public/sprites/` hat einen Eintrag in der zugehörigen CREDITS-Datei; ein Verzeichnis ohne Eintrag macht den Test rot.
- **AC-02** Die Credits-Seite zeigt alle Einträge der CREDITS-Dateien, erzeugt aus den CREDITS-Dateien als einziger Quelle (Test).
- **AC-03** Alle CC-BY-Einträge (Warped Caves, LPC-Reittiere) zeigen Urheber, Lizenz und Quelle auf der Seite (Test).
- **AC-04** Die Seite ist über die Landingpage erreichbar, hat `installPageChrome()`; `task check` ist grün.

## Offene Fragen

keine

## Notizen

Lücke 17 aus `docs/plan-weiterentwicklung.md` § 4. Sound-Credits kommen mit B-167 und B-168 dazu.
