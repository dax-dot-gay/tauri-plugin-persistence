import { Context } from "tauri-plugin-persistence-api";
import "./App.css";
import { useState } from "react";

async function runTestsAtPath(path: string): Promise<{
    basePath: string;
    tests: { [key: string]: true | string };
}> {
    let contextResult = await Context.open("TEST", path);
    let context: Context;
    if (contextResult.error()) {
        return {
            basePath: "",
            tests: { createContext: JSON.stringify(contextResult.error()) },
        };
    } else {
        context = contextResult.data() as Context;
    }

    const tests: { [key: string]: true | string } = {};
    tests.createContext = true;

    const basePath = (await context.get_base_path()).ok_or("");
    tests.resolveBasePath = basePath === "" ? "Path resolution error" : true;

    return { basePath, tests };
}

function App() {
    const [selectedPath, setSelectedPath] = useState(
        "../../../../contexts/testing"
    );
    const [results, setResults] = useState<{
        basePath: string;
        tests: { [key: string]: true | string };
    }>({ basePath: "", tests: {} });
    console.log(results);
    return (
        <>
            <div>
                <span>
                    <input
                        value={selectedPath}
                        onChange={(e) => setSelectedPath(e.target.value)}
                    />
                    <button
                        onClick={() =>
                            runTestsAtPath(selectedPath).then(setResults)
                        }
                    >
                        RUN AT PATH
                    </button>
                </span>
            </div>
            <p>---</p>
            <div>Base Path: {results.basePath}</div>
            <div>Context Creation: {results.tests.createContext}</div>
            <div>Base Path Resolution: {results.tests.resolveBasePath}</div>
        </>
    );
}

export default App;
