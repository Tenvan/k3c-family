import { Component, type ErrorInfo, type ReactNode } from 'react';
import { Button, Flex } from '../atoms';
import { NoticeCard } from '../molecules';

type Props = { name: string; children: ReactNode };
type State = { error: Error | null };

/** Fängt Render-Fehler einer Seite ab: Statt eines schwarzen Fensters zeigt die Seite den Fehler und lässt sich neu laden;
 * die anderen Seiten und die Kopfzeile bleiben bedienbar. Jede Seite einer Wails-App steht in einer eigenen Grenze. */
export class PageBoundary extends Component<Props, State> {
  state: State = { error: null };

  static getDerivedStateFromError(error: Error): State {
    return { error };
  }

  componentDidCatch(error: Error, info: ErrorInfo) {
    console.error(`💥 Seite ${this.props.name} abgestürzt:`, error, info.componentStack);
  }

  render() {
    const { error } = this.state;
    if (!error) return this.props.children;
    return (
      <Flex direction="column" gap="3" p="4" role="alert" data-page-error={this.props.name}>
        <NoticeCard title={`Die Seite „${this.props.name}“ ist abgestürzt`} tone="error">
          {error.message || String(error)}
        </NoticeCard>
        <Flex>
          <Button onClick={() => this.setState({ error: null })}>Seite neu laden</Button>
        </Flex>
      </Flex>
    );
  }
}
