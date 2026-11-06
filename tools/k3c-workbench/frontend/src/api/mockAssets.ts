import type { AssetEintrag, AssetsBackend, Frame, PixelDiff, Review, Spec, Stil, Vorlage } from './typesAssets';

// Mock der Asset-Methoden (WB1.1): die Beispielantworten aus tools/paper/fixtures/ui (PV1.4), geladen beim ersten
// Aufruf. Wie die Go-Stubs (internal/assetsui) kennt er nur das Asset monarch.
const FIXTURES = import.meta.glob<string>('../../../../paper/fixtures/ui/*.json', { query: '?raw', import: 'default' });

async function fixture<T>(name: string): Promise<T> {
  const load = Object.entries(FIXTURES).find(([path]) => path.endsWith(`/${name}.json`))?.[1];
  if (!load) throw new Error(`Fixture ${name}.json fehlt`);
  return JSON.parse(await load()) as T;
}

function known(id: string) {
  if (id !== 'monarch') throw new Error(`Asset „${id}“ fehlt`);
}

export function mockAssets(): AssetsBackend {
  return {
    assetsList: () => fixture<AssetEintrag[]>('AssetsList'),
    assetsGet: async (id) => {
      known(id);
      return fixture<Spec>('AssetsGet');
    },
    assetsStyle: () => fixture<Stil>('AssetsStyle'),
    assetsPrompts: () => fixture<Vorlage[]>('AssetsPrompts'),
    assetsReview: () => fixture<Review>('AssetsReview'),
    assetsFrame: async (id) => {
      known(id);
      return fixture<Frame>('AssetsFrame');
    },
    assetsPixelDiff: async (id) => {
      known(id);
      return fixture<PixelDiff>('AssetsPixelDiff');
    },
  };
}
