# B-326 · K1.3 darf den sechs neuen Gegnern Platzhalter-Sprites und Zuordnungs-Zeilen geben

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** K1
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

K1.3 legt sechs neue Gegner in `data/enemies.json` an (`lavaSlime`, `ironBeetle`, `fireSpirit`, `crystalSpider`, `shardling`, `crystalGuardian`). `tests/sprites.test.ts` verlangt für jeden Gegner ein vorhandenes Sprite in `data/sprites.json` (1 Test rot), `src/tools/zuordnung.test.ts` (B-161) für jede ID aus `enemies.json` eine Zeile in `docs/assets/zuordnung*.md` (6 Tests rot); `task check` ist rot. Beide Dateien stehen nicht in den Erlaubten Dateien von K1.3, Sprites sind ausdrücklich Nicht-Ziel (B-010).

## Ziel

K1.3 kann mit grünem `task check` abgeschlossen werden, ohne Grafiken zuzuordnen.

## Beteiligte und Zielgruppen

Entwicklung (SIM, CLI für Grafik); 🧑 entscheidet.

## Anforderungen

- Entscheidung, ob K1.3 `data/sprites.json` (je neuer Gegner ein vorhandenes Ersatz-Sprite, z. B. mit Tönung wie `greed`) und `docs/assets/zuordnung-objekte.md` (Tabelle „Gegner“, sechs Zeilen) ergänzen darf.

## Nicht-Ziele

Grafiken suchen oder zuordnen (eigenes Ticket der Domäne CLI), Namen der Gegner.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › „Wenn etwas nicht passt“; Erlaubte Dateien von K1.3; Lücken nennen ein Ticket (Kopf von `zuordnung-objekte.md`).

## Beispiele

Vorschlag je Gegner eine Zeile wie bei `smithy`: `| `lavaSlime` | `data/enemies.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-326** |`. In `data/sprites.json` je Gegner ein vorhandenes Sprite als Platzhalter (Vorschlag: Lavaschleim `mushroom` rot getönt, Eisenkäfer `hell-hound` grau, Feuergeist `ghost` orange, Kristallspinne `hell-hound` blau, Splitterwicht `mushroom` hellblau, Kristallwächter `hell-gato` blau); dann wäre die Zeile `zugeordnet` statt Lücke. Alternativ wählt 🧑 die Sprites.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Freigabe einer Datei.

## Akzeptanzkriterien

- **AC-01** 🧑 hat entschieden; bei Ja stehen `data/sprites.json` und `docs/assets/zuordnung-objekte.md` in den Erlaubten Dateien von K1.3 und `task check` ist grün.

## Offene Fragen

Darf K1.3 Platzhalter-Sprites in `data/sprites.json` und die Zeilen in `docs/assets/zuordnung-objekte.md` eintragen, und welche Sprites? 🧑

**Entscheidung 🧑 2026-10-06 (Chat):** „Platzhalter erlauben“. Umgesetzt in K1.3 nach dem Vorschlag: Lavaschleim `mushroom` `#e63946`, Eisenkäfer `hell-hound` `#8f8f8f`, Feuergeist `ghost` `#f4a261`, Kristallspinne `hell-hound` `#4ea8de`, Splitterwicht `mushroom` `#a9def9`, Kristallwächter `hell-gato` `#4361ee`; Zeilen `zugeordnet` mit Vermerk „Platzhalter“. Echte Figuren: B-329. `task check` grün.

## Notizen

`task check:go` ist mit K1.3 grün, rot sind nur `tests/sprites.test.ts` (1) und `src/tools/zuordnung.test.ts` (6, je neuer Gegner einer).
