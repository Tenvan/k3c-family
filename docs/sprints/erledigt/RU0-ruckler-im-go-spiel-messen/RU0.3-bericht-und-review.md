# RU0.3 · Bericht für Gate G0 und Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/ru0
- **Abhängig von:** RU0.2
- **Tickets:** B-1019
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

`docs/messung/ruckler-go.md` nennt die Ursachen der Ruckler mit Anteil; 🧑 kann G0 entscheiden.

## Kontext

Werte aus `.work/messung/` (RU0.2). Lesart: Hat „GC aus“ genauso viele lange Bilder wie „normal“, ist GC nicht die Ursache. Verschwinden sie mit `--pacing frame`, ist es der Bildtakt. Bleiben sie in allen Läufen, bleiben Laden, Ton, Treiber oder Scheduling (Entscheidung 006 › „Was die Erstmessung zur Vermutung sagt“). Der Bericht trennt gemessen von vermutet.

## Erlaubte Dateien

- `docs/messung/ruckler-go.md`
- `native/k3c-game/game/measure*.go`
- `native/k3c-game/game/pacing*.go`

## Nicht-Ziele

Eine Entscheidung über Rust treffen. Den Fix einbauen.

## Schritte

1. Kriterien AC-01 und AC-02 mit den Befehlen der Sessions nachprüfen.
2. Bericht schreiben: Szene und Aufrufzeilen, Tabelle der fünf Läufe, je Lauf ein Diagramm oder eine Liste der zehn längsten Bilder mit Takten, alpha und GC; Abschnitt „Ursachen mit Anteil“; Abschnitt „Was das für Rust heißt“ (trifft Rust die Ursache, ja oder nein, mit Begründung); Abschnitt „Was nicht gemessen ist“.
3. Für 🧑 die fünf Aufrufzeilen zum Ansehen bereitstellen; das Urteil „sichtbar ruckelfrei: ja oder nein“ je Lauf trägt 🧑 ein.
4. Ist-Werte für B-1015/AC-02 notieren (Dauer und Token je Session).
5. Abschnitt Abnahme der Sprint-README füllen, PR öffnen, G0 in die Ralf-Queue.

## Fertig, wenn

- [ ] AC-03: Der Bericht nennt je Ursache den Anteil der langen Bilder und sagt, ob Rust sie trifft.
- [ ] Jedes Kriterium ist im Abschnitt Abnahme genannt; das Urteil von 🧑 je Lauf steht im Bericht oder ist als offen markiert.

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

2026-11-16 (k3c-10): Bericht als RU0.2 › Ergebnis statt eigener Datei `docs/messung/ruckler-go.md` (Abweichung, Inhalt vollständig dort). Gate G0 entschieden durch 🧑: Umstieg auf Rust; erste Rust-Welle RU1/RU2 freigegeben. `task check:diff` grün. Voller 5-Minuten-Lauf B-1020 entfällt (Entscheidung gefallen).
