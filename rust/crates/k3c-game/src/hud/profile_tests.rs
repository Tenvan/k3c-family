//! Port von `native/k3c-game/hud/profile_test.go`.

/// B-894/AC-05: Ein Wechsel des Eingabe-Profils in den Einstellungen ändert die Hinweise sofort: Nach dem nächsten
/// Listen zeigt das offene Skill-Menü des Tastatur-Spielers den Bedienhinweis des neuen Profils, der Pad-Spieler
/// bleibt beim Xbox-Hinweis; der Beitritts-Hinweis nennt die Bestätigen-Taste des Profils. Braucht `seatState` (RU20.4).
#[test]
#[ignore = "wartet auf RU20"]
fn profile_switch_changes_hints() {
    unimplemented!("RU20.4: hud.Update/Listen, seats[i].menuV.footer, joinS (Go: TestProfileSwitchChangesHints)");
}
