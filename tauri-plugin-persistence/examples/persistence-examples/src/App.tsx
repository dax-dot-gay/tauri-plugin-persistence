import { Context, Database } from "tauri-plugin-persistence-api";
import { useState } from "react";
import "@mantine/core/styles.css";
import {
    Button,
    Group,
    MantineProvider,
    Stack,
    TextInput,
} from "@mantine/core";
import { assert_result, TestResult } from "./util";

async function runTests(
    path: string
): Promise<{ path: string; tests: { [key: string]: string | boolean } }> {
    const contextResult = await Context.open("test_context", path);
    if (contextResult.error()) {
        return {
            path: "",
            tests: { contextCreation: JSON.stringify(contextResult.error()) },
        };
    }

    const context = contextResult.data() as Context;
    const tests: { [key: string]: string | boolean } = {};
    const pathResult = await context.get_base_path();
    const resolvedPath = pathResult.data() ?? "";
    tests.contextCreation = true;
    tests.contextPathResolution = assert_result(pathResult);
    tests.specificPathResolution = assert_result(
        await context.get_absolute_path_to("test.db")
    );

    const databaseResult = await context.database("test_db", "test.db");

    tests.databaseCreation = assert_result(databaseResult);

    if (databaseResult.data()) {
        const db = databaseResult.data() as Database;
        const collection = db.collection<{ _id: string; value: string }>(
            "test_collection"
        );
        tests.databaseInsert = assert_result(
            await collection.insert({ _id: "TEST_ID", value: "test_value" })
        );
        tests.databaseCount = assert_result(
            await collection.count_documents(),
            (v) => (v >= 1 ? true : "Item was not inserted.")
        );
    }

    return { path: resolvedPath, tests };
}

function App() {
    const [selectedPath, setSelectedPath] = useState(
        "../../../../contexts/testing"
    );
    const [results, setResults] = useState<{
        path: string;
        tests: { [key: string]: string | boolean };
    }>({ path: "", tests: {} });

    return (
        <MantineProvider defaultColorScheme="dark">
            <Stack gap="sm" p="sm">
                <Group gap="sm" wrap="nowrap" align="end">
                    <TextInput
                        size="md"
                        label="Context path"
                        value={selectedPath}
                        onChange={(e) => setSelectedPath(e.target.value)}
                        style={{ flexGrow: 1 }}
                    />
                    <Button
                        size="md"
                        onClick={() => runTests(selectedPath).then(setResults)}
                    >
                        Run Tests
                    </Button>
                </Group>
                <TextInput
                    readOnly
                    value={results.path}
                    label="Resolved Path"
                />
                <TestResult name="contextCreation" tests={results.tests} />
                <TestResult
                    name="contextPathResolution"
                    tests={results.tests}
                />
                <TestResult
                    name="specificPathResolution"
                    tests={results.tests}
                />
                <TestResult name="databaseCreation" tests={results.tests} />
                <TestResult name="databaseInsert" tests={results.tests} />
                <TestResult name="databaseCount" tests={results.tests} />
            </Stack>
        </MantineProvider>
    );
}

export default App;
