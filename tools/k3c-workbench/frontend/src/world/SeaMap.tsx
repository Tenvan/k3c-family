import { Text } from '@webkit/atoms';
import type { WorldDetail } from '../api';
import { bandText, gradientOf, groupEtappen, islandAnchor, paletteOf, stageId, widthPct } from './crossModel';
import type { LevelModel } from './levelView';

// Seekarte des Welten-Editors (B-963): bleibt beim Scrollen stehen, zeigt die Etappen untereinander und die Inseln als
// Knoten mit Mini-Querschnitt; ein Klick springt zur Insel im Querschnitt.

interface Props {
  detail: WorldDetail;
  models: ReadonlyMap<string, LevelModel>;
  onJump: (islandId: string) => void;
}

export function SeaMap({ detail, models, onJump }: Props) {
  const groups = groupEtappen(detail);
  return (
    <aside className="wd-map" aria-label="Seekarte">
      <Text as="div" size="2" weight="bold">🗺️ Seekarte</Text>
      {groups.map((g, gi) => (
        <div key={g.etappe}>
          {gi > 0 && <Text as="div" size="1" color="gray" className="wd-map-sea">⛵ alle fertig → nächste Etappe</Text>}
          <div className="wd-map-etappe" data-etappe={g.etappe}>
            <Text as="div" size="1" weight="bold">{bandText(g, groups[gi - 1])}</Text>
            <div className="wd-map-row">
              {g.islands.map(({ island, index }) => (
                <a key={island.id} href={`#${islandAnchor(island.id)}`} className="wd-map-node" data-jump={island.id}
                  onClick={(e) => { e.preventDefault(); onJump(island.id); }}>
                  <span className="wd-map-sil">
                    {island.depths.map((_, s) => {
                      const m = models.get(stageId(island.id, s));
                      return <i key={s} style={{ width: `${m ? widthPct(m, models) : 50}%`, background: gradientOf(paletteOf(detail.biomeInfo, m?.biomeId ?? '')) }} />;
                    })}
                  </span>
                  <Text size="1" weight="bold">{index + 1} · {island.name}</Text>
                  <Text size="1" color="gray">{island.depths.length} Tiefen</Text>
                </a>
              ))}
            </div>
          </div>
        </div>
      ))}
      <Text as="div" size="1" color="gray" className="wd-map-sea">🏁 Weltende (Ziel des Raums)</Text>
    </aside>
  );
}
