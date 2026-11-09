import { clientLog } from '../core/clientLog';
import { readGameRenderInfo } from '../core/gameRenderInfo';
import data from '../../data/stress.json';
import { KNOWN } from './known';
import { RealStress, realStressId, type RealHost } from './realStress';
import { parseScenarios, type Scenario } from './stressPlan';
import { runSearch } from '../core/runConfig';

/** Startet den Lauf für `?stress=<szenario>&dev=1` in der `GameScene`; sonst nichts. Gibt den Lauf zurück. */
export function startRealStress(host: RealHost, search: string = runSearch()): RealStress | null {
  const id = realStressId(search);
  if (!id) return null;
  let scenario: Scenario | undefined;
  try {
    scenario = parseScenarios(data as Record<string, unknown>, KNOWN)[id];
  } catch (e) {
    clientLog('error', `🚫 Szenario-Datei ungültig: ${e instanceof Error ? e.message : String(e)}`);
    return null;
  }
  if (!scenario) {
    clientLog('error', `🚫 Stress-Szenario unbekannt: ${id}`);
    return null;
  }
  const stress = new RealStress(host, scenario, id, search, () => readGameRenderInfo(host.game));
  const tick = (): void => stress.update();
  host.events.on('update', tick);
  host.events.once('shutdown', () => {
    host.events.off('update', tick);
    stress.abort();
  });
  return stress;
}
