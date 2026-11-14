# NV2.4 · Sichtprüfung und Messung am Mini-PC

- **Status:** verworfen
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** nv2/4-sichtpruefung-mini-pc
- **Abhängig von:** NV2.3
- **Tickets:** B-778
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

🧑 prüft den Rig-Monarchen am Mini-PC mit dem Auge und misst dort; danach steht fest, ob der Monarch als Rig ins native Spiel geht.

## Kontext

Paket aus NV2.3. Zu beurteilen: Schatten (dreht mit dem Teil), Schärfe der gedrehten Teile, Umhang, Sitz auf dem Pferd, getauschte Waffe und Rüstung. Kriterien AC-01 bis AC-05 liest die Session nur aus den Ergebnissen von NV2.1 bis NV2.3.

## Erlaubte Dateien

- Planung B-778, NV2 (über MCP)

## Nicht-Ziele

Code ändern; Befunde werden Tickets.

## Schritte

1. Paket am Mini-PC starten, Zustände durchschalten, mit zwei Gamepads prüfen.
2. Messlauf starten, Bericht zurückspielen.
3. Entscheidung und Befunde in B-778 › Notizen; Abnahme in der Sprint-README.

## Fertig, wenn

- [ ] AC-06: Messwerte und Sichtprüfung vom Mini-PC in B-778 › Notizen, Entscheidung von 🧑 notiert.

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

Verworfen (🧑 2026-11-11, Chat: NV2 schließen): AC-06 verschoben auf B-786 (Mini-PC erst, wenn das Spiel am PC fertig ist). Der Rig-Weg wird in NV3 (B-789) für alle Figuren weitergeführt; die Sichtprüfung am PC ist dort NV3.5. AC-01 bis AC-05 laut Ergebnis NV2.1 bis NV2.3. Keine manuelle Prüfung durchgeführt.
