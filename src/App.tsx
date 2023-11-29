import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useMemo, useState } from "react";

import { Button } from "./components/ui/button";

function prettyBytes(num: number) {
  const neg = num < 0;
  if (neg) num = -num;
  if (num < 1) return (neg ? "-" : "") + num + " B";
  const units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  const exponent = Math.min(
    Math.floor(Math.log(num) / Math.log(1000)),
    units.length - 1
  );
  num = +(num / Math.pow(1000, exponent)).toFixed(2);
  const unit = units[exponent];
  return (neg ? "-" : "") + num + " " + unit;
}

function prettyNumber(num: number) {
  return num.toLocaleString();
}

// Not a string, number, boolean, null, undefined, or array
function isOnlyObject(value: any): value is Record<string, any> {
  return (
    typeof value === "object" &&
    !Array.isArray(value) &&
    value !== null &&
    !(value instanceof Date)
  );
}

type NetStatValue = {
  received: number;
  sent: number;
};

type NetStat = {
  interface: {
    bytes: NetStatValue;
    unicast_packets: NetStatValue;
    non_unicast_packets: NetStatValue;
    discards: NetStatValue;
    errors: NetStatValue;
    unknown_protocols: number;
  };
  ipv4: {
    packets_received: number;
    received_header_errors: number;
    received_address_errors: number;
    datagrams_forwarded: number;
    unknown_protocols_received: number;
    received_packets_discarded: number;
    received_packets_delivered: number;
    output_requests: number;
    routing_discards: number;
    discarded_output_packets: number;
    output_packet_no_route: number;
    reassembly_required: number;
    reassembly_successful: number;
    reassembly_failures: number;
    datagrams_successfully_fragmented: number;
    datagrams_failing_fragmentation: number;
    fragments_created: number;
  };
  ipv6: {
    packets_received: number;
    received_header_errors: number;
    received_address_errors: number;
    datagrams_forwarded: number;
    unknown_protocols_received: number;
    received_packets_discarded: number;
    received_packets_delivered: number;
    output_requests: number;
    routing_discards: number;
    discarded_output_packets: number;
    output_packet_no_route: number;
    reassembly_required: number;
    reassembly_successful: number;
    reassembly_failures: number;
    datagrams_successfully_fragmented: number;
    datagrams_failing_fragmentation: number;
    fragments_created: number;
  };
  icmpv4: {
    messages: NetStatValue;
    errors: NetStatValue;
    destination_unreachable: NetStatValue;
    time_exceeded: NetStatValue;
    parameter_problems: NetStatValue;
    source_quenches: NetStatValue;
    redirects: NetStatValue;
    echo_replies: NetStatValue;
    echos: NetStatValue;
    timestamps: NetStatValue;
    timestamp_replies: NetStatValue;
    address_masks: NetStatValue;
    address_mask_replies: NetStatValue;
    router_solicitations: NetStatValue;
    router_advertisements: NetStatValue;
  };
  icmpv6: {
    messages: NetStatValue;
    errors: NetStatValue;
    destination_unreachable: NetStatValue;
    packet_too_big: NetStatValue;
    time_exceeded: NetStatValue;
    parameter_problems: NetStatValue;
    echos: NetStatValue;
    echo_replies: NetStatValue;
    mld_queries: NetStatValue;
    mld_reports: NetStatValue;
    mld_dones: NetStatValue;
    router_solicitations: NetStatValue;
    router_advertisements: NetStatValue;
    neighbor_solicitations: NetStatValue;
    neighbor_advertisements: NetStatValue;
    redirects: NetStatValue;
    router_renumberings: NetStatValue;
  };
  tcp_ipv4: {
    active_opens: number;
    passive_opens: number;
    failed_connection_attempts: number;
    reset_connections: number;
    current_connections: number;
    segments_received: number;
    segments_sent: number;
    segments_retransmitted: number;
  };
  tcp_ipv6: {
    active_opens: number;
    passive_opens: number;
    failed_connection_attempts: number;
    reset_connections: number;
    current_connections: number;
    segments_received: number;
    segments_sent: number;
    segments_retransmitted: number;
  };
  udp_ipv4: {
    datagrams_received: number;
    no_ports: number;
    receive_errors: number;
    datagrams_sent: number;
  };
  udp_ipv6: {
    datagrams_received: number;
    no_ports: number;
    receive_errors: number;
    datagrams_sent: number;
  };
};

type NetStatKeys = keyof NetStat;
type NetStatChildKeys = keyof NetStat[NetStatKeys];

function App() {
  const [netStatResult, setNetStatResult] = useState<NetStat | null>(null);
  const [netConnectionsResult, setNetConnectionsResult] = useState<any>({});
  const [processesResult, setProcessesResult] = useState<
    {
      image_name: string;
      pid: number;
      session_name: string;
    }[]
  >([]);
  const [enabled, setEnabled] = useState(false);
  const [selectedProcess, setSelectedProcess] = useState("");

  const processesWithConnections = useMemo(() => {
    return processesResult
      .map((process) => {
        const connections = netConnectionsResult[process.pid];
        return {
          ...process,
          connections,
        };
      })
      .filter((process) => !!process.connections);
  }, [processesResult, netConnectionsResult]);

  async function net_stat() {
    const response: string = await invoke("net_stat");

    try {
      setNetStatResult(JSON.parse(response));
    } catch {
      setNetStatResult(null);
    }
  }

  async function net_connections() {
    const response: string = await invoke("net_connections");

    try {
      setNetConnectionsResult(JSON.parse(response));
    } catch {
      setNetConnectionsResult({});
    }
  }

  async function processes() {
    const response: string = await invoke("processes");

    try {
      const processes = JSON.parse(response);
      setProcessesResult(processes);
    } catch {
      setProcessesResult([]);
    }
  }

  useEffect(() => {
    let interval: NodeJS.Timeout | null = null;
    if (enabled) {
      interval = setInterval(() => {
        net_stat();
      }, 250);
    }

    return () => {
      interval && clearInterval(interval);
    };
  }, [enabled]);

  useEffect(() => {
    let interval: NodeJS.Timeout | null = null;
    if (enabled) {
      interval = setInterval(() => {
        net_connections();
      }, 2500);
    }

    return () => {
      interval && clearInterval(interval);
    };
  }, [enabled]);

  useEffect(() => {
    let interval: NodeJS.Timeout | null = null;
    if (enabled) {
      interval = setInterval(() => {
        processes();
      }, 2500);
    }

    return () => {
      interval && clearInterval(interval);
    };
  }, [enabled]);

  // Initial load
  useEffect(() => {
    net_stat();
    net_connections();
    processes();
  }, []);

  return (
    <div className="container">
      <Button type="button" onClick={() => setEnabled((prev) => !prev)}>
        run net stat ({enabled ? "on" : "off"})
      </Button>
      {netStatResult ? (
        <div className="flex gap-8 flex-wrap">
          {(Object.keys(netStatResult) as NetStatKeys[]).map((key) => (
            <div key={key}>
              <div>{key}</div>
              {(Object.keys(netStatResult[key]) as NetStatChildKeys[]).map(
                (key2) => (
                  <p key={key2}>
                    {key2}:{" "}
                    {isOnlyObject(netStatResult[key][key2]) &&
                    "received" in netStatResult[key][key2] &&
                    "sent" in netStatResult[key][key2] ? (
                      <span className="tabular-nums">
                        <span
                          title={`Received: ${(+(
                            // @ts-expect-error - I know what I'm doing
                            netStatResult[key][key2].received
                          )).toLocaleString()}`}
                        >
                          {/* @ts-expect-error - I know what I'm doing */}
                          {prettyBytes(netStatResult[key][key2].received)}
                        </span>
                        {" / "}
                        <span
                          title={`Sent: ${(+(
                            // @ts-expect-error - I know what I'm doing
                            netStatResult[key][key2].sent
                          )).toLocaleString()}`}
                        >
                          {/* @ts-expect-error - I know what I'm doing */}
                          {prettyBytes(netStatResult[key][key2].sent)}
                        </span>
                      </span>
                    ) : (
                      <span className="tabular-nums">
                        {prettyNumber(netStatResult[key][key2])}
                      </span>
                    )}
                  </p>
                )
              )}
            </div>
          ))}
        </div>
      ) : null}

      <select
        value={selectedProcess}
        onChange={(e) => setSelectedProcess(e.target.value)}
      >
        {processesWithConnections.map((process) => (
          <option key={process.pid} value={process.pid}>
            {process.image_name} - {process.pid} - {process.session_name}
          </option>
        ))}
      </select>

      <pre>
        {JSON.stringify(netConnectionsResult[selectedProcess], null, 2)}
      </pre>
    </div>
  );
}

export default App;
