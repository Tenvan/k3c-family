# B-856 · assets_set ruft die Paper-CLI gesammelt auf statt zwölfmal je Änderung

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

WZ48.2: `assets_set` startet bun zwölfmal (validate, diff, sechsmal frame, dreimal pixeldiff, render), je 0,23–0,4 s; das bremst Tests und Workbench.

## Ziel

Ein Sammelaufruf der Paper-CLI erledigt die Schritte eines `assets_set` in einem Prozess.

## Beteiligte und Zielgruppen

Agenten und 🧑 bei Asset-Änderungen; Claude (Umsetzung).

## Anforderungen

- Neuer CLI-Befehl (z. B. `paper batch`) nimmt eine Liste von Schritten und liefert JSON je Schritt.
- `paperassets` nutzt ihn; Ergebnis byte-gleich zu heute.

## Nicht-Ziele

Render-Logik ändern.

## Regeln und Einschränkungen

Klasse A (Schuld, DEV); Vertrag `docs/standards/paper-mcp.md` nachziehen.

- **Bereich:** `tools/paper/cli/**`, `tools/paper/bin/**`, `tools/k3c-workbench/internal/paperassets/**`, `tools/k3c-workbench/internal/review/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** exe

## Beispiele

`assets_set` auf monarch → ein bun-Start statt zwölf.

## Ausnahme- und Fehlerfälle

Ein Schritt scheitert → Fehler je Schritt, Rest bricht ab wie heute.

## Akzeptanzkriterien

- **AC-01** `assets_set` startet bun höchstens zweimal (Test zählt Aufrufe).
- **AC-02** Fixtures `tools/paper/fixtures/mcp/*.txt` unverändert.

## Offene Fragen

keine

## Notizen

Befund WZ48.2 (2026-11-14).
