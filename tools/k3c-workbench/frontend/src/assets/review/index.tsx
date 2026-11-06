import { Flex, Heading, Text } from '@webkit/atoms';

/** Platzhalter der Review-Seite im Bereich Assets (PV1.8). WB1 hängt sie in die Navigation, RV1 füllt sie
 *  (Vertrag docs/standards/paper-mcp.md › Oberfläche). */
export function ReviewPage() {
  return (
    <Flex direction="column" gap="2" p="4">
      <Heading size="4">Asset-Review</Heading>
      <Text color="gray">Review folgt mit RV1</Text>
    </Flex>
  );
}
