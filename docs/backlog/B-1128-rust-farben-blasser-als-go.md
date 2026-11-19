# B-1128 · Rust-Spiel: Farben im Fenster deutlich blasser als im Go-Spiel

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU54
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Smoke-Abnahme durch Ralf (RU48): Die Farben im Rust-Fenster sind deutlich blasser als im Go-Spiel. Ursache (im Code belegt, Fenster-Screenshot offen): `Gfx::new` in `k3c-platform/src/backend/gpu.rs` nimmt `surface.get_default_config(..)`, das `caps.formats.first()` wählt; unter Windows ist das ein sRGB-Format (`*Srgb`). Shader und Texturen liefern schon sRGB-kodierte Werte (Texturen `Rgba8Unorm`, Farben aus Go), die sRGB-Fläche kodiert sie ein zweites Mal: Mitteltöne werden hell und blass. Aufnahmen (`still`, `--shots`) rastern in Software und zeigen den Fehler nicht.

## Ziel

Das Fenster zeigt dieselben Farben wie Go/Ebitengine (keine zweite sRGB-Kodierung).

## Beteiligte und Zielgruppen

Ralf (Abnahme am PC), Second Worker (Bildvergleich B-1129).

## Anforderungen

- Nicht-sRGB-Fläche wählen (bevorzugt `Bgra8Unorm`/`Rgba8Unorm`, sonst erstes Format ohne sRGB, sonst das erste).
- Reine Auswahlfunktion mit Test.
- Keine neue Abhängigkeit, keine Allokation je Bild.

## Nicht-Ziele

Neues Farbmanagement, Nacht-/Licht-Overlay (B-1056), Bildvergleich Go/Rust (B-1120, B-1129).

## Regeln und Einschränkungen

Rust-Leitplanken RU42; Plattform-Crate bleibt `unsafe`-frei in der Bibliothek.

- **Bereich:** `rust/crates/k3c-platform/src/backend/gpu.rs`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Fläche meldet [Bgra8UnormSrgb, Bgra8Unorm] → Bgra8Unorm.

## Ausnahme- und Fehlerfälle

Fläche kennt nur sRGB-Formate: das erste bleibt, Warnung im Log (`🚦 Fläche nur als sRGB`); eine sRGB-Ansicht mit Shader-Umrechnung wäre ein größerer Umbau und würde erst bei Bedarf Ticket (nicht beobachtet).

## Akzeptanzkriterien

- **AC-01** Auswahl-Test: nie ein sRGB-Format, wenn ein linear kodiertes angeboten wird.
- **AC-02** Ralf sieht im Fenster satte Farben wie in Go (manuelle Abnahme).
- **AC-03** Rest-Abweichungen im Bildvergleich laufen im Abnahme-Ticket B-1129 (und B-1120).

## Offene Fragen

Ob weitere Farbabweichungen bleiben (Overlay B-1056, Blend), zeigt erst der Bildvergleich B-1129.

## Notizen

Quelle: wgpu 30.0.2 `Surface::get_default_config` = `formats.first()`.
