# tools/k3c-studio/internal/mcpsrv/

## Responsibility

MCP-Server `k3c-studio` (Streamable HTTP, nur 127.0.0.1, Port 5181, dev 5182): Tools für Welten, Hub und Assets sowie `level_generate`. Dienste, Tasks, Prüfläufe, Planung, Logs und Sim bleiben in `k3c-workbench`; die Tool-Namen beider Server sind disjunkt (`tests/mcpToolsDisjoint.test.ts`).

## Design

- `server.go` (`New`, `Start`, `Stop`, `Restart`, `URL`, `Stats`, `Instructions`, `ResolvePort`) und `kit.go` als dünne Anbindung an `tools/workkit/mcpkit`; `instructions.md` ist der MCP-Text des Servers (Kopf nennt Zuständigkeit und verweist auf `k3c-workbench`).
- `tools.go`: Registrierung; `checkout_policy.go`: Argument `checkout` aller Tools (Worktree-Auflösung).
- Welten: `tools_world.go` (`world_list`, `world_get`, `world_create`, `world_delete`, `world_set`, `world_override_set`, `world_chunks_set`, `world_check` über `internal/world`), `tools_world_story.go` (`world_story_set`).
- Hub: `tools_hub.go` (`hub_get`, `hub_check`, `hub_save` über `internal/hubplan`).
- Assets (über `internal/paperassets`): `tools_assets.go` (`assets_guide`, `assets_list`, `assets_get`, `assets_create`), `tools_assets_set.go` (`assets_set`, `assets_style_get`, `assets_style_set`, `assets_delete`), `tools_assets_render.go` (`assets_render` mit Jobs), `tools_assets_preview.go` (`assets_preview`), `tools_assets_review.go` (`assets_review`), `tools_assets_research.go` (`assets_research`, `assets_hits`, `assets_hit_add`, `assets_hit_drop`, `assets_import`), `tools_assets_prompts.go` (MCP-Prompts).
- `tools_engine.go`: `level_generate` (rechnet im Prozess über `engine/simcli`).

## Flow

1. Aufruf → `mcpkit` löst den Checkout auf → Tool-Handler → `internal/world`/`hubplan`/`paperassets` → Engine bzw. Render-CLI (`bun tools/paper/bin/paper.ts`).
2. Schreibende Tools prüfen vor dem format-stabilen Schreiben.

## Integration

- Konsument: `tools/k3c-studio` (`app.go`: `setMCP`, Stats für die Oberfläche).
- Abhängigkeiten: `tools/workkit/mcpkit`, `tools/assetkit`, interne Pakete des Studios, `engine/simcli`.
