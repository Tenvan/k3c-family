# B-1042 · Jede Welt hat ein großes Leitmotiv in der Ferne des Hubs

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** BB1
- **Projekt:** PAP
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` §§ 3, 5: In K2C hat jede Welt ein riesiges Leitmotiv in der Ferne (Berg mit Serpentinen, Athene-Statuen, Schlange, Schädelfels, Baumgeist), das im gerodeten Hub frei sichtbar ist. Unsere Hintergründe (`hintergruende/*`) haben das nicht je Welt.

## Ziel

Man erkennt die Welt am Hub-Hintergrund auf einen Blick.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (PAP), Fachprüfung `k3c-grafik`, `k3c-welten`. Entscheidung der Motive: 🧑.

## Anforderungen

- Je Welt (standard, sengoku, nil, britannien) ein Leitmotiv als Fern-Ebene, 3–5 M hoch, hell und blass (Luftperspektive), darf oben anschneiden.
- Zuordnung je Welt (Welt-Asset-Paket, Entscheidung 010).

## Nicht-Ziele

Zeichnen im Spiel und Parallaxe (BB2), Polar-Welt.

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/hintergruende/**`, `data/paper.json`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung, Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Sengoku: Vulkanberg mit Torii auf dem Grat. Nil: Pyramide und Sphinx. Britannien: Steinkreis auf einem Hügel. Standard: Burgberg.

## Ausnahme- und Fehlerfälle

Tiefe Stufen zeigen das Leitmotiv nicht.

## Akzeptanzkriterien

- **AC-01** Vier Leitmotive angelegt und zugeordnet, `k3c-grafik` ohne schweren Befund.
- **AC-02** `check_run task:check:diff` grün.

## Offene Fragen

Motiv je Welt: Vorschläge unter Beispiele, 🧑 entscheidet.

## Notizen

Gedächtnis `reference_k2c-bildauswertung` (Bilder 2, 3, 4, 6, 16).
