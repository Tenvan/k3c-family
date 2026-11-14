# PT6.1 · Bestandsaufnahme und Varianten für Asset-Pakete je Welt (B-880)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/pt6
- **Abhängig von:** –
- **Tickets:** B-880
- **Kriterien:** AC-01

## Ziel

Entscheidungsvorlage `docs/decisions/010-asset-pakete-je-welt.md` (Entwurf) mit Bestandsaufnahme, drei Varianten und Empfehlung für Asset-Pakete je Welt.

## Kontext

Siehe B-880. Quellen der Bestandsaufnahme: `data/paper.json`, `public/grafik/<gruppe>/index.paper.json`, `data/worlds/`, `data/embed.go`, `native/k3c-game/art`, `native/Taskfile.yml` (game:dist).

## Erlaubte Dateien

- `docs/decisions/010-asset-pakete-je-welt.md`

## Nicht-Ziele

Umsetzung, Folge-Tickets (erst nach Entscheidung von Ralf, AC-02).

## Schritte

1. Bestandsaufnahme lesen.
2. Vorlage schreiben.
3. `task check:diff` grün.

## Fertig, wenn

- [x] AC-01: `docs/decisions/010-asset-pakete-je-welt.md` liegt vor (Ausgangslage, Varianten A bis C, Empfehlung, offene Fragen).

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

AC-01 geprüft: Vorlage 010 angelegt (Entwurf), `task check:diff` grün. Empfehlung: Variante A (Welt-Manifest, gemeinsamer Pool), C als späterer Ausbau. B-880 bleibt offen bis zur Entscheidung (AC-02). Keine neuen Tickets.
