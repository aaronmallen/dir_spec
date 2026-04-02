# Testing

This guide covers how to write and organize tests in Gest.

The goal is to test meaningful behavior without over-testing trivial code. When in doubt, focus on logic that could
break -- transformations, edge cases, and complex calculations.

## Core Principle

**Tests are the source of truth.** Tests define the expected behavior of the system. If a test passes, the behavior it
describes is correct by definition.

**Never modify existing integration tests unless the issue explicitly calls for behavioral change.** If an integration
test is failing, the implementation needs to change -- not the test. The only exception is when the issue or spec
explicitly requires a change in observable behavior.

## Running Tests

```bash
mise run test                        # Run all tests
mise run test -- --test xyz          # Run tests matching "xyz"
```

## What to Test

**Test:**

- Functions and methods with logic (arithmetic, transformations, conditionals)
- Display/formatting implementations
- Custom comparison or ordering implementations
- Edge cases and boundary conditions
- Inverse operations (roundtrip tests)
- Error paths and failure modes

**Skip:**

- Simple constructors that just assign fields
- Trivial getters that return field values
- Thin wrappers that only delegate to another function

Before writing a test, ask: "Does this test verify actual logic, or just that field assignment works?"

## Test Structure

Tests should be organized in a way that mirrors the module structure. Group tests by the function or method they
exercise, and name test functions descriptively using the pattern `it_<does_something>`.

## Conventions

**Naming:** Test functions use the pattern `it_<does_something>`. Group names match the function or method being tested.

**Ordering:** Test groups follow [code style][code-style] ordering -- static/associated functions first
(alphabetically), then instance methods (alphabetically).

**Test body structure:** Separate setup from assertions with a blank line. For tests with multiple assertion groups,
separate each group with a blank line.

**Integration tests:** Integration tests validate end-to-end behavior and are the strondir_spec contract in the
codebase. They must not be modified to make a failing implementation pass. If an integration test fails after a code
change, the code change is wrong unless the issue explicitly calls for a behavioral change.

[code-style]: https://github.com/aaronmallen/dir_spec/blob/main/docs/dev/code-style.md
