# GEX

An `ex`tension manager for `Ge`cko browsers. I created this program to further simplify and
automate a system (re)install. Currently only tested on `Linux`.

- THIS PROGRAM IS IN VERY EARLY STAGES

## MOZILLA API

`gex` uses two URLs to communicate to Mozilla's APIs:

- `https://addons.mozilla.org/api/v5/addons/addon` to find information about an addon
- `https://addons.mozilla.org/api/v5/addons/search` to search for addons similar to a provided
  slug

Each method appends the addons's slug at the end of the URL.

A slug is a unique string by which Mozilla identifies its addons.

A slug can easily be found by looking at an addon's URL while browsing
[Mozilla's Addon page](https://addons.mozilla.org/en-US/firefox/):

```
https://addons.mozilla.org/en-US/firefox/addon/darkreader
```

In this case, `darkreader` is this addon's slug.

More information on Mozilla's Add-On API can be found
[here](https://addons-server.readthedocs.io/en/latest/topics/api/addons.html).

## FUNCTIONALITY

### USER PROFILES

At first, `gex` finds your browser's data path, where all user data is stored. This is achieved by
providing `gex` with your browser, and the profile you wish to use. The default browser is Firefox.

If no profile is provided, `gex` uses the one in use by looking at `installs.ini`.

If a profile is provided, it scans your `profiles.ini` file located in the base directory of your
browser's data path and validates if the profile you provided exists.

### DATABASE

This program uses three databases:

- `extensions.json`
- `addons.json`
- manifest files of each addon

Each contain important information about the addons. After finding the user profile, `gex` reads
and keeps information about all of the installed plugins in memory.

### INSTALL

After gex configures itself, it searches an addon by its slug. This page returns a `json` containing
information about the addon, if it exists. Each addon contains a URL which points to the it's latest
xpi file. The addon gets installed to the profile's `extensions` folder with it's `guid` being the
filename.

### LIST

The profile's `addons.json` file returns each `amoListingURL` final slug.

### REMOVE

All provided slugs are transformed into their respective ids. Each addon in `addons.json` contains
`amoListingURL` and `id` values. `amoListingURL` is just an URL of Mozilla's addon website
(`https://addons.mozilla.org/`) with the addon's slug appended. By finding the slug, we get the
addon's id, which is later used to identify the addon throughout all three databases, and removed
from the profile.

### SEARCH

The provided slug is sent to `https://addons.mozilla.org/api/v5/addons/search`. A `json` of the
first 50 addon, sorted by the number of users, is returned to the user.

### UPDATE

Not providing slugs means that you wish to update all installed packages. If slugs are provided,
it is checked whether they are already installed or not. The currently installed addon's versions
are checked against their up-stream versions. After filtering which addons require an update, the
install module is called. The updater is practically a wrapper around the installer.

## INSTRUCTIONS

Running `gex -h` issues the following output. Keep in mind that not all commands are implemented:

```
An extension installer for Gecko browsers

Usage: gex <COMMAND>

Commands:
  install  [aliases: i]
  list     [aliases: ls]
  remove   [aliases: rm]
  search   [aliases: s]
  update   [aliases: u]
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### EXAMPLES

To install [Dark Reader](https://addons.mozilla.org/en-US/firefox/addon/darkreader) for `Firefox`,
simply run the following command:

```
gex install darkreader
```

To achieve the same for `Librewolf`:

```
gex install darkreader -b librewolf
```

... for the `default` profile:

```
gex install darkreader -b librewolf -p default
```

## CAVEATS

The desired browser must have been previously opened in order for this program to function.

## LICENSE

This software, like all my other software, is licensed under the [GPL v3.0 license](https://www.gnu.org/licenses/gpl-3.0.en.html).
Please consider licensing your software under this license as well.

## THANKS

- [firefoxext](https://github.com/Interfiber/firefoxext) by Interfiber for being an inspiration.
  This program is its spiritual successor. :)

- [lsd](https://github.com/lsd-rs/lsd) for providing a well-designed and structured code base.

- [bob](https://github.com/MordechaiHadad/bob) is another package manager.
