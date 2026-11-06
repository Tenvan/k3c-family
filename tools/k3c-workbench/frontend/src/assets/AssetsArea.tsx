import { Tabs } from '@webkit/organisms';
import { CatalogPage } from './catalog/CatalogPage';
import { ReviewPage } from './review';
import { SoundPage } from './sound/SoundPage';
import { SourcesPage } from './sources/SourcesPage';
import { StylePage } from './style/StylePage';

/** Bereich Assets (Entscheidung 005 › 5): Review (RV1), Katalog, Styleguide, Sound, Quellen – nur Ansehen; Änderungen
 *  laufen über die MCP-Tools assets_*. Die Seiten stehen als Tabs.Content im Tabs.Root von AppFrame (App.tsx). */
export function AssetsArea() {
  return (
    <>
      <Tabs.Content value="review">
        <ReviewPage />
      </Tabs.Content>
      <Tabs.Content value="katalog">
        <CatalogPage />
      </Tabs.Content>
      <Tabs.Content value="styleguide">
        <StylePage />
      </Tabs.Content>
      <Tabs.Content value="sound">
        <SoundPage />
      </Tabs.Content>
      <Tabs.Content value="quellen">
        <SourcesPage />
      </Tabs.Content>
    </>
  );
}

