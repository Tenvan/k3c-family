# B-648 · Balancing-Bericht urteilt nach Ziel und zeigt den Fortschritt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** BAS
- **Erstellt:** 2026-10-27
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Balancing-Bericht (`tools/k3c-workbench/internal/balance`, Zusammenfassung `summary.go`, `insights*.go`) setzt Pass/Fail aus **allen** Zielkorridoren zusammen. Ein Lauf, in dem der Endboss der Insel 1 fällt (Tag 19), steht trotzdem auf „Fail“, weil z. B. Hub-Stufe 2 vor Tag 5 oder die Burg-Nacht-Korridore anderer Grade verfehlt sind. Der Bericht zeigt keinen Spielfortschritt, nennt keine Ursache und vergleicht nicht mit dem letzten Lauf (Interview 2026-10-27, 🧑 Ralf).

## Ziel

Success und Fail eines Laufs hängen vom Erreichen des Ziels ab, nicht von einzelnen verfehlten Kennzahlen. Der Bericht zeigt Tester und Gamer den Fortschritt im Spiel, nennt den Blocker und liefert der KI ein kompaktes Ergebnis.

## Beteiligte und Zielgruppen

Ralf (🧑) als Tester und Gamer, Agenten, die Läufe über `sim_test` starten und auswerten.

## Anforderungen

- **Zwei Urteile (E1):** „Ziel erreicht ja/nein“ und „Qualität x von y Korridoren“, beide sichtbar; nur das Ziel färbt den Lauf grün/rot.
- **Rollen (E2):** Feld `role` je Ziel in `data/balance-targets.json`: `goal` bestimmt Pass/Fail, `guard` warnt bei Verletzung, `info` nur Diagnose.
- **Meilenstein-Leiter (E3):** Wald → Goblin-Anführer → … → Endboss, je Meilenstein Tag des Erreichens oder „nicht erreicht“.
- **Vergleich (E4):** automatisch mit dem letzten Lauf gleicher Konfiguration, Pfeile ▲▼ je Meilenstein; die feste Baseline bleibt für die CI.
- **Blocker-Satz (E5):** ein Satz oben, z. B. „Stecken geblieben bei Meilenstein X (Tag N): Hub 1, 0 Kämpfer, Gold 2“.
- **Aufbau (E6):** Ergebnis → Leiter → Blocker → Diagnose (Zielliste und Kennzahlen unten, einklappbar).
- **KI-Format (E7):** kleines `result.json` neben dem Bericht: `goalReached`, `milestones[{id, day|null}]`, `blocker`, Delta zum letzten Lauf, nächster Hebel.
- **Knapp (E8):** gelb nur bei `goal`-Zielen (knapp verfehlt oder gerade erreicht, z. B. Sieg Tag 39 von 40); Guards zeigen ok/verletzt.
- **Konfiguration (E9):** Kopfzeile mit Welt, Bot, Spielern, Tagen, Hilfen, Seed und „entspricht Stufen-Test voll“ bzw. „weicht ab: …“.
- **Seeds (E10):** Erfolgsquote je Meilenstein („7/10 Seeds, Median Tag 14“); Success = Zielquote des Ziels erreicht.
- **Hilfen (E11):** das Urteil nennt die Hilfe-Stufe („Ziel erreicht (mit Hilfen voll)“); ein Lauf ohne Hilfen zählt als Fortschritt, nicht als Regression.
- **Absicherung (E12):** Rahmenregeln in `docs/rules/`; Go-Test prüft, dass das Gesamturteil nur an `goal`-Zielen hängt und die Leiter alle Bosse der Welt abdeckt.

## Nicht-Ziele

Kein Ändern der Korridorwerte und keine Balance-Arbeit (B-645). Keine neue Auswertung außerhalb des Berichts.

## Regeln und Einschränkungen

Daten in `data/` (Rollen als Feld, nicht im Code); Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Berichte bleiben deterministisch; Vorlagen und Glossar beachten; Sim-Verhalten unverändert.

## Beispiele

- Lauf „Insel 1 solo (voll)": Endboss fällt Tag 19, Hub-Korridor verfehlt → Ziel erreicht (grün), Qualität 7 von 16.
- Lauf „ohne Hilfen": Endboss nie → Ziel nicht erreicht; Leiter endet bei „Wald“, Blocker „Hub 1, 0 Kämpfer, Gold 2 ab Tag 5“.

## Ausnahme- und Fehlerfälle

- Szenario ohne `goal`-Ziel: Bericht warnt, Urteil „kein Ziel definiert“.
- Kein früherer Lauf gleicher Konfiguration: Vergleichsspalte leer, kein Fehler.
- Mehrere Seeds mit gemischtem Ausgang: Quote statt Einzelurteil (E10).

## Akzeptanzkriterien

- **AC-01** Ein Lauf mit besiegtem Endboss und verfehlten Guard-Korridoren ist im Bericht und in `result.json` „Ziel erreicht“ (Go-Test).
- **AC-02** `data/balance-targets.json` kennt `role` (goal, guard, info) je Ziel; fehlende Rolle ist ein Prüffehler.
- **AC-03** Die Meilenstein-Leiter deckt alle Bosse und Stufenwechsel der Welt ab (Go-Test gegen `data/worlds`).
- **AC-04** Der Bericht beginnt mit Ergebnis, Leiter, Blocker; Zielliste und Kennzahlen stehen einklappbar darunter (HTML-Bericht, Sichtprüfung).
- **AC-05** `result.json` enthält `goalReached`, `milestones`, `blocker`, Delta zum letzten Lauf, nächsten Hebel.
- **AC-06** Die Kopfzeile nennt die Konfiguration und die Übereinstimmung mit dem Stufen-Test.
- **AC-07** Mehrere Seeds ergeben eine Quote je Meilenstein; Hilfe-Stufe steht im Urteil.
- **AC-08** Regeln stehen in `docs/rules/` (Quelle), der Go-Test aus AC-01 und AC-03 läuft in `task check:go`.

## Offene Fragen

- Zählt „ohne Hilfen“ später als Fail (Regression), sobald B-645 gelöst ist? Bis dahin nur Fortschritt. Entscheidung: 🧑.

## Notizen

2026-10-29 (Claude, Sichtung): erledigt in BS3. Der Bericht zeigt Ziel ja/nein und Qualität x von y, Meilenstein-Leiter, Blocker-Satz und `result.json`.
