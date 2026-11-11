# NG1.3 · Grafik-Bibliothek art: Sheets, Rigs, Schrift

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng1/3-grafik-bibliothek
- **Abhängig von:** NG1.1
- **Tickets:** B-792
- **Kriterien:** AC-06

## Ziel

Paket `art` lädt die Grafik eines Levels (Papier-Sheets, Rig-Bibliothek) und die Schrift und meldet fehlende Grafik klar.

## Kontext

Spec: B-792 (Anforderung Grafik `art`). Vorbilder: `native/k3c-ebiten/paper.go` (Gruppen-Indizes `public/grafik/<gruppe>/index.paper.json`, `manifest.json`, `data/paper.json`), `native/k3c-ebiten/rigs.go` und `native/k3c-rig/paint` (Laden der Rig-Teile aus `native/k3c-rig/assets`). Texturen je Level statt alle (NAT › Notizen: alle Sheets 757 MiB RGBA). Schrift aus der Entscheidung 🧑 zu B-792 (Default: freie OFL-Schrift, Datei über `assets_*` angelegt). Grafik ist nicht eingecheckt (`task gen:paper`); Tests nutzen kleine Test-Sheets unter `art/testdata/`.

## Erlaubte Dateien

- `native/k3c-game/art/**`
- Planung NG1, B-792 (über MCP)

## Nicht-Ziele

Zeichnen der Fachbereiche; neue Grafik zeichnen; `gen:paper` ändern.

## Schritte

1. Test zuerst: Laden eines Test-Levels aus `art/testdata`, fehlende Datei → Fehler mit Pfad.
2. Sheet-Bibliothek je Level (Index, Manifest, Frames).
3. Rig-Bibliothek über `native/k3c-rig/paint`.
4. Schrift laden (Rollen-Größen kommen später aus NG6).
5. Texturbudget (Summe RGBA) einmal je Level ins Log (Emoji 📂).

## Fertig, wenn

- [ ] AC-06: Tests für Laden, fehlende Grafik (Meldung mit Pfad) und Texturbudget grün; `go test ./art/` grün.
- [ ] Datei ≤ 400, Funktion ≤ 60 Zeilen.

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
