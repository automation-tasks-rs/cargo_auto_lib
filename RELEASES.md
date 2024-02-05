# Changelog cargo_auto_lib

## Version 1.1.102 (2024-02-05)

### Other changes in 1.1.102

- Researching a way to make the Public API consistent for future releases I made a breaking change. I incremented the minor version number and reset the patch number to signal it is a breaking change. These functions are rarely used, so it will not be a big deal.

I removed the exported module `utils_mod` from the Public API.  Its functions are now in the root module of the Public API.
How to refactor the code:

- find `utils_mod::` and replace with empty string.

## Version 1.0.96 (2024-02-04)

### Other changes in 1.0.96

- Refactor many `unwrap()` to error handling with `thiserror`. The problem is that it could introduce unwanted braking changes. I need to research how to make my Public API more manageable.
