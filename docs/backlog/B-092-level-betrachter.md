# B-092 · Eine Testseite zeigt ein generiertes Level (Seed und Biom) ohne zu spielen

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** U3
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-02 🧑 Chat („ja, so starten“; Revision 2: Start ohne `fresh`, B-096; umfasst Sprint U3)

## Ausgangslage

Wer ein Level beurteilen will (Balancing, Generator-Änderungen), muss es spielen oder `level_generate` über den Agenten aufrufen. Es gibt Testseiten für Szenarien (`testing.html`), Figuren und Grafiken, aber keine für das Level selbst.

## Ziel

Eine Seite `leveltest.html`: Seed und Biom wählen, das generierte Level als Streifen sehen (Abschnitte nach Art, Objekte nach Art, Portale, Hub, Ausgang), Warnungen der Prüfung lesen, und das Level mit einem Klick im Spiel starten. Die Seite ist von der Testseite (`testing.html`) aus erreichbar. Nutzen: Level in Sekunden beurteilen, auch auf dem Sofa mit Controller.

## Beteiligte und Zielgruppen

Entwickler, 🧑 beim Balancing; bedienbar mit Tastatur und Controller (Xbox). Einstieg von der Testseite (🧑 will das Level dort aufrufen können).

## Anforderungen

- Die Daten kommen von `GET /api/level` (B-091), die Seite rechnet kein Level selbst.
- Seed (freier Text, Zufalls-Seed per Taste/Knopf) und Biom (Auswahl) einstellbar; scrollen und zoomen mit linkem Stick bzw. Tastatur.
- Die Abbildung (Abschnitt/Objekt → Zeichenmodell) ist eine reine, getestete Funktion.
- `testing.html` bekommt einen Abschnitt „Level“ mit einer Kachel „Level-Betrachter“, die `leveltest.html` über `openPage()` öffnet; die Kachel ist wie die Szenarien-Kacheln mit Controller bedienbar (Steuerkreuz wählt, A öffnet). Die Kachel auf der Landingpage bleibt zusätzlich.
- Knopf „Im Spiel starten“ öffnet `game.html?autostart=1&save=<Seed>` (ohne `fresh`) über `openPage()`: Der Seed eines neuen Spiels ist der Name des Spielstands, daher nur für das Biom `forest` (Tiefe 0) und Seeds im Namensformat `^[a-z0-9-]{1,32}$`. Sonst ist der Knopf aus und nennt den Grund (Start mit beliebigem Seed und Höhle/Mine braucht B-095). Die Lobby lädt einen vorhandenen Spielstand mit diesem Namen (der Server lehnt `create` mit `fresh` bei vorhandenem Stand ab, B-096) oder legt ihn beim ersten Mal neu an; der Seitentext sagt das.
- Der Zufalls-Seed liefert immer einen startbaren Namen (Kleinbuchstaben und Ziffern, `crypto.getRandomValues`).

## Nicht-Ziele

Spielen oder Simulieren, Level bearbeiten, Levels speichern oder teilen; Start mit beliebigem Seed oder in Höhle und Mine (B-095).

## Regeln und Einschränkungen

Regel „Seiten & Navigation“ aus `CLAUDE.md`: `installPageChrome()`, Eintrag in `src/landing/pages.ts`, Vollbild nur über `toggleFullscreen()`, kein Seitenwechsel außer `openPage()`/`goHome()`; oben ca. 70 px frei; Taste B und View + Menu unbelegt. Domäne PLAT (`*.html`, `src/tools/`, `src/landing/`); `testing.html` und `src/tools/testing.ts` gehören dazu.

## Beispiele

Seed `test`, Biom `forest` → 22 Abschnitte à 50 Units, zwei Portale, Hub in der Mitte, ein Ausgang; Warnungsliste leer.

## Ausnahme- und Fehlerfälle

Seed oder Biom nicht startbar → „Im Spiel starten“ ist aus, der Grund steht daneben. Server nicht erreichbar oder 400 → klarer Hinweistext (z. B. „Server nicht erreichbar, `task start`“), keine leere Seite. Level mit Warnungen → Warnungen sichtbar und markiert.

## Akzeptanzkriterien

- **AC-01** `leveltest.html` ruft `installPageChrome()` auf und steht in `src/landing/pages.ts` (Abschnitt Test); `tests/projectRules.test.ts` ist grün.
- **AC-02** Die reine Abbildungs-Funktion ist mit Vitest geprüft (Abschnitte, Objekte nach Art, Portale, Hub, Ausgang, Warnungen).
- **AC-03** Das Golden-Level `testdata/golden/level-forest.json` (erstes Level) ergibt im Zeichenmodell genau seine Abschnitte und Objekte nach Art; das Beispiel Seed `test`, Biom `forest` hat 22 Abschnitte.
- **AC-04** Ohne Server zeigt die Seite einen Hinweis statt leerer Fläche (Test oder Beobachtung).
- **AC-05** 🧑 hat die Seite mit Controller am TV bedient (Seed ändern, scrollen, im Spiel starten).
- **AC-06** `testing.html` enthält den Abschnitt „Level“ mit der Kachel „Level-Betrachter“, die `leveltest.html` öffnet; ein Test prüft den Eintrag und dass die Controller-Auswahl die Kachel erreicht.
- **AC-07** Eine reine Funktion liefert die Start-URL nur für `forest` und Seeds im Namensformat (sonst `null` mit Grund); Vitest deckt gültige, zu lange, großgeschriebene und fremde Biome ab.

## Offene Fragen

keine. Entschieden 2026-10-02 durch 🧑 (Chat, „ja, so starten“): „Im Spiel starten“ startet ohne `fresh` und spielt einen vorhandenen gleichnamigen Spielstand weiter (Revision 2; die frühere Annahme, der Start ersetze ihn, stimmte nicht). Ein Export der Objekt-Positionen ist nicht Teil dieser Spec.

## Notizen

B-091 (Endpunkt) ist erledigt. Start mit Seed und Tiefe: B-095. U3 abgeschlossen, AC-05 (TV-Abnahme) wartet auf 🧑. Start über vorhandenen gleichnamigen Spielstand: B-096.
