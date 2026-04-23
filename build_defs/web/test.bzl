"""Back-compat re-export — new code should load the specific files.

`load("//build_defs/web:playwright.bzl", "playwright_test")`
`load("//build_defs/web:vitest.bzl", "vitest_test")`
"""

load("//build_defs/web:playwright.bzl", _playwright_test = "playwright_test")
load("//build_defs/web:vitest.bzl", _vitest_test = "vitest_test")

playwright_test = _playwright_test
vitest_test = _vitest_test
