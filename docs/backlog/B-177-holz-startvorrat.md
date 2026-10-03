# B-177 · Die Insel startet mit einem Holz-Startvorrat

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** H1
- **Erstellt:** 2026-10-03
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint H1

## Ausgangslage

Der Insel-Vorrat startet leer (`engine/sim/island.go` › `createIsland`: `Stock: &Stock{}`), und das Regelwerk nennt keinen Startvorrat. Alle Gebäude der Hub-Stufe 1 kosten Holz plus Gold (`data/buildings.json`: Mauer 20 Holz + 5 Gold, Turm 50 + 20, Werkstatt 40 + 15, Farm 30 + 10), kein Gebäude ist nur mit Gold baubar. Vor dem ersten Bauplatz muss ein Bauer erst Bäume fällen (10 Holz je Baum in 4 s Arbeit plus Laufweg); die Zielvorgabe „erste Mauer vor Ende Tag 1“ (`docs/rules/materialien-gebaeude.md` § 3.1) ist damit ohne Hilfe kaum zu halten. 🧑 hat am 2026-10-03 entschieden (Chat): Startvorrat Holz, Mauern bleiben Holz plus Gold.

## Ziel

Eine neue Insel startet mit einem Holz-Startvorrat aus den Daten, sodass beide Mauern und ein Turm sofort bezahlt werden können. Nutzen: Der Spielstart ist spielbar, ohne die Holzwirtschaft abzuschaffen.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen) am TV und am Handy; 🧑 hat die Regel beschlossen, die Höhe ist ein Vorschlag (Feintuning in B-155).

## Anforderungen

- `data/hub.json` bekommt `islandStartStock` mit Startwert `{ "wood": 100 }` (Vorschlag: 2 Mauern 40 + 1 Turm 50 = 90 Holz plus Reserve); `engine/sim/data.go` liest das Feld, `engine/sim/island.go` setzt den Vorrat **nur bei einer neuen Insel**, nie beim Laden eines Spielstands.
- Der Startvorrat darf das Lager-Maximum der Insel nicht überschreiten (Burg-Kapazität 300 je Material); eine Datenprüfung scheitert sonst.
- Der Startvorrat gilt in allen Schwierigkeitsgraden gleich (die Wirtschaft ändert sich nicht mit dem Grad, `docs/rules/wirtschaft.md` § 4).
- Regel in `docs/rules/materialien-gebaeude.md` (§ 1) und ein Verweis in `docs/game-design.md`: „Die Insel startet mit 100 Holz“ (Startwert).
- Deterministisch, 2+ Spieler.

## Nicht-Ziele

Gebäude nur mit Gold, Änderung der Baukosten, Balancing der Zielkorridore (B-155), Startvorrat für andere Materialien.

## Regeln und Einschränkungen

Werte in `data/`, Logik in `engine/sim/`. Domänen-Ausnahme (die Freigabe der Spec erlaubt sie): Der Sprint darf die Regel in `docs/rules/materialien-gebaeude.md` und den Verweis in `docs/game-design.md` (REG) nachführen. Golden-Daten ändern sich nur über `task golden:update` mit Begründung im Commit (Q09). Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Neue Insel → Vorrat Holz 100, Stein 0; zwei Spieler zahlen die beiden Mauern (je 5 Gold) ohne dass ein Bauer Holz geholt hat. Ein Spielstand mit Vorrat Holz 0 wird geladen → der Vorrat bleibt 0.

## Ausnahme- und Fehlerfälle

Startvorrat größer als das Lager-Maximum → Datenprüfung (Test) schlägt fehl. Spielstand ohne Vorrat-Feld (ältere Version) → leerer Vorrat wie bisher, kein Startvorrat.

## Akzeptanzkriterien

- **AC-01** Test in `engine/sim/`: Eine neue Insel hat Holz 100 und Stein, Kupfer, Eisen, Kristall 0 laut `data/hub.json`; zweimal mit demselben Seed gleiches Ergebnis.
- **AC-02** Test: Ein geladener Spielstand (Fixture aus `testdata/saves/`) behält seinen gespeicherten Vorrat, der Startvorrat wird nicht aufgeschlagen.
- **AC-03** Test: Eine Datenprüfung scheitert, wenn ein Startwert das Lager-Maximum überschreitet.
- **AC-04** Test: Auf einer neuen Insel lassen sich beide Mauern und ein Turm ohne vorheriges Holzfällen bezahlen und bauen (festes Seed, 2 Spieler).
- **AC-05** Golden-Daten sind mit `task golden:update` aktualisiert, die Begründung steht im Commit; `task check:go` ist grün.
- **AC-06** `docs/rules/materialien-gebaeude.md` nennt den Startvorrat Holz 100 und `docs/game-design.md` verweist darauf (Suche nach „Startvorrat“).

## Offene Fragen

Höhe 100 Holz ist ein Vorschlag von 🧑-Beschluss „Startvorrat Holz“; Feintuning mit B-155 und dem Balancing-Tester. 🧑 bestätigt die Zahl mit der Freigabe.

## Notizen

Anlass: Hinweis von 🧑 am 2026-10-03: Gebäude der ersten Stufe sind ohne Material nicht baubar. Alternative „Mauern nur mit Gold“ und „volles Lager (300 Holz)“ wurden verworfen.
