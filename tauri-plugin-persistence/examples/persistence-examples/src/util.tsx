import { Error as RError, Result } from "tauri-plugin-persistence-api";
import { Badge, Group, Text } from "@mantine/core";
import { startCase } from "lodash";

export function assert_result<T>(
    result: Result<T, RError>,
    check?: (result: T) => true | string
): true | string {
    return (
        result
            .map(
                (data) => Result.ok(check ? check(data) : true),
                (err) => Result.ok(JSON.stringify(err))
            )
            .data() ?? "INVALID_ASSERTION"
    );
}

export function TestResult({
    name,
    tests,
}: {
    name: string;
    tests: { [key: string]: string | boolean };
}) {
    return (
        <Group gap="sm">
            <Text>{startCase(name)}</Text>
            {tests[name] === true ? (
                <Badge color="green" size="md">
                    SUCCESS
                </Badge>
            ) : tests[name] === undefined ? (
                <Badge size="md" color="red">
                    Test not run.
                </Badge>
            ) : (
                <Badge size="md" color="red">
                    {tests[name]}
                </Badge>
            )}
        </Group>
    );
}
