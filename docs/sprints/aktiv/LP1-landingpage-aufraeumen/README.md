# LP1 · PLAT · Landingpage für Spieler, Entwicklerseite für Werkzeuge

- **Status:** aktiv
- **Domäne:** PLAT
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-335, B-292
- **Start-Commit:** 19c176e
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑 (Revision 1, Kachel-Zuordnung, dev.html, /dm im neuen Fenster, Grenzfall projectRules.test.ts; umfasst B-335 und B-292)

## Ausgangslage

Die Landingpage (`index.html`, Kacheln aus `src/landing/pages.ts`, gezeichnet von `src/landing/landing.ts`) mischt drei Spiel-Kacheln mit neun Werkzeug-Kacheln (Abschnitt „Tests & Werkzeuge“); `/dm` hat keine Kachel. Details: B-335. „Neues Spiel“ trägt bereits einen eigenen Spielstandnamen je Start (`newGameHref`, Test in `pages.test.ts`), der Nachweis im Browser fehlt: B-292 (bisher in PL1).

## Ziel

Die Familie sieht auf der Landingpage nur Spielen und Lizenzen, alle Werkzeuge liegen gegliedert auf einer eigenen Entwicklerseite. Am Ende sichtbar: Landingpage mit Weiterspielen, Neues Spiel, Online spielen, Lizenzen und einer kleinen Kachel „Entwicklung“; diese öffnet `dev.html` mit den Abschnitten Entwicklung, Performance und Balancing.

## Beteiligte und Zielgruppen

Familie am TV und PC (Landingpage); 🧑 als Entwickler und Tester (Entwicklerseite), nimmt am PC ab (LP1.4). Umsetzung autonom.

## Anforderungen

`B-335 › Anforderungen`, `B-292 › Anforderungen`, mit den Beschlüssen von 🧑 (2026-10-07, Chat):

- **Spieler-Kacheln:** Weiterspielen, Neues Spiel, Online spielen, Lizenzen & Danksagung; dazu **eine kleine** Kachel „Entwicklung“ → `dev.html` (kleiner und unauffälliger als die Spieler-Kacheln).
- **Entwicklerseite:** neue Seite `dev.html` (Skript `src/tools/dev.ts`), aufgebaut wie `testing.html`. Abschnitte: **Entwicklung** (Testing, Level-Betrachter, Gamepad-Test, Unsere Aufstellung, Alle Figuren, Alle Grafiken, Hörprobe, Dungeon Master), **Performance** (Monitor; Performance-Modi kommen mit B-334), **Balancing** (heute ohne Aufrufe: Hinweis „noch keine Aufrufe“).
- `testing.html` bleibt eine eigene Seite und ist Kachel auf der Entwicklerseite.
- **Dungeon Master** (`dm.html`, läuft außerhalb der Shell) öffnet in einem neuen Fenster bzw. Tab (`window.open`), nicht im Rahmen.
- Bedienung der Entwicklerseite mit Maus, Tastatur und Controller wie `testing.html` (Fokus-Weiterschaltung `nextFocus`, A klickt, B frei).

## Nicht-Ziele

Performance-Modi (B-334); Balancing-Seite (eigenes Ticket, sobald es Aufrufe gibt); Übersetzung von Shell und Werkzeug-Seiten (B-215, PL2); Passwortschutz der Entwicklerseite; Neugestaltung der Spieler-Kacheln; Server oder Protokoll ändern.

## Regeln und Einschränkungen

- Domäne PLAT (`src/landing/`, `src/tools/`, `*.html`). Seiten-Regeln aus `CLAUDE.md`: `dev.html` ruft `installPageChrome()` auf, Wechsel nur über `openPage()`, zurück über `goHome()`; B unbelegt, View + Menu reserviert.
- **Grenzfall INF (Beschluss 🧑 mit der Freigabe):** `tests/projectRules.test.ts` verlangt heute jede Seite in `PAGES`. LP1.1 erweitert die Prüfung nur so weit, dass eine Seite entweder auf der Landingpage oder auf der Entwicklerseite eingetragen sein muss; sonst bleibt der Test unverändert.
- Texte der Kacheln bleiben wie heute im Code (Deutsch); die Übersetzung gehört zu B-215/PL2.
- Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit.
- Laut Entscheidung 🧑 (2026-10-07) ruhen Anzeige- und Touch/Tasten-Abnahmen im Spiel bis B-337; die Abnahme LP1.4 betrifft nur die Seiten-Navigation (Landingpage ↔ Entwicklerseite).

## Beispiele

`B-335 › Beispiele`, `B-292 › Beispiele`. Dazu: Entwicklerseite → „Dungeon Master“ → `/dm` öffnet sich in neuem Tab, die Landingpage bleibt im alten offen. Abschnitt Balancing zeigt „noch keine Aufrufe“.

## Ausnahme- und Fehlerfälle

`B-335 › Ausnahme- und Fehlerfälle`, `B-292 › Ausnahme- und Fehlerfälle`. Pop-up-Sperre beim neuen Fenster für `/dm` → Hinweis auf der Seite mit der Adresse statt stiller Fehlschlag.

## Akzeptanzkriterien

- **AC-01** Landingpage nur mit den Spieler-Kacheln und genau einer kleinen Kachel zur Entwicklerseite (B-335/AC-01).
- **AC-02** Entwicklerseite listet alle bisherigen Werkzeug-Kacheln plus Dungeon Master, gegliedert nach Entwicklung, Performance und Balancing; jede führt zur richtigen Seite (B-335/AC-02).
- **AC-03** `task check` grün, `tests/projectRules.test.ts` deckt die neue Seite ab (B-335/AC-03).
- **AC-04** 🧑 hat Landingpage und Entwicklerseite am PC mit Tastatur abgenommen (B-335/AC-04).
- **AC-05** „Neues Spiel“ startet auch bei vorhandenem Spielstand `familie` (B-292/AC-01, B-292/AC-02, B-292/AC-03).

## Offene Fragen

keine (Kachel-Zuordnung, Aufbau der Entwicklerseite und `/dm` von 🧑 am 2026-10-07 entschieden)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| LP1.1 | `LP1.1-entwicklerseite.md` | Umsetzung | autonom | fertig |
| LP1.2 | `LP1.2-neues-spiel-nachweis.md` | Umsetzung | autonom | fertig |
| LP1.3 | `LP1.3-review.md` | Review | autonom | fertig |
| LP1.4 | `LP1.4-abnahme-pc.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-07, Review LP1.3 (Agent Claude Opus 5.5): AC-01, AC-03 (LP1.1: `pages.test.ts`, `projectRules.test.ts`, `task check` grün), AC-02 (LP1.1 `devTiles.test.ts`, LP1.2 alle neun Kacheln im Browser-Pane), AC-05 (LP1.2: B-292/AC-01–03, Log `🏰 Raum erstellt`, kein `save_exists`) mit Nachweis; AC-04 angenommen, Validierung offen (LP1.4, 🧑 am PC).
Behoben (schwer, hätte LP1.4 verhindert): Entwicklerseite war mit der Tastatur nur per Tab bedienbar; Pfeiltasten wählen jetzt wie auf der Landingpage. Grenzfall `projectRules.test.ts` geprüft: nur die Seiten-Prüfung geändert. B-292 archiviert; neues Ticket B-339 (Frage 🧑: Tastensymbole je Controller-Familie).
Version: v0.15.0 vorgeschlagen (Minor: neue Entwicklerseite, Landingpage nur für Spieler).
