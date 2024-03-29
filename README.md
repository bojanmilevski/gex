# GEX

An `ex`tension manager for `Ge`cko browsers. I created this program to further simplify and
automate a system (re)install. Currently only tested on `Linux`.

- THIS PROGRAM IS IN VERY EARLY STAGES

## FUNCTIONALITY

### USER PROFILES

At first, `gex` finds your browser's data path, where all user data is stored. This is achieved by
providing `gex` with your browser, and the profile you wish to use. The default browser is Firefox.

If no profile is provided, `gex` uses the one in use by looking at `installs.ini`.

If a profile is provided, it scans your `profiles.ini` file located in the base directory of your
browser's data path and validates if the profile you provided exists.

### MOZILLA API

`gex` uses three URLs to communicate to Mozilla's APIs:

- `https://addons.mozilla.org/firefox/downloads/file` to download an extension
- `https://addons.mozilla.org/api/v5/addons/addon` to find information about an extension
- `https://addons.mozilla.org/api/v5/addons/search` to search for extensions similar to a provided
  slug

Each method appends the extension's slug at the end of the URL.

A slug is a unique string by which Mozilla identifies its extensions.

A slug can easily be found by looking at an extension's URL while browsing
[Mozilla's Addon page](https://addons.mozilla.org/en-US/firefox/):

```
https://addons.mozilla.org/en-US/firefox/addon/darkreader
```

In this case, `darkreader` is this extension's slug.

More information on Mozilla's Add-On API can be found
[here](https://addons-server.readthedocs.io/en/latest/topics/api/addons.html).

### INSTALLING

After profile validation, it searches an extension by its slug. A request is sent to
`https://addons.mozilla.org/api/v5/addons/addon` with the slug appended to this string. This page
returns a `json` containing information about the extension, if it exists. The extension gets
installed to the profile's `extensions` folder with it's `guid` being the filename.

### SEARCHING

The slug provided to this subcommand is sent to `https://addons.mozilla.org/api/v5/addons/search`.
A `json` of the first 50 extensions, sorted by the number of users, is returned to the user.

### UPDATING (CURRENTLY NOT IMPLEMENTED)

...

### DELETING (CURRENTLY NOT IMPLEMENTED)

...

### LISTING

The provided profile's `addons.json` file returns each `amoListingURL` final slug.

## INSTRUCTIONS

Running `gex -h` issues the following output. Keep in mind that not all commands are implemented:

```
An extension installer for Gecko browsers

Usage: gex <COMMAND>

Commands:
  delete   [aliases: rm]
  install  [aliases: i]
  list     [aliases: ls]
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

This software is licensed under the [GPL v3.0 License](https://www.gnu.org/licenses/gpl-3.0.en.html).

## SPECIAL THANKS

- [firefoxext](https://github.com/Interfiber/firefoxext) by Interfiber for being an inspiration.
  This program is its spiritual successor. :)

- [lsd](https://github.com/lsd-rs/lsd) for providing a well-designed and structured code base.

- [bob](https://github.com/MordechaiHadad/bob) is another package manager.
