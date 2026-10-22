# GEB11.1 · Wachstumsreihe der Burg festlegen (nur Grafik), Regel nachziehen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** geb11/1-wachstumsreihe-festlegen
- **Abhängig von:** –
- **Tickets:** B-458
- **Kriterien:** AC-01

## Ziel

Die Wachstumsreihe der Burg ist festgelegt (Grundentscheidung „nur Grafik und Client“) und als Darstellungsregel in `docs/rules/materialien-gebaeude.md` § 2 nachgezogen; Hub-Stufen 1–5 und Freischaltungen bleiben.

## Kontext

- Ersetzt den Workshop mit 🧑 (Auftrag „GEB komplett autonom“, 2026-10-22): Claude legt fest, 🧑 ändert im Nachgang.
- Der Snapshot kennt `World.hubLevel` und `World.hubUpgrade` (`gold`, `paid`, `state`: fehlt solange Gold offen, `waitingMaterial`, `waitingWorker`; fehlt auf der letzten Stufe), `src/model/types.ts`, `engine/sim/economy_view.go`. Daraus leitet der Client Zwischenlooks ab, ohne Regel-, Protokoll- oder Spielstand-Änderung.
- Echte neue Stufen (mehr Hub-Stufen, andere Freischaltungen, Schilde) sind eine kritische Entscheidung (Server, Golden, Balancing) und stehen als Vorlage in der Sprint-README › Offene Fragen.

## Erlaubte Dateien

- `docs/rules/materialien-gebaeude.md` (§ 2, außerhalb des erzeugten Blocks)
- Planungsdateien des Sprints

## Nicht-Ziele

`data/hub.json`, `engine/`, Protokoll, Spielstand, Kosten.

## Schritte

1. Felder des Snapshots prüfen (`hubLevel`, `hubUpgrade`).
2. Reihe festlegen: Lagerfeuer → Zelt → Hütte → Holzburg → Stein-, Kupfer-, Eisen-, Kristallburg, Gerüst während des Baus.
3. Als § 2.1 in `materialien-gebaeude.md` eintragen; Ergebnis mit „autonom, Claude“.

## Fertig, wenn

- [ ] AC-01: § 2.1 nennt Looks, Bedingung je Look und dass Stufen und Freischaltungen unverändert bleiben; die Abgrenzung zu GEB4/GEB9 steht in der Sprint-README.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
