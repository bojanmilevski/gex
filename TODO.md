# HIGH PRIORITY

- "super" addon struct
  - consists of manifest, extensions.json addon info, addons.json addon info
  - passed to database.add(), database.remove()
  - created in try_run()
- anyhow
- chrono serde fields
- complete addons.json addon, extensions.json addon
- database duplicates (check before adding)
- each crate has it's own error module
- fill in all fields on crate::addon::addon based on https://addons.mozilla.org/api/v5/addons/addon/{slug}
- fix mut runnable trait
- fix try_configure_from configurable trait
- implement remaining operations (delete, list, update)
- rename structs
- replace unwrap with ok_or errors
- smallvec, tinyvec, set?
- url serde fields

# MID PRORITY

- async tasks execute without waiting
- clap_complete
- clap_mangen
- man file
- multi progress bar
- rayon
- send multiple extensions in one query in order to avoid multiple requests - avoid spamming mozilla servers
- tokio::task::spawn vec and then run
- verbose, debug, log (with path) flag

# LOW PRIORITY

- &str instead of String
- access modifiers
- code documentation
- configuration file (~/.config/gex/config.json)
- description languages
- if install vec is empty, print "no extension found"
- install for multiple browsers
- install for multiple profiles
- manually enable/disable extensions
- more browsers and respective data paths
- prompt for reinstall, force install etc.
- references, ownerships, lifetimes
- remove clone, cloned, to_owned, as_str, format!, String::from
- search params (sort output by rating, score...)
- thunderbird support
- windows and macos compatibility
- search displays installed addon for profile, browser
