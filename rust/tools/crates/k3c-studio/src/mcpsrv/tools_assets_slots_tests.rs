//! Port von `tools_assets_slots_test.go`, MC1/AC-06: die Register-Varianten von `assets_list`, `assets_get` und
//! `assets_set` und `assets_catalog` sind entfernt. Dass die Recherche-Tools bleiben, prüft erst RU70.13, die sie bringt
//! (bis dahin stehen sie in `PENDING_TOOLS` der Golden).

use serde_json::json;

use super::super::INSTRUCTIONS;
use super::super::test_helpers::connect;
use crate::paperassets::test_helpers::schreibe;

#[tokio::test(flavor = "multi_thread")]
async fn alte_register_tools_entfernt() {
    // Mini-Repo mit Slot-Register (Go `assetRepo`, soweit die Liste der Tools es braucht).
    let dir = crate::test_helpers::tmp_dir();
    let register = json!({"_comment": "Test", "slots": [
        {"id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "Farm", "wahl": []},
        {"id": "sfx:coin", "art": "sound", "kategorie": "sfx", "beschreibung": "Münze", "bus": "sfx", "wahl": []}]});
    schreibe(dir.path(), "data/sounds.json", register.to_string());
    let tools = connect(dir.path()).await.tools().await;
    let mut geprueft = 0;
    for tool in &tools {
        let name = tool["name"].as_str().unwrap_or_default();
        assert_ne!(name, "assets_catalog", "assets_catalog ist noch registriert");
        if !["assets_list", "assets_get", "assets_set"].contains(&name) {
            continue;
        }
        geprueft += 1;
        let text = tool["description"].as_str().unwrap_or_default();
        assert!(
            !text.contains("data/sounds.json") && !text.contains("Slot"),
            "{name} beschreibt noch das Register"
        );
        for alt in ["art", "kategorie", "status", "wahl"] {
            assert!(
                tool["inputSchema"]["properties"].get(alt).is_none(),
                "{name} hat noch das Feld {alt}"
            );
        }
    }
    assert_eq!(geprueft, 3);
    assert!(
        !INSTRUCTIONS.contains("assets_catalog"),
        "instructions.md nennt assets_catalog"
    );
}
