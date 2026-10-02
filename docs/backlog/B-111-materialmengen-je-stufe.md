# B-111 · Die Materialmengen je Stufe passen zu den Kosten von Hub-Ausbau, Mauern und Gebäuden

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** R2
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 2 (Breite der Stufen ergänzt)

## Ausgangslage

R2.2 hat Kosten für Hub-Ausbau, Mauer- und Turm-Stufen und Gebäude beschlossen (`docs/rules/materialien-gebaeude.md`). Wie viel Material eine Stufe überhaupt liefert, folgt heute allein aus den Biom-Daten (`resourcesPerChunk`, `chunkWeights`, Länge) und `economy.json` (10 Einheiten je Objekt). **Erwartungswerte, aus den Daten gerechnet und nicht gegen den Generator gemessen (ungeprüft):**

| Stufe | Länge (Mittel) | Chunks ohne Hub (ca.) | Bäume | Felsen | Kupfererz | Material gesamt |
|---|---|---|---|---|---|---|
| Wald | 1000 | 18 | ≈ 61 | ≈ 5 | 0 | ≈ 610 Holz, ≈ 54 Stein |
| Höhle | 800 | 14 | ≈ 2 | ≈ 42 | 0 | ≈ 420 Stein, ≈ 23 Holz |
| Mine | 625 | 10 | 0 | ≈ 15 | ≈ 9 | ≈ 146 Stein, ≈ 87 Kupfer |

Dazu kommt 10 % Gegner-Drop (5 je Drop) der Hauptressource. Die beschlossenen Kosten übersteigen das Angebot deutlich: **Kupfer** (Hub-Stufe 3: 150, Kupfermauer 2 × 40, Kupferturm 2 × etwa 100, Schmiede 80, Heilplatz 50: zusammen etwa 560 je Hub) braucht ein Vielfaches der etwa 87 im ganzen Level; **Stein** (Hub-Stufe 2: 100, Steinmauer 60, Steinturm etwa 150, Tor 30, Kaserne 60, Lager 50, Taverne 60, Treppen 200) braucht etwa 700 **je Hub**, bei etwa 620 im ganzen Level (Wald 54, Höhle 420, Mine 146). Weil jeder Hub (Wald, Höhle, Mine) seinen eigenen Ausbau hat, ist der Bedarf der Insel ein Mehrfaches des Angebots. **Holz** (Stufe 1 ≈ 360) passt. Eisen und Kristall gibt es noch nicht als Stufen.

## Ziel

Die Materialmengen je Stufe und Insel sind festgelegt und passen zu den Kosten: Jedes Material ist knapp, aber der Hub-Ausbau ist erreichbar. Nutzen: Ohne diese Entscheidung kann der Hub-Ausbau nie erreicht werden, oder das Material ist so reichlich, dass die Stufen ihren Sinn verlieren.

## Beteiligte und Zielgruppen

🧑 entscheidet (Workshop, Agent bereitet vor); SIM und der Balancing-Tester (B-099) setzen und prüfen die Werte.

## Anforderungen

Die ersten Entscheidungen, genau gestellt:

0. **Wie breit sind die Stufen in die Tiefe?** (Frage von 🧑 2026-10-02) Gleich breit, nach unten breiter oder nach unten schmaler; Ist-Stand schmaler (Wald 900–1100, Höhle 700–900, Mine 550–700 Units). Folgen: Laufweg, Ressourcendichte, Rechenzeit (alle Stufen laufen weiter), Gegnerdruck.

1. **Welche Seite wird angepasst?** (a) Biom-Mengen erhöhen (mehr Felsen, Erz je Chunk, evtl. größere Mengen je Objekt), (b) Kosten senken, (c) beides, oder (d) weitere Quellen (Truhen, Gegner- und Boss-Drops, Händler/Tausch), sodass die Chunks nicht allein tragen.
2. **Welches Verhältnis Angebot zu Bedarf?** Zielwert je Material und Insel: z. B. Angebot 120–150 % des Bedarfs bis zur nächsten Hub-Stufe (Reserve für Zerstörungen), und welcher Anteil darf ungenutzt bleiben.
3. **Pro Hub oder pro Insel gerechnet?** Jeder Hub (Wald, Höhle, Mine) hat einen eigenen Ausbau; das Material kommt aus dem Insel-Vorrat (B-108). Das Ziel gilt je Hub-Stufe oder für die ganze Insel?

Ergebnis: je Material und Stufe eine Zielmenge im Regelwerk (`materialien-gebaeude.md`) mit Zielkorridor (z. B. Material am Tagesbeginn, Zeitpunkt der Hub-Stufen).

## Nicht-Ziele

Umsetzung in Go (SIM-Ticket), Kosten der Gebäude neu erfinden (nur anpassen, wenn Entscheidung 1 das verlangt), Gegner- und Boss-Werte (Regelwerk III).

## Regeln und Einschränkungen

Werte stehen in `data/`, Regeln in `docs/rules/`; Material gehört der Insel, Lager-Maximum 300 je Hub plus 300 je Lager (R2.2); jede Regel gilt für 2+ Spieler; Messung mit dem Balancing-Tester (B-099).

## Beispiele

nicht relevant – reine Entscheidung, kein Verhalten.

## Ausnahme- und Fehlerfälle

nicht relevant – reine Entscheidung, kein Verhalten.

## Akzeptanzkriterien

- **AC-01** Die Breite je Stufe und je Material und Stufe steht die Zielmenge im Regelwerk (`docs/rules/materialien-gebaeude.md`), mit Begründung.
- **AC-02** Die Zielkorridore für Material und Hub-Stufen sind angepasst (mit den beschlossenen Mengen).
- **AC-03** Die Messung der tatsächlichen Mengen aus dem Generator (Seeds) liegt als Wert im Ticket oder als B-099-Kennzahl vor.

## Offene Fragen

Die drei Entscheidungen oben (🧑).

## Notizen

Entstanden am 2026-10-02 aus dem Aufruf von `/oh-my-claudecode:ask-navigator` mit „Materialmengen pro Stufe“. Der Nebel-Test (Ziel in einem Satz, erste drei Entscheidungen genau) fällt positiv aus, deshalb keine Karte, sondern ein Frage-Ticket. Die Erwartungswerte rechnen Chunk-Gewichte mit den Mittelwerten der Bereiche (Hub 100 Units = 2 Chunks); die tatsächliche Verteilung kann mit dem Level-Generator (`level_generate`, k3c-dev) gemessen werden.
