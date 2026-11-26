# B-1038 · Bürger und Truppen sind 0,55–0,65 Monarchenhöhen groß

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

Standard `docs/standards/bildaufbau.md` § 2 (nach K2C, 2026-11-17): Bürger und Truppen sind dort 0,5–0,6 M hoch (M = berittener Monarch). Bei uns sind es 0,84 M (`data/paper.json` › `figuren.truppen.peasant.hoehe` 102,5 neben dem Monarchen 121,5).

## Ziel

Bürger und Truppen wirken neben dem Monarchen klein, wie in K2C; der Monarch sticht heraus.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (PAP), Fachprüfung `k3c-grafik`. Abnahme am TV: 🧑.

## Anforderungen

- Zielhöhen in `data/paper.json` › `figuren.truppen` auf 0,55–0,65 M (≈ 67–79 px), Gegner nach Tabelle § 2.
- Lesbarkeit prüfen: Werkzeug, Bogen und Schild bleiben am TV erkennbar; wenn nicht, die Specs vereinfachen statt größer zu bleiben.
- Marken über Figuren (Krone, Balken) folgen der neuen Höhe.

## Nicht-Ziele

Monarch und Reittiere ändern; Trefferzonen und Werte der Engine.

## Regeln und Einschränkungen

- **Bereich:** `data/paper.json`, `public/grafik/truppen/**`, `tests/bildaufbauMasstab.test.ts`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –
- Assets nur über `assets_*`; Golden und `gen:paper` im Abnahme-Ticket (B-742).

## Beispiele

Bauer neben dem berittenen Monarchen: reicht bis etwa zum Sattel statt bis zur Schulter.

## Ausnahme- und Fehlerfälle

Schwarm-Gegner (Ratten) und Bosse behalten eigene Größen nach Tabelle § 2.

## Akzeptanzkriterien

- **AC-01** `data/paper.json` setzt Bürger und Truppen auf 0,55–0,65 M; `k3c-grafik` ohne schweren Befund.
- **AC-02** Beobachtung am TV (🧑): Werkzeug und Waffe der Truppen bleiben in 2 Ansichten erkennbar.
- **AC-03** `check_run task:check:diff` grün.

## Offene Fragen

Entschieden (autonom, Claude, 2026-11-19): 0,6 M (Truppen 0,58–0,60 M); 🧑 justiert am TV (ein Wert je Rolle in `data/paper.json`).

## Notizen

Auswertung je Screenshot: Gedächtnis `reference_k2c-bildauswertung`.
