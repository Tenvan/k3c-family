# B-650 · Boss-HP je Zusatzspieler bekommt ein eigenes Feld, getrennt von der Wellengröße

- **Domäne:** SIM
- **Typ:** Idee
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

`data/waves.json › perExtraPlayer` (heute 0,25) skaliert zwei Dinge gemeinsam: die Wellengröße (`engine/sim/island_options.go:90`) und die HP aller Bosse, Mini- wie Endboss (`engine/sim/boss.go:38`, `spawnBoss`). Wellen und Boss-HP lassen sich so nicht getrennt für Mehrspieler einstellen. Befund BS3.6 (B-645): coop2 kommt mit 0,15 bis 0,25 bis Stufe 5, der Splitter-Titan (Stufe 5, Welle 3) räumt das Heer ab; ein Wert, der Wellen und Boss zugleich trifft, löst das nicht.

## Ziel

Die HP der Bosse wachsen mit der Spieleranzahl über ein eigenes Feld in `data/waves.json`, unabhängig von der Wellengröße. Mehrspieler-Balancing hat so zwei getrennte Multiplikatoren; Basiswerte bleiben Solo-Werte.

## Beteiligte und Zielgruppen

🧑 entscheidet über Feldname und Startwert; Claude setzt um und misst.

## Anforderungen

- Neues Feld `bossHpPerExtraPlayer` in `data/waves.json` (Zahl, ≥ 0), beschrieben im `notes`-Text der Datei. Der HP-Faktor eines Bosses ist `1 + bossHpPerExtraPlayer × (Spieler − 1)` (Spieler = Spieler der Insel, mindestens 1), `spawnBoss` nutzt es statt `perExtraPlayer`; die Rundungsregel bleibt (auf die gerundeten HP).
- `perExtraPlayer` wirkt danach nur noch auf die Wellengröße (`island_options.go`).
- Startwert: gleich `perExtraPlayer` (0,25), damit sich nichts ändert; Golden bleiben unverändert (`task go:golden` zeigt keinen Diff).
- Daten-Prüfung: Feld ist Pflicht, Zahl ≥ 0 (wie die anderen Felder der Datei, Lader in `engine/sim/data.go`, `waves`-Struct).
- Regel `docs/rules/wirtschaft.md` § 8 und `docs/rules/bosse.md` nennen beide Felder; Glossar nur bei neuem Begriff.
- `balance-hints.json` bekommt einen Hebel „Boss-HP je Zusatzspieler“ für Boss-Ziele (Gruppe `basis`); `vary` (`waves.json:bossHpPerExtraPlayer=0.1`) muss funktionieren (`checkReloaded`, `variant.go`).
- Deterministisch, 2+ Spieler: gleiche Eingabe, gleiche HP.

## Nicht-Ziele

Neue Boss-Mechanik; Bot-Änderungen; eigene Faktoren je Boss oder je Stufe (`bosses.json hpFactor` bleibt); das Justieren des Werts selbst (danach BS3.6 mit `task balance:milestone`).

## Regeln und Einschränkungen

Coop-Werte nur über Multiplikator-Felder, Basiswerte bleiben Solo (Beschluss 🧑 2026-10-28). Keine Rücksicht auf alte Spielstände. Werte nur in `data/`, Code liest sie. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

Solo: Faktor 1. 2 Spieler mit `bossHpPerExtraPlayer` 0,1 und `perExtraPlayer` 0,25: Welle ×1,25, Boss-HP ×1,1. 3 Spieler: Welle ×1,5, Boss-HP ×1,2 (Endboss 1 Spieler 16706 HP → 3 Spieler 20047).

## Ausnahme- und Fehlerfälle

Feld fehlt oder negativ: der Datenlader meldet den Fehler (`k3c-server -check-data` lehnt den Stand ab, der alte läuft weiter). Ein Spieler: kein Zuschlag.

## Akzeptanzkriterien

- **AC-01** `bossHpPerExtraPlayer` steht in `data/waves.json`, wird geladen und geprüft (Test: fehlt/negativ → Fehler).
- **AC-02** Boss-HP skalieren mit dem neuen Feld, Wellengröße nur mit `perExtraPlayer`: Test mit abweichenden Werten (z. B. 0,1 und 0,5) für Mini- und Endboss, 1/2/3 Spieler (`boss_test.go`, `boss_endboss_test.go`, `island_options_test.go`).
- **AC-03** Mit gleichem Startwert bleiben alle Golden und `task check:diff` grün ohne `go:golden`-Diff.
- **AC-04** `vary`/`K3C_VARY` mit `waves.json:bossHpPerExtraPlayer=…` wirkt (Test in `variant`-Tests), Hebel in `balance-hints.json`.
- **AC-05** Regeltexte (`wirtschaft.md` § 8, `bosse.md`) nennen beide Felder.

## Offene Fragen

Feldname `bossHpPerExtraPlayer` (Vorschlag) – 🧑 bestätigt oder nennt einen anderen. Startwert 0,25 (unverändert) oder gleich ein niedrigerer Wert für den Splitter-Titan (Vorschlag: erst unverändert, danach in BS3.6 messen).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
