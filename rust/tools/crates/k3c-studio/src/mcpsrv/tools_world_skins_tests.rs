//! Port von `tools_world_skins_test.go`. B-1266: `world_set` trägt `endBoss` und `skins` bis `world_get` durch.
//! Welt-Events folgen mit B-1260 (k3c-data kennt welt-events nicht).

use serde_json::json;

use super::tests::World;

#[tokio::test(flavor = "multi_thread")]
async fn world_tools_end_boss_skins_roundtrip() {
    let Some(mut w) = World::start().await else { return };
    let create = json!({"id": "hanse", "name": "Hanse", "extends": "standard"});
    w.ok("world_create", create, &[]).await;
    let patch = json!({"skins": {"gegner/wolf": "gegner/ninja"},
        "islands": [{"id": "island1", "endBoss": "forestChieftain"}]});
    w.ok("world_set", json!({"id": "hanse", "patch": patch}), &[]).await;
    let wants = ["\"endBoss\": \"forestChieftain\"", "\"gegner/ninja\""];
    w.ok("world_get", json!({"id": "hanse"}), &wants).await;
    w.stop().await;
}
