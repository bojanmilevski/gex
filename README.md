# GEX

An `ex`tension installer for `Ge`cko browsers. I created this extension to further simplify and
automate a system (re)install. Currently only tested on `Linux`.

## FUNCTIONALITY

At first, `gex` finds your browser's data path, where all user data is stored. This is achieved by
providing `gex` with your browser and the profile you wish to use. It then scans your `profiles.ini`
file located in the base directory of your browser's data path and validates if the profile you
provided exists. After the validation, it searches a plugin by it's slug. A slug can easily be found
by looking at an extension's URL:

```
https://addons.mozilla.org/en-US/firefox/addon/darkreader
```

In this case, `darkreader` is this extension's slug. A request is sent to
`https://addons.mozilla.org/api/v5/addons/addon` with the slug appended to this string. This page
returns a `.json` containing information about the extension, if it exists. The extension gets
installed to the profile's `extensions` folder with it's `guid` being the filename.

More information on Mozilla's Add-On API can be found
[here](https://addons-server.readthedocs.io/en/latest/topics/api/addons.html).

### CAVEATS

The desired browser must have been previously opened in order for the `default` and
`default-release` profiles to be created.

### ASSUMPTIONS

This program assumes that you are using `firefox` as your default browser. It installs all
extensions to `~/.mozilla/firefox/<currently_selected_profile>/extensions`.

## INSTRUCTIONS

Running `gex -h` issues the following output:

```
An extension installer for Gecko browsers

Usage: gex [OPTIONS] <--install <EXTENSIONS>...|--search <SEARCH>|--delete <DELETE>|--update <UPDATE>|--list>

Options:
  -i, --install <EXTENSIONS>...
  -s, --search <SEARCH>
  -d, --delete <DELETE>
  -u, --update <UPDATE>
  -l, --list
  -b, --browser <BROWSER>        [default: firefox]
  -p, --profile <PROFILE>
  -v, --verbose
  -o, --log
  -h, --help                     Print help
  -V, --version                  Print version
```

### EXAMPLES

To install [Dark Reader](https://addons.mozilla.org/en-US/firefox/addon/darkreader) for `Firefox`,
simply run the following command:

```
gex -i darkreader
```

To achieve the same for `Librewolf`:

```
gex -i darkreader -b librewolf
```

... for the `default` profile:

```
gex -i darkreader -b librewolf -p default
```

## LICENSE

This software is licensed under the [GPL v3.0 License](https://www.gnu.org/licenses/gpl-3.0.en.html).

## SPECIAL THANKS

- [Interfiber's `firefoxext` program written in `C`](https://github.com/Interfiber/firefoxext)
  for being an inspiration. This program is it's spiritual successor. :)

- [lsd](https://github.com/lsd-rs/lsd) for providing a well designed and structured code base.
