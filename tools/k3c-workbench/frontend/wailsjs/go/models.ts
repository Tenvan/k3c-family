export namespace console {
	
	export class Line {
	    source: string;
	    stream: string;
	    text: string;
	    seq: number;
	
	    static createFrom(source: any = {}) {
	        return new Line(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.source = source["source"];
	        this.stream = source["stream"];
	        this.text = source["text"];
	        this.seq = source["seq"];
	    }
	}

}

export namespace github {
	
	export class Run {
	    ci: string;
	    title: string;
	    url: string;
	    created: string;
	
	    static createFrom(source: any = {}) {
	        return new Run(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.ci = source["ci"];
	        this.title = source["title"];
	        this.url = source["url"];
	        this.created = source["created"];
	    }
	}
	export class SprintPR {
	    number: number;
	    title: string;
	    url: string;
	    state: string;
	    ci: string;
	    merge: string;
	
	    static createFrom(source: any = {}) {
	        return new SprintPR(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.number = source["number"];
	        this.title = source["title"];
	        this.url = source["url"];
	        this.state = source["state"];
	        this.ci = source["ci"];
	        this.merge = source["merge"];
	    }
	}
	export class Data {
	    sprints: Record<string, SprintPR>;
	    develop?: Run;
	    error?: string;
	    fetched?: string;
	
	    static createFrom(source: any = {}) {
	        return new Data(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.sprints = this.convertValues(source["sprints"], SprintPR, true);
	        this.develop = this.convertValues(source["develop"], Run);
	        this.error = source["error"];
	        this.fetched = source["fetched"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	

}

export namespace logs {
	
	export class Entry {
	    // Go type: time
	    time: any;
	    level: string;
	    ns: string;
	    checkout: string;
	    msg: string;
	    data?: Record<string, any>;
	
	    static createFrom(source: any = {}) {
	        return new Entry(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.time = this.convertValues(source["time"], null);
	        this.level = source["level"];
	        this.ns = source["ns"];
	        this.checkout = source["checkout"];
	        this.msg = source["msg"];
	        this.data = source["data"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class Group {
	    ns: string;
	    fingerprint: string;
	    level: string;
	    count: number;
	    // Go type: time
	    first: any;
	    // Go type: time
	    last: any;
	    example: string;
	
	    static createFrom(source: any = {}) {
	        return new Group(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.ns = source["ns"];
	        this.fingerprint = source["fingerprint"];
	        this.level = source["level"];
	        this.count = source["count"];
	        this.first = this.convertValues(source["first"], null);
	        this.last = this.convertValues(source["last"], null);
	        this.example = source["example"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}

}

export namespace main {
	
	export class ErrorsView {
	    groups: logs.Group[];
	    entries: number;
	    bytesRead: number;
	    budgetHit: boolean;
	    missing: boolean;
	
	    static createFrom(source: any = {}) {
	        return new ErrorsView(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.groups = this.convertValues(source["groups"], logs.Group);
	        this.entries = source["entries"];
	        this.bytesRead = source["bytesRead"];
	        this.budgetHit = source["budgetHit"];
	        this.missing = source["missing"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class MCPState {
	    addr: string;
	    listening: boolean;
	    error: string;
	
	    static createFrom(source: any = {}) {
	        return new MCPState(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.addr = source["addr"];
	        this.listening = source["listening"];
	        this.error = source["error"];
	    }
	}
	export class Info {
	    version: string;
	    mcp: MCPState;
	
	    static createFrom(source: any = {}) {
	        return new Info(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.version = source["version"];
	        this.mcp = this.convertValues(source["mcp"], MCPState);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class LogQuery {
	    minLevel: string;
	    ns: string;
	    pattern: string;
	    limit: number;
	
	    static createFrom(source: any = {}) {
	        return new LogQuery(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.minLevel = source["minLevel"];
	        this.ns = source["ns"];
	        this.pattern = source["pattern"];
	        this.limit = source["limit"];
	    }
	}
	export class LogView {
	    entries: logs.Entry[];
	    bytesRead: number;
	    budgetHit: boolean;
	    skipped: number;
	    missing: boolean;
	
	    static createFrom(source: any = {}) {
	        return new LogView(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.entries = this.convertValues(source["entries"], logs.Entry);
	        this.bytesRead = source["bytesRead"];
	        this.budgetHit = source["budgetHit"];
	        this.skipped = source["skipped"];
	        this.missing = source["missing"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	
	export class McpOverview {
	    mcp: MCPState;
	    stats: mcpsrv.Snapshot;
	
	    static createFrom(source: any = {}) {
	        return new McpOverview(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.mcp = this.convertValues(source["mcp"], MCPState);
	        this.stats = this.convertValues(source["stats"], mcpsrv.Snapshot);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class UsageRules {
	    outlierFactor: number;
	    outlierFloorMs: number;
	    baselineCalls: number;
	    percentileErrorPct: number;
	
	    static createFrom(source: any = {}) {
	        return new UsageRules(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.outlierFactor = source["outlierFactor"];
	        this.outlierFloorMs = source["outlierFloorMs"];
	        this.baselineCalls = source["baselineCalls"];
	        this.percentileErrorPct = source["percentileErrorPct"];
	    }
	}
	export class McpUsageView {
	    // Go type: usage
	    session: any;
	    // Go type: usage
	    allTime: any;
	    minutes: usage.Bucket[];
	    rules: UsageRules;
	
	    static createFrom(source: any = {}) {
	        return new McpUsageView(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.session = this.convertValues(source["session"], null);
	        this.allTime = this.convertValues(source["allTime"], null);
	        this.minutes = this.convertValues(source["minutes"], usage.Bucket);
	        this.rules = this.convertValues(source["rules"], UsageRules);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class ResGauge {
	    name: string;
	    bytes: boolean;
	
	    static createFrom(source: any = {}) {
	        return new ResGauge(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.bytes = source["bytes"];
	    }
	}
	export class ResOp {
	    name: string;
	    calls: number;
	    totalMs: number;
	    maxMs: number;
	    alloc: number;
	    maxAlloc: number;
	
	    static createFrom(source: any = {}) {
	        return new ResOp(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.calls = source["calls"];
	        this.totalMs = source["totalMs"];
	        this.maxMs = source["maxMs"];
	        this.alloc = source["alloc"];
	        this.maxAlloc = source["maxAlloc"];
	    }
	}
	export class ResPoint {
	    t: number;
	    heapLive: number;
	    total: number;
	    private: number;
	    rss: number;
	    childrenRss: number;
	    children: number;
	    goroutines: number;
	    handles: number;
	    gaugeValues: number[];
	    gcCycles: number;
	
	    static createFrom(source: any = {}) {
	        return new ResPoint(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.t = source["t"];
	        this.heapLive = source["heapLive"];
	        this.total = source["total"];
	        this.private = source["private"];
	        this.rss = source["rss"];
	        this.childrenRss = source["childrenRss"];
	        this.children = source["children"];
	        this.goroutines = source["goroutines"];
	        this.handles = source["handles"];
	        this.gaugeValues = source["gaugeValues"];
	        this.gcCycles = source["gcCycles"];
	    }
	}
	export class ResProc {
	    name: string;
	    pid: number;
	    rss: number;
	
	    static createFrom(source: any = {}) {
	        return new ResProc(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.pid = source["pid"];
	        this.rss = source["rss"];
	    }
	}
	export class ResTrend {
	    name: string;
	    bytes: boolean;
	    perHour: number;
	    windowMin: number;
	
	    static createFrom(source: any = {}) {
	        return new ResTrend(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.bytes = source["bytes"];
	        this.perHour = source["perHour"];
	        this.windowMin = source["windowMin"];
	    }
	}
	export class ResourceView {
	    enabled: boolean;
	    started: number;
	    intervalS: number;
	    windowMin: number;
	    points: ResPoint[];
	    gauges: ResGauge[];
	    ops: ResOp[];
	    trends: ResTrend[];
	    children: ResProc[];
	    profiles: string[];
	
	    static createFrom(source: any = {}) {
	        return new ResourceView(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.enabled = source["enabled"];
	        this.started = source["started"];
	        this.intervalS = source["intervalS"];
	        this.windowMin = source["windowMin"];
	        this.points = this.convertValues(source["points"], ResPoint);
	        this.gauges = this.convertValues(source["gauges"], ResGauge);
	        this.ops = this.convertValues(source["ops"], ResOp);
	        this.trends = this.convertValues(source["trends"], ResTrend);
	        this.children = this.convertValues(source["children"], ResProc);
	        this.profiles = source["profiles"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class ServicesView {
	    services: services.Status[];
	    error: string;
	
	    static createFrom(source: any = {}) {
	        return new ServicesView(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.services = this.convertValues(source["services"], services.Status);
	        this.error = source["error"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class Source {
	    name: string;
	    kind: string;
	    state: string;
	    detail: string;
	
	    static createFrom(source: any = {}) {
	        return new Source(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.kind = source["kind"];
	        this.state = source["state"];
	        this.detail = source["detail"];
	    }
	}
	export class TaskCatalog {
	    namespaces: taskcat.Namespace[];
	    count: number;
	    loadedAt: string;
	    error: string;
	
	    static createFrom(source: any = {}) {
	        return new TaskCatalog(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.namespaces = this.convertValues(source["namespaces"], taskcat.Namespace);
	        this.count = source["count"];
	        this.loadedAt = source["loadedAt"];
	        this.error = source["error"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class TaskGates {
	    states: Record<string, string>;
	    pending: number;
	
	    static createFrom(source: any = {}) {
	        return new TaskGates(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.states = source["states"];
	        this.pending = source["pending"];
	    }
	}

}

export namespace mcpsrv {
	
	export class Call {
	    id: number;
	    ref: string;
	    startedAt: string;
	    ts: string;
	    startSeq: number;
	    endSeq: number;
	    running: boolean;
	    tool: string;
	    args: string;
	    durationMs: number;
	    ok: boolean;
	    error?: string;
	    summary: string;
	
	    static createFrom(source: any = {}) {
	        return new Call(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.id = source["id"];
	        this.ref = source["ref"];
	        this.startedAt = source["startedAt"];
	        this.ts = source["ts"];
	        this.startSeq = source["startSeq"];
	        this.endSeq = source["endSeq"];
	        this.running = source["running"];
	        this.tool = source["tool"];
	        this.args = source["args"];
	        this.durationMs = source["durationMs"];
	        this.ok = source["ok"];
	        this.error = source["error"];
	        this.summary = source["summary"];
	    }
	}
	export class ToolStats {
	    name: string;
	    description: string;
	    calls: number;
	    errors: number;
	    avgMs: number;
	    lastCall: string;
	    lastError: string;
	
	    static createFrom(source: any = {}) {
	        return new ToolStats(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.description = source["description"];
	        this.calls = source["calls"];
	        this.errors = source["errors"];
	        this.avgMs = source["avgMs"];
	        this.lastCall = source["lastCall"];
	        this.lastError = source["lastError"];
	    }
	}
	export class Snapshot {
	    startedAt: string;
	    clients: number;
	    peakClients: number;
	    inFlight: number;
	    peakInFlight: number;
	    totalCalls: number;
	    errors: number;
	    tools: ToolStats[];
	
	    static createFrom(source: any = {}) {
	        return new Snapshot(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.startedAt = source["startedAt"];
	        this.clients = source["clients"];
	        this.peakClients = source["peakClients"];
	        this.inFlight = source["inFlight"];
	        this.peakInFlight = source["peakInFlight"];
	        this.totalCalls = source["totalCalls"];
	        this.errors = source["errors"];
	        this.tools = this.convertValues(source["tools"], ToolStats);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}

}

export namespace planning {
	
	export class Change {
	    kind: string;
	    item: string;
	    id: string;
	    text: string;
	    project?: string;
	    sprint?: string;
	
	    static createFrom(source: any = {}) {
	        return new Change(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.kind = source["kind"];
	        this.item = source["item"];
	        this.id = source["id"];
	        this.text = source["text"];
	        this.project = source["project"];
	        this.sprint = source["sprint"];
	    }
	}
	export class Ticket {
	    nr: string;
	    title: string;
	    domain: string;
	    typ: string;
	    prio: string;
	    env: string;
	    status: string;
	    sprint: string;
	    project: string;
	    spec: string;
	
	    static createFrom(source: any = {}) {
	        return new Ticket(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.nr = source["nr"];
	        this.title = source["title"];
	        this.domain = source["domain"];
	        this.typ = source["typ"];
	        this.prio = source["prio"];
	        this.env = source["env"];
	        this.status = source["status"];
	        this.sprint = source["sprint"];
	        this.project = source["project"];
	        this.spec = source["spec"];
	    }
	}
	export class Session {
	    nr: string;
	    typ: string;
	    agent: string;
	    status: string;
	    titel: string;
	    text?: string;
	    deps?: string[];
	    env?: string;
	    domain?: string;
	
	    static createFrom(source: any = {}) {
	        return new Session(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.nr = source["nr"];
	        this.typ = source["typ"];
	        this.agent = source["agent"];
	        this.status = source["status"];
	        this.titel = source["titel"];
	        this.text = source["text"];
	        this.deps = source["deps"];
	        this.env = source["env"];
	        this.domain = source["domain"];
	    }
	}
	export class Sprint {
	    id: string;
	    title: string;
	    domain: string;
	    project: string;
	    deps?: string[];
	    status: string;
	    reife: string;
	    spec: string;
	    tickets: string[];
	    sessions: Session[];
	    worktree?: string;
	    worktrees?: string[];
	
	    static createFrom(source: any = {}) {
	        return new Sprint(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.id = source["id"];
	        this.title = source["title"];
	        this.domain = source["domain"];
	        this.project = source["project"];
	        this.deps = source["deps"];
	        this.status = source["status"];
	        this.reife = source["reife"];
	        this.spec = source["spec"];
	        this.tickets = source["tickets"];
	        this.sessions = this.convertValues(source["sessions"], Session);
	        this.worktree = source["worktree"];
	        this.worktrees = source["worktrees"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	export class Project {
	    id: string;
	    title: string;
	    status: string;
	    rang: string;
	    sprints: string[];
	    file: string;
	
	    static createFrom(source: any = {}) {
	        return new Project(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.id = source["id"];
	        this.title = source["title"];
	        this.status = source["status"];
	        this.rang = source["rang"];
	        this.sprints = source["sprints"];
	        this.file = source["file"];
	    }
	}
	export class Data {
	    projects: Project[];
	    sprints: Sprint[];
	    tickets: Ticket[];
	    done: number;
	
	    static createFrom(source: any = {}) {
	        return new Data(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.projects = this.convertValues(source["projects"], Project);
	        this.sprints = this.convertValues(source["sprints"], Sprint);
	        this.tickets = this.convertValues(source["tickets"], Ticket);
	        this.done = source["done"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}
	
	
	
	
	export class WorktreeDiff {
	    branch: string;
	    path: string;
	    changes: Change[];
	    error?: string;
	
	    static createFrom(source: any = {}) {
	        return new WorktreeDiff(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.branch = source["branch"];
	        this.path = source["path"];
	        this.changes = this.convertValues(source["changes"], Change);
	        this.error = source["error"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}

}

export namespace services {
	
	export class LevelCounts {
	    counts: Record<string, number>;
	    total: number;
	    budgetHit: boolean;
	
	    static createFrom(source: any = {}) {
	        return new LevelCounts(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.counts = source["counts"];
	        this.total = source["total"];
	        this.budgetHit = source["budgetHit"];
	    }
	}
	export class Status {
	    name: string;
	    description: string;
	    tags?: string[];
	    port: number;
	    health: string;
	    log: string;
	    state: string;
	    pid: number;
	    // Go type: time
	    startedAt: any;
	    restarts: number;
	    lastError: string;
	    changeFile: string;
	    // Go type: time
	    changeAt: any;
	    cpu: number;
	    memory: number;
	    seq: number;
	
	    static createFrom(source: any = {}) {
	        return new Status(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.description = source["description"];
	        this.tags = source["tags"];
	        this.port = source["port"];
	        this.health = source["health"];
	        this.log = source["log"];
	        this.state = source["state"];
	        this.pid = source["pid"];
	        this.startedAt = this.convertValues(source["startedAt"], null);
	        this.restarts = source["restarts"];
	        this.lastError = source["lastError"];
	        this.changeFile = source["changeFile"];
	        this.changeAt = this.convertValues(source["changeAt"], null);
	        this.cpu = source["cpu"];
	        this.memory = source["memory"];
	        this.seq = source["seq"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}

}

export namespace taskcat {
	
	export class Task {
	    name: string;
	    namespace: string;
	    leaf: string;
	    desc: string;
	    summary: string;
	    aliases: string[];
	    file: string;
	    line: number;
	
	    static createFrom(source: any = {}) {
	        return new Task(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.namespace = source["namespace"];
	        this.leaf = source["leaf"];
	        this.desc = source["desc"];
	        this.summary = source["summary"];
	        this.aliases = source["aliases"];
	        this.file = source["file"];
	        this.line = source["line"];
	    }
	}
	export class Namespace {
	    name: string;
	    tasks: Task[];
	
	    static createFrom(source: any = {}) {
	        return new Namespace(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.tasks = this.convertValues(source["tasks"], Task);
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}

}

export namespace taskrun {
	
	export class Run {
	    name: string;
	    state: string;
	    pid: number;
	    args: string[];
	    // Go type: time
	    startedAt: any;
	    // Go type: time
	    endedAt: any;
	    exitCode: number;
	    durationMs: number;
	    reason: string;
	
	    static createFrom(source: any = {}) {
	        return new Run(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.name = source["name"];
	        this.state = source["state"];
	        this.pid = source["pid"];
	        this.args = source["args"];
	        this.startedAt = this.convertValues(source["startedAt"], null);
	        this.endedAt = this.convertValues(source["endedAt"], null);
	        this.exitCode = source["exitCode"];
	        this.durationMs = source["durationMs"];
	        this.reason = source["reason"];
	    }
	
		convertValues(a: any, classs: any, asMap: boolean = false): any {
		    if (!a) {
		        return a;
		    }
		    if (a.slice && a.map) {
		        return (a as any[]).map(elem => this.convertValues(elem, classs));
		    } else if ("object" === typeof a) {
		        if (asMap) {
		            for (const key of Object.keys(a)) {
		                a[key] = new classs(a[key]);
		            }
		            return a;
		        }
		        return new classs(a);
		    }
		    return a;
		}
	}

}

