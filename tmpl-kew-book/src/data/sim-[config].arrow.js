import { readFile, writeFile } from "fs/promises";
import { parse } from "@babel/parser";
import { Kew } from "../components/kew.js";
import * as _traverse from "@babel/traverse";
const traverse = _traverse.default.default;

const configHash = process.argv[2].replace("--config=", "");

const configFile = await readFile(`src/data/kew/${configHash}.js`);
let configStr = configFile.toString();

const options = {
  allowImportExportEverywhere: true,
  allowAwaitOutsideFunction: true,
  allowReturnOutsideFunction: true,
};

const cell = parse(configStr, options);

function findGlobals(ast) {
  const globals = new Map();

  function saveGlobal(path, name = path.node.name) {
    if (name == "sim") return;
    // init entry if needed
    if (!globals.has(name)) {
      globals.set(name, []);
    }
    // append ref
    const refsForName = globals.get(name);
    refsForName.push(path);
  }

  traverse(
    ast,
    {
      // ReferencedIdentifier
      ReferencedIdentifier: (path) => {
        // skip if it refers to an existing variable
        const name = path.node.name;
        if (path.scope.hasBinding(name, true)) return;

        // check if arguments refers to a var, this shouldn't happen in strict mode
        if (name === "arguments") {
          if (isInFunctionDeclaration(path)) return;
        }

        // save global
        saveGlobal(path);
      },
      ThisExpression: (path) => {
        if (isInFunctionDeclaration(path)) return;
        saveGlobal(path, "this");
      },
    },
    {
      type: "Program",
    }
  );
  return globals;
}

const globals = findGlobals(cell);

const fn = new Function("sim", ...globals.keys(), configStr);

let actualConfig;
fn(function (config) {
  actualConfig = typeof config == "function" ? config() : config;
});

if (!actualConfig) throw new Error("config didnt work");

const kew = new Kew(actualConfig);
const out = kew.to_arrow();

process.stdout.write(out);
