# NA1.8 · Abnahme am PC: Tasten-Icons im HUD

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** sprint/na1
- **Abhängig von:** NA1.7
- **Tickets:** B-911
- **Kriterien:** AC-08

## Ziel

🧑 sieht am PC die Papierschnitt-Icons für Tastatur und Xbox-Pad im HUD und bestätigt sie (B-911/AC-03).

## Kontext

Gerät: PC

Umsetzung NA1.4 (Icons, Gruppe `icons`) und NA1.5 (HUD). Spiel bauen und starten: `task build`, dann `dist/k3c-game/k3c-game.exe`, oder `task native:game`. Zwei Spieler: Tastatur und ein Xbox-Pad. Xbox und TV entfallen (Entscheidung 009).

## Erlaubte Dateien

- `public/grafik/icons/**`

## Nicht-Ziele

Codeänderungen; neue Belegungen. Wünsche zur Optik werden ein Ticket (Projekt NAT) oder ein kleiner Spec-Patch über `assets_set`.

## Schritte

1. Spiel starten, Spieler 1 Tastatur, Spieler 2 Pad.
2. Skill-Menü öffnen (Tastatur K, Pad D-Pad runter): Zeilen zeigen Papier-Kappen mit Beschriftung bzw. Xbox-Tasten in Farbe.
3. F1-Übersicht: Kappen F1 … F12 lesbar.
4. Eingabe-Profil wechseln (z. B. pfeile): Beschriftung folgt, breite Kappe bei „Entf“.
5. Ergebnis eintragen: bestätigt oder Wünsche (Farben, Größe).

## Fertig, wenn

- [ ] AC-08: 🧑 bestätigt die Tasten-Icons am PC (B-911/AC-03).

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
