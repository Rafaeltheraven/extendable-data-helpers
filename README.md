# extendable-data-helpers
Helper methods for [extendable-data](https://crates.io/crates/extendable-data). Exists because I can't just EXPORT REGULAR FUNCTIONS IN A PROC-MACRO PACKAGE.

Methods in here are really for internal use. Ideally, these would be in an internal package for `extendable-data`, but as hard as I tried I have found no way to export these methods properly. Until the rust compiler stops being annoying, we'll be stuck with this structure. Ironically, all of the heavy lifting happens in this package.
