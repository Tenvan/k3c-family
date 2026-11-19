# B-1148 · Schiffsrumpf auf 4 bis 5 M Länge zeichnen

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

Das Schiff ist 200 x 164 px, mit `skalierung` 2,2 (BB1.1) 440 px = 3,6 M lang und 2,97 M hoch. Der Standard `docs/standards/bildaufbau.md` nennt 4 bis 5 M Länge.

## Ziel

Der Rumpf ist 4 bis 5 M lang, die Höhe bleibt bei etwa 3 M.

## Beteiligte und Zielgruppen

Spieler am TV; Umsetzung Claude (PAP); Abnahme: Ralf.

## Anforderungen

- Spec `schiff` neu mit gestrecktem Rumpf (Rahmenbreite 4 bis 5 M nach `skalierung`), Stufen und Ladeplätze (B-997) bleiben abzählbar.
- `k3c-grafik` prüft; Texturbudget beachten.

## Nicht-Ziele

Gerüst und Bauplatz (B-1147).

## Regeln und Einschränkungen

Entscheidung 005, nur `assets_*`; Gen im Abnahme-Ticket. Planungsangaben:
- **Bereich:** `public/grafik/gebaeude-paper/schiff.paper.json`, `data/paper.json`, `tests/bildaufbauMasstab.test.ts`
- **Abhängig von:** B-1147
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

Schiff 4,5 M lang, 3 M hoch.

## Ausnahme- und Fehlerfälle

Faktor über 2 ohne Unschärfe nicht erreichbar: Spec größer zeichnen statt skalieren.

## Akzeptanzkriterien

- **AC-01** Test `tests/bildaufbauMasstab.test.ts`: Schiff-Länge 4 bis 5 M, Höhe 2,8 bis 3,2 M.
- **AC-02** Abnahme am TV (Ralf): Proportionen des Schiffs stimmig.

## Offene Fragen

keine

## Notizen

Von BB1.5 angelegt.
