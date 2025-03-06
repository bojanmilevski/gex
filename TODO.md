# NEW

- replace HashMap with proper struct in Addon (src/addon.rs)
- AUR link
- async closures are not a thing

# VERY HIGH PRIORITY

- log
- FIX: TODO:
- unwraps
- url serde fields
- remove `clone`, `cloned`, `to_owned`, `as_str`, `format!`, `String::from`

# HIGH PRIORITY

- fix initializable trait
- fix runnable trait
- rename structs

# MID PRORITY

- async tasks execute without waiting
- autostart browser in headless if no profile present
- multi progress bar
- rayon
- send multiple extensions in one query in order to avoid multiple requests - avoid spamming mozilla servers
- tokio::task::spawn vec and then run

# LOW PRIORITY

- `&str` instead of `String`
- access modifiers
- `clap_complete`
- code documentation
- configuration file (`~/.config/gex/config.json`)
  - cli args override config file
- description languages
- html parser for description
- if install vec is empty, print "no extension found"
- install for multiple browsers
- install for multiple profiles
- man file (`clap_mangen`)
- manually enable/disable extensions
- more browsers and respective data paths
- prompt for reinstall, force install etc.
- references, ownerships, lifetimes
- search displays installed addon for profile, browser
- search params (sort output by rating, score...)
- thunderbird support
- windows and macos compatibility
