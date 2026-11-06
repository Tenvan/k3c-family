import { Flex, Heading, Text } from '@webkit/atoms';
import { Tabs } from '@webkit/organisms';
import { ReviewPage } from './review';

/** Bereich Assets (Entscheidung 005 › 5): Review (RV1), Katalog, Styleguide, Sound, Quellen – nur Ansehen; Änderungen
 *  laufen über die MCP-Tools assets_*. Die Seiten stehen als Tabs.Content im Tabs.Root von AppFrame (App.tsx). */
export function AssetsArea() {
  return (
    <>
      <Tabs.Content value="review">
        <ReviewPage />
      </Tabs.Content>
      <Tabs.Content value="katalog">
        <Placeholder title="Katalog" />
      </Tabs.Content>
      <Tabs.Content value="styleguide">
        <Placeholder title="Styleguide" />
      </Tabs.Content>
      <Tabs.Content value="sound">
        <Placeholder title="Sound" />
      </Tabs.Content>
      <Tabs.Content value="quellen">
        <Placeholder title="Quellen" />
      </Tabs.Content>
    </>
  );
}

function Placeholder({ title }: { title: string }) {
  return (
    <Flex direction="column" gap="2" p="4">
      <Heading size="4">{title}</Heading>
      <Text color="gray">Folgt in WB1</Text>
    </Flex>
  );
}
