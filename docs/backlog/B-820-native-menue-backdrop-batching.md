# B-820 · Menü-Hintergrund wechselt den Filter je Scheibe und bricht das Batching

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-12
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

`native/k3c-game/menu/backdrop.go:84-97` (`painter.disc`, Filter-Wechsel in Zeile 95): Jede Scheibe des Papier-Hintergrunds setzt `FilterLinear` und danach wieder `FilterNearest`. Zwischen Rechtecken (Nearest) und Scheiben (Linear) wechselt der Zustand je Aufruf, Ebitengine kann die Draw-Aufrufe des Hintergrunds nicht bündeln. Befund aus dem Review NG13.6, ohne gemessenen Leistungsverlust.

## Ziel

Der Menü-Hintergrund zeichnet mit möglichst wenigen Zustandswechseln; Rechtecke und Scheiben lassen sich bündeln, das Bild bleibt gleich.

## Beteiligte und Zielgruppen

Spieler am PC und Mini-PC (Menü-Leistung); Claude (Umsetzung).

## Anforderungen

- Scheiben und Rechtecke des Hintergrunds nutzen einen gemeinsamen Filter oder werden nach Filter gruppiert gezeichnet.
- Bild des Hintergrunds sichtbar unverändert (Kanten der Scheiben bleiben glatt).
- Keine Allokation je Bild (`view/doc.go`).

## Nicht-Ziele

Umbau der Karten oder Illustrationen (B-818), Leistungsmessung des ganzen Menüs.

## Regeln und Einschränkungen

Klasse A (Schuld, CLI, ohne Spielregeln, Protokoll, Spielstand-Format, neue Abhängigkeit). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

- **Bereich:** `native/k3c-game/menu/backdrop.go`, `native/k3c-game/menu/backdrop_test.go`
- **Abhängig von:** NG13
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Hauptmenü wird gezeichnet → der Hintergrund setzt den Filter höchstens einmal je Gruppe statt je Scheibe.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Zeichenreihenfolge ohne Eingaben oder Fehlerpfade.

## Akzeptanzkriterien

- **AC-01** Test in `menu`: Der Hintergrund wechselt den Filter je Bild höchstens einmal (Zähler oder Aufrufliste des Painters).
- **AC-02** `task native:game:e2e` grün, Bilder der Menü-Stationen unverändert.

## Offene Fragen

keine

## Notizen

Review NG13.6 (2026-11-12), `k3c-reviewer`.
