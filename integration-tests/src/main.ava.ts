import { Worker, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";
import { json } from "stream/consumers";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const contract = await root.createSubAccount("test-account");
  const contract2 = await root.createSubAccount("hello-world-contract");
  // Get wasm file path from package.json test script in folder above
  await contract.deploy(process.argv[2]);
  await contract2.deploy("./src/hello_near.wasm");

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});
test("test my batch actions method", async (t) => {
  const { root, contract } = t.context.accounts;
  const { contract2 } = t.context.accounts;
  // Call the method and make assertions on the result
  console.log(t.context.accounts);
});
