# B-645 · Der Meilenstein-Lauf coop2 scheitert an den Bots, nicht an den Werten: Es entsteht keine Wirtschaft

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** BS3
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BS3.4 (Meilenstein 1): Der Insel-Lauf `coop2`, Welt `standard`, Seed `k3c-dev`, 40 Tage bleibt auf der Münzskala rot, fünf Diagnose-Läufe (`run-1` bis `run-5`, Stand Commit nach BS3.3). Der Endboss erscheint nur, wenn ein Monarch die Kristallhöhle erreicht (60 % der Läufe mit dem alten Abstiegs-Notausstieg), und besiegt wird er in 0 %. Messwerte: Hub-Stufe am Ende 1 auf jeder Stufe, Kämpfer am Ende 0, Leerlauf der Bauern 94–100 %, Burg der Startstufe fällt ab Nacht 3 bis 5, auf der Kristallhöhle fielen im Mittel 700 bis 1700 Monarchen (Respawn-Schleife am Endboss). `wirtschaft` statt `coop2` (`run-4`) ist besser (Leerlauf 44–69 % an Tag 1–2), scheitert aber ebenso: Burg fällt ab Nacht 3, keine Hub-Stufe 2, keine Kämpfer.

## Ziel

Der Meilenstein-Lauf kann grün werden: Die Bots bauen auf der Münzskala eine Wirtschaft auf (Hub-Stufe, Kämpfer, Material), und der Endboss ist mit den Werten aus BS2 erreichbar.

## Beteiligte und Zielgruppen

🧑 entscheidet über Profil und Werte, Claude setzt um.

## Anforderungen

- Befund (belegt in `reports/simtest-run-1` bis `-5`): `coop2` markiert weder Bäume noch Felsen (die Rollen „Mauern zuerst“ und „Wirtschaft zuerst“ kennen nur Mauern und Anwerben); die Bauern arbeiten deshalb nie, Holz und Stein kommen nicht nach. Mit Startgold 14 und Tageseinkommen 2 trägt das keinen Hub-Ausbau (Stufe 2: 4 + 4 + 6 Gold, 120 Stein).
- Befund: Wer beide Monarchen absteigen lässt (Regel aus B-643), lässt den Hub unverteidigt; steigt nur einer ab, bleibt er in der Höhle hängen (Linie-1-Mauern allein mit 14 Gold, kein Bauer).
- Befund: Der Monarch allein schlägt den Endboss nie (10 Schaden je 0,7 s gegen einige 10 000 HP: `bosses.json` Faktor 30 mal Tiefen-Skalierung 1,5^4 mal Spielerfaktor); nur eine Armee aus Truppen kann das leisten.
- Entscheidung nötig: (A) `coop2` bekommt die Bausteine von `wirtschaft` (Markieren, Bögen, Hub) je Rolle, der Meilenstein misst dann dieses Profil; (B) eigener Bot „Insel“ mit Aufgabenteilung (Verteidiger am Hub, Material-Läufer in der Höhle); (C) zusätzlich Werte: Endboss-HP senken, Startgold oder Tageseinkommen anheben. Empfehlung (ungeprüft): A, danach höchstens zwei Werte-Runden.

## Nicht-Ziele

Neue Sim-Mechanik; Läufe über alle Seeds (BS6).

## Regeln und Einschränkungen

Bots nur über `sim.PlayerCommand`; deterministisch; Werte nur in `data/` nach `wirtschaft.md` § 8; Meilenstein verlangt `coop2` (`zielkorridore.md` § 6, Änderung nur durch 🧑).

## Beispiele

`sim_test {bots: coop2, world: standard, days: 40}` heute: Hub-Stufe 1, Kämpfer 0, Endboss 0 %. Ziel: Endboss besiegt, Schiff (Bedarf) mit Ampel.

## Ausnahme- und Fehlerfälle

Bleibt es nach A und zwei Werte-Runden rot: Befund an 🧑, kein weiteres Justieren ohne Entscheidung.

## Akzeptanzkriterien

- **AC-01** `sim_test {bots: coop2, world: standard, days: 40}` (Seed `k3c-dev`): Endboss Insel 1 besiegt (`endbossDefeated` 100 %), Bericht mit Schiff-Zeile.

## Offene Fragen

A, B oder C (siehe Anforderungen); ob der Meilenstein-Bot `coop2` oder ein neues Profil heißt (🧑).

## Notizen

Diagnose-Werte (Commit nach BS3.3): Insel-Bericht zeigt je Tiefe Ankunft, Burgfall, gefallene Monarchen, Hub-Stufe und Kämpfer am Ende (`kennzahlen.island`). Die Wirtschaft von `wirtschaft` im Standardszenario (Tag 1–5, Wald) ist unabhängig davon verletzt: Hub-Stufe 2 und 3 nie, Kämpfer nie ≥ 4 an Tag 3; die Korridore aus `zielkorridore.md` § 3 stammen aus der alten Skala (Startgold 100) und sind als Richtwerte neu abzustimmen (BS3.4/BS6).

**Diagnose ohne Hilfen, Solo, Seed k3c-dev (2026-10-27, Claude, Commit f21a2007):** Der Lauf bricht in Nacht 5 zusammen und erholt sich nie.
- Tag 1–5: Startgold 14, danach 2 Gold je Morgen. Die Bauern (8, ab Tag 3) häufen Holz an (bis 645, Wunsch 150) und holen kaum Stein (0–20, der Hub-Ausbau braucht 25); Hub bleibt Stufe 1, Kämpfer 2.
- Tag 6: alle 8 Bauern und beide Kämpfer sind tot, die Stufe 0 hat danach 0 Truppen. Ab da bleibt das Gold bei 2 (Tageseinkommen = Ausgabe), der Vorrat leert sich, bis Tag 35 passiert nichts mehr.
- Ursache vermutet (ungeprüft): Die Welle von Nacht 5 trifft einen Hub ohne Ausbau und ohne Verteidiger; die Münzskala (2 Gold/Tag) lässt keinen Wiederaufbau zu.
- Mit Hilfen (voll) fällt der Endboss an Tag 19: Der Bot selbst kann also durchspielen, es fehlt die Ökonomie der ersten fünf Tage.
- Hebel zur Entscheidung (🧑): (1) Tageseinkommen oder Startgold anheben (`economy.json`), (2) Welle 1–5 im Wald schwächer, (3) Bot verteidigt die Burg nachts besser (Bauern in die Burg statt Holz am Waldrand), (4) Kosten Hub-Ausbau Stufe 2 (Gold 4 plus Stein 25) senken.

**Hebel 3 probiert (2026-10-27, Claude, nur Bot, keine Werte):** Die Bauern suchen nachts schon von selbst den Hub auf (`isDangerous`). Sie sterben trotzdem, weil Nacht 5 (10 Goblins, Goblin-Anführer, Gierling) 2 Mauern, 2 Türme und 2 Bogenschützen überrollt. Das Gold (etwa 22 bis Nacht 5) reicht nicht für 8 Bauern und eine Verteidigung. Gemessen mit `peasantLimit` (Solo, Seed k3c-dev, ohne Hilfen):
- 6 oder 8 Bauern: Alles stirbt in Nacht 5, Tag 6 ohne Truppen, Stillstand (wie oben).
- 2 bis 5 Bauern: Mehr Gold fließt in Bögen (7–10 Kämpfer bis Tag 6), der Goblin-Anführer fällt, der Bot steigt an Tag 7 in die Höhle ab.
- Gewählt: 4 (mitten im funktionierenden Bereich; die Grenze zwischen 5 und 6 ist scharf, ungeprüft für andere Seeds).
- Danach neuer Engpass: Hub bleibt Stufe 1, in der Höhle 0 Kämpfer (das Heer bleibt im Wald), Gold 2–8 über Tage, kein Troll-König bis Tag 40. Mit Hilfen `voll` fällt der Endboss weiter an Tag 20.
- Offene Hebel: Tageseinkommen oder Hub-Ausbaukosten (Werte, 🧑 entscheidet), oder der Bot wartet mit dem Abstieg, bis er in der Höhle bestehen kann.

**Hebel „Abstieg abwarten“ probiert (2026-10-27, Claude, nur Bot, keine Werte):** Mit Mindestgold beim Abstieg (0, 12, 22, 30) bleibt der Ausgang gleich: Kämpfer 0 in der Höhle, Troll-König fällt nie. Das Gold ist beim Abstieg schon über 22 (Belohnung des Goblin-Anführers); das Heer bleibt ohnehin in der Wald-Stufe zurück, Warten stärkt die Höhle also nicht. Verworfen, nicht im Code.
Die Ursache liegt in der Höhle (Messung Tag 7–16): Die Wellen (der Pool füllt sich durch die gesammelten Rohstoffe, alle 3–4 Tage) räumen Mauern, Türme und Bauern ab; mit 2 Gold je Morgen baut der Bot nur wieder auf, der Hub-Ausbau (4 Gold, 25 Stein, Stein ist da) kam nicht dazwischen. Zwei Bot-Korrekturen, im Code (Commit 99fdf123):
- Ohne Bauern wirbt der Bot zuerst an (sonst blieben bezahlte Bauten im Zustand waitingWorker liegen).
- Liegt der Hub-Ausbau bereit, zahlt er zuerst, und neue Bauten lassen die Rücklage für den Ausbau stehen.
Wirkung: Hub-Stufe 2 in der Höhle ab etwa Tag 30 (vorher nie), Troll-König weiter nie; mit Hilfen `voll` fällt der Endboss unverändert an Tag 20.
Offen (🧑): Die Höhle verlangt vom Bot mehr, als 2 Gold je Morgen hergeben. Hebel Werte (Tageseinkommen, Beute, Hub-Ausbau) oder eine schwächere Pool-Füllung der Höhle.

**Hebel 1 und 4, zwei Werte-Runden (2026-10-27, Claude, Solo, Seed k3c-dev, ohne Hilfen, Stufen-Test 3, nur `economy.json` im Speicher geändert, danach zurückgesetzt):**
| Runde | startGold | dawnGoldPerPlayer | Ergebnis bis Tag 40 |
|---|---|---|---|
| Ausgang | 14 | 2 | Goblin-Anführer; Höhle: Hub 1, 0 Kämpfer (mit Bot-Korrekturen Hub 2 ab Tag 30) |
| 1 | 20 | 4 | Goblin-Anführer; Höhle: Hub 2 ab Tag 15, weiter 0 Kämpfer, kein Troll-König |
| 2 | 30 | 8 | Goblin-Anführer und Troll-König (vor Tag 15); Höhle 8 Kämpfer; Spieler in der Mine ab Tag 17–20, dort Hub 1, Ratten-Königin fällt nicht |
- Der Hub-Ausbau (Hebel 4) war nicht der Engpass: Er kostet nur 4 Gold und 25 Stein; was fehlt, ist laufendes Gold für Wiederaufbau und Bögen nach jeder Welle.
- Ab etwa 8 Gold je Morgen trägt sich die Höhle. Das ist das Vierfache des Werts der Münzskala (wirtschaft.md § 8: Einnahmen folgen der Kurve, Tageseinkommen 2). Werte in `data/` bleiben unverändert; Entscheidung bei 🧑.
- Optionen: (a) Tageseinkommen 3–4 plus Beute der Wellen anheben (Gegner lassen Gold fallen, `enemyResourceDrop` ist heute Material), (b) Stufen ab Tiefe 1 zahlen mehr Morgengold (nach Tiefe skaliert), (c) Welle der Höhle schwächer (Pool-Füllung), (d) Skala beibehalten, der Meilenstein läuft mit Hilfen.

**Option b umgesetzt (2026-10-27, Claude, Beschluss 🧑):** Neues Feld `economy.json › dawnGoldPerDepth` = 6; Morgengold = `dawnGoldPerPlayer` (2) + 6 · Tiefe (Höhle 8, Mine 14, Eisenstollen 20, Kristallhöhle 26; `engine/sim/economy_dawn.go`, Regel in `wirtschaft.md` § 8, Golden der 6 tiefen Szenarien neu geschrieben). Messung Stufen-Test ohne Hilfen (Solo, Seed k3c-dev): Zuschlag 3 und 4: Troll-König fällt nie; 6: Goblin-Anführer, Troll-König (vor Tag 15), Ratten-Königin (vor Tag 20); 8: gleich. Mit Hilfen `voll` unverändert Endboss Tag 20.
**Neuer Engpass:** Ab Stufe 3 (Eisenstollen) bleibt der Bot trotz 20 Gold je Morgen hängen: Lava-Golem und Endboss fallen bis Tag 40 nicht, der Spieler steht ab Tag 20 auf Stufe 3 mit Hub 1. Das ist kein Einkommensproblem mehr (Zuschlag 8 ändert nichts); nächste Untersuchung: Bot in Stufe 3 (Material-Tor, Kämpfer, Welle).

**Stufe 3 (Eisenstollen) untersucht, nur am Bot (2026-10-27, Claude):** Zwei Ursachen, beide im Bot.
1. Der Lava-Golem (3240 HP) kam mit der Welle zu einem Hub ohne Kämpfer (1 Bauer, 2 Landstreicher); der Bot pumpte den Aggressionspool (Knoten markieren), sobald die Mauern der Linie 1 standen, also vor dem Heer. Jetzt pumpt er erst ab `fighterWant` Kämpfern (`bot_abstieg.go`).
2. Die 20 Gold je Morgen gingen restlos in den Wiederaufbau von Mauern, Türmen und Farmen, die jede Welle alle 2–3 Tage abräumt; für Bögen blieb nie Gold, Kämpfer 0. Jetzt kommen Bögen (`fighterStep`) vor neuen Bauten (`bot_wirtschaft.go`).
Wirkung (ohne Hilfen, Solo, k3c-dev, Stufen-Test 4): alle vier Minibosse fallen (Lava-Golem), Kämpfer 10 in den Stufen 1–3, Hub 2–3 ab Tag 20–30, Spieler erreicht die Kristallhöhle um Tag 30; der Endboss fällt bis Tag 40 noch nicht (Kristallhöhle: 0 Kämpfer, Hub 3). Mit Hilfen `voll` unverändert Tag 20. Nächster Schritt: Kristallhöhle (Heer vor dem Endboss) und die Tage bis dorthin, ohne Werte.
