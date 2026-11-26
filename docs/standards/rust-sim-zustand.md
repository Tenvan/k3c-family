# Zustandsmodell der Rust-Sim

Quelle: Sprint RU8 (B-1014, Vorentscheidungen). Ergänzt den Port-Leitfaden `docs/standards/rust-port.md` (§ 4). Gilt für
`k3c-sim`, `k3c-level`, `k3c-room`. Ablage: `rust/crates/k3c-sim/src/state/`.

Go hält die Welt als Geflecht aus Zeigern (`[]*Coin`, `*Player`), das jede Funktion frei ändert. Rust lässt das nicht zu.
Das Modell: **Die Welt besitzt alles, Verweise sind IDs, keine Zeiger.** Die Entscheidungen sind fest; wer anders rechnen
will, legt ein Ticket an.

## Vorentscheidungen

1. `World` besitzt alle Dinge direkt als `Vec<Player>`, `Vec<Coin>`, `Vec<Troop>` … in Entstehungs-Reihenfolge. Kein `Box`, `Rc`, `RefCell`, kein Lebenszeit-Parameter an Zustands-Typen.
2. Verweise zwischen Dingen sind IDs wie in Go (`WorkerID *int` → `worker_id: Option<i64>`). Nachschlagen über `fn troop(&self, id: i64) -> Option<&Troop>` und `troop_mut`; lineare Suche wie in Go, kein Index-Speicher.
3. Go-Zeiger auf Einzelteile (`*Travel`, `*Merchant`, `*Crown`, `*Job`, `*float64`) werden `Option<T>`.
4. Rückverweise (`island *Island`, `hubW *World`) gibt es in Rust nicht. Funktionen, die die Insel brauchen, bekommen sie als Parameter.
5. Der Material-Vorrat `Stock` gehört in einer Insel allen Stufen. Rust: Die Insel besitzt ihn; vor dem Schritt einer Stufe tauscht sie ihn mit `std::mem::swap` in die Stufe und danach zurück. Solange eine Stufe rechnet, liest niemand den Vorrat der Insel. Braucht Go an einer Stelle beides zugleich: Ticket, nicht raten.
6. Unveränderliche Wertesätze (`vals *WorldValues`, `hubData *HubData`) werden `Arc<WorldValues>` und `Arc<HubData>`. Das ist die einzige erlaubte Teilung.
7. Zwei Dinge zugleich ändern: Schleifen über Indizes; bei verschachtelten Listen die innere Liste herausnehmen und zurücklegen. Die Reihenfolge der Änderungen bleibt die von Go.
8. Löschen wie in Go an derselben Stelle: `remove(i)` für `append(a[:i], a[i+1:]...)`, `retain` für das Filtern am Platz, `swap_remove` nur, wo Go tauscht.
9. `NextID` zählt wie in Go; IDs werden an denselben Stellen vergeben.
10. Felder mit `json:"-"` und kleingeschriebene Felder stehen im Zustand, aber nicht im JSON: `#[serde(skip)]` und `Default`.
11. `rng *rng.Rng` wird `rng: Rng` in `World`. Jeder Würfel-Aufruf bleibt an seiner Stelle der Reihenfolge.
12. `Events` wird wie in Go je Schritt geleert.

Dazu gilt: kein `HashMap` (`BTreeMap`; `engine/internal/rules/map_range_test.go` verbietet Durchläufe über Maps auch im Go-Altstand),
kein `unsafe`, `Rc`, `RefCell`, `Cell`, `Mutex`.

## Beispiele je Fall

### Verweis (Entscheidung 2 und 3)

```go
// Go
type ResourceNode struct{ WorkerID *int `json:"workerId"` }
func troopByID(w *World, id int) *Troop { for _, t := range w.Troops { if t.ID == id { return t } }; return nil }
func isWorker(w *World, id *int) bool   { if id == nil { return false }; return troopByID(w, *id) != nil }
```

```rust
// Rust
pub struct ResourceNode { #[serde(rename = "workerId")] pub worker_id: Option<i64> }
impl World {
    pub fn troop(&self, id: i64) -> Option<&Troop> { self.troops.iter().find(|t| t.id == id) }
    pub fn troop_mut(&mut self, id: i64) -> Option<&mut Troop> { self.troops.iter_mut().find(|t| t.id == id) }
}
pub fn is_worker(w: &World, id: Option<i64>) -> bool { id.is_some_and(|id| w.troop(id).is_some()) }
```

Ein Go-Zeiger auf ein Ding der Welt als Parameter (`p *Player`) wird der Index in der Liste (`pi: usize`) oder die ID,
wenn die Funktion ohnehin in `w` sucht. Ein `Option<T>`-Teil (`Job`, `Crown`) liegt im Ding selbst: `t.job = None`.

### Reihenfolge (Entscheidung 1, 9)

```go
w.Troops = append(w.Troops, &Troop{ID: w.newID(), Kind: "vagrant"})
```

```rust
let id = w.new_id();
w.troops.push(Troop { id, kind: "vagrant".into(), ..Default::default() });
```

`Vec` behält die Entstehungs-Reihenfolge. `NextID` zählt in `new_id()` genau wie Go.

### Löschen (Entscheidung 8)

```go
w.Coins = append(w.Coins[:i], w.Coins[i+1:]...) // Reihenfolge bleibt
kept := w.Pickups[:0]; for _, p := range w.Pickups { if !taken(p) { kept = append(kept, p) } }; w.Pickups = kept
```

```rust
w.coins.remove(i);
w.pickups.retain(|p| !taken(p));
```

`swap_remove` nur, wo Go selbst das letzte Element nach vorn holt.

### Teilung des Wertesatzes (Entscheidung 6)

```go
vals *WorldValues // jede Stufe einer Welt zeigt auf denselben Satz
```

```rust
vals: Option<Arc<WorldValues>>, // Arc::clone(&w.vals) reicht den Satz weiter, nie ändern
```

Funktionen, die `w` ändern und zugleich Werte lesen, holen den Satz zuerst heraus: `let v = Arc::clone(&vals);`.

### Vorrat tauschen (Entscheidung 5)

```go
for _, st := range isl.Stages { st.Stock = isl.Stock } // alle Stufen teilen den Zeiger
```

```rust
// Die Insel besitzt `stock`. Vor dem Schritt einer Stufe hinein, danach zurück:
std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
step_stage(&mut isl.stages[i]);
std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
```

In `World` ist `stock` ein `Stock` (kein `Option`); der Zeiger von Go hat dort keine Bedeutung außer der Teilung.

### Zwei Dinge zugleich ändern (Entscheidung 7)

```go
for _, t := range w.Troops { for _, c := range w.Coins { if near(t, c) { c.BlockedUntil = w.Time } } }
```

```rust
for i in 0..w.troops.len() {
    let mut coins = std::mem::take(&mut w.coins); // innere Liste heraus, w bleibt frei lesbar
    for c in coins.iter_mut() { if near(&w.troops[i], c) { c.blocked_until = w.time; } }
    w.coins = coins;
}
```

Kein `clone()` ganzer Listen im Schritt.

### Rückverweis (Entscheidung 4)

```go
type Site struct{ hubW *World; hubLv int } // Zwischenspeicher mit Rückverweis auf die Welt
func siteHubLevel(w *World, s *Site) int   // …
```

```rust
pub struct Site { #[serde(skip)] pub hub_lv: i64 /* ohne hub_w */ }
pub fn site_hub_level(w: &World, s: &Site) -> i64 // die Welt kommt als Parameter
```

Ein Zwischenspeicher, der nur ein Rückverweis plus Wert war, fällt ohne `w` weg oder rechnet je Aufruf; das Ergebnis
bleibt gleich, weil der Wert nur eine Ableitung ist. Dasselbe gilt für `island`: `World` hat kein Feld dafür.

### JSON-Felder (Entscheidung 10, 12)

```go
Protected bool `json:"-"`   // nur Laufzeit
skillHeld int                // kleingeschrieben, ohne JSON
Events    []Event `json:"events"` // wird bei jedem Step geleert
```

```rust
#[serde(skip)] pub protected: bool,
#[serde(skip)] pub skill_held: i64,
pub events: Vec<Event>, // am Anfang des Schritts `w.events.clear()`
```

Lese-Verhalten: `#[serde(default)]` auf jedem Typ, damit fehlende Felder 0 sind wie in Go. `omitempty` wird zu
`skip_serializing_if` (Hilfen in `state/omit.rs`).

## Dateien

| Datei | Inhalt |
|---|---|
| `state/mod.rs` | Module, Wiedergabe der öffentlichen Typen |
| `state/omit.rs` | `skip_serializing_if`-Hilfen |
| `state/types*.rs` | Go-Altstand `engine/sim/types.go` (in Teile gegliedert, je ≤ 400 Zeilen) |
| `state/shared.rs` | Typen aus anderen Go-Dateien, die der Zustand braucht (Crown, Merchant, Island-Teile …); die Gruppe, die ihre Go-Datei portiert, ergänzt sie |
| `state/pending.rs` | Platzhalter für Funktionen späterer Sprints (Leitfaden § 7) |
