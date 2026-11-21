# B-1217 · Eine Welt hat eine Art (Ableger oder komplett neu); die Statistik erwartet danach eigene Assets, Einheiten und Werte

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WL2
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-21 (Mindestausstattung noch offen, Vorschlag gilt bis zur Entscheidung)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Eine Welt erbt über `extends` (data/worlds/<id>.json) Assets und Werte der Basis-Welt Standard; ob die Welt nur ein Ableger sein soll oder ein eigener Look mit eigenen Einheiten, steht nirgends. Die Statistik „Stand der Welt“ im Welten-Editor (`world/statsCount.ts`) zählt nur Inseln, Stufen, Etappen, Geschichte und geplante Chunks.

## Ziel

Jede Welt hat eine Art: **Ableger/Episode** (gleiche Assets und Werte der Eltern-Welt werden zunächst erwartet) oder **komplett neu** (neuer Look, neue Einheiten, eigene Werte werden erwartet). Die Statistik zeigt je Art, was für eine funktionierende Welt noch fehlt.

## Beteiligte und Zielgruppen

Ralf (🧑) legt Welten an und entscheidet die Art; Claude füllt fehlende Assets und Werte per MCP (assets_*, world_override_set).

## Anforderungen

- Die Art (`ableger` | `neu`) steht in der Welt-Datei und wird beim Anlegen (Dialog „Welt anlegen“, MCP world_create) gewählt; Standard bei vorhandenem `extends` ist `ableger`, ohne Vorgabe `neu`.
- Statistik bei **Ableger**: geerbte Assets und Werte sind in Ordnung; eigene Assets und Überschreibungen sind optional (Anzeige als Info).
- Statistik bei **neu**: Zeilen „Eigene Assets“ (Zahl exklusiver Assets, Soll: je Gruppe Figuren, Gegner, Gebäude, Hintergründe, Reittiere mindestens eines), „Eigene Einheiten“ (Truppen und Gegner mit eigenem Asset) und „Eigene Werte“ (Überschreibungen aus Werte-Reiter) zeigen vorhanden/nötig und was fehlt.
- Die Zeile „Inseln mit geplanten Chunks“ wird zur Info (grau, kein Fehlen): leere Pläne heißen reiner Zufall (docs/rules/geplante-chunks.md) und sind für eine spielbare Welt nicht nötig.
- Prompts im Welten-Editor: „Neue Einheiten und Assets für diese Welt“ nennt die fehlenden Gruppen.

## Nicht-Ziele

Keine neuen Assets oder Werte selbst; keine Änderung der Engine-Regeln (Go eingefroren, Entscheidung 013).

## Regeln und Einschränkungen

Neues Feld `kind` der Welt-Datei (docs/rules/welten.md § 2): Formatänderung von data/worlds/*.json; Rust-Lader (k3c-data) und Go-Lader der Workbench kennen es, alte Dateien ohne Feld bleiben gültig. Die Art lässt sich im Editor (Auswahl im Kopf, Marke neben dem Namen) und per MCP (world_set, Feld kind) setzen. Freigabe Ralf 2026-11-21.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `tools/k3c-workbench/**`, `data/worlds/**`, `rust/crates/k3c-data/**`, `engine/sim/**`, `docs/rules/welten.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

Neue Welt „Ägypten“ mit Art `neu`: Statistik meldet „Eigene Assets 0/5 · fehlt: Figuren, Gegner, Gebäude, Hintergründe, Reittiere“. Welt „Standard II“ als Ableger von Standard: keine Pflicht, grün.

## Ausnahme- und Fehlerfälle

Welt-Datei ohne `art`: gilt als `neu`, wenn sie kein `extends` hat, sonst `ableger`. Unbekannter Wert: world_check meldet den Fehler.

## Akzeptanzkriterien

- **AC-01** Statistik rechnet je Art (Vitest `statsCount.test.ts`: Ableger ohne eigene Assets grün, neu ohne eigene Assets mit „fehlt“).
- **AC-02** „Welt anlegen“ und world_create nehmen die Art an, die Datei enthält sie, world_check prüft den Wert.
- **AC-03** Die Zeile für geplante Chunks ist eine Info ohne Warnfarbe.

## Offene Fragen

Was ist die Mindestausstattung einer neuen Welt (Gruppen und Zahl)? Vorschlag oben; 🧑 entscheidet bei der Freigabe.

## Notizen

Anlass: Rückfrage Ralf 2026-11-21 zur Zeile „Inseln mit geplanten Chunks“ und Wunsch nach eigenen Einheiten und Assets je neuer Welt.

**Umsetzung, Stand 2026-11-21 (Branch wl2/welt-art):**

- Datei-Feld heißt `kind` (Code englisch), Werte `spinoff` | `new` statt `art`/`ableger`/`neu`. Ohne Angabe: `spinoff` mit `extends`, sonst `new`; `spinoff` ohne `extends` und unbekannte Werte sind Fehler (Go `sim.worldKind`, Rust `k3c_data::world::check_kind`, Workbench `world.Detail.Kind`).
- Statistik: Zeile „Eigene Assets“ (neu: je Gruppe Figuren, Gegner, Gebäude, Hintergründe, Reittiere eines, sonst „fehlt: …“; Ableger: nur Zahl, optional; Basis-Welt: keine Zeile); „geplante Chunks“ ist nur noch Info (AC-03).
- Offen: Zeilen „Eigene Einheiten“ und „Eigene Werte“, Auswahl der Art beim Anlegen (Dialog und world_create), Prompt „Neue Einheiten und Assets“ (AC-02), Entscheidung der Mindestausstattung (🧑).
