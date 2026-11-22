import { devEvents, devRpc } from '@webkit/lib/devRpc';
import type { Backend, Events } from './types';

// Client-Server-Betrieb (k3c-workbench -headless/-dev, Go: uiapi.go über tools/workkit/uikit): Transport in
// tools/webkit/lib/devRpc.ts.
const rpc = devRpc('k3c-workbench');

export function httpBackend(): Backend {
  return {
    mock: false,
    info: () => rpc('Info'),
    services: () => rpc('Services'),
    servicesReload: () => rpc('ServicesReload'),
    serviceStart: (name) => rpc('ServiceStart', name),
    serviceStop: (name, force) => rpc('ServiceStop', name, force),
    serviceRestart: (name) => rpc('ServiceRestart', name),
    servicesStartAll: () => rpc('ServicesStartAll'),
    servicesStopAll: () => rpc('ServicesStopAll'),
    serviceLogLevels: (name) => rpc('ServiceLogLevels', name),
    sources: () => rpc('Sources'),
    consoleTail: (source) => rpc('ConsoleTail', source),
    logsQuery: (source, q) => rpc('LogsQuery', source, q),
    logsErrors: (source, level) => rpc('LogsErrors', source, level),
    mcpOverview: () => rpc('McpOverview'),
    mcpRestart: () => rpc('McpRestart'),
    mcpInstructions: () => rpc('McpInstructions'),
    mcpCalls: () => rpc('McpCalls'),
    mcpUsage: () => rpc('McpUsage'),
    resources: (windowMin) => rpc('Resources', windowMin),
    tasks: () => rpc('Tasks'),
    tasksReload: () => rpc('TasksReload'),
    taskStart: (name, args) => rpc('TaskStart', name, args),
    taskStop: (name) => rpc('TaskStop', name),
    taskRuns: () => rpc('TaskRuns'),
    planningData: () => rpc('PlanningData'),
    planningList: () => rpc('PlanningList'),
    planningItems: (ids) => rpc('PlanningItems', ids),
    planningDocs: () => rpc('PlanningDocs'),
    planningSet: (id, field, value) => rpc('PlanningSet', id, field, value),
    planningDoc: (name) => rpc('PlanningDoc', name),
    planningGet: (id) => rpc('PlanningGet', id),
    githubStatus: (force) => rpc('GitHubStatus', force),
    worktreeCheck: () => rpc('WorktreeCheck'),
    gitOverview: () => rpc('GitOverview'),
    gitAction: (action) => rpc('GitAction', action),
    gitDeleteBranches: (names, remote) => rpc('GitDeleteBranches', names, remote),
    gitPRs: () => rpc('GitPRs'),
    gitPRCreate: (title, body) => rpc('GitPRCreate', title, body),
    gitPRAction: (number, action) => rpc('GitPRAction', number, action),
    gitCheckMerged: (names) => rpc('GitCheckMerged', names),
    gitStamp: () => rpc('GitStamp'),
    ticketCreate: (input) => rpc('TicketCreate', input),
    ticketRecent: (n) => rpc('TicketRecent', n),
    openUrl: (url) => {
      window.open(url, '_blank', 'noopener');
    },
    on: devEvents<Events>(),
  };
}
