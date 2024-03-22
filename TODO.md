# HIGH PRIORITY

- "super" addon struct
  - consists of manifest, extensions.json addon info, addons.json addon info
  - passed to database.add(), database.remove()
  - created in try_run()
- complete addons.json addon, extensions.json addon
- database duplicates (check before adding)
- each crate has it's own error module
- fix mut runnable trait
- fix try_configure_from configurable trait
- implement remaining operations (delete, list, update)
- rename structs
- replace unwrap with ok_or errors
- smallvec, tinyvec, set?

# MID PRORITY

- async tasks execute without waiting
- multi progress bar
- parallel extension installs
- send multiple extensions in one query in order to avoid multiple requests - avoid spamming mozilla servers
- tokio::task::spawn vec and then run

# LOW PRIORITY

- &str instead of String
- access modifiers
- code documentation
- configuration file (~/.config/gex/config.json)
- description languages
- if install vec is empty, print "no extension found"
- install for multiple browsers
- install for multiple profiles
- log flag, path, level
- man file
- manually enable/disable extensions
- more browsers and respective data paths
- prompt for reinstall, force install etc.
- references, ownerships, lifetimes
- remove clone, cloned, to_owned, as_str, format!, String::from
- search params (sort output by rating, score...)
- thunderbird support
- verbose flag
- windows and macos compatibility
- search displays installed addon for profile, browser
