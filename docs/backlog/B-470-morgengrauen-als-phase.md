# B-470 · Der Tageszyklus hat eine eigene Phase Morgengrauen (Q65, 3:1:2:1)

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** W12
- **Projekt:** WRT
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Regel Q65 (2026-10-04, `docs/rules/wirtschaft.md` § 3, Glossar › Tageszyklus) verlangt einen Tageszyklus aus **vier Phasen** Tag, Dämmerung, Nacht, Morgengrauen im Verhältnis 3:1:2:1, Startwert **6/2/4/2 min** (14 min). `dawn` (Gold je Spieler, Taverne, Händler, Autosave, Gegner ziehen ab) fällt nach Q65 bei Beginn des Morgengrauens.

Gebaut sind heute **drei Phasen** `day`, `dusk`, `night` mit **10/1/5 min** im Wald (`data/biomes/forest.json` › `cycle`: `dayMinutes`, `twilightMinutes`, `nightMinutes`; `cycleAt` und `stepCycle` in `engine/sim/world_cycle.go`). `dawn` ist nur ein **Ereignis** beim Übergang Nacht → Tag, nicht eine Phase. Der globale Zyklus läuft auch unter Tage weiter (`globalDayNight`).

Der Client zeichnet das Morgengrauen heute als **letzte 10 % der Nacht**: `daylight()` in `src/scenes/render/viewRules.ts` (Z. 30–34) liefert für `night` bei Fortschritt über 0,9 eine Helligkeit von 0 bis 1. Eine unbekannte Phase wird wie `night` behandelt; bei Fortschritt über 0,9 ergibt `daylight()` daher auch für sie eine Helligkeit über 0 (`viewRules.ts:34`), sie ist also nicht durchgehend dunkel. Eine neue Phase `dawn` würde der Client ohne Anpassung nur so behandeln und nicht als eigene Phase mit eigenem Helligkeitsverlauf zeichnen.

Weitere Stellen, die die Phasen kennen: `sim.DevSetPhase` (`engine/sim/dev.go`, Z. 49–56) springt mit einer Tabelle `starts`/`length` ohne `dawn` in die Phasen; `engine/sim/campaign.go` (Z. 82) bezieht sich auf die Phasen des Zyklus. Verweise „bis B-470“ stehen in `docs/rules/zielkorridore.md` (Z. 71 und 73) und `docs/game-design.md` (Z. 88).

Der Welt-Betrachter im Asset-Studio (B-460) bildet das Morgengrauen bis dahin als Phase `night` mit Fortschritt 0,9 + 0,1·p ab.

## Ziel

Der Tageszyklus hat eine eigene Phase `dawn` (Morgengrauen) zwischen Nacht und Tag: Sie ist in `engine/sim/world_cycle.go` gebaut, ihre Dauer steht in `data/biomes/*.json` › `cycle`, der Client zeichnet sie als eigene Phase, und das Ereignis `dawn` fällt bei ihrem Beginn (Q65). Die Abbildung des Morgengrauens als letzte 10 % der Nacht im Welt-Betrachter (B-460) entfällt.

## Beteiligte und Zielgruppen

Spieler erleben vier Phasen statt drei; 🧑 entscheidet über die Spec und nimmt ab. Agenten setzen um (SIM, dann CLI, REG für Werte), ein getrennter Reviewer prüft. Betroffen sind außerdem der Welt-Betrachter im Asset-Studio (B-460), die Musik je Zustand (B-168) und die Balancing-Runde Wirtschaft (B-155, BR1).

## Anforderungen

- Der Zyklus (`cycleAt` in `engine/sim/world_cycle.go`) kennt vier Phasen in der Reihenfolge `day`, `dusk`, `night`, `dawn`; `CycleInfo.Phase` nimmt auch `dawn` an, Fortschritt und Restzeit gelten je Phase.
- Die Dauer der Phase `dawn` steht als Wert in `cycle` der Biome mit `type: dayNight` (`data/biomes/*.json`, einzige Quelle für Go und Client); Startwert 6/2/4/2 min (Tag, Dämmerung, Nacht, Morgengrauen, Q65).
- Das Ereignis `dawn` und seine Folgen (Gold je Spieler, Taverne, Händler, Autosave, Gegner ziehen ab, Ende der Nachtereignisse) fallen bei **Beginn der Phase `dawn`**, nicht mehr bei Beginn des Tages; der Übergang `dawn` → `day` löst nichts aus (Q65).
- Die Welle startet weiter mit Beginn der Nacht; der Aggressionspool unter Tage bleibt unverändert.
- Die Entwickler-Funktion `sim.DevSetPhase` (`engine/sim/dev.go`, Z. 49–56) kennt `dawn`: Ihre Tabelle `starts`/`length` wird um die vierte Phase ergänzt (heute ohne `dawn`), sodass sie in alle vier Phasen springt. `engine/sim/campaign.go` (Z. 82) wird mit angepasst, soweit es sich auf die Phasen des Zyklus bezieht (Inhalt beim Umbau prüfen, ungeprüft).
- Der Client zeichnet die Phase `dawn` mit eigener Helligkeit (Verlauf von dunkel nach hell über die Phase); die Sonderrolle „letzte 10 % der Nacht“ in `daylight()` entfällt. Eine unbekannte Phase wird weiter wie `night` behandelt; bei Fortschritt über 0,9 ergibt `daylight()` für sie aber eine Helligkeit über 0 (`viewRules.ts:34`), bis die Sonderrolle entfernt ist, danach bleibt sie dunkel.
- Der Welt-Betrachter im Asset-Studio setzt Morgengrauen als echte Phase `dawn` und streicht die Abbildung `night` mit 0,9 + 0,1·p (B-460).
- Die Regeln und Begriffe (`docs/rules/wirtschaft.md` § 3, Glossar, B-466, dazu `docs/rules/zielkorridore.md` Z. 71 und 73 und `docs/game-design.md` Z. 88) beschreiben danach den gebauten Stand ohne „bis B-470“ bzw. „heute bis B-470“.
- Deterministisch (gleiche Eingaben → gleicher Zyklus); gilt in allen Räumen und Stufen, 2+ Spieler ohne Änderung der Eingabe.

## Nicht-Ziele

- Balancing der Dauern und der Wirtschaft gegen die Zielkorridore: REG, B-155 (BR1); dieses Ticket setzt nur den Startwert 6/2/4/2 min aus Q65.
- Das Musikstück für das Morgengrauen: B-168, SO4 (Frage offen, Q16 kennt keinen eigenen Zustand).
- Neue Spielmechanik im Morgengrauen (Ereignisse, Gegner, Gebäude-Wirkung) über das Verschieben von `dawn` hinaus.
- Neue Biome oder Zyklus-Typen; der Aggressionspool (`aggressionPool`) bleibt.

## Regeln und Einschränkungen

- **Domänen:** SIM für `engine/sim/world_cycle.go`, `engine/sim/dev.go` (`sim.DevSetPhase`, Tabelle `starts`/`length` um `dawn` ergänzen, Z. 49–56), `engine/sim/campaign.go` (Z. 82, Phasenbezug anpassen), `engine/level` (Zyklus-Felder der Biome) und das Feld in `data/biomes/*.json` mit dem vorläufigen Startwert aus dem REG-Beschluss Q65 (Feld legt SIM an, danach ändert REG nur Werte, `docs/arbeitsweise.md`); danach CLI für `src/scenes/render/viewRules.ts` und den Client; Studio-Anpassung (B-460) und Doku (`docs/rules/`, Glossar, `docs/game-design.md`) in einer eigenen Session oder als Nachtrag. Reihenfolge SIM → CLI.
- **Doku-Nachzüge:** `docs/rules/wirtschaft.md` § 3, Glossar, `docs/rules/zielkorridore.md` (Z. 71 und 73: Verweise „bis B-470“) und `docs/game-design.md` (Z. 88: Verweis „bis B-470“) beschreiben nach dem Umbau den gebauten Stand; keine Verweise auf B-470 mehr.
- **Goldens:** Die Goldens in `testdata/golden/` verschieben sich (neue Phase, neue Dauern, `dawn` zu anderer Zeit). Das ist eine gewollte Regeländerung: **eigener Commit** mit `task go:golden` und Begründung im Commit-Text (welche Regel, welcher Wert, welche Kennzahl); `git diff --stat testdata/golden` vorher lesen.
- **Protokoll und Spielstand:** `docs/protocol.md` und die Protokoll-Version (`clientProtocol.ts`) prüfen: die Phase `dawn` ist ein neuer Wert von `cycle.phase`; ob das ein Versionssprung ist, entscheidet die Spec (Offene Fragen). Keine Rücksicht auf alte Spielstände (Memory).
- **Determinismus:** nur `engine/rng`, kein `math/rand`, im Client kein `Math.random()`.
- **Werte gehören nach `data/`**, nicht in den Code. Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Logging** mit Emoji nach `CLAUDE.md` (🚦 Zustandswechsel, 🌙 Nacht/Welle).

## Beispiele

- Wald, Startwert 6/2/4/2 min: nach 12 Minuten Spielzeit beginnt das Morgengrauen (Tag 6 + Dämmerung 2 + Nacht 4); in diesem Augenblick fällt `dawn`: +5 Gold je lebendem Spieler, Taverne, Händler, Autosave, Gegner ziehen ab. Nach 14 Minuten beginnt Tag 2, ohne weiteres Ereignis `dawn`.
- Der Client zeichnet das Morgengrauen über die ganze Phase von dunkel nach hell, nicht mehr nur in den letzten 10 % der Nacht (bisher 0,5 min von 5 min Nacht).
- Welt-Betrachter (B-460): Eintrag Morgengrauen setzt die Phase `dawn` mit Fortschritt p; die Abbildung `night` mit 0,9 + 0,1·p ist entfernt.
- Unter Tage (Aggressionspool): kein Wechsel der Phasen, der globale Zyklus der Oberwelt läuft weiter.

## Ausnahme- und Fehlerfälle

- Biom-Datei mit `type: dayNight` ohne die Dauer des Morgengrauens → Laden schlägt mit klarer Meldung fehl (kein stiller Standardwert).
- Dauer des Morgengrauens 0 oder negativ → Laden lehnt ab (wie die übrigen Dauern).
- Client erhält eine unbekannte Phase → wird wie `night` behandelt (bei Fortschritt über 0,9 liefert `daylight()` dabei eine Helligkeit über 0, `viewRules.ts:34`), keine Ausnahme.
- Spielstand aus der Zeit vor B-470 → keine Rücksicht, der Zyklus läuft nach dem Laden mit den neuen Dauern weiter (Memory „Keine Rücksicht auf alte Spielstände“).
- Biom mit Aggressionspool → kein eigenes `cycle`-Feld für Phasen nötig (nicht relevant).

## Akzeptanzkriterien

- **AC-01** `cycleAt` liefert vier Phasen `day`, `dusk`, `night`, `dawn` in dieser Reihenfolge mit Fortschritt 0..1 und Restzeit je Phase; für den Wald mit 6/2/4/2 min liegen die Grenzen bei 360 s, 480 s, 720 s und 840 s. `sim.DevSetPhase` springt in jede der vier Phasen (Tabelle `starts`/`length` in `engine/sim/dev.go` enthält `dawn`), `engine/sim/campaign.go` ist angepasst. Nachweis: Go-Test in `engine/sim` an den Grenzen und in der Mitte jeder Phase, Go-Test von `DevSetPhase` für alle vier Phasen.
- **AC-02** Die Dauer des Morgengrauens steht in `data/biomes/forest.json` › `cycle` (Startwert 2 min, Tag 6, Dämmerung 2, Nacht 4); fehlende oder nicht positive Dauer wird beim Laden abgelehnt. Nachweis: Go-Test von `engine/level` bzw. des Ladens.
- **AC-03** Das Ereignis `dawn` mit seinen Folgen (Gold, Taverne, Händler, Autosave, Gegner ziehen ab) fällt genau einmal je Zyklus bei Beginn der Phase `dawn`; der Übergang `dawn` → `day` löst es nicht aus. Nachweis: Go-Test mit Zähler der Ereignisse über zwei Zyklen.
- **AC-04** Der Client zeichnet `dawn` als eigene Phase mit steigender Helligkeit von 0 bis 1 über die Phase; `night` bleibt über die ganze Phase dunkel. Nachweis: Vitest von `daylight()` für `dawn`, `night` und eine unbekannte Phase.
- **AC-05** Der Welt-Betrachter (B-460) setzt Morgengrauen als Phase `dawn`; die Abbildung `night` mit 0,9 + 0,1·p ist entfernt. Nachweis: Test der Optionen-Logik im Studio, `git grep -n "0.9" -- tools/k3c-studio` ohne die Abbildung.
- **AC-06** Die Goldens sind in einem eigenen Commit mit Begründung verschoben; `git diff --stat testdata/golden` ist durch die Regel erklärt. Nachweis: Commit-Text, `task check` grün.
- **AC-07** `docs/rules/wirtschaft.md` § 3, Glossar (Tageszyklus, Tag, Dämmerung, Nacht, Morgengrauen, `dawn`, Tagesanbruch), `docs/rules/zielkorridore.md` (Z. 71 und 73), `docs/game-design.md` (Z. 88) und `docs/protocol.md` beschreiben den gebauten Stand ohne „bis B-470“ bzw. „heute bis B-470“. Nachweis: `git grep -n "B-470" docs/glossar.md docs/rules docs/game-design.md` ist leer.

## Offene Fragen

Entscheidung von 🧑:
1. **Protokoll-Version:** Ist die neue Phase `dawn` in `cycle.phase` ein Versionssprung (Protokoll Version 3 → 4) oder gilt sie als kompatible Erweiterung? Vorschlag von Claude (ungeprüft): Phase ist ein neuer Wert eines bestehenden Feldes, ältere Clients zeichnen sie dunkel; Sprung nur, wenn `docs/protocol.md` das so verlangt.
2. **Helligkeit im Morgengrauen:** linear von 0 nach 1 über die ganze Phase oder eine andere Kurve? Vorschlag (ungeprüft): linear wie die Dämmerung (rückwärts).
3. **Wann:** Einordnung in einen Sprint (SIM → CLI), vor oder nach BR1 (B-155), weil sich die Zielkorridore auf 6/2/4/2 min beziehen.

## Notizen

- Verwandt: B-466 (Glossar und Regeln verweisen für den Umbau auf dieses Ticket statt auf B-213), B-460 (Welt-Betrachter bildet Morgengrauen bis dahin als `night` mit 0,9 + 0,1·p ab), B-168 und SO4 (Musik im Morgengrauen; verweisen noch auf B-213), B-155 und BR1 (Balancing der Dauern, Zielkorridore „Grenze nach Q65“), Q65 in `docs/fragenkatalog.md`.
- Code heute: `engine/sim/world_cycle.go` (`cycleAt`, `stepCycle`, Ereignis `dawn` beim Übergang zu `day`), `engine/sim/dev.go` Z. 49–56 (`sim.DevSetPhase`, Tabelle `starts`/`length` ohne `dawn`), `engine/sim/campaign.go` Z. 82 (Phasenbezug, mit anzupassen), `src/scenes/render/viewRules.ts` (`daylight()`, Z. 30–34: unbekannte Phase wie `night`, bei Fortschritt über 0,9 Helligkeit über 0), `data/biomes/forest.json` › `cycle`.
- Doku-Nachzüge: Verweise „bis B-470“ in `docs/rules/zielkorridore.md` (Z. 71 und 73), `docs/game-design.md` (Z. 88), `docs/rules/wirtschaft.md` § 3 und im Glossar; AC-07 prüft sie per `git grep`.
- Angelegt 2026-10-17 aus dem Review der Specs WZ7–WZ10.
