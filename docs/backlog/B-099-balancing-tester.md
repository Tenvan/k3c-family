# B-099 · Ein automatischer Balancing-Tester prüft Regeln und Werte gegen messbare Ziele

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Balancing-Werte stehen in `data/*.json`, die Simulation ist deterministisch (`engine/sim`, `engine/rng`) und läuft headless (`sim.CreateWorld`, `sim.Step`, Werkzeug `sim_run` in k3c-dev, M6). Ob Werte „gut“ sind, wird bisher nur von Hand und im Spiel beurteilt. Regeln aus dem Regelwerk (B-004) nennen noch keine messbaren Ziele.

## Ziel

Ein Werkzeug spielt viele Läufe automatisch (Seeds × Spieleranzahl × Bot-Profil × Tiefe), misst Kennzahlen und prüft sie gegen **Zielkorridore** („in Zeitraum X höchstens/mindestens Y“). Nutzen: Balancing wird wiederholbar und schnell; eine Wertänderung zeigt sofort, welche Ziele sie verletzt.

## Beteiligte und Zielgruppen

🧑 setzt Ziele und entscheidet über Wertänderungen (REG); Entwickler und Agenten führen das Werkzeug aus; Ergebnisse speisen die Workshops.

## Anforderungen

Vorschläge aus dem Best-Practice-Bereich (🧑 wählt aus, Umfang pro Sprint):

- **Szenario-Matrix:** Seed-Menge (z. B. 100, feste Liste statt Zufall, damit reproduzierbar) × Spieleranzahl (1–4) × Bot-Profil × Biom/Tiefe × Dauer in Tagen. Jeder Lauf ist durch (Seed, Parameter, Datenstand) vollständig bestimmt.
- **Bot-Profile statt echter Spieler:** wenige einfache, deterministische Verhaltensweisen (passiv, sparsam, baut zuerst Mauern, baut zuerst Wirtschaft, kooperativ mit 2 Spielern). Ein Bot spielt nur über `PlayerCommand`, nie über interne Abkürzungen.
- **Kennzahlen je Lauf:** Tick der ersten Gold-Schwelle, Gold und Material zu Dämmerung und Dämmerungsbeginn je Tag, Zeit bis zur ersten Mauer/zum ersten Bogen, Verluste je Welle, Überleben je Welle, Tick des Burgfalls, Wirtschaftsfluss (Einnahmen/Ausgaben je Tag), erreichte Tiefe.
- **Ziele als Korridore, nicht als Mittelwert:** je Kennzahl Untergrenze/Obergrenze für p10–p90 über die Seeds (z. B. „Überlebensquote Welle 3 mit Bot ‚sparsam‘ zwischen 70 und 95 %“; „erste Mauer vor Tag 2“). Ziele liegen als Daten vor (z. B. `data/balance-targets.json`, Format später), Regeln im Regelwerk nennen sie.
- **Bericht:** Tabelle je Szenario mit Median, p10, p90 und Ampel (im Korridor, knapp, verletzt); maschinenlesbar (JSON) und lesbar (Markdown/Text). Verletzte Ziele nennen Kennzahl, Szenario und die auslösenden Seeds zum Nachspielen.
- **Sensitivität:** Ein-Parameter-Variation („Wert W von `data/economy.json` ±10 %, ±25 %“) mit Bericht, welche Kennzahlen kippen, bevor irgendetwas automatisch optimiert wird.
- **Regressionsschutz:** Eine Baseline der Kennzahlen (wie die Golden-Dateien) und ein CI-Lauf mit kleiner Seed-Menge, der bei Verletzung der Ziele oder Drift außerhalb einer Toleranz warnt (zuerst nur Bericht, später Gate).
- **Später, optional:** automatisches Suchen von Werten (Zufallssuche oder Hill-Climbing) innerhalb von Grenzen aus `data/`; Ergebnis ist ein Vorschlag, den 🧑 freigibt.

## Nicht-Ziele

Automatisches Ändern von `data/*.json` ohne Freigabe, KI-Spieler mit Lernen, Spielgefühl und Spaß (bleibt Playtest), Lasttest des Servers (SP11).

## Regeln und Einschränkungen

Deterministisch (`engine/rng`, keine Wanduhr), Läufe über `PlayerCommand`; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Werte ändern nur REG mit Beschluss. Reihenfolge: erst messbare Ziele im Regelwerk (R1), dann Bots und Kennzahlen (SIM), dann Bericht und CI.

## Beispiele

`k3c-balance run --seeds 100 --players 2 --bot saver --days 5` → Bericht: „Erste Mauer: Median Tag 1,6 (Ziel ≤ Tag 2) ✓ · Überleben Welle 3: 64 % (Ziel 70–95 %) ✗, Seeds …“.

## Ausnahme- und Fehlerfälle

Lauf bricht ab (Fehler in der Simulation) → Lauf als „ungültig“ im Bericht mit Seed, nicht stillschweigend übergangen. Ziel ohne passende Kennzahl → Fehler beim Laden. Zu wenige Seeds für die Aussage → Hinweis im Bericht.

## Akzeptanzkriterien

- **AC-01** Ein Lauf mit denselben Seeds, Parametern und Daten liefert byte-gleiche Kennzahlen (Test).
- **AC-02** Mindestens zwei Bot-Profile und die Kennzahlen aus dem Abschnitt Anforderungen sind umgesetzt und getestet.
- **AC-03** Ziele als Korridore werden aus Daten geladen und im Bericht als im Korridor/knapp/verletzt bewertet (Test).
- **AC-04** Der Bericht nennt zu jedem verletzten Ziel die Seeds zum Nachspielen.
- **AC-05** Ein CI-Lauf mit kleiner Seed-Menge erzeugt den Bericht.

## Offene Fragen

- Wo lebt das Werkzeug (Paket `engine/balance` mit eigenem Befehl, oder als Tool in k3c-dev)? Entscheidet 🧑 bei der Planung des Sprints.
- Welche Kennzahlen und Zielkorridore gelten zuerst? Kommen aus den Regelwerk-Workshops (R1.2, R1.3), je Regel nennen sie einen messbaren Zielwert.
- Wie viele Seeds sind genug (Rechenzeit gegen Aussagekraft)? Wird mit den ersten Läufen gemessen.

## Notizen

Vorbereitung in R1 abgeschlossen: Zielkorridore stehen in `docs/rules/wirtschaft.md` (§ 1, 3, 4) und `docs/rules/stufen.md` (§ 1, 3); Kennzahlen: `docs/rules/ist-abgleich.md` › Messgrößen. Antworten auf die Offenen Fragen: Kennzahlen und Ziele siehe diese Dateien (Standardszenario Wald, 2 Spieler, Bot „sparsam“, je 100 Seeds, Normal). R2 (2026-10-02): Zielkorridore für Material und Gebäude stehen in `docs/rules/materialien-gebaeude.md` (§ 1, 2, 3.1, 4). Neue Kennzahlen: Adern-Ausbeute je Minute, Zeit am Lager-Maximum, Material-Ausgaben je Bau, Wartezeit „bezahlt bis gebaut“, Zerstörungen je Welle, Zeitpunkt der Hub-Stufen. R3 (2026-10-02): Zielkorridore für Monarch und Bürger stehen in `docs/rules/monarch.md` und `buerger.md`. Neue Kennzahlen: Skill-Punkte im Pool und je Spieler, Skill-Einsatz, `playerDown` und Wiederbelebungsquote, Anteil des Monarchen am Schaden, Kämpfer je Hub zu Tagesbeginn, Verluste je Welle (`troopLost`), Zeitpunkt des ersten Elite-Upgrades. Der Tester soll je Schwierigkeitsgrad, je Spieleranzahl 1–4 und mit allen aktiven Stufen einer Insel laufen (B-100, B-101) und die Last messen (SP11).
