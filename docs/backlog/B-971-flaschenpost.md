# B-971 · Am Strand treibt eine Flaschenpost mit Geschichtszeile und Hinweis an

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WN4
- **Projekt:** WZN
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Die Geschichte (B-965) blendet nur an Wendepunkten ein; Erkunden wird davon nicht belohnt.

Stand im Code: Truhen sind Level-Objekte (`kind: "chest"`, 10–25 Gold, jede 3. gefundene gibt einen Skill-Punkt; `engine/sim/economy.go`, `docs/rules/wirtschaft.md` § 1); „versteckte“ Truhen gibt es nicht. Tagesanbruch meldet die Sim als Ereignis `dawn` je Stufe (`engine/sim/world_cycle.go`), den Abend als `dusk`. Einsiedler (B-972) und Reittier-Gatter (B-970) sind noch nicht umgesetzt (Sprint SI1). Nicht jede Insel hat Tiefe 0 (z. B. `nil` Insel 3 mit Tiefen 1, 2, 4).

## Ziel

Alle paar Tage liegt morgens am Strand eine Flaschenpost. Wer sie aufhebt, sieht eine Zeile Kunstsprache und bekommt eine Markierung, die zu einem lohnenden Ort der Insel führt.

Dieses Ticket liefert Regeltext, Daten und Simulation (Erscheinen, Aufheben, Flut, Ziel, Markierung, Spielstand). Darstellung im Spiel: B-980.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen). Claude schreibt Regel und Sim, 🧑 gibt frei; die Häufigkeit stimmt später das Balancing ab (derzeit gesperrt).

## Anforderungen

Entscheidungen von 🧑 (wörtlich übernommen):

- **Hinweis (🧑 2026-11-15): Markierung in der Welt.** Die Flasche zeigt einen Ort: versteckte Truhe, Einsiedler (B-972) oder Reittier-Gatter (B-970). Ohne Text verständlich.
- **Erscheinen (🧑 2026-11-15): morgens am Strand, selten.** Alle paar Tage liegt bei Tagesanbruch (`dawn`) eine Flasche an einem der beiden Inselenden auf Stufe 1, deterministisch aus dem Seed (`engine/rng`). Sie glitzert, der Monarch hebt sie im Vorbeireiten auf; bleibt sie liegen, holt die Flut sie am Abend.
- **Markierung (🧑 2026-11-15): Leuchtspur für beide, bis gefunden.** Nach dem Aufheben blendet die Geschichtszeile kurz ein (B-965), danach zeigt ein dezenter Pfeil am Bildrand beider Spielerhälften die Richtung, am Ziel leuchtet der Ort. Die Markierung bleibt, bis ein Monarch dort war; immer nur eine zur Zeit. (Pfeil, Leuchten, Einblendung: B-980.)
- Funktioniert mit 2 Spielern gleichzeitig.

Ergänzt:

- **R1 Rhythmus:** Die erste Flasche kann am Tag `firstDay` erscheinen, danach frühestens alle `everyDays` Tage; beide Werte stehen in `data/bottle.json` (Startwerte 3 und 4, vorläufig, Balancing später). Gezählt wird je Insel ab Ankunft.
- **R2 Ort:** Stufe 1 (Tiefe 0) der Insel, am linken oder rechten Inselende (Abstand zum Rand aus `data/bottle.json`); die Seite wählt ein eigener Zufallsstrom aus dem Insel-Seed und dem Tag (`engine/rng`), sodass andere Zufallsfolgen der Stufe unverändert bleiben.
- **R3 Bedingung:** Eine Flasche erscheint nur, wenn ein Ziel vorhanden ist (R5), keine Markierung läuft und keine Flasche liegt. Sonst verschiebt sich der Rhythmus auf den nächsten Tagesanbruch, an dem die Bedingung gilt.
- **R4 Aufheben und Flut:** Berührt ein lebender Monarch die Flasche (Aufhebe-Reichweite wie bei Münzen), ist sie aufgehoben (Ereignis `bottleFound` mit Spieler und Zeilen der Geschichte). Berühren zwei im selben Takt, gilt der höchste Spieler-Index. Liegt sie beim nächsten `dusk` noch, verschwindet sie (Ereignis `bottleLost`); der Rhythmus zählt ab ihrem Erscheinen weiter.
- **R5 Ziel:** Beim Aufheben wählt die Sim ein Ziel aus den lohnenden Orten der Insel über alle Stufen: heute jede noch nicht gefundene Truhe; Einsiedler (B-972) und Reittier-Gatter (B-970) kommen als weitere Arten hinzu, sobald es sie gibt (die Zielart ist ein Feld, die Liste der Arten erweiterbar). Auswahl deterministisch aus dem Flaschen-Zufallsstrom.
- **R6 Markierung:** Genau eine je Insel: Zielart, Stufe und Ort. Sie gilt für alle Spieler der Insel und endet, wenn ein lebender Monarch das Ziel erreicht (Reichweite aus `data/bottle.json`) oder das Ziel nicht mehr besteht (Truhe von jemandem gefunden) – Ereignis `bottleTargetReached`. Der Zustand der Stufe nennt Flasche und Markierung für die Darstellung.
- **R7 Geschichtszeile:** Die n-te aufgehobene Flasche der Kampagne nennt den Eintrag `n mod Anzahl` aus `story.bottles` der Welt (Feld und Prüfung aus B-965, Texte aus B-976); ohne `bottles` gibt es nur die Markierung.
- **R8 Spielstand:** Liegende Flasche, laufende Markierung, Tag der letzten Flasche und Zähler der aufgehobenen Flaschen stehen im Spielstand der Insel; `IslandSaveVersion` steigt um eins (nächste freie Version nach B-966), neue Fixture. Beim Inselwechsel verfallen Flasche und Markierung, der Zähler zieht mit.
- **R9 Determinismus:** Gleicher Seed und gleiche Eingaben ergeben dieselben Flaschen, Ziele und Ereignisse. Die Standard-Welt bekommt Flaschen; das Golden ändert sich gewollt (`task go:golden` mit Begründung im Commit).
- **R10 Regeltext:** `docs/rules/wirtschaft.md` bekommt den Abschnitt Flaschenpost (Rhythmus, Ort, Aufheben, Flut, Ziel, Markierung) im Tabellenformat der Regelwerke; Glossar: Flaschenpost, Markierung.

## Nicht-Ziele

Schwäche des Endgegners als Hinweis, feste sammelbare Flaschen je Insel, Flaschen als Folge anderer Ereignisse, Hinweis nur auf der Seekarte. Darstellung, Pfeil und Leuchten (B-980). Eigene Flaschenpost-Truhen. Einsiedler und Gatter selbst (B-972, B-970). Balancing der Häufigkeit (gesperrt).

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Sim in `rust/crates/k3c-sim` (Entscheidung 013); die Go-Engine bekommt keine neue Mechanik.
- Berührt Spielregeln und Sim; Häufigkeit später über Balancing (derzeit gesperrt). Keine Simulation, kein Balancing. Das Golden ändert sich gewollt; Rust schreibt eigene Golden erst nach RU41, bis dahin ändert die Sim-Session `testdata/golden/**` nur mit Begründung im Commit.
- Zufall nur über `k3c-rng`. Werte nach `data/bottle.json` (neu, über `rust/crates/k3c-data` geladen), nicht übersteuerbar je Welt (vorerst).
- Spielstand-Format: Version erhöhen, Fixture (`docs/arbeitsweise.md` › Spielstand-Format ändern); keine Rücksicht auf alte Spielstände. Die Version steigt nach B-966 (beide ändern das Format, daher die Abhängigkeit).
- Das Feld `story.bottles` und seine Prüfung stammen aus B-965, die Texte aus B-976; dieses Ticket nutzt sie nur.
- Review vor dem Commit (Domäne SIM, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `docs/rules/wirtschaft.md`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-data/**`, `data/bottle.json`, `testdata/golden/**`
- **Abhängig von:** B-965, B-966, RU37
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** golden

## Beispiele

Sengoku, Tag 5, Tagesanbruch: Am linken Inselende glitzert eine Flasche. Monarch 1 reitet darüber, eine Zeile Kunstsprache blendet ein, in beiden Hälften zeigt ein Pfeil nach rechts unten. Monarch 2 folgt ihm in Stufe 2 und findet die leuchtende Truhe; die Markierung erlischt.

Mit `firstDay` 3 und `everyDays` 4: Flasche am Tag 3; bleibt sie liegen, holt sie die Flut am Abend von Tag 3, die nächste kann am Tag 7 erscheinen. Wird sie am Tag 3 aufgehoben und das Ziel erst am Tag 9 erreicht, erscheint die nächste am Tag 10.

## Ausnahme- und Fehlerfälle

- Kein lohnender Ort mehr auf der Insel → keine Flasche (R3).
- Tagesanbruch, während eine Markierung läuft → keine neue Flasche; es bleibt bei einer Markierung.
- Insel ohne Tiefe 0 → keine Flaschenpost auf dieser Insel.
- Ziel-Truhe wird von einem Spieler gefunden, der die Flasche nicht aufgehoben hat → Markierung endet.
- Ziel liegt in einer Stufe, deren Hub gefallen ist → Markierung bleibt, bis das Ziel erreicht ist oder nicht mehr besteht.
- Spieler ohne Krone (Krone gefallen) berührt die Flasche → hebt sie nicht auf; gestorbener Spieler ebenso.
- Speichern mit liegender Flasche oder laufender Markierung → nach dem Laden unverändert.
- Inselwechsel mit laufender Markierung → verfällt.

## Akzeptanzkriterien

- **AC-01** `docs/rules/wirtschaft.md` hat den Abschnitt Flaschenpost mit Rhythmus, Ort, Aufheben, Flut, Ziel und Markierung und nennt `data/bottle.json`; `docs/glossar.md` führt Flaschenpost und Markierung.
- **AC-02** Go-Test in `engine/sim`: Mit festem Seed erscheint die Flasche bei `dawn` am Tag `firstDay` an einem Inselende der Tiefe 0; zwei Läufe mit gleichem Seed ergeben denselben Ort und dieselbe Seite.
- **AC-03** Go-Test: Ein Monarch in Reichweite hebt die Flasche auf (`bottleFound`); zwei Monarchen im selben Takt → höchster Spieler-Index; ohne Aufheben verschwindet sie bei `dusk` (`bottleLost`).
- **AC-04** Go-Test: Das Ziel ist eine noch nicht gefundene Truhe der Insel (auch in einer anderen Stufe); die Markierung endet, wenn ein Monarch das Ziel erreicht oder die Truhe gefunden wird (`bottleTargetReached`); währenddessen erscheint keine neue Flasche; ohne Ziel erscheint keine Flasche.
- **AC-05** Go-Test: Der Zustand jeder Stufe nennt Flasche (Ort) und Markierung (Zielart, Stufe, Ort) für alle Spieler der Insel gleich.
- **AC-06** Go-Test: Die n-te Flasche nennt Eintrag `n mod Anzahl` aus `story.bottles`; ohne `bottles` nennt `bottleFound` keine Zeilen.
- **AC-07** Go-Test: Spielstand mit liegender Flasche und mit laufender Markierung speichert und lädt byte-gleich; `IslandSaveVersion` erhöht, neue Fixture; Inselwechsel verwirft Flasche und Markierung, der Zähler zieht mit.
- **AC-08** Go-Test: Insel ohne Tiefe 0 bekommt nie eine Flasche; die übrigen Zufallsfolgen der Stufe sind mit und ohne Flaschenpost gleich (Level und Wellen eines festen Seeds unverändert).
- **AC-09** `check_run task:check:go` grün; das Golden ist neu geschrieben, der Commit begründet die Änderung (nur neue Flaschen-Ereignisse und -Zustand).

## Offene Fragen

- Abstand in Tagen? → Default: erste Flasche ab Tag 3, danach frühestens alle 4 Tage (vorläufig, Balancing später, `data/bottle.json`).
- Gibt es heute versteckte Truhen als Ziel, oder braucht es eine eigene Flaschenpost-Truhe? → Geprüft: Es gibt nur gewöhnliche Truhen im Level. Default: Ziel ist eine noch nicht gefundene Truhe der Insel, keine eigene Flaschenpost-Truhe (vorläufig, 🧑 bestätigt).
- Was enthält die Flasche, wenn kein Ziel mehr übrig ist? → Default: Dann erscheint keine Flasche (vorläufig, 🧑 bestätigt).
- Welche Zeilen der Geschichte: eigene Flaschenpost-Zeilen je Welt? → Default: ja, `story.bottles` je Welt, reihum; Format in B-965, Texte in B-976 (vorläufig, 🧑 bestätigt).
- Neue Flasche, während eine Markierung läuft? → Default: Es erscheint keine (vorläufig, 🧑 bestätigt).
- Wer hebt auf? → Default: jeder lebende Spieler mit Krone, im Vorbeireiten ohne Taste (vorläufig).
- Ziele Einsiedler und Gatter → Default: keine Abhängigkeit; die Zielart ist erweiterbar, B-972 und B-970 melden ihre Orte nach, wenn sie umgesetzt sind (vorläufig).

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt. 2026-11-15 nach GS4 verschoben und zerlegt: Darstellung im Spiel → B-980. Die Spielstand-Version teilt sich die Sperre `spielstand` mit GS3 (B-966); GS4 hängt deshalb von GS3 ab, damit die Versionen nacheinander steigen.
