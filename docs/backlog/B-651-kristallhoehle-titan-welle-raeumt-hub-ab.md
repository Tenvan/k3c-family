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

**Stand 2026-10-28 (Claude), B-651 ruht seit der Gebäudeänderung B-652.** Die Hebel A bis E (Bot, Pool, Boss-HP) sind in den früheren Notizen belegt (`bossHpPerExtraPlayer`, `perExtraPlayer` 0,25, Pool-Begrenzung verworfen). Mit den verteilten Plätzen (B-652, BS12) und den Bot-Anpassungen läuft coop2 deutlich weiter als zuvor.

**Endspiel-Bot (Commit nach `24c10e64`):** (1) Unter Tage baut der Bot die Werkstatt zuerst (`deepBuildOrder`): ohne Werkstatt keine Bögen, jede Welle räumt sie ab. (2) Knoten nahe am Bau des Endbosses werden nicht markiert (`nearLair`, 60 Units, solange < 16 Kämpfer): Kristall-Knoten am Bau weckten den Endboss, die Beschwörung räumte Tag 27/28 die Kristallhöhle ab (belegt: Endboss aktiv bis Tag 31 nicht mehr, vorher Tag 28). (3) Abstieg erst mit 400 Holz, 16 Kämpfer-Ziel in der Endboss-Stufe.
**Mehr-Seed-Lauf (k3c-dev, 1, 2, 3; 40 Tage):** Goblin-Anführer 4/4 (Tag 5), Troll-König 4/4 (Tag 15), Ratten-Königin 4/4 (Tag 20), Lava-Golem 4/4 (Tag 22), Splitter-Titan 3/4 (Tag 32), Endboss 0/4. Vor dem Endspiel-Bot: 4/4, 4/4, 3/4, 1/4, 1/4, 0/4.
**Befund Endboss (belegt per Zustandsspur Seed k3c-dev, Tag 26–41):** Die Kristallhöhle hält Hub 3; das Heer pendelt zwischen 0 und 10 Bögen, Gold liegt am Deckel 30 ungenutzt. Der Endboss schläft (nie 16 Kämpfer), der Bot greift ihn nicht an. Die Wellen der Kristallhöhle (Welle 5–9 an Tag 32–41, alle 1–2 Tage) räumen die 10 Bögen jeweils ab; es fehlt Verteidigung, nicht Nachschub (Anwerbe-Lager: 2 Landstreicher, 25 s Respawn, kein Deckel in `data/`).

**Offen (Entscheidung 🧑):** (1) Endspiel-Plan weiter (stärkere Verteidigung in der Kristallhöhle: Türme vor Mauern, Gold vor Deckel in Elite/Hub-Ausbau investieren, Hub 4+), oder (2) Meilenstein 1 bis „Splitter-Titan besiegt“ fassen, den Endboss dem Stufen-Test (`task balance:fullgame`, Hilfen) überlassen.
