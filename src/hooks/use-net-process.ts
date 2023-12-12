import { NetConnections, NetStat, Process } from "@/types";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

function useNetStat(enabled: boolean) {
  const [netStatResult, setNetStatResult] = useState<NetStat | null>(null);

  async function net_stat() {
    const response: string = await invoke("net_stat");

    try {
      setNetStatResult(JSON.parse(response));
    } catch {
      setNetStatResult(null);
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

  return { netStatResult, net_stat };
}

function useNetConnections(enabled: boolean) {
  const [netConnectionsResult, setNetConnectionsResult] =
    useState<NetConnections>({});

  async function net_connections() {
    const response: string = await invoke("net_connections");

    try {
      setNetConnectionsResult(JSON.parse(response));
    } catch {
      setNetConnectionsResult({});
    }
  }

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

  return { netConnectionsResult, net_connections };
}

function useProcesses(enabled: boolean) {
  const [processesResult, setProcessesResult] = useState<Process[]>([]);

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
        processes();
      }, 2500);
    }

    return () => {
      interval && clearInterval(interval);
    };
  }, [enabled]);

  return { processesResult, processes };
}

export function useNetProcess(enabled: boolean) {
  const { netStatResult, net_stat } = useNetStat(enabled);
  const { netConnectionsResult, net_connections } = useNetConnections(enabled);
  const { processesResult, processes } = useProcesses(enabled);

  // Initial load
  useEffect(() => {
    net_stat();
    net_connections();
    processes();
  }, []);

  return {
    netStatResult,
    netConnectionsResult,
    processesResult,
  };
}
