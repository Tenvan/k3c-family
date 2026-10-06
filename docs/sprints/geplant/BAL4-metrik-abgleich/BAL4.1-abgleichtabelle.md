# BAL4.1 · Abgleichtabelle aus Reports und Tester-Läufen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** bal4/1-abgleichtabelle
- **Abhängig von:** –
- **Tickets:** B-160
- **Kriterien:** AC-01, AC-04

## Ziel

Eine Abgleichtabelle in `docs/rules/` stellt je gemeinsamer Kennzahl den Simulatorwert neben den Wert echter Abende mit Abweichung; Kennzahlen mit zu wenigen Sitzungen sind als „nicht aussagekräftig“ mit Sitzungsanzahl markiert.

## Kontext

- **Voraussetzungen (Spec), alle vor dem Start prüfen:** Spielmetrik-Reports unter `reports/` aus B-150 (entsteht in S2, `S2.3-spielmetrik-report.md`; Schema 1 steht dort in `docs/protocol.md` › „Spielmetrik-Report“), Tester mit `task balance` aus BAL2 (`BAL2.2-task-balance-bericht.md`) und Profilen aus BAL1/BAL3, mindestens ein echter Spieleabend (P1, Protokoll in `docs/playtests/`). Fehlt etwas davon: Session `blockiert`, Ticket, nicht raten. Die Hardware-Regel (`docs/arbeitsweise.md` › Hardware entkoppelt) gilt nicht für diese Daten: ohne echte Sitzungen gibt es keine Abgleichswerte.
- **Offen, nicht beschlossen (Q12 und Q24 beantworten es nicht):** die **Toleranz je Kennzahl** und die Definition „echter Abend“ (z. B. Mindestdauer, Mindestzahl Spieler, Spieleabend-Sitzung ja oder nein). Diese Session legt die Tabelle mit Spalte „Toleranz“ an und lässt sie leer (`offen, entscheidet 🧑`); BAL4.2 füllt sie. Sitzungszählung für „nicht aussagekräftig“: eine Mindestanzahl ist ebenfalls offen; bis zum Beschluss die Anzahl der Sitzungen je Kennzahl nennen und als „vorläufig: Schwelle offen“ vermerken.
- **Kennzahlen, die beide Seiten haben:** Report (Q12/B-150): Tod durch was, Nächte überlebt, Zeit bis zum ersten Bau, Gold je Tag (Gold am Ende), Verbindungsabbrüche (nur Report). Tester: Überleben je Welle, erste Mauer, Gold am Morgen u. a. (B-099 › Kennzahlen, `docs/rules/zielkorridore.md` § 1–3). Die Schnittmenge bestimmt die Session aus den echten Feldern von Schema 1 und den Kennzahl-IDs des Testers, nicht aus dieser Liste. Kennzahl nur auf einer Seite messbar → Lücke als Ticket (B-160 › Ausnahmefälle).
- **Tabellenformat (B-160):** je Kennzahl Simulator (Median, p10–p90), echter Abend (Wert je Sitzung), Abweichung, Toleranz, Beschluss. Datei: Vorschlag der Planung `docs/rules/abgleich-simulator.md` (B-160: „Name legt die Session fest“).
- **Simulator-Läufe:** Szenario passend zum echten Abend (Spieleranzahl, Grad aus dem Report), 100 feste Seeds, Bot-Profile aus BAL1/BAL3 (Kind-Bot als Gegenstück zu Kindern, falls BAL3 ihn lieferte). Läufe schreiben nach `reports/`; Befehl und Flags aus BAL2.2 nachsehen, Läufe als EXE mit festem Pfad (kein `go run`).
- Metrik **nur** aus Berichten in `reports/` (Spec), keine Handeingaben aus Gedächtnis oder Chat.
- **Entsteht in P1:** `docs/playtests/vorlage.md` und das Protokoll des ersten Abends.

## Erlaubte Dateien

- `docs/rules/abgleich-simulator.md` (neu; Name ist Vorschlag)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Beschlüsse und Toleranzen (BAL4.2), Änderungen in `data/` (BAL4.3), Erfassen der Metrik (B-150), Spielspaß-Bewertung, Code.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Voraussetzungen prüfen (siehe Kontext).
2. Reports einlesen (Anzahl Sitzungen, Datum, Spieleranzahl, Grad je Sitzung) und nach „echter Abend“ vorläufig trennen; die Definition bleibt offen.
3. Tester-Läufe im passenden Szenario mit 100 Seeds ausführen.
4. Tabelle schreiben: je gemeinsame Kennzahl Simulator (Median, p10–p90), echter Abend, Abweichung, Anzahl Sitzungen; Toleranz und Beschluss als `offen, entscheidet 🧑`.
5. Kennzahlen mit zu wenigen Sitzungen als „nicht aussagekräftig (n Sitzungen)“ markieren und keine Abweichung als Befund führen; einseitig messbare Kennzahlen als Ticket.
6. `task check`. Ergebnis mit Liste der Kennzahlen, Sitzungsanzahl und Tickets, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: `docs/rules/abgleich-simulator.md` enthält jede gemeinsame Kennzahl aus Report und Tester mit Simulatorwert, Wert echter Abende und Abweichung.
- [ ] AC-04: Jede Kennzahl mit zu wenigen Sitzungen trägt „nicht aussagekräftig“ mit der Sitzungsanzahl.
- [ ] Alle Werte stammen aus `reports/` und Tester-Läufen; Toleranzen und Beschlüsse stehen als `offen, entscheidet 🧑`.

## Prüfen

```bash
task check
```

## Ergebnis

–
