# B-965 · Jede Welt und Insel erzählt eine kurze Geschichte in Kunstsprache mit eingestreuten echten Wörtern

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GS1
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

🧑 2026-11-15: Welten und Inseln haben keine Geschichte. Ausformulierte Texte liest niemand, wenn es zu viel wird.

Stand im Code: Die Welt-Datei (`data/worlds/<id>.json`, `docs/rules/welten.md` § 2) kennt nur `id`, `name`, `notes`, `extends`, `overrides`, `combat`, `islands`; unbekannte Felder lehnt der strikte Decoder ab (`engine/sim/island_worlds.go`, `island_data.go`). Etappen und Zielwahl sind umgesetzt (B-961, `engine/sim/island_targets.go`). Die Sim meldet die Wendepunkte schon als Ereignisse (`bossSpawned`, `bossDefeated`, `islandSwitch`). Vorhandene Welten: `standard` (eingecheckt), `sengoku`, `nil`, `britannien` (lokal, nicht eingecheckt, `welten.md` § 7). Eine Welt „Polar“ gibt es noch nicht.

## Ziel

Jede Welt und jede Insel hat eine kleine Geschichte mit den Themen Start, Etappen und Endgegner. Sie steht in einer Kunstsprache aus wenigen Wörtern; kurze echte Wörter im Satz deuten die Handlung nur an.

Dieses Ticket liefert Regeltext, Format der Welt-Datei und die Engine-Funktionen, die zu einem Wendepunkt die Zeilen nennen. Anzeige: B-974. Werkzeuge: B-975. Texte der vier Welten: B-976.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen im Split-Screen) lesen die Zeilen. Claude schreibt Regel, Format und Texte, 🧑 gibt die Spec frei und nimmt die Texte ab (B-976).

## Anforderungen

Entscheidungen von 🧑 aus der Ideenrunde (wörtlich übernommen):

- Geschichte je Welt und je Insel: Start, Etappen, Endgegner.
- Kunstsprache mit kleinem Wortschatz, dazwischen 2 bis 3 echte Wörter je Zeile als Andeutung.
- **Machart (🧑 2026-11-15): je Welt eigener Klang.** Jede Welt hat eigene Laute und einen eigenen kleinen Wortschatz (Sengoku japanisch, Nil ägyptisch, Polar nordisch); die echten Wörter tragen den Sinn.
- **Anzeige (🧑 2026-11-15): Einblendung an Wendepunkten.** Ein bis zwei Zeilen blenden kurz ein und wieder aus: bei Ankunft auf der Insel, beim Abschluss einer Etappe, vor dem Endgegner. Kein Tastendruck, das Spiel läuft weiter. (Umsetzung der Anzeige: B-974.)
- **Entstehung (🧑 2026-11-15): Claude schreibt, 🧑 nimmt ab.** Je Welt ein kleiner Wortschatz und feste Zeilen in der Welt-Datei, geschrieben über die Welten-MCP-Tools; „Welt anlegen“ erzeugt die Geschichte gleich mit. (Werkzeuge: B-975, Texte: B-976.)
- Kurz: wenige Zeilen, nichts zum Durchlesen.

Ergänzt (Format und Engine, Defaults siehe Offene Fragen):

- **R1 Format Welt:** Die Welt-Datei bekommt das optionale Feld `story` mit `lexicon` (Wortschatz der Kunstsprache, Liste Kleinbuchstaben-Wörter), `start` (Zeilen beim Start einer neuen Kampagne), `etappen` (Objekt Etappen-Nummer → Zeilen beim Abschluss dieser Etappe), `end` (Zeilen beim Kampagnensieg) und `bottles` (Liste von Einträgen für die Flaschenpost; hier nur Format und Prüfung, genutzt erst von B-971).
- **R2 Format Insel:** Jede Insel bekommt das optionale Feld `story` mit `arrival` (Ankunft), `boss` (vor dem Endgegner), `done` (Abschlusszeile nach dem Sieg über den Endboss; B-966 zeigt sie als Andenken).
- **R3 Zeilen:** Jeder Eintrag ist eine Liste aus 1 bis 2 Zeilen. Echte Wörter stehen in GROSSBUCHSTABEN, Kunstwörter klein (am Satzanfang groß begonnen); je Zeile 2 bis 3 echte Wörter. Echte Wörter sind deutsch und werden nicht übersetzt.
- **R4 Prüfung beim Laden** (`welten.md` § 3, fail closed wie die übrigen Felder): unbekanntes Feld in `story`, mehr als 2 Zeilen je Eintrag, leere Zeile, Zeile länger als die Obergrenze, Kunstwort nicht im `lexicon` (Groß-/Kleinschreibung egal), weniger als 2 oder mehr als 3 echte Wörter, `lexicon` größer als die Obergrenze, Etappen-Nummer ohne Insel dieser Etappe → Welt wird abgelehnt, Meldung nennt Welt, Insel, Feld und Zeile. Obergrenzen (Zeilenlänge, Größe des Wortschatzes) stehen als Werte in `data/story.json`, nicht im Code.
- **R5 Alles optional:** Fehlt `story` (Welt oder Insel) oder ein Eintrag, gibt es dort keine Zeilen; nichts anderes ändert sich. `extends` vererbt die Geschichte nicht (Inseln erbt eine Welt auch nicht).
- **R6 Wendepunkte (Engine nennt die Zeilen):** Reine Funktionen in `engine/sim` liefern zu einem Wendepunkt die Zeilen, ohne Zustand, ohne neue Ereignisse und ohne Zufall:
  - Neue Kampagne: `start` der Welt, danach `arrival` der Startinsel.
  - Ankunft nach einem Inselwechsel (`islandSwitch`): `arrival` der Zielinsel.
  - Endboss der Insel ausgelöst (`bossSpawned` des Endbosses): `boss` der Insel.
  - Endboss der Insel besiegt (`bossDefeated` des Endbosses): `done` der Insel; schließt der Sieg die Etappe ab (alle übrigen Inseln der Etappe abgeschlossen), danach `etappen[<Nr>]`; ist es die letzte Insel der Welt, statt dessen `end`.
  - Laden eines Spielstands löst keine Zeilen aus.
- **R7 Determinismus und 2 Spieler:** Dieselbe Welt und derselbe Wendepunkt ergeben immer dieselben Zeilen; sie gelten für alle Spieler der Insel gleich, in welcher Stufe sie auch stehen. Simulation, Golden und Spielstand ändern sich nicht.
- **R8 Regeltext:** `docs/rules/welten.md` beschreibt Format (§ 2), Prüfung (§ 3) und Wendepunkte (neuer Abschnitt bei § 6); die Begriffe Geschichte, Kunstsprache, Wortschatz, Wendepunkt, Abschlusszeile stehen im Glossar.

## Nicht-Ziele

Ausformulierte Texte, Dialoge, Vertonung (vorerst). Anzeige im Spiel (B-974). Welten-Werkzeuge und Editor (B-975). Die Texte selbst (B-976). Mechanik der Flaschenpost (B-971; hier nur Format und Prüfung von `story.bottles`). Übersetzung der echten Wörter. Neue Sim-Ereignisse, Änderung am Spielstand.

## Regeln und Einschränkungen

- Entscheidung 001: Logik in `engine/`, das Spiel zeichnet nur. Welt-Format und Prüfung nach `docs/rules/welten.md` § 2–3 (strikter Decoder, Meldungen deutsch).
- Sim-Code läuft im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`). Keine Simulation, kein Balancing.
- Werte (Obergrenzen) nach `data/story.json`, vorläufig.
- Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen.

- **Bereich:** `docs/rules/welten.md`, `engine/sim/**`, `data/story.json`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Zeilen (von 🧑 in der Ideenrunde gewählt; je Zeile 2 bis 3 echte Wörter, eine Zeile darf zwei kurze Sätze haben):

- Sengoku: „Oru kamae KRONE no sen. Shiro nu BRENNT.“
- Nil: „Akh'et ra KRONE sen-hotep. Ma'u FLUT kha, kha SAND.“
- Polar: „Hjørn vald KRONE ok ís. Skal EIS brjóta – VOLK ska.“

Format (Ausschnitt `data/worlds/sengoku.json`, Texte nur als Muster, die echten schreibt B-976):

```json
"story": { "lexicon": ["oru", "kamae", "no", "sen", "shiro", "nu"],
  "start": ["Oru kamae KRONE no sen. Shiro nu BRENNT."],
  "etappen": { "1": ["Sen no MEER, oru KRONE kamae."] },
  "end": ["Shiro no FRIEDE, kamae KRONE sen."] },
"islands": [ { "id": "island1", "name": "Reisfeld-Ebene", "story": {
  "arrival": ["Oru no REIS, sen SCHIFF nu."],
  "boss": ["Nu kamae MASKE, shiro TIEF."],
  "done": ["MASKE nu, KRONE oru sen."] } } ]
```

Welt mit Etappen 1, 2, 2 (Inseln a, b, c): Sieg über den Endboss von a → `done` von a, danach `etappen["1"]`. Sieg auf c (b noch offen) → nur `done` von c. Sieg auf b danach, b ist die letzte Insel → `done` von b, danach `end`.

## Ausnahme- und Fehlerfälle

- Welt ohne `story` → keine Zeilen, kein Fehler.
- Insel ohne `boss` → am Endboss keine Zeile, die übrigen Wendepunkte bleiben.
- Kunstwort fehlt im `lexicon` → Welt abgelehnt: `welt "sengoku": story.start Zeile 1: Wort "kame" fehlt im lexicon`.
- Zeile mit nur einem echten Wort oder mit vier → abgelehnt mit Zeile und Anzahl.
- `etappen` nennt eine Etappe, die keine Insel hat → abgelehnt.
- Satzzeichen, Apostroph und Bindestrich in Kunstwörtern (`akh'et`, `sen-hotep`) sind erlaubt; geprüft wird das Wort ohne Satzzeichen am Rand.
- Miniboss besiegt oder ausgelöst → keine Zeile (nur Endboss).
- Siegvariante ohne Endboss (z. B. Tage überleben) → `done` und `end` beim Sieg der Variante auf der letzten Insel; auf früheren Inseln bleibt es beim Endboss (heutige Bedingung des Inselwechsels).

## Akzeptanzkriterien

- **AC-01** `docs/rules/welten.md` beschreibt die Felder `story` der Welt und der Insel (§ 2), die Prüfungen mit Meldungen (§ 3) und die Wendepunkte mit Beispiel (bei § 6); `docs/glossar.md` führt Geschichte, Kunstsprache, Wortschatz, Wendepunkt, Abschlusszeile.
- **AC-02** Go-Test in `engine/sim`: Eine Welt-Datei mit `story` wie im Beispiel lädt; eine Welt ohne `story` lädt unverändert (bestehende Welt-Tests und Golden bleiben grün, `check_run task:check:go`).
- **AC-03** Go-Test: Jede Prüfung aus R4 lehnt mit einer Meldung ab, die Welt, Feld und Zeile nennt (Tabellentest, je Fall eine Zeile).
- **AC-04** Go-Test: Die Wendepunkt-Funktionen liefern für die Welt aus dem Beispiel (Etappen 1, 2, 2) die Zeilen aus „Beispiele“ für Start, Ankunft, Endboss ausgelöst, Endboss besiegt mit und ohne Etappen-Abschluss und für die letzte Insel.
- **AC-05** Go-Test: Zwei Aufrufe mit derselben Welt und demselben Wendepunkt liefern dieselben Zeilen; die Funktionen ändern weder Insel noch Stufe (Zustand vorher = nachher), der Spielstand einer Insel mit Geschichte ist byte-gleich zu dem ohne.
- **AC-06** `data/story.json` enthält die Obergrenzen (Zeilenlänge, Wortschatz) und ist eingebettet (`data/embed.go` liest `*.json`); der Test aus AC-03 nutzt sie.

## Offene Fragen

- Einblendung im Split-Screen: je Spielerhälfte oder über die ganze Breite? → Default: je Spielerhälfte, in beiden Hälften gleichzeitig (vorläufig, 🧑 bestätigt); umgesetzt in B-974.
- „Abschluss einer Etappe“: Etappe der Welt (Glossar, B-961) oder ein Abschnitt innerhalb der Insel? → Default: Etappe der Welt, also beim Sieg über den Endboss der letzten offenen Insel der Etappe (vorläufig, 🧑 bestätigt). Ohne `etappe` ist jede Insel eine eigene Etappe; dann folgt die Etappenzeile auf jede Abschlusszeile, wenn die Welt sie angibt.
- „Vor dem Endgegner“: welcher Moment? → Default: wenn der Endboss ausgelöst wird (Ankunft eines Spielers im Bau, Ereignis `bossSpawned`) (vorläufig, 🧑 bestätigt).
- Sprache der echten Wörter → Default: deutsch, unübersetzt (vorläufig, 🧑 bestätigt).
- Obergrenzen → Default: Zeile höchstens 64 Zeichen, Wortschatz höchstens 24 Wörter je Welt (vorläufig, in `data/story.json`).
- Engine nennt Wendepunkte über Funktionen statt über neue Ereignisse → Default: Funktionen, damit Golden und Spielstand unberührt bleiben (vorläufig, 🧑 bestätigt).
- Welt „Polar“ gibt es nicht → Default: Klang nordisch gilt, sobald die Welt entsteht; heute bekommen `standard`, `sengoku`, `nil`, `britannien` eine Geschichte (B-976).

## Notizen

Idee aus der Ideenrunde k3c-07 (2026-11-15), Grundzüge mit 🧑 geklärt. 2026-11-15 zerlegt (vorher Größe L): Anzeige → B-974, Werkzeuge → B-975, Texte → B-976; dieses Ticket behält Regel, Format und Engine. Spec bleibt Entwurf bis zur Freigabe durch 🧑.
