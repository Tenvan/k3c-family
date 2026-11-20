# B-1174 · Das Balancing berechnet Startwerte vorab, kalibriert sie in der Simulation und tunt über die Empfindlichkeit je Hebel

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU31
- **Projekt:** RSU
- **Erstellt:** 2026-11-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Frage von 🧑 (2026-11-20): Das Balancing geht heute von den gegebenen Werten der Welten aus, simuliert sie durch und justiert danach nach. Ablauf: Zielkorridore in `data/balance-targets.json` → Bots spielen bis zu 100 Seeds (`engine/balance`, in Rust `k3c-balance`, RU25 bis RU31) → Bewertung gegen die Korridore → Hebel mit fester Schrittweite aus `data/balance-hints.json` → Tuning als Sweep oder Suche über ein bis zwei Werte (`tune.go`). Startwerte werden nicht vorab berechnet; die Empfindlichkeit je Hebel (`sensitivity.go`) wird gemessen, aber nicht für die Schrittweite genutzt. Das kostet viele Simulationsläufe.

## Ziel

Das Balancing findet Werte schneller: Ein Rechenmodell liefert Startwerte, die die Zielkorridore rechnerisch treffen sollten, die Simulation prüft und kalibriert das Modell, und das Tuning wählt die Schrittweite aus der gemessenen Empfindlichkeit je Hebel statt aus einer festen Zahl. **Pflicht für den Welten-Editor und die MCP-Tools `world_*`:** Jede dort angelegte oder geänderte Welt bekommt eine berechnete, gesunde Grundbasis (Werte, die die Zielkorridore rechnerisch treffen) und wird gegen das Modell geprüft.

## Beteiligte und Zielgruppen

🧑 entscheidet über Spec, Umfang und die Aufhebung der Balancing-Sperre. Agenten setzen um; die Auswertung der Berichte bleibt bei den vorhandenen Einsichten. Wer Welten über den Welten-Editor oder die MCP-Tools baut (Agenten, 🧑), bekommt dadurch von Anfang an spielbare Werte.

## Anforderungen

Drei Stufen, jede einzeln nutzbar, dazu die Anbindung an den Welten-Editor:

1. **Rechenmodell für Startwerte** (`k3c-balance`, Modul `model`): Aus den Daten in `data/` (Wirtschaft, Gebäude, Truppen, Gegner, Wellen, Schwierigkeit, Koop) rechnet es ohne Simulation vor: Einkommen je Tag (Bauern, Erträge) gegen Baukosten und damit die Zeitpunkte erste Mauer, erster Turm, Hub-Stufe 2 und 3; Angriffs- gegen Verteidigungsstärke je Welle mit den Faktoren für Grad, Spielerzahl und Tiefe (`data/difficulty.json`). Aus einem Zielkorridor folgt ein Startwert je Hebel aus `balance-hints.json`. Umfang der ersten Fassung: Wirtschaft und Zeitpunkte; der Kampf nur grob (Schranken statt Punktwert). Vorbild im Bestand: `ship_need.go` (Schiff-Bedarf gegen Bestand).
2. **Kalibrierung durch Simulation:** Ein Lauf misst dieselben Kennzahlen, die das Modell vorhersagt. Die Abweichung je Kennzahl (Modell gegen Median der Seeds) wird als Bericht ausgegeben und als Korrekturfaktor je Kennzahl gespeichert (`data/`), damit das Modell beim nächsten Mal näher liegt. Dort, wo das Modell wegen Bot-Verhalten und Zufall nicht trägt (Laufwege, Leerlauf, Zusammenspiel), bleibt die Simulation allein maßgebend.
3. **Tuning über die Empfindlichkeit:** `Sensitivity` liefert je Hebel die Änderung der Kennzahl pro Einheit. `Tune` wählt daraus die Schrittweite (Schätzung wie Newton: Abstand zum Korridor geteilt durch Empfindlichkeit, mit Dämpfung und Grenzen aus den Hints) statt der festen Zahl; fehlt die Empfindlichkeit, gilt die feste Schrittweite wie bisher.
4. **Welten-Editor und MCP-Tools (Pflicht, 🧑 2026-11-20):** Welten lassen sich dort fast beliebig bauen (Inseln, Tiefen, Chunks, Bosse, Overrides). Darum gilt:
   - **Grundbasis beim Anlegen:** `world_create` und der Welten-Editor setzen eine vom Modell berechnete gesunde Grundbasis (Wirtschaft, Skalierung je Tiefe, Wellen- und Boss-Stärke aus der Basis-Welt und den Faktoren der Insel).
   - **Basisstand je Welt:** Die Welt-Datei trägt einen `balanceBase`-Block (Hash der balancerelevanten Werte, Modellversion, Korrekturfaktoren, ob kalibriert, Zeitpunkt). Er sagt, für welchen Stand der Welt die Basisdaten und die Kalibrierung gelten.
   - **Pflicht zum Neuberechnen (🧑 2026-11-20):** Nach dem Generieren und nach massiven Änderungen einer Welt müssen die Basisdaten für die Kalibrierung neu berechnet werden. „Massiv“ heißt: eine Insel, Tiefe, Boss oder Chunk-Art kommt hinzu oder fällt weg, oder das Modell sagt für mindestens eine Kennzahl eine Änderung über der Schwelle voraus (Vorschlag 10 % des Korridors). Bis dahin steht die Welt als „Basisdaten veraltet“ da.
   - **Anzeige:** Der Welten-Editor zeigt ein Banner „Basisdaten veraltet, Balancing-Läufe dieser Welt können ungültig sein“, dazu die Ampel je Kennzahl neben der Werte-Tabelle (B-1173). `world_check` und jede Antwort der Änderungs-Tools (`world_set`, `world_override_set`, `world_chunks_set`, `world_story_set`) nennen denselben Zustand und die Vorher-Nachher-Wirkung auf die Kennzahlen.
   - **Neuberechnen möglich:** Im Editor ein Knopf „Basisdaten neu berechnen“ und ein MCP-Tool `world_base_recalc` (mit `confirm`). Es rechnet die Grundbasis neu und schreibt `balanceBase`; die Kalibrierung (Stufe 2) läuft erst nach Widerruf der Sperre und wird bis dahin als „unkalibriert“ ausgewiesen.
   - **Alte Balancing-Läufe:** Berichte, Baselines und Replays tragen den Hash des Datenstands (`DataHash`, `baselines.go`). Stimmt er nicht mehr mit dem `balanceBase` der Welt überein, markieren Bericht, Baseline-Vergleich und die Tools `sim_test`/`level_generate` das Ergebnis als „über veraltetem Basisstand gerechnet“ (Warnung, kein Abbruch). Vergleiche zwischen Basisständen werden als solche gekennzeichnet.
   - **Aufruf:** über die Spiel-EXE (RU33, RU34), weil das Modell in Rust liegt und die Workbench kein Rust-Modell im Prozess rechnet.

## Nicht-Ziele

Das Modell ersetzt die Simulation nicht. Keine Änderung der Spielregeln oder Werte in `data/` durch dieses Ticket (nur neue Dateien für Korrekturfaktoren; die Grundbasis steht in der Welt-Datei der neuen Welt). Kein Machine-Learning. Kein Port von `engine/balance` (das ist RU25 bis RU31); diese Arbeit gehört **nicht** zu den Sessions RU31.1 bis RU31.6. Kein Spielbarkeitsbeweis: Das Modell macht die Welt rechnerisch gesund, ob sie Spaß macht, entscheidet der Spieltest.

## Regeln und Einschränkungen

Neue Mechaniken nur in Rust (Entscheidung 013, Entwurf): Code in `rust/crates/k3c-balance/`, die Go-Engine bekommt nichts; die Workbench ruft das Modell über die Spiel-EXE auf (RU34, Sperre `exe`). Port-Sprint RU31 bleibt ein reiner Port ohne Regeländerung; dieses Ticket kommt **nach** RU31.5 (Tuning-Port) und nutzt dessen Typen. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Keine neue Abhängigkeit ohne Freigabe 🧑.

**Abgrenzung zur Balancing-Sperre (🧑 2026-11-11, bis auf Widerruf):** Stufe 1, die Rechnung der Stufe 3 und die Welt-Prüfung des Welten-Editors (Punkt 4) sind reine Berechnung mit Unit-Tests auf festen Eingaben und brauchen keinen Balancing-Lauf; sie sind unter der Sperre erlaubt. Stufe 2 (Kalibrierung) und jeder Wirkungslauf sind Simulations- und Balancing-Läufe: Sie laufen erst nach Widerruf der Sperre durch 🧑, nur über `sim_test` und im Abnahme-Ticket des Sprints (B-742, Lane zweit darf sie nach der Ausnahme von 2026-11-18). Die Sperre selbst wird durch dieses Ticket nicht geändert. Solange Stufe 2 fehlt, rechnet das Modell mit Korrekturfaktor 1 und kennzeichnet die Prüfung als „unkalibriert“.

- **Bereich:** `rust/crates/k3c-balance/**`, `data/balance-model.json` (neu), `testdata/rust/balance/**`, für Punkt 4 zusätzlich `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/internal/mcpsrv/**`, `tools/k3c-workbench/frontend/src/world/**`
- **Abhängig von:** RU31.5 (Tuning-Port); Punkt 4 zusätzlich RU34 (Workbench ruft die Spiel-EXE) und WZ62 (Welten-Editor); Stufe 2 zusätzlich RU40 (Balancing-Läufe im Client) und Widerruf der Sperre
- **Braucht 🧑:** Freigabe, Entscheidung (Aufhebung der Sperre für Stufe 2; Pflicht als Sperre oder Warnung, siehe Offene Fragen)
- **Größe:** L
- **Sperre:** exe (Punkt 4, wie RU34)

## Beispiele

Ziel „Hub-Stufe 2 vor Tag 5 in 60 bis 85 % der Läufe“: Das Modell rechnet aus Einkommen und Kosten den Tag des Erreichens (z. B. Tag 4,3) und den Spielraum zum Korridor; die Simulation misst Tag 4,9. Die Kalibrierung speichert den Faktor 1,14. Das Tuning sieht Empfindlichkeit 0,8 Tage je 10 % Baukosten und wählt den Schritt, statt 0,1 fest zu probieren.

Welten-Editor: Ein Agent legt per `world_create` eine Welt „Nordmeer 2“ mit drei Inseln an. Die Welt bekommt sofort eine berechnete Grundbasis (Skalierung je Tiefe, Boss-HP, Wellenstärke); `world_check` meldet alle Kennzahlen als gesund. Setzt er per `world_override_set` die Boss-HP auf das Dreifache, meldet die Prüfung „Endboss-Tag über dem Korridor“ und schlägt vor, den Wert auf etwa das 1,6-fache zu senken.

## Ausnahme- und Fehlerfälle

Liegt die Abweichung Modell zu Simulation über der Schwelle, bleibt die Kennzahl vom Modell ausgenommen und das Tuning arbeitet wie bisher. Fehlt `data/balance-model.json`, rechnet das Modell mit Faktor 1. Zwei Hebel mit gegenläufiger Wirkung auf zwei Ziele werden gemeldet wie heute in `insights_levers`. Eine Welt mit Elementen, die das Modell nicht kennt (neue Biome, Events ohne Rechenregel), bekommt die Grundbasis für den bekannten Teil und den Hinweis „nicht abgedeckt“ für den Rest, nie eine falsche Zahl. Läuft die Spiel-EXE nicht, melden die `world_*`-Tools den Fehler und legen die Welt ohne Grundbasis an, mit Warnung.

## Akzeptanzkriterien

- **AC-01** (Stufe 1, ohne Lauf) Für eine feste Eingabe (Daten der Standard-Welt, Testdatei unter `testdata/rust/balance/`) liefert das Modell Zeitpunkte und Einkommen je Tag; Tests prüfen die Werte und dass jeder Startwert den Korridor rechnerisch trifft.
- **AC-02** (Stufe 3, ohne Lauf) Für einen festen Empfindlichkeits-Bericht wählt `Tune` den Schritt aus der Empfindlichkeit; Test: Der Zielwert liegt nach Rechnung im Korridor, bei fehlender Empfindlichkeit gilt die feste Schrittweite.
- **AC-03** (Stufe 2, nach Widerruf der Sperre, im Abnahme-Ticket) Modell und Simulation weichen bei den abgedeckten Kennzahlen im Median über 100 Seeds höchstens um 15 % ab; der Kalibrierbericht nennt die Abweichung je Kennzahl.
- **AC-04** (Nutzen, im Abnahme-Ticket) Für ein Ziel aus `balance-targets.json` braucht das Tuning mit Modell und Empfindlichkeit höchstens halb so viele Simulationsläufe wie mit fester Schrittweite (gleicher Ausgangswert).
- **AC-05** (Welten-Editor und MCP, ohne Lauf) Eine über `world_create` angelegte Welt trägt eine berechnete Grundbasis samt `balanceBase`, und `world_check` meldet für sie jede abgedeckte Kennzahl als gesund (Test mit mindestens zwei Welten mit unterschiedlicher Inselzahl); ein über `world_override_set` ungesund gesetzter Wert wird mit Kennzahl und Vorschlag gemeldet; der Welten-Editor zeigt die Ampel.
- **AC-06** (Basisdaten veraltet, ohne Lauf) Eine massive Änderung (neue Insel, Boss oder Chunk-Art; Kennzahl über der Schwelle) setzt die Welt auf „Basisdaten veraltet“; Editor-Banner, `world_check` und die Änderungs-Tools nennen es; `world_base_recalc` setzt `balanceBase` neu und hebt den Zustand auf (Tests mit fester Welt).
- **AC-07** (alte Läufe, ohne Lauf) Ein Bericht oder eine Baseline mit abweichendem `DataHash` wird als „über veraltetem Basisstand gerechnet“ gekennzeichnet; Test mit zwei Datenständen.

## Offene Fragen

- 🧑: Soll die erste Fassung nur Wirtschaft und Zeitpunkte abdecken (Vorschlag) oder auch den Kampf?
- 🧑: Genauigkeitsziel 15 % in AC-03 in Ordnung?
- 🧑: Wann wird die Balancing-Sperre für Stufe 2 aufgehoben (Vorschlag: nach RU40 und dem G4-Bericht)?
- Entschieden (🧑 2026-11-20): „Pflicht“ heißt: Nach Generieren und massiven Änderungen müssen die Basisdaten neu berechnet werden; das wird angezeigt und lässt sich auslösen, weil bisherige Balancing-Läufe ungültig werden können. Gemeint ist eine Pflicht zur Neuberechnung und Anzeige; ob Speichern oder ein Balancing-Lauf über veraltetem Stand hart blockiert wird, ist nicht entschieden (Default: Warnung, kein Abbruch).
- 🧑: Schwelle für „massiv“: Vorschlag 10 % des Korridors je Kennzahl oder Strukturwechsel (Insel, Tiefe, Boss, Chunk-Art); passt das?
- Einordnung: Eigener Folge-Sprint nach RU31 (Vorschlag `BL1`), weil RU31 ein reiner Port bleibt; bis dahin hängt das Ticket an RU31. Punkt 4 wäre dort eine eigene Session nach RU34 und WZ62.

## Notizen

Einordnung in die Planung: Kette RU25 (Bots), RU28 (Messungen), RU31 (Einsichten, Berichte, Tuning) portiert den Go-Stand ohne neue Funktion. Dieses Ticket setzt darauf auf: Stufe 1 und die Rechnung der Stufe 3 können direkt nach RU31.5 folgen (reine Rechnung), Punkt 4 (Welten-Editor und MCP) nach RU34 und WZ62, Stufe 2 nach RU40 (Balancing-Läufe im Client) und dem Widerruf der Sperre. Betroffene MCP-Tools: `world_create`, `world_check`, `world_set`, `world_override_set`, `world_chunks_set`, `world_story_set`. Quelle der Diskussion: Frage zum Balancing, 2026-11-20; Ralf: „eine berechnete gesunde Grundbasis wäre hier Pflicht“.
