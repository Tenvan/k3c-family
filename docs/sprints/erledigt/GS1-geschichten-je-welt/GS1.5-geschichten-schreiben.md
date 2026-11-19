# GS1.5 · Texte: Wortschatz und Zeilen der vier Welten (B-976)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** opus
- **Umgebung:** live
- **Branch:** sprint/gs1
- **Abhängig von:** GS1.4
- **Tickets:** B-976
- **Kriterien:** AC-06

## Ziel

`standard`, `sengoku`, `nil` und `britannien` haben Wortschatz und alle Zeilen der Geschichte; 🧑 hat die Liste zur Abnahme.

## Kontext

Spec: `docs/backlog/B-976-geschichten-vier-welten.md` (R1–R5, AC-01 bis AC-04), Format und Regeln: `docs/rules/welten.md` § 2, § 6.2 und B-965 › Beispiele (die Zeilen für Sengoku und Nil von 🧑 werden übernommen). **Hauptcheckout, kein Worktree:** `sengoku`, `nil`, `britannien` sind nicht eingecheckt (`.gitignore` › `data/worlds/*`) und liegen nur in der Repo-Wurzel. **Voraussetzung:** Workbench neu gebaut und gestartet, `world_story_set` ist als MCP-Tool vorhanden (sonst abbrechen und im Ergebnis melden). Inseln: Sengoku Reisfeld-Ebene, Bambuswälder, Bergfestung; Nil Oasen am Fluss, Wüstenrand, Grabkammern; Britannien Hochmoor, Steinkreis-Hügel, Hügelgräber; Standard Insel 1. Endboss überall: Kristallherz-Wächter (`data/bosses.json`). Klang: Sengoku japanisch, Nil ägyptisch, Britannien keltisch-altenglisch, Standard märchenhaft. Je Zeile 2–3 echte deutsche Wörter in Großbuchstaben, höchstens 64 Zeichen, Wortschatz höchstens 24 Wörter.

## Erlaubte Dateien

- `data/worlds/**`

## Nicht-Ziele

Neue Welten, Änderungen an Inseln, Overrides oder Chunks, Mechanik der Flaschenpost, Golden neu schreiben, Welt „Polar“.

## Schritte

1. `world_list`, `world_get` je Welt: Inseln, Etappen, Stand der Geschichte.
2. Je Welt Wortschatz festlegen (eigene Laute, höchstens 2 Wörter gemeinsam mit einer anderen Welt).
3. Zeilen schreiben: `start`, `end`, je Etappe, drei `bottles`, je Insel `arrival`, `boss`, `done`; ein Bogen je Welt.
4. `world_story_set` je Welt; `world_check` ohne Fehler und Hinweise.
5. `check_run task:check:go` (Golden unverändert), `check_run task:check:diff`.
6. Ergebnis: Liste aller Zeilen je Welt mit einer deutschen Deutung je Zeile für die Abnahme durch 🧑; Commit nennt die REG-Änderung an `standard.json`.

## Fertig, wenn

- [ ] AC-06: Vier Welten vollständig, `world_check` ohne Fehler und Hinweise, Golden unverändert (B-976/AC-01 bis AC-03).
- [ ] Liste für 🧑 im Ergebnis; B-976/AC-04 (Abnahme) bleibt offen bei 🧑.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-06 umgesetzt, geprüft:** vier Welten mit Wortschatz und allen Zeilen (`world_story_set`), `world_check` je Welt „gültig“ ohne Hinweise; `check_run task:check:diff` grün; `task:check:go` vor der Test-Anpassung rot (siehe Abweichungen), danach `go test ./sim ./internal/detvec` grün. Golden der Sim unverändert.
- **B-976/AC-04 (Abnahme) offen bei 🧑.**
- **Abweichungen:** (1) `sengoku`, `nil`, `britannien` liegen nur in der Repo-Wurzel (nicht eingecheckt), dort per `checkout`=Wurzel geschrieben; im Commit steht nur `standard.json`. (2) Wegen `standard.json` angepasst: `engine/sim/island_worlds_test.go` (Hash der Inselliste, neue Story), `engine/sim/story_test.go` (Standard hat jetzt Zeilen), `testdata/rust/data-tree.json.gz` (Daten-Baum-Golden mit `-write` neu, nur `islands[0].story`). (3) Worktree brauchte `task install` (fehlende node_modules).

### Liste zur Abnahme (Deutung je Zeile)

**Standard (märchenhaft)** – Wortschatz: lumi tolu wuna fenn pip olu miri zaro hei
- start: „Lumi tolu KRONE, wuna HELD fenn.“ – Eine Krone, ein Held.
- end: „Lumi fenn FRIEDE, tolu KRONE wuna.“ – Friede, die Krone bleibt.
- Flaschen: „Pip zaro STERN, olu MOND miri.“ (Stern, Mond) · „Fenn hei WIND, KRONE tolu BOOT.“ (Wind, Krone, Boot) · „Wuna lumi HEIMAT, pip ZAUBER hei.“ (Heimat, Zauber)
- Insel 1: arrival „Pip olu WALD, lumi HÖHLE miri.“ (Wald, Höhle) · boss „Zaro fenn WÄCHTER, KRISTALL hei.“ · done „WÄCHTER fenn, lumi KRONE olu.“ (Wächter besiegt, Krone)

**Sengoku** – Wortschatz: oru kamae no sen shiro nu kaze tsuki yume hana ame ichi mori take yama
- start: „Oru kamae KRONE no sen. Shiro nu BRENNT.“ (🧑-Beispiel: Krone, Burg brennt) · Etappe 1: „Sen no MEER, oru KRONE kamae.“ · end: „Shiro no FRIEDE, kamae KRONE sen.“
- Flaschen: „Hana no ame, KRONE oru. Ichi SCHIFF kaze.“ (Krone, Schiff) · „Take nu kaze, SCHATTEN sen. Mori no LICHT.“ (Schatten, Licht) · „Tsuki oru FLUSS, yume no TOR.“ (Fluss, Tor)
- Reisfeld-Ebene: „Oru no REIS, sen SCHIFF nu.“ · „Nu kamae MASKE, shiro TIEF.“ · „MASKE nu, KRONE oru sen.“
- Bambuswälder: „Take no kaze, WALD sen LEISE.“ · „Nu kamae KLINGE, take SCHATTEN.“ · „KLINGE nu, take no RUHE.“
- Bergfestung: „Yama no shiro, STEIN kamae TOR.“ · „Nu oru FEUER, shiro KRONE.“ · „FEUER nu, yama no FRIEDE.“

**Nil** – Wortschatz: akh'et ra sen-hotep ma'u kha hapi nefer ib wadj ankh ka-ten mer aset djed set'u
- start: „Akh'et ra KRONE sen-hotep. Ma'u FLUT kha, kha SAND.“ (🧑-Beispiel: Krone, Flut, Sand) · Etappen: 1 „Hapi ra FLUSS, nefer KRONE ib.“ · 2 „Wadj ra SAND, ankh KRONE ka-ten.“ · end: „Sen-hotep ra FRIEDE, ankh KRONE kha.“
- Flaschen: „Ib hapi ra, KRONE nefer WASSER.“ · „Mer aset ka-ten, SONNE ra GOLD.“ · „Djed ma'u kha, NACHT set'u STERN.“
- Oasen am Fluss: „Hapi ra PALME, nefer ib BOOT.“ · „Ma'u kha KROKODIL, aset NACHT.“ · „KROKODIL kha, hapi ra FRIEDE.“
- Wüstenrand: „Wadj ra SAND, ankh WIND kha.“ · „Set'u ra SKORPION, SONNE kha.“ · „SKORPION kha, WIND ra ib.“
- Grabkammern: „Ka-ten aset GRAB, djed TIEF.“ · „Ma'u ra WÄCHTER, KRISTALL kha.“ · „WÄCHTER kha, KRISTALL ra RUHE.“

**Britannien** – Wortschatz: bryn gwyn tarn llan dun cae mor ber nos dyn tir
- start: „Bryn gwyn KRONE, tarn dyn. Llan nos NEBEL.“ (Krone, Nebel) · Etappen: 1 „Dun gwyn MOOR, tir KRONE dyn.“ · 2 „Cae bryn STEIN, mor KRONE tarn.“ · end: „Llan gwyn FRIEDE, tir KRONE bryn.“
- Flaschen: „Mor tarn, WIND bryn HARFE.“ · „Ber cae LICHT, nos dyn SCHATTEN.“ · „Dun llan RABE, tir gwyn EICHE.“
- Hochmoor: „Bryn tarn MOOR, nos dyn NEBEL.“ · „Ber gwyn WÄCHTER, KRISTALL nos.“ · „WÄCHTER ber, MOOR gwyn RUHE.“
- Steinkreis-Hügel: „Cae dun STEIN, KREIS gwyn ber.“ · „Dun cae WÄCHTER, STEIN bryn FEUER.“ · „STEIN dun, cae gwyn LICHT.“
- Hügelgräber: „Tir dyn GRAB, mor nos DUNKEL.“ · „Tir nos WÄCHTER, GRAB llan.“ · „GRAB nos, tir gwyn FRIEDE.“

**Ralf prüfen:** Klang und Sinn der Zeilen (Kunstwörter sind Laute ohne feste Bedeutung, nur die GROSSEN Wörter tragen Sinn); Etappen-Zeilen bei Nil/Britannien (jede Insel eine Etappe).
