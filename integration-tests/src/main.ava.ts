import { Worker, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const evaluator = await root.createSubAccount("evaluator-contract");
  const helloWorld = await root.createSubAccount("hello-world-contract");

  // Get wasm file path from package.json test script in folder above
  await evaluator.deploy(process.argv[2]);
  await helloWorld.deploy("./src/aux_contracts/hello_near.wasm");

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, evaluator, helloWorld };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("Test Hello Near", async (t) => {
  const { root, evaluator, helloWorld } = t.context.accounts;
  const result = await evaluator.call(evaluator, "evaluate_hello_near", { contract_name: helloWorld.accountId }, { gas: "300000000000000" });
  t.is(result, true);
});
