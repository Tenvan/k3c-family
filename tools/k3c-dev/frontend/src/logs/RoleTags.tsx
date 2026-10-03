import { Badge, Flex } from '@radix-ui/themes';

/** Feste Farbe je Einordnung, damit Spiel/Tool und Backend/Client auf einen Blick auseinanderfallen. */
const COLORS = { Spiel: 'amber', Tool: 'violet', Backend: 'blue', Client: 'green' } as const;

export function RoleTags({ tags }: { tags?: string[] }) {
  if (!tags?.length) return null;
  return (
    <Flex gap="1" wrap="wrap">
      {tags.map((t) => (
        <Badge key={t} size="1" variant="soft" color={COLORS[t as keyof typeof COLORS] ?? 'gray'}>{t}</Badge>
      ))}
    </Flex>
  );
}
