# B-651 · Die Titan-Welle der Kristallhöhle räumt den Hub eines Einzelmonarchen in Sekunden ab (coop2)

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** BS3
- **Projekt:** BAS
- **Erstellt:** 2026-10-28
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Diagnose BS3.6 (Claude, 2026-10-28, `task balance:milestone` mit `K3C_DIAG=1`, `TestDiagnoseCoop2`; coop2, standard, k3c-dev, perExtraPlayer und bossHpPerExtraPlayer 0,25): Der Meilenstein hängt beim Splitter-Titan (Kristallhöhle, Stufe 4, Welle 3). Beobachtet:
- Nur Monarch 1 (ungerader Index) steigt ab (Regel B-643); Monarch 0 bleibt mit 10 Kämpfern im Wald. In Stufe 3 stehen 8 Kämpfer, in Stufe 4 (Kristallhöhle) 0, ab Ankunft (Tag 21–22) bis Tag 41.
- Tag 23: Die Welle der Kristallhöhle bringt 36 Gegner (Splitter-Titan 5569 HP, 32 Splitterlinge, 3 Kristallspinnen) über das Portal x120 zum Hub x325. Eine Minute später ist die Stufe leer und der Monarch steht mit 0 Gold am Hub: Der Hub ist geräumt, der Monarch hat 26 Gold je Morgen, gibt sie sofort wieder aus (Wiederaufbau) und hat nie genug für Bögen. Muster wiederholt sich alle 7–8 Tage (Titan Tag 23, 30, 38).
- Boss-HP sind nicht die Ursache: `bossHpPerExtraPlayer` 0,1/0,15/0,25 gibt dasselbe Ergebnis (B-650).
- Stufe 3 (Eisenstollen): Der Lava-Golem fällt mit 8 Kämpfern, ein Monarch allein schafft es also, wenn das Heer vorher steht.

## Ziel

Der Meilenstein-Lauf coop2 (standard, `k3c-dev`) besiegt den Splitter-Titan und danach den Endboss, ohne Basiswerte der Solo-Welt zu ändern.

## Beteiligte und Zielgruppen

🧑 entscheidet über den Hebel, Claude misst und setzt um.

## Anforderungen

Hebel zur Entscheidung (ungeprüft, nach Wirkung geschätzt):
- (A) Bot: Die Kämpfer wandern mit dem Monarchen, oder ein zweiter Monarch steigt ab (heute nur ungerade Indizes, `bot_abstieg.go`). Wirkung vermutlich groß, ändert coop2-Bot, keine Werte.
- (B) Bot: Vor der Titan-Welle Gold für Bögen statt Wiederaufbau zurücklegen (Reihenfolge in `bot_wirtschaft.go`).
- (C) Wert (Multiplikator): Wellengröße der Kristallhöhle über `data/worlds/standard.json` (Insel-Tabelle) senken, sofern das keine Solo-Basis ist (prüfen: Welt-Tabelle gilt für alle Spielerzahlen, dann Basiswert und nach Regel von 🧑 nicht erlaubt).
- (D) Prüfen, ob 32 Splitterlinge als Schwarm gezählt werden sollten (`bosses.json` Schwarm-Gegner zählen als ganzer Schwarm): ob die Welle hier unbeabsichtigt 32 Einzelgegner statt eines Schwarms bringt (ungeprüft, Verdacht).

## Nicht-Ziele

Neue Mechanik; Solo-Basiswerte ändern (Beschluss 🧑 2026-10-28).

## Regeln und Einschränkungen

Bots nur über `sim.PlayerCommand`, deterministisch; Coop nur über Multiplikator-Felder; Spieltests per `task balance:milestone` (Go-Test), nicht per Workbench-Neubau.

## Beispiele

`K3C_DIAG=1 go test -tags fullgame -run TestDiagnoseCoop2 -v ./internal/balance` in `tools/k3c-workbench` schreibt Tages- und 20-s-Zeilen (Tag 22–31) beider tiefster Stufen.

## Ausnahme- und Fehlerfälle

Bleibt der Lauf nach (A) und (B) rot: Befund an 🧑, kein weiteres Justieren ohne Entscheidung.

## Akzeptanzkriterien

- **AC-01** `task balance:milestone` (coop2, standard, k3c-dev, 40 Tage): Splitter-Titan besiegt.
- **AC-02** Endboss Insel 1 besiegt (= BS3 AC-05).

## Offene Fragen

Welcher Hebel zuerst: A (zweiter Monarch/Heer mitnehmen) ist nach der Messung der aussichtsreichste (ungeprüft).

## Notizen

**Hebel A umgesetzt und gemessen (2026-10-28, Claude, nur Bot, keine Werte, Commit 93ed4aef):** `mayDescend` (`bot_abstieg.go`): Der Hub-Halter (gerader Index) folgt, sobald sein Heer den Hub hält (Kämpfer ≥ 8, Hub-Stufe ≥ 2). `endbossWant` 16 (`bot_boss.go`): Der Endboss am Bau wird erst mit 16 Kämpfern geweckt (ohne Wirkung in der Messung, bleibt als Schutz).
Wirkung (coop2, standard, k3c-dev, `task balance:milestone`): beide Monarchen ab Tag 22 in der Kristallhöhle, 52 Gold je Morgen, am Morgen von Tag 23 sechs Kämpfer. **Die Welle von Tag 23 (32 Gegner: Splitter-Titan 5569 HP, 24 Splitterlinge, 7 Kristallspinnen) räumt sie trotzdem ab**; danach 0 Kämpfer, Gold im Wiederaufbau. Ergebnis wie vorher: Stufe 5 ab Tag 22, Titan fällt nicht (Troll-König T16 statt T14).

**Hebel B (Bot wartet mit dem Abstieg) nach Datenlage verworfen, nicht im Code (2026-10-28, Claude):** Der Aggressionspool einer Stufe wächst auch ohne Spieler mit `percentPerMinute` 1 (`data/biomes/crystal.json › cycle`, gleich in cave, mine, ironhold), das sind etwa 13 Punkte je Tag (Kristallhöhle: Pool 56 an Tag 20, 70 an Tag 21, 83 an Tag 22, 97 an Tag 23). Wer später absteigt, trifft auf einen volleren Pool und damit auf die größere Welle; die Titan-Welle kommt in der ersten Nacht nach Ankunft. Ein Warten verschlechtert das Ergebnis, ein früherer Abstieg verbessert es, ist aber an den Sieg über den Miniboss der Vorstufe gekoppelt (Lava-Golem fällt Tag 21–22). Truppen wandern nicht mit (je Stufe), ein Heer in der Vorstufe hilft unten nicht.

**Offene Hebel (Entscheidung 🧑):** (C) Mechanik: Der Pool einer Stufe wächst nur, solange dort ein Spieler ist (oder `percentPerMinute` gilt nur mit Spieler). Das ist eine Regeländerung für alle Spielerzahlen (Solo eingeschlossen), `percentPerMinute` selbst ist ein Basiswert und fällt nach deiner Regel aus; Wirkung vermutlich groß (Pool bei Ankunft 0 statt 83), Spec nötig (`stufen.md`, Glossar ›Pool‹). (D) Prüfen, ob die 24–32 Splitterlinge als Schwarm zählen sollten (ungeprüft).
