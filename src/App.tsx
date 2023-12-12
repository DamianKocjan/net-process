import { useMemo, useState } from "react";

import { COLUMNS } from "./components/connections/columns";
import { DataTable } from "./components/connections/data-table";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "./components/ui/accordion";
import { Button } from "./components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "./components/ui/card";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "./components/ui/select";
import { useNetProcess } from "./hooks/use-net-process";
import { NetStatChildKeys, NetStatKeys } from "./types";
import { prettyBytes, prettyHeading, prettyNumber } from "./utils/formatters";
import { isOnlyObject } from "./utils/is-only-object";

function App() {
  const [enabled, setEnabled] = useState(false);
  const { netConnectionsResult, netStatResult, processesResult } =
    useNetProcess(enabled);
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

  return (
    <div className="container space-y-4 pt-8">
      <Button
        type="button"
        className="w-full"
        size="lg"
        onClick={() => setEnabled((prev) => !prev)}
      >
        Refresh ({enabled ? "on" : "off"})
      </Button>

      <Accordion
        type="multiple"
        defaultValue={["item-1", "item-2"]}
        className="w-full"
      >
        <AccordionItem value="item-1">
          <AccordionTrigger>Net stats</AccordionTrigger>
          <AccordionContent>
            {netStatResult ? (
              <div className="flex gap-4 overflow-x-auto">
                {(Object.keys(netStatResult) as NetStatKeys[]).map((key) => (
                  <Card key={key} className="min-w-max">
                    <CardHeader>
                      <CardTitle>{prettyHeading(key)}</CardTitle>
                    </CardHeader>
                    <CardContent>
                      {(
                        Object.keys(netStatResult[key]) as NetStatChildKeys[]
                      ).map((key2) => (
                        <div key={key2}>
                          <div className="font-bold">{prettyHeading(key2)}</div>
                          <p className="text-xs text-muted-foreground">
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
                                  {prettyBytes(
                                    // @ts-expect-error - I know what I'm doing
                                    netStatResult[key][key2].received
                                  )}
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
                        </div>
                      ))}
                    </CardContent>
                  </Card>
                ))}
              </div>
            ) : null}
          </AccordionContent>
        </AccordionItem>

        <AccordionItem value="item-2">
          <AccordionTrigger>Connections</AccordionTrigger>
          <AccordionContent>
            <Select
              value={selectedProcess}
              onValueChange={(value) => setSelectedProcess(value)}
            >
              <SelectTrigger className="w-full">
                <SelectValue placeholder="Select a process" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>Processes</SelectLabel>
                  {processesWithConnections.map((process) => (
                    <SelectItem
                      key={process.pid}
                      value={process.pid.toString()}
                    >
                      {process.image_name} - {process.pid} - (
                      {process.session_name} - {process.session_number})
                    </SelectItem>
                  ))}
                </SelectGroup>
              </SelectContent>
            </Select>

            {netConnectionsResult[selectedProcess] ? (
              <DataTable
                columns={COLUMNS}
                data={netConnectionsResult[selectedProcess]}
              />
            ) : null}
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  );
}

export default App;
