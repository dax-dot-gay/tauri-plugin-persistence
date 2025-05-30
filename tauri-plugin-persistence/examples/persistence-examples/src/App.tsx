import { Context, Database, FileHandle } from "tauri-plugin-persistence-api";
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
        tests.databaseFind = assert_result(
            await collection.find_one({ _id: "TEST_ID" }),
            (v) => (v?.value == "test_value" ? true : "Item validation failed")
        );
        tests.databaseRemove = assert_result(
            await collection.delete_one({ _id: "TEST_ID" }),
            (v) => (v ? true : "Item deletion failed")
        );
    }

    const filehandleResult = await context.open_file("test_file.txt", {
        mode: "create",
        new: true,
        overwrite: true,
    });
    tests.fileCreation = assert_result(filehandleResult);

    if (filehandleResult.data()) {
        const file = filehandleResult.data() as FileHandle;
        tests.writeToFile = assert_result(
            await file.write_text("TEST FILE DATA")
        );
        tests.closeFile = assert_result(await file.close());
    }

    const fileReadResult = await context.open_file("test_file.txt", {
        mode: "read",
    });
    tests.fileReadOpen = assert_result(fileReadResult);

    if (fileReadResult.data()) {
        const file = fileReadResult.data() as FileHandle;
        tests.readFile = assert_result(await file.read_text(), (r) =>
            r === "TEST FILE DATA" ? true : "File contents do not match."
        );
        await file.close();
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
                <TestResult name="databaseFind" tests={results.tests} />
                <TestResult name="fileCreation" tests={results.tests} />
                <TestResult name="writeToFile" tests={results.tests} />
                <TestResult name="closeFile" tests={results.tests} />
                <TestResult name="fileReadOpen" tests={results.tests} />
                <TestResult name="readFile" tests={results.tests} />
            </Stack>
        </MantineProvider>
    );
}

export default App;
