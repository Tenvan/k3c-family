# B-1150 · Leitmotive mit Luftperspektive: weniger Kontrast

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die vier Leitmotive (BB1.4) haben harte Hell-Dunkel-Kontraste durch Auflagen `basis` und `schatten`; der Britannien-Hügel (biom-wald) ist kräftig grün, das Torii (koenigsrot) der stärkste Fleck, der Rauch (leinen licht) sitzt hell gegen den Himmel. Der Standard `docs/standards/bildaufbau.md` § 3 verlangt Luftperspektive.

## Ziel

Leitmotive wirken fern: gedämpfte Sättigung und Kontrast, bleiben aber lesbar.

## Beteiligte und Zielgruppen

Spieler am TV; Umsetzung Claude (PAP); Motiv-Wahl und Abnahme: Ralf.

## Anforderungen

- Auflagen entfernen oder Farben gedämpft (Material, Ton), Spec `leitmotiv-*` neu.
- Alternative: Tönung im Spiel (Dunst, RU55.4); dann Entscheidung vor der Umsetzung.

## Nicht-Ziele

Anbindung im Spiel (B-1146).

## Regeln und Einschränkungen

Entscheidung 005, nur `assets_*`; Gen im Abnahme-Ticket. Planungsangaben:
- **Bereich:** `public/grafik/hintergruende/leitmotiv-*.paper.json`, `public/grafik/hintergruende/golden.json`
- **Abhängig von:** BB1
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

Torii auf dem Vulkanberg ist nicht mehr der hellste und stärkste Farbfleck der Szene.

## Ausnahme- und Fehlerfälle

Motiv wird unlesbar: Sättigung nur teilweise dämpfen.

## Akzeptanzkriterien

- **AC-01** Spec: keine Auflagen `schatten` mehr in den vier Leitmotiven (oder dokumentiert gedämpft), Test in `tests/bildaufbauMasstab.test.ts`.
- **AC-02** Abnahme in der Workbench (Ralf): Motive wirken fern und bleiben lesbar.

## Offene Fragen

Zeichnung oder Dunst im Spiel? Entscheidet Ralf.

## Notizen

Von BB1.5 angelegt (Befund BB1.4).
