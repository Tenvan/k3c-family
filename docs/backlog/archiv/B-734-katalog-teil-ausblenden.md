# B-734 · Im Katalog lässt sich das gewählte Teil in der Vorschau aus- und einblenden

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** WZ43
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Im Katalog (`tools/k3c-workbench/frontend/src/assets/catalog/`) wählt ein Klick in die Vorschau ein Teil (Rahmen, Prompt-Buttons). Das Bild ist ein fertig gerendertes PNG (`AssetsFrame` → `paper frame`), ein Teil lässt sich darin nicht ausblenden. Wunsch 🧑 2026-11-07.

## Ziel

Verdeckte Teile und den Aufbau eines Assets sehen, indem man ein Teil ausblendet.

## Beteiligte und Zielgruppen

🧑 und Claude-Sessions, die Assets im Katalog ansehen.

## Anforderungen

- Knopf „Teil ausblenden“/„Teil einblenden“ beim gewählten Teil; ausgeblendete Teile bleiben beim Wechsel von Frame und Animation ausgeblendet, bis zum Asset-Wechsel.
- `paper frame` kennt `--ohne <teil>` (mehrfach): das Teil wird ohne Lagen gerendert, Gelenke und übrige Teile bleiben gleich.
- RPC `AssetsFrameOhne(id, animation, frame, ohne []string)` (Arbeitsstand), Render-Cache je Kombination.

## Nicht-Ziele

Teile ändern oder speichern (nur Ansehen, Entscheidung 005).

## Regeln und Einschränkungen

Domäne DEV (CLI-Änderung in `tools/paper`, Vertrag `docs/standards/paper-mcp.md`). UI nur über `@webkit`. Umgesetzt mit #514, Beleg in WZ43.3 (Tests) und WZ43.4 (Sichtprüfung 🧑).

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/**`, `tools/paper/cli/frame*.ts`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Monarch, Teil umhang gewählt, „Teil ausblenden“ → Bild ohne Umhang, Rücken sichtbar; Abspielen läuft ohne Umhang.

## Ausnahme- und Fehlerfälle

Unbekanntes Teil → Fehler `teil-unbekannt`, das letzte Bild bleibt stehen.

## Akzeptanzkriterien

- **AC-01** CLI-Test: `frame --ohne umhang` rendert ohne die Lagen des Umhangs.
- **AC-02** Katalog-Test und Web-Client: ausgeblendetes Teil fehlt im Bild, Einblenden bringt es zurück.

## Offene Fragen

keine

## Notizen

Aus der Session zu B-732/B-733.
