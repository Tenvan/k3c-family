# B-447 · Der Server nennt je Mauer-, Turm- und Tor-Platz Linie und Sperrgrund (Linie fehlt, Tor nur außen)

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-207 (gesperrte Bauplätze mit Grund, W11.4) braucht neben der Platz-Stufe (`sites[].hubLevel`, W11.2) zwei weitere Sperrgründe: „Linie fehlt“ (Linie k braucht die Mauer der Linie k−1 derselben Seite, Q48) und „Tor nur an der äußersten Linie“ (Q47). Das Protokoll trägt dafür keine Angabe: `Site` in `src/model/types.ts` hat weder Linie noch Seite, `docs/protocol.md` › Wirtschaft kennt nur `hubLevel`. Der Client darf die Linie nicht aus `x` und den Mauer-Offsets nachrechnen (Entscheidung 001, Linien 2–5 streuen je Seed 0 bis 4 Units; Beleg `data/hub.json › wallLines`). B-208 verlangte ursprünglich „Art, Lage, Seite, Linie“; W11.2 hat nur `hubLevel` geliefert.

## Ziel

Der Server nennt je Mauer-, Turm- und Tor-Platz die Linie und, wenn der Platz gerade nicht bezahlbar ist, den Sperrgrund; der Client zeigt „Linie fehlt“ bzw. „Tor nur außen“ ohne Regel-Logik.

## Beteiligte und Zielgruppen

Spielende am TV (auch Kinder, bis 2 lokal); Agent in SRV liefert das Feld, Agent in CLI zeigt es (Erweiterung von B-207); 🧑 entscheidet.

## Anforderungen

- `sites[].line` (1–5) für Mauer, Turm und Tor; Fehlen = Hub-Platz oder älterer Server.
- `sites[].locked` mit dem Grund `hubLevel`, `line` oder `gate`, solange der Platz nicht bezahlbar ist (fehlt, wenn bezahlbar). Maßgeblich ist dieselbe Prüfung wie bei der Zahlung (`engine/sim/lines.go › lineOpen`), keine zweite Regel.
- Additiv, keine neue Protokoll-Version (Vorbild W11.2); Beispiel unter `testdata/protocol/`; Budget Q08 bleibt grün.

## Nicht-Ziele

Anzeige im Client (B-207 erweitert dann seine Funktion); Änderung der Freischalt-Regeln (B-206, B-112); neue Protokoll-Version.

## Regeln und Einschränkungen

Domäne SRV (`engine/net/protocol.go`, nur lesende Sicht in `engine/sim/site_view.go`); `docs/protocol.md`, `testdata/protocol/`, `src/model/types.ts` (nur Feld). Protokoll-Änderung = eigene Session (`docs/arbeitsweise.md` › Domänen). `engine/sim` importiert nichts aus `engine/net`. Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Hub-Stufe 2, rechts fehlt Linie 1 → Mauer der Linie 2 rechts: `{ "line": 2, "locked": "line" }`. Hub-Stufe 1 → Mauer der Linie 2: `locked: "hubLevel"` (der Client kennt dann `hubLevel` = 2). Linie 1 und 2 gebaut → Tor der Linie 1 `locked: "gate"`, Tor der Linie 2 bezahlbar (kein `locked`).

## Ausnahme- und Fehlerfälle

Mehrere Gründe gleichzeitig → der erste in der Reihenfolge `hubLevel`, `line`, `gate`. Gebauter oder bezahlter Platz → kein `locked`. Älterer Client → ignoriert die Felder.

## Akzeptanzkriterien

- **AC-01** Go-Test (`engine/net`): Für Hub-Stufe 1 und 2, mit und ohne gebaute Linie 1, tragen Mauer, Turm und Tor `line` und den richtigen `locked`-Grund; ein bezahlbarer Platz hat keinen.
- **AC-02** `docs/protocol.md` › Wirtschaft und ein Beispiel unter `testdata/protocol/` nennen beide Felder; `task check` und `task check:go` grün, Budget Q08 (`engine/room/stages_bench_test.go`).

## Offene Fragen

1. Bleibt „Linie fehlt“ in W11 draußen (Vorschlag, W11 zeigt nur „frei“ und „ab Hub-Stufe n“), und kommt dieses Ticket in einen späteren Sprint (SRV, dann CLI)? Oder soll W11 eine SRV-Session dafür aufnehmen? Entscheidet 🧑.

## Notizen

Entstanden aus der Prüfung „W11 und GEB5“ (2026-10-16): B-207 › Anforderungen verlangt „Linie fehlt“ und „Tor nur außen“, das Protokoll nach W11.2 trägt nur `hubLevel` (Beleg `src/model/types.ts › Site`, `docs/protocol.md`). Nummer B-447 im Worktree vergeben; bei Kollision beim Merge umnummerieren.
