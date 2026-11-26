# B-1039 · Schiff ist mit Segel etwa 3 Monarchenhöhen hoch und 4–5 lang

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** PN1
- **Projekt:** NAC
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` § 2: In K2C ist das Schiff mit Segel ≈ 3 M hoch und 4–5 M lang (Langschiff, Trireme). Unser `gebaeude-paper/schiff` ist mit Segel 164 px ≈ 1,35 M.

## Ziel

Das Schiff ist am Hafen das große Ziel der Insel und trägt sichtbar die Fracht und die Truppen.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (PAP), Fachprüfung `k3c-grafik`. Abnahme: 🧑.

## Anforderungen

- `schiff` alle 4 Stufen gleichmäßig auf ≈ 3 M Höhe mit Segel skalieren (Rumpf ≈ 0,7 M), Länge 4–5 M; Laderaum-Plätze (B-997) bleiben abzählbar.
- Bauplatz-Breite und Gerüst (`geruest-6`) passen zur neuen Größe oder werden als eigenes Ticket gemeldet.
- Optional ein Leitmotiv je Welt am Bug (Drachenkopf, Auge), nur als Variante.

## Nicht-Ziele

Neue Schiffsmechanik, Schiffsfahrt-Animation (`art/voyage`).

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/gebaeude-paper/schiff.paper.json`, `data/paper.json`, `tests/bildaufbauMasstab.test.ts`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –
- Größe über `data/paper.json` › `skalierung` (Default 2,2 → 2,97 M), Spec nur bei Bedarf.

## Beispiele

Monarch am Steg neben dem fertigen Schiff: Segel reicht etwa dreimal so hoch wie er.

## Ausnahme- und Fehlerfälle

Die Seekarten-Miniatur `seekarte-schiff` bleibt unverändert.

## Akzeptanzkriterien

- **AC-01** `schiff` ist mit Segel 2,8–3,2 M hoch, alle Stufen gültig, `k3c-grafik` ohne schweren Befund.
- **AC-02** Am TV (🧑): Schiff steht im Hafen ohne Überlappung mit Steg und Ladeplätzen.
- **AC-03** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

Gedächtnis `reference_k2c-bildauswertung` (Bilder 5, 6, 7, 13).
