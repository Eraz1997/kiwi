import {
  CircleAlert,
  Info,
  ListRestart,
  SquareChevronRight,
  TextCursorInput,
} from "lucide-solid";
import {
  Component,
  For,
  Match,
  Show,
  Switch,
  createResource,
  createSignal,
} from "solid-js";
import { Button } from "src/components/ui/button";
import { Card } from "src/components/ui/card";
import { Field } from "src/components/ui/field";
import { Spinner } from "src/components/ui/spinner";
import { Table } from "src/components/ui/table";
import { Text } from "src/components/ui/text";
import { createBackendClient } from "src/hooks/createBackendClient";
import { ContainerLog } from "src/types";
import { Box, HStack, VStack } from "styled-system/jsx";

type Props = {
  serviceName: string | null;
};

export const LogsViewer: Component<Props> = (props) => {
  const now = new Date().getTime();
  const [startDate, setStartDate] = createSignal<Date>(
    new Date(now - 60 * 60 * 1000),
  );
  const [endDate, setEndDate] = createSignal<Date>(new Date(now));

  const adminClient = createBackendClient("admin");

  const [logs, { refetch: refresh }] = createResource<ContainerLog[]>(
    async () => {
      if (!props.serviceName) {
        return [];
      }
      const { jsonPayload: logs } = await adminClient.get(
        `/services/${props.serviceName}/logs?from_date=${encodeDate(startDate())}&to_date=${encodeDate(endDate())}`,
      );
      return logs;
    },
  );

  return (
    <Show
      when={props.serviceName}
      fallback={
        <VStack gap="6">
          <Spinner size="xl" />
          <Text size="lg">Loading service details...</Text>
        </VStack>
      }
    >
      <Card.Root>
        <Card.Header>
          <Card.Title>Logs</Card.Title>
        </Card.Header>
        <Card.Body>
          <VStack gap="6">
            <Switch>
              <Match when={logs.loading}>
                <Spinner size="xl" />
              </Match>
              <Match when={!logs.loading}>
                <Box overflowY="auto" maxH="2xl">
                  <Table.Root>
                    <Table.Body>
                      <For each={logs()}>
                        {(log) => (
                          <Table.Row>
                            <Table.Cell>
                              <Switch>
                                <Match when={log.log_type === "Output"}>
                                  <Info />
                                </Match>
                                <Match when={log.log_type === "Error"}>
                                  <CircleAlert />
                                </Match>
                                <Match when={log.log_type === "Input"}>
                                  <TextCursorInput />
                                </Match>
                                <Match when={log.log_type === "Console"}>
                                  <SquareChevronRight />
                                </Match>
                              </Switch>
                            </Table.Cell>
                            <Table.Cell>{log.message}</Table.Cell>
                          </Table.Row>
                        )}
                      </For>
                    </Table.Body>
                  </Table.Root>
                </Box>
              </Match>
            </Switch>
            <HStack gap="4" flexWrap="wrap">
              <Field.Root flexBasis="40%" flexGrow="1">
                <Field.Label>From Date</Field.Label>
                <Field.Input
                  onChange={(event) =>
                    setStartDate(new Date(event.target.value))
                  }
                  value={startDate().toISOString()}
                />
              </Field.Root>
              <Field.Root flexBasis="40%" flexGrow="1">
                <Field.Label>To Date</Field.Label>
                <Field.Input
                  onChange={(event) => setEndDate(new Date(event.target.value))}
                  value={endDate().toISOString()}
                />
              </Field.Root>
              <Button onClick={refresh} flexGrow="1">
                Refresh <ListRestart />
              </Button>
            </HStack>
          </VStack>
        </Card.Body>
      </Card.Root>
    </Show>
  );
};

const encodeDate = (date: Date): string => {
  return encodeURIComponent(date.toISOString().replace("Z", ""));
};
