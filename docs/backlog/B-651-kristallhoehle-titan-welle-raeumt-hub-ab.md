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

Hebel-Übersicht (A umgesetzt, B verworfen, siehe Notizen). **Spec zu Hebel C (Entwurf, Claude, 2026-10-28): Der Aggressionspool einer Stufe wächst nur, solange dort ein lebender Spieler ist.**

1. **Regel:** In `engine/sim/world_cycle.go` (Zeile ~76, `addPool` mit `percentPerMinute`) wächst der Pool der Zeit nur, wenn mindestens ein lebender Monarch in dieser Stufe steht (`len(w.Players) > 0` mit HP > 0; Stufenwechsel einzeln je Spieler, `stufen.md` § 1). Ohne Spieler: Pool und Wellenzähler bleiben stehen, es kommen keine Wellen in leere Stufen. Die Anteile je Kill (+5 %) und je gesammelter Ressource (+1 %) bleiben unverändert (sie entstehen nur mit Spielern). Das bestehende Einfrieren (`poolFrozen`, B-646) bleibt.
2. **Bei Ankunft** beginnt der Pool dort, wo die Stufe zuletzt stand (neue Stufe: 0). Die erste Welle kommt nach rund 100 Pool-Punkten aus Zeit, Kills und Rohstoffen, nicht mehr in der ersten Nacht.
3. **Gilt für jede Spielerzahl** (Solo eingeschlossen), `percentPerMinute` und alle Daten bleiben unverändert; keine neuen Datenfelder.
4. **Regeltext:** `docs/rules/stufen.md` § 2 (Satz „der Pool wächst nur mit Spieler in der Stufe“), Glossar ›Aggressionspool‹ (`docs/glossar.md`) und `docs/rules/gegner.md` § 3 nennen die Regel.
5. **Folgen (zu messen):** Golden der sechs tiefen Szenarien und `sim-*`-Wellen-Golden verändern sich (`task go:golden` mit Begründung im Commit); `balance-targets.json`-Korridore zu Wellen unter Tage bleiben, die Messung „Wellen im Abstand 4–12 min“ gilt nur mit Spieler in der Stufe. Die Bots müssen ihre Stufen nicht verlassen, um Wellen zu stoppen.
6. Deterministisch, 2+ Spieler: Die Regel hängt nur vom Zustand ab (kein Zufall, keine Uhr).

Weitere Hebel: (D) Schwarm-Zählung der 24–32 Splitterlinge prüfen (ungeprüft).

## Nicht-Ziele

Neue Mechanik außer Regel C; Solo-Basiswerte ändern (Beschluss 🧑 2026-10-28); `percentPerMinute` oder Wellentabellen ändern; Spielstand-Kompatibilität (keine Rücksicht auf alte Spielstände).

## Regeln und Einschränkungen

Bots nur über `sim.PlayerCommand`, deterministisch; Coop nur über Multiplikator-Felder; Spieltests per `task balance:milestone` (Go-Test), nicht per Workbench-Neubau.

## Beispiele

`K3C_DIAG=1 go test -tags fullgame -run TestDiagnoseCoop2 -v ./internal/balance` in `tools/k3c-workbench` schreibt Tages- und 20-s-Zeilen (Tag 22–31) beider tiefster Stufen.

## Ausnahme- und Fehlerfälle

Bleibt der Lauf nach (A) und (B) rot: Befund an 🧑, kein weiteres Justieren ohne Entscheidung.

## Akzeptanzkriterien

- **AC-01** `task balance:milestone` (coop2, standard, k3c-dev, 40 Tage): Splitter-Titan besiegt.
- **AC-02** Endboss Insel 1 besiegt (= BS3 AC-05).
- **AC-03** (Regel C) Test in `engine/sim`: Eine Stufe ohne lebenden Spieler lässt den Pool über 10 Spielminuten bei seinem Wert und löst keine Welle aus; mit einem Spieler wächst er um `percentPerMinute` je Minute.
- **AC-04** (Regel C) Test: Ein Spieler, der eine lange unbesuchte Stufe betritt, findet den Pool bei seinem letzten Wert (0 bei neuer Stufe), die erste Welle kommt nicht in der ersten Nacht nach Ankunft.
- **AC-05** (Regel C) `stufen.md` § 2, `gegner.md` § 3 und Glossar nennen die Regel; Golden neu mit Begründung, `task check:diff` grün.

## Offene Fragen

- Freigabe der Spec C durch 🧑 (Mechanik, gilt auch Solo).
- Ob „Spieler anwesend“ nur lebende Monarchen zählt (Vorschlag: ja) oder auch tote im Respawn (15 s).

## Notizen

**Hebel A umgesetzt und gemessen (2026-10-28, Claude, nur Bot, keine Werte, Commit 93ed4aef):** `mayDescend` (`bot_abstieg.go`): Der Hub-Halter (gerader Index) folgt, sobald sein Heer den Hub hält (Kämpfer ≥ 8, Hub-Stufe ≥ 2). `endbossWant` 16 (`bot_boss.go`): Der Endboss am Bau wird erst mit 16 Kämpfern geweckt. Wirkung (coop2, standard, k3c-dev, `task balance:milestone`): beide Monarchen ab Tag 22 in der Kristallhöhle, 52 Gold je Morgen, am Morgen von Tag 23 sechs Kämpfer. **Die Welle von Tag 23 (32 Gegner: Splitter-Titan 5569 HP, 24 Splitterlinge, 7 Kristallspinnen) räumt sie trotzdem ab.** Stufe 5 ab Tag 22, Titan fällt nicht.

**Hebel B (Bot wartet mit dem Abstieg) verworfen:** Der Aggressionspool wächst auch ohne Spieler mit `percentPerMinute` 1 (`data/biomes/*.json › cycle`), etwa 13 Punkte je Tag (Kristallhöhle Pool 56 an Tag 20, 83 an Tag 22). Wer später absteigt, trifft auf einen volleren Pool; Truppen wandern nicht mit.

**Hebel C (Pool wächst nur mit lebendem Spieler) umgesetzt, gemessen, zurückgenommen (2026-10-28, 🧑):** Code und Tests grün, Golden unverändert, aber der Meilenstein-Lauf wird schlechter: coop2 bleibt nach „Stufe 3 erreicht“ (Tag 17) in der Mine hängen, die Ratten-Königin fällt nicht (Welle 3 kam erst Tag 31 und räumte das Heer von 10 auf 0 ab; warum das Heer diesmal verliert, ungeprüft). Eine neu betretene Stufe startet bei Pool 0, die Wellen kommen etwa alle 3 Tage; der vorgefüllte Pool beschleunigte bisher den Weg nach unten. Nicht im Code. Spec-Entwurf C bleibt in „Anforderungen“.

**Offen (Entscheidung 🧑):** (D) Schwarm-Zählung der Splitterlinge prüfen; (E) Pool-Start bei Ankunft statt 0 oder voll auf einen Zwischenwert (z. B. höchstens 30), damit unbesuchte Stufen sich nicht bis zur Titan-Welle füllen, aber der Weg nach unten nicht stockt (ungeprüft, ändert Mechanik und Solo).
