# B-811 · Plantagen-Bäume, gegnerische Geschosse und die Anleitung sehen im nativen Spiel aus wie im Browser-Client

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Drei Abweichungen des nativen Spiels vom Browser-Client: Plantagen-Bäume zeichnen beim Fällen weder Balken noch Schrumpfen (`native/k3c-game/props/plan.go:290` gegen `worldRenderer.ts:258`, `:284-285`); gegnerische Geschosse sind nicht rot (`worldRenderer.ts:163` gegen `native/k3c-game/fx/paint.go:174-180`); die Anleitung hat kein Bild für Münze und Mond (`hints/guideHints.ts:22-26`).

## Ziel

Diese drei Stellen sehen im nativen Spiel aus wie im Browser-Client.

## Beteiligte und Zielgruppen

Spieler, Claude (Umsetzung), Ralf (Sichtprüfung).

## Anforderungen

- Baum beim Fällen: Balken und Schrumpfen wie im Browser.
- Gegnerische Geschosse rot, eigene unverändert.
- Anleitung zeigt Münze und Mond als Bild.

## Nicht-Ziele

Figuren-Anzeigen (B-806), Menü-Optik (B-812).

## Regeln und Einschränkungen

Entscheidung 008, keine neuen Assets ohne `assets_*`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/props/**`, `native/k3c-game/fx/**`, `native/k3c-game/hud/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Bauer fällt Baum → Balken läuft, Baum schrumpft. Gegner schießt → rotes Geschoss.

## Ausnahme- und Fehlerfälle

Bild für Münze oder Mond fehlt → Text ohne Bild, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Test in `props`: gefällter Baum liefert Balkenanteil und Schrumpffaktor wie im Browser.
- **AC-02** Test in `fx`: Geschossfarbe nach Seite (Gegner rot).
- **AC-03** Test in `hud`: Anleitung enthält Münze und Mond mit Bild.

## Offene Fragen

keine

## Notizen

Gefunden im Funktionsabgleich NG9.1 (2026-11-11).
