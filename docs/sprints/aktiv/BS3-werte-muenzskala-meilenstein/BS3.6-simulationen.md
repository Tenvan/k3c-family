# BS3.6 · Simulationen BS3: Meilenstein-Lauf coop2 und Stufen-Tests

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** ?
- **Branch:** bs3/6-simulationen
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** –

## Ziel

Alle Simulationen von BS3 laufen hier, nicht in Umsetzungs-Sessions: Stufen-Tests (`task balance:fullgame`) und der Meilenstein-Lauf `coop2`, Welt standard, Seed `k3c-dev` (AC-05). Ergebnis: Berichte mit Bericht-IDs, rote Befunde als Tickets.

## Kontext

Regel von 🧑 (2026-10-28): Umsetzungs-Sessions fahren keine Simulationen, nur Code. Pro Sprint gibt es eine eigene autonome Sim-Session vor der Abnahme. Übernimmt die Lauf-Schritte aus BS3.4 (Schritte 1–4) und den Diagnosestand aus B-645 › Notizen. Läufe nur über `sim_test` (B-348), lange Läufe blockierend (`wait`).

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `pfad/…`

## Nicht-Ziele

Code- und Mechanikänderungen; Wert-Justierung nur in `data/` nach BS2-Regeln, höchstens ~5 Runden, sonst Befund-Ticket und 🧑 fragen.

## Schritte

1. Konkrete, nummerierte Schritte in sinnvoller Reihenfolge.

## Fertig, wenn

- [ ] AC-01: Jeder Punkt ist prüfbar: ein Befehl mit erwartetem Ergebnis, ein Test, eine Datei, eine Beobachtung.
- [ ] Punkte ohne Kriterium sind technische Voraussetzungen (z. B. CI grün).

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
