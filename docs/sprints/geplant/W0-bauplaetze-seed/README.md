# W0 · SIM · Bauplätze aus dem Seed

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-206
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bauplätze sind feste Offsets aus `data/hub.json` › `sites` ab der Seed-abhängigen Hub-Mitte (`engine/sim/world.go`, `emptySite`); je Seite gibt es genau eine Mauer (±44) und einen Turm (±36), kein Tor, keine Farm und keine Plätze für die Gebäude der Hub-Stufen 2 bis 4. Am 2026-10-04 hat 🧑 das Bau-Modell geändert (Beschlüsse Q43–Q55): Bauen an festen Punkten wie in Kingdom Two Crowns, Hub-Gebäude auf festen Hub-Plätzen, Mauer, Turm und Tor auf fünf Mauerlinien je Seite. Das ersetzt den wachsenden Hub aus Q26 (nur dessen Bauzeiten bleiben) und ist Voraussetzung für W1 bis W4.

## Ziel

Jede Stufe hat nach dem Seed alle Bauplätze des neuen Modells: feste Hub-Plätze mit Hub-Stufe aus den Daten, je Seite fünf Mauerlinien mit Mauer-, Turm- und Tor-Platz (Linie 1 fest, Linien 2–5 je Seed gestreut) und eine Farm je Seite; Linien und Tore sind nur in der Reihenfolge der Beschlüsse bezahlbar.

Am Ende sichtbar: `task check:go` grün mit Tests für Layout, Freischaltung und Daten-Abstände, alte Spielstände laden, Golden-Daten mit Begründung „Q43: Bauplätze aus dem Seed“ aktualisiert, Level-Golden und `rng.json` unverändert.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen) und Bauern; SIM setzt um, die Folge-Sprints W1–W4 bauen auf den Plätzen auf; Startwerte pflegt REG; 🧑 gibt die Spec frei und bestätigt die Startwerte.

## Anforderungen

B-206 › Anforderungen. Sprint-eigene Anforderungen aus den Beschlüssen vom 2026-10-04:

- Q43: feste Bauplätze; kein Platz bewegt sich. Hub-Gebäude auf festen Hub-Plätzen (Offset zur Hub-Mitte, `data/hub.json`), freigeschaltet über die Hub-Stufe; Mauer, Turm und Tor auf Mauerlinien (Daten + Seed).
- Q47: Tor-Platz je Linie (Mauer + 4), bezahlbar nur an der äußersten gebauten Linie. Q48: Linie k bezahlbar ab Hub-Stufe k, wenn Linie k−1 derselben Seite gebaut ist (Material egal), je Seite.
- Q49: 5 Linien je Seite bei ±44/64/84/104/124 (Startwerte), Turm 8 Units innen, Tor 4 Units außen; Linie 1 = ±44 mit Turm ±36 (kompatibel); alles unter dem Portal-Mindestabstand 150.
- Q50: Linie 1 und Hub-Plätze fest; Linien 2–5 streuen je Seed ±4 Units über einen **eigenen RNG-Strom**; Ressourcen, Portale, Camps und Golden-Level bleiben unverändert.
- Q51: Hub-Plätze dürfen zwischen Linie 1 und 2 liegen; die Farm ist ein fester Weltplatz je Seite zwischen Linie 1 und 2.
- Q52: Angebots-Zahlziele sind Anhänge mit festem `dx` am Gebäude (`data/buildings.json`); Q53: Schwert = Werkstatt `dx +4`. Q55: Treppen (+16/+24) und Händler (+8/+12, nur Tiefe 0) bleiben Hub-Plätze.
- Q54: Innere Türme und Tore bleiben stehen und wirken weiter.

## Nicht-Ziele

Hub-Ausbau an der Burg und Material-Stufen von Mauer und Turm (W1, B-112), Zerstörung, Reparatur und Spielstand-Version 3 (W1.3, S1.4), Wirkung von Tor, Kaserne, Taverne und den übrigen Gebäuden (W3, B-116), Plantage (W2.1), Angebots-Zahlziele als Spiel-Mechanik (W4.2, W4.3a, W4.3b; hier nur ihre `dx` in den Daten), Protokoll-Felder für Plätze (B-208), Anzeige freier und gesperrter Plätze mit Grund (B-207), Platz-Arten im Client (B-209).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/level/` und `engine/sim/`; deterministisch (`engine/rng`, nie `math/rand`), mit 2+ Spielern gleichzeitig. Der bestehende RNG-Strom des Generators (`rng.New(b.ID + ":" + seed)`, `engine/level/level.go`) wird nicht angefasst, die Streuung bekommt einen eigenen Strom. Golden-Daten nach `docs/arbeitsweise.md` › „Golden aktualisieren“ (B-137), kein neues Spielstand-Format (die gemeinsame Version 3 ist W1.3/S1.4, Beschluss Q42). Neue Felder an `Site` und `Layout` ohne JSON-Ausgabe, bis B-208 das Protokoll festlegt. Datei ≤ 400 Zeilen, Funktion ≤ 60, Session ≤ ~400 Code-Zeilen. Der Sprint bleibt in der Domäne SIM. Alle Offsets sind **Startwerte, 🧑 bestätigt bei Spec-Freigabe**.

## Beispiele

- Seed 7, Wald: Linie 1 bei Hub-Mitte ±44, Linie 3 links z. B. bei −86; zweiter Lauf mit Seed 7 → dieselben Positionen; Portale, Ressourcen und Camps wie vor W0.
- Hub-Stufe 2, links Mauer der Linie 1 gebaut: Mauer und Turm der Linie 2 links bezahlbar, rechts nicht (rechte Linie 1 fehlt); Linie 3 nirgends (Hub-Stufe 3 fehlt).
- Linke Linien 1 und 2 gebaut, Hub-Stufe 2: Tor der Linie 2 links bezahlbar, Tor der Linie 1 links nicht (nicht die äußerste).

## Ausnahme- und Fehlerfälle

- Spielstand v1/v2 mit gebauter Mauer bei ±44 und Turm bei ±36 → bleibt gebaut, die neuen Plätze sind `unpaid`.
- Mauer der Linie k−1 zerstört, Linie k schon gebaut → Linie k bleibt; neue Plätze außen richten sich nach der gebauten Linie k (Lesart der Planung, 🧑 bestätigt).
- Tor der Linie k gebaut, danach Linie k+1 gebaut → das innere Tor bleibt stehen und wirkt (Q54); das Tor der Linie k+1 wird bezahlbar.
- Zwei Spieler zahlen gleichzeitig an zwei Plätzen derselben Seite → beide Zahlungen zählen, das Ergebnis hängt nicht von der Spieler-Reihenfolge ab.

## Akzeptanzkriterien

- **AC-01** Das Layout jeder Stufe enthält je Seite 5 Mauerlinien mit Mauer-, Turm- (8 Units innen) und Tor-Platz (4 Units außen); Linie 1 liegt fest bei ±44, die Linien 2–5 streuen je Seed höchstens ±4 Units um ihren Startwert aus einem eigenen RNG-Strom; gleicher Seed ergibt gleiche Plätze (Test) (B-206/AC-01).
- **AC-02** Ressourcen-, Portal- und Camp-Positionen sind gegenüber vor W0 unverändert, `testdata/golden/level-*.json` und `rng.json` bleiben gleich (Test) (B-206/AC-02).
- **AC-03** Linie k ist erst ab Hub-Stufe k und nach gebauter Linie k−1 derselben Seite bezahlbar, das Tor nur an der äußersten gebauten Linie seiner Seite; innere Türme und Tore bleiben nach dem Bau einer äußeren Linie stehen und wirken; geprüft mit im Test gesetzter Hub-Stufe, da der Hub-Ausbau erst mit W1 kommt (Test) (B-206/AC-03, B-206/AC-04).
- **AC-04** Jeder Bau aus `docs/rules/materialien-gebaeude.md` § 3 hat genau einen Hub-Platz (Ausnahmen: Mauer, Turm und Tor je Linie, Farm je Seite); alle Zahlziele einschließlich Burg, Händler und Angebots-`dx` halten untereinander ≥ 4 Units Abstand, auch bei voller Streuung der Linien (Test über die Daten) (B-206/AC-05).
- **AC-05** Die Fixtures `testdata/saves/v1/` und `v2/` laden, alle dort gebauten Plätze bleiben gebaut (Test); Golden-Daten mit Begründung „Q43: Bauplätze aus dem Seed“ aktualisiert; `task check:go` und `task check` grün (B-206/AC-06).
- **AC-06** Zwei Spieler bezahlen Plätze gleichzeitig ohne Reihenfolge-Effekt (Test).

## Offene Fragen

Alle drei verhindern die Freigabe; 🧑 entscheidet bei der Spec-Freigabe. Die Sessions nennen den Vorschlag als Startwert.

1. **Farm gegen Turm der Linie 2** (Q49, Q50, Q51 mit AC-04): Zwischen Tor 1 (±48) und Turm 2 (±56, mit Streuung ±52 bis ±60) ist nur bei Streuung ≥ 0 Platz für die Farm mit 4 Units Abstand. Vorschlag: Farm ±52 und Linie 2 streut nur nach außen (0 bis +4). Alternativen: Farm ±52 mit kleinerem Abstand zum Turm hinnehmen; Linie 2 ohne Streuung.
2. **Tor k gegen Turm k+1** (Q49, Q50 mit AC-04): Nominal liegen 8 Units dazwischen; streuen zwei Nachbarlinien unabhängig ±4 gegeneinander, fallen Tor k und Turm k+1 zusammen. Vorschlag: Streuung ±2 je Linie (Abstand bleibt ≥ 4). Alternative: ±4 mit der Regel „Linie k+1 mindestens 16 Units außerhalb von Linie k“ (Generator rückt nach).
3. **Camps innerhalb der Linien** (Q50): Das Camp liegt heute in allen Golden-Seeds bei Hub-Mitte ±75 (Chunk direkt neben dem Hub, `placeEvents` bevorzugt Hub-Nähe), also zwischen Linie 2 und 3 und 1 Unit neben Turm 3 (±76). Q50 verbietet, Camps zu verschieben; eine Validierung „keine Camps innerhalb der äußersten Linie“ wäre für jeden Seed rot. Vorschlag: W0.2 prüft nur Portale, Camps innerhalb der Linien sind erlaubt und ihr Abstand zu Linien-Plätzen wird gemessen und in B-206 › Notizen abgelegt. Das gilt ebenso für Ressourcen, Truhen und Skill-Punkte aus dem Level, die zwischen ±50 und ±150 liegen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W0.1 | `W0.1-platz-daten.md` | Umsetzung | autonom | offen |
| W0.2 | `W0.2-linien-generator.md` | Umsetzung | autonom | offen |
| W0.3 | `W0.3-plaetze-sim.md` | Umsetzung | autonom | offen |
| W0.4 | `W0.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
