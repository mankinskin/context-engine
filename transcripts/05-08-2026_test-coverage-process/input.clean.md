We have a clear problem: we are not testing enough, our test coverage is too low, and our tools do not behave the way we think they do.

Our task is to define a clear process for making sure that every tool we have is tested, that we write good tests, and that we achieve complete, ideally 100% test coverage.

I propose the following process:

1. Take stock of our current test environment.
   - Identify which tests already exist.
   - Check whether those tests are the right tests and whether they are useful.
   - Determine where those tests run and what kind of tests they are.

2. Inventory all of our tools and the code we actually write and need to test.
   - Determine the full surface area of our tools.
   - Identify everything that should be covered by tests.

3. Compare our defined requirements against existing tests.
   - Check which requirements are already covered.
   - Check which requirements are not yet covered.
   - Identify the tests we still need to write.

4. Make the tests realistic and useful.
   - The tests should be automated, but they should also mirror the real use case one-to-one.
   - The execution environment should match reality as closely as possible.
   - Ideally, the process should run in a sandbox or fixed environment and work with real data, or something close to that.
   - We should create fixtures collected from real data so that we can replay them and verify that our tools exhibit the correct behavior.
   - We should write as many different tests as possible, including difficult and adversarial tests.
   - We should systematically work through all logical cases and all combinations where that is feasible.
   - We should build a test matrix and define equivalence classes instead of only trying a few combinations and assuming they will work.
   - We may also need to improve the specs themselves so they are easier to test, by creating a clearer model and a clearer specification.

5. Consider whether we want to use automated test coverage measurement tools later.
   - For now, the priority is a manual inventory: what tests we already have, what tests are missing, and how we can improve them.
   - We should turn this process into an agent or instruction template so that we can revisit the whole system regularly, perhaps once a month, inventory our tests, and keep improving them.
   - We should also create a template that helps us write good tests in general.
