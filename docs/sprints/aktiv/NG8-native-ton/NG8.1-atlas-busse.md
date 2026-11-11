# NG8.1 · Ton-Atlas laden, Busse und Lautstärke

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** ton
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng8/1-atlas-busse
- **Abhängig von:** NG1
- **Tickets:** B-801
- **Kriterien:** AC-01

## Ziel

Paket `audio` lädt den Ton-Atlas, spielt Slots und hat die Busse Effekte und Musik mit eigener Lautstärke.

## Kontext

Spec: B-801. Modul `native/k3c-game` (NG1): Vertrag `view/` (Ereignisse des Takts, Optionen), Platzhalter in `audio/` ersetzen, `game/` und `view/` unverändert; Importregel nur `view`, `art`, `k3c/engine/sim`, `k3c/data`. Ton-Dekoder aus der Festlegung in NG1 (Default `ebiten/v2/audio/vorbis`). Atlas: `public/audio/atlas.ogg`, `atlas.json` (Start und Dauer je Slot), Rückfall Einzeldatei; Vorbild `src/audio/atlas.ts`, `audioCore.ts`; Slot-Register `data/assets.json`. Tests ohne Audio-Gerät (Logik getrennt von der Wiedergabe).

## Erlaubte Dateien

- `native/k3c-game/audio/**`
- Planung NG8, B-801 (über MCP)

## Nicht-Ziele

Zuordnung der Ereignisse (NG8.2); neue Klänge; `tools/soundatlas`.

## Schritte

1. Test zuerst: `atlas.json` lesen, Start und Dauer je Slot, fehlender Slot → Einzeldatei bzw. stumm mit Meldung.
2. Wiedergabe über `audio.Context`, Busse mit Lautstärke.
3. Kein Audio-Gerät → stumm, eine Meldung im Log.

## Fertig, wenn

- [ ] AC-01: Tests grün.
- [ ] `task check:diff` grün.

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
