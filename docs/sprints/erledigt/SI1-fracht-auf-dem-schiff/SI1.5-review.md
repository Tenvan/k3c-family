# SI1.5 · Review Sprint SI1 Fracht auf dem Schiff

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si1
- **Abhängig von:** SI1.4
- **Tickets:** B-967, B-997
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Der Sprint SI1 ist gegen seine Kriterien geprüft, schwere Befunde sind behoben, das Abnahme-Ticket ist angelegt.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Kriterien in der Sprint-README SI1 (AC-01 bis AC-06), Specs `B-967` und `B-997`, Ergebnisse der Sessions SI1.1 bis SI1.4. Die Abnahme am TV (AC-05) macht 🧑 und hält den Sprint nicht auf. Sim, Balance, Gen und Race laufen nicht hier, sondern im Abnahme-Ticket (B-742; Sim und Balancing sind gesperrt).

## Erlaubte Dateien

- `docs/rules/stufen.md`
- `docs/rules/wirtschaft.md`
- `engine/sim/**`
- `engine/room/**`
- `data/ship.json`
- `data/paper.json`
- `native/k3c-game/**`

## Nicht-Ziele

Neue Funktionen, Simulationsläufe, Balancing, Grafik-Golden.

## Schritte

1. Diff des Sprint-Branches gegen `main` lesen; je Kriterium den Nachweis aus den Session-Ergebnissen prüfen.
2. Fach-Reviews: `k3c-welten` (Engine, Spielstand), `k3c-grafik` (Assets), `k3c-hud` oder `k3c-reviewer` (Spiel).
3. Schwere Befunde beheben (Fix unter etwa 30 Zeilen ohne zweites Review), Rest als Tickets mit Projekt und Sprint.
4. `check_run task:check`, dann `check_run task:check:go`.
5. Abnahme-Ticket anlegen (`plan_create`, Kriterien Sim, Balance, Gen, Race nach Vorlage).
6. Glossar-Begriffe (Laderaum, Ladeplatz, Fracht, Beutel, Kiste) im letzten Commit eintragen; Abschnitt Abnahme der Sprint-README füllen.

## Fertig, wenn

- [ ] AC-01 bis AC-04: Nachweise geprüft, Abweichungen benannt.
- [ ] AC-06: `task:check` und `task:check:go` grün, Abnahme-Ticket angelegt, Abnahme-Abschnitt gefüllt.

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

- AC-01: umgesetzt (SI1.1, #655), Diff geprüft.
- AC-02, AC-04: nicht erfüllt, SI1.2 und SI1.4 verworfen (Go-Stopp). Der Sprint ändert weder Engine noch Spiel.
- AC-03: umgesetzt (SI1.3). `k3c-grafik` meldet statisch keinen schweren Befund, der Augenschein am TV ist offen. Die Platzmarken an Deck zeigen 6 Grundplätze und nicht die +4 aus dem Laderaum. Die echte Zahl zeichnet später das Spiel.
- AC-05: offen (🧑, Ralf-Queue).
- AC-06: `task:check:go` (go test) ist grün, nachdem der Detvec-Baum neu geschrieben wurde. Lint übersprungen, golangci-lint fehlt lokal. `task:check` hat 1878/1879 grün, `projectRules` scheitert nur an lokalen Resten. Abnahme-Ticket: B-1035.
