# B-466 · Der Glossar-Eintrag Tageszyklus verweist für das Morgengrauen auf das richtige Ticket

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ13
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-17, Ralf im Chat (Revision 1)

## Ausgangslage

Die Regel Q65 (Tageszyklus aus vier Phasen Tag, Dämmerung, Nacht, Morgengrauen im Verhältnis 3:1:2:1, Startwert 6/2/4/2 min) ist beschlossen, aber noch nicht gebaut: `engine/sim/cycle.go` kennt drei Phasen `day`, `dusk`, `night` (Wald 10/1/5 min, `data/biomes/forest.json` › `cycle`), `dawn` ist nur ein Ereignis beim Beginn des Tages, und `src/scenes/viewRules.ts` zeichnet das Morgengrauen als letzte 10 % der Nacht. Der Text „heute 10/1/5 min ohne Morgengrauen“ im Glossar stimmt damit; falsch ist der Verweis auf das Umbau-Ticket: Mehrere Stellen nennen dafür **B-213**, doch B-213 ist ein anderes, erledigtes Ticket (MarkdownView in k3c-dev). Das Ticket für den Umbau ist **B-470** (Befund aus der Spec von B-460, 2026-10-17).

Stellen mit dem falschen Verweis auf B-213 (Zeilen Stand 2026-10-17):
- `docs/glossar.md` (INF): Z. 44 Dämmerung, Z. 45 `dawn`, Z. 121 Morgengrauen („Code-Name legt B-213 fest“), Z. 123 Nacht, Z. 182 Tag, Z. 183 Tagesanbruch, Z. 184 Tageszyklus („Umbau mit B-213“).
- `docs/rules/wirtschaft.md` (REG): Z. 31 (§ 3 Tageszyklus: „Heute (bis B-213)“ und „Umsetzung B-213“).
- Weitere Regel-Dokumente (REG): `docs/rules/zielkorridore.md` Z. 71 und 73 („Grenze heute (bis B-213)“), `docs/game-design.md` Z. 88.
- Nur genannt, nicht in diesem Ticket geändert: `docs/backlog/B-168-musik-je-zustand.md` Z. 62 und `docs/sprints/aktiv/SO4-musik/README.md` Z. 59 (Frage nach dem Musikstück im Morgengrauen; Verweis „klärt B-213“), `docs/sprints/geplant/BR1-balancing-wirtschaft/BR1.1-vorbereitung-messung.md` Z. 60 („bis B-213“), `docs/fragenkatalog.md` Z. 78, 177, 181, 447 (Beschlussprotokoll Q65, nennt B-213 als damals geplante Nummer).

## Ziel

Glossar und Regel-Dokumente behalten die Zielregel Q65 (vier Phasen 3:1:2:1, Startwert 6/2/4/2 min) unverändert und verweisen für den Umbau auf das richtige Ticket **B-470** statt auf B-213. Wer „bis B-213“ liest, landet nicht mehr bei einem fremden Ticket.

## Beteiligte und Zielgruppen

Agenten und 🧑, die Glossar und Regeln als verbindliche Begriffe lesen.

## Anforderungen

- In `docs/glossar.md` jede Nennung von B-213 für den Tageszyklus durch B-470 ersetzen (Z. 44, 45, 121, 123, 182, 183, 184); die Zielregel (Phasen, Verhältnis 3:1:2:1, Startwerte, Q65) bleibt wörtlich erhalten.
- Der Eintrag Morgengrauen (Z. 121) sagt statt „Code-Name legt B-213 fest“: noch nicht als Phase gebaut, B-470 baut sie; der Client zeichnet das Morgengrauen heute als letzte 10 % der Nacht.
- Der Eintrag Tageszyklus (Z. 184) nennt die heutigen drei Phasen 10/1/5 min (Wald) und `dawn` als Ereignis beim Beginn des Tages, Umbau mit B-470.
- In `docs/rules/wirtschaft.md` (Z. 31) „bis B-213“ und „Umsetzung B-213“ durch B-470 ersetzen; in `docs/rules/zielkorridore.md` (Z. 71, 73) und `docs/game-design.md` (Z. 88) dasselbe.
- Gleiche Schreibweise wie bisher („Heute (bis B-470): …“); keine inhaltliche Regel- oder Wertänderung.

## Nicht-Ziele

- Den Tageszyklus ändern oder bauen (B-470), Dauern oder Werte anpassen (REG, B-155).
- Die Zielregel Q65 ändern: Sie bleibt im Glossar und in den Regeln stehen.
- Tickets, Sprints und Sessions ändern, die B-213 nennen (B-168, SO4, BR1.1): Sie werden hier nur aufgelistet; ihre Verantwortlichen ziehen den Verweis bei Gelegenheit nach (Notiz).
- Chunk-Breite im Glossar (macht WZ8.1).

## Regeln und Einschränkungen

Reine Doku-Änderung, zwei Domänen: **INF** für `docs/glossar.md`, **REG** für `docs/rules/wirtschaft.md`, `docs/rules/zielkorridore.md` und `docs/game-design.md`. Das sind zwei kleine Änderungen (zwei Sessions oder als Nachtrag einer passenden INF- bzw. REG-Session); als Grenzfall in einer Session ist es vertretbar, weil nur ein Verweis ersetzt wird und keine Regel entsteht. Das Glossar ist verbindlich: Begriffe zuerst dort, Abweichungen gleicht der Review-Commit an (`docs/arbeitsweise.md`).

## Beispiele

- Glossar › Tageszyklus lautet danach: „… Startwert 6/2/4/2 min (14 min); läuft in allen Stufen. Heute drei Phasen 10/1/5 min (Wald), `dawn` nur als Ereignis beim Beginn des Tages, Umbau mit B-470.“ Die Zielregel davor bleibt unverändert.
- Glossar › Tag: „Startwert 6 min (heute Wald 10 min, bis B-470); …“ statt „bis B-213“.
- `docs/rules/wirtschaft.md` § 3: „Heute (bis B-470): Tag 10, Dämmerung 1, Nacht 5 min, `dawn` bei Beginn des Tages.“ und „Umsetzung B-470“.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Textkorrektur.

## Akzeptanzkriterien

- **AC-01** `docs/glossar.md` verweist für den Tageszyklus (Zeilen Dämmerung, `dawn`, Morgengrauen, Nacht, Tag, Tagesanbruch, Tageszyklus) auf B-470 und nicht mehr auf B-213. Nachweis: `git grep -n "B-213" docs/glossar.md` ist leer, `git grep -n "B-470" docs/glossar.md` zeigt die sieben Stellen.
- **AC-02** Die Zielregel Q65 (vier Phasen 3:1:2:1, Startwert 6/2/4/2 min, 14 min) steht unverändert im Glossar und in `docs/rules/wirtschaft.md`. Nachweis: `git diff` der beiden Dateien ändert nur die Verweise und die „Heute“-Angabe im Eintrag Tageszyklus, keine Zahl der Zielregel.
- **AC-03** `docs/rules/wirtschaft.md` verweist für das Morgengrauen auf B-470 statt auf B-213; dasselbe gilt für `docs/rules/zielkorridore.md` und `docs/game-design.md`. Nachweis: `git grep -n "B-213" -- docs/rules docs/game-design.md` ist leer.
- **AC-04** Die Einträge Morgengrauen und Tageszyklus im Glossar beschreiben den heutigen Code richtig (drei Phasen, `dawn` als Ereignis, Morgengrauen im Client als letzte 10 % der Nacht). Nachweis: Sichtprüfung gegen `engine/sim/cycle.go` und `src/scenes/viewRules.ts`.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): `docs/fragenkatalog.md` (Beschlussprotokoll Q65, Z. 78, 177, 181, 447) bleibt unverändert; es dokumentiert die damals geplante Nummer B-213. Offen: nichts.

## Notizen

- Gefunden beim Review der Specs WZ7–WZ10 (2026-10-17). Korrektur am selben Tag: Die erste Fassung vermutete, der Glossar-Text „ohne Morgengrauen“ sei veraltet; das trifft nicht zu, die Phase Morgengrauen ist nicht gebaut.
- Verwandt: B-470 (baut die Phase Morgengrauen), B-460 (Welt-Betrachter, bildet Morgengrauen bis B-470 als `night` mit 0,9 + 0,1·p ab), B-155 (Balancing der Dauern).
- Verweise in Tickets und Sprints, die diese Ticket-Änderung nicht anfasst, aber bei Gelegenheit auf B-470 gehören: B-168 Z. 62, SO4-README Z. 59, BR1.1 Z. 60.
