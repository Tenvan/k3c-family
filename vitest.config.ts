import { configDefaults, defineConfig } from 'vitest/config';

// Vitest der Repo-Wurzel: Regel- und Planungstests (tests/) und der Papierschnitt-Renderer (tools/paper). Der Browser-Client
// und sein Vite-Build sind entfernt (B-848).
// .claude/worktrees enthält komplette Checkouts anderer Branches; deren Tests gehören nicht zu diesem Lauf.
// Das Frontend unter tools/k3c-workbench/frontend prüft sein eigenes Vitest (Alias @webkit, task check:dev); die Tests von
// tools/webkit laufen im Vitest von k3c-workbench.
export default defineConfig({
  test: { exclude: [...configDefaults.exclude, '.claude/**', 'tools/*/frontend/**', 'tools/webkit/**'] },
});
