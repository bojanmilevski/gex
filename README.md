# GEX

An `ex`tension installer for `Ge`cko browsers. I created this extension in order to further simplify and automate a system (re)install. Currently only works on `Linux`.

## FUNCTIONALITY

At first, `gex` finds your browser's data path, where all user data is stored. This is achieved by providing `gex` with your browser and the profile you wish to use. It then scans your `profiles.ini` file located in the base directory of your browser's data path and validates if the profile you provided exists. After the validation, it searches a plugin by it's slug. A slug can easily be found by looking at an extension's URL:

```
https://addons.mozilla.org/en-US/firefox/addon/darkreader
```

In this case, the final string (`darkreader`) is this extension's slug. A request is sent to `https://addons.mozilla.org/api/v5/addons/search/?q=` with the slug appended to this string. This page returns a `.json` file which contains a list of results containing extensions that met the search query. Out of the queried extensions, the one that matches the slug gets installed to the profile's `extensions` folder with it's `guid` being the filename.

### CAVEATS

The only caveat with `gex` is that the desired browser must have been previously opened in order for the `default` and `default-release` profiles to be created. `Gecko` browsers have a (rather weird) random profile naming scheme. This causes unnecessary overhead for programmers.

### ASSUMPTIONS

This program assumes that you are using `firefox` as your default browser and the `default-release` profile as your currently selected profile. It installs all extensions to `~/.mozilla/firefox/*.default-release/extensions`.

## INSTRUCTIONS

Running `gex -h` issues the following output:

```
Usage:  gex [OPTIONS]

Options:
	-i, --install <INSTALL>...
	-p, --profile <PROFILE>		[default: default-release]
	-b, --browser <BROWSER>		[default: firefox]
    -h, --help			Print help
	-V, --version			Print version
```

### EXAMPLES

To install [Dark Reader](https://addons.mozilla.org/en-US/firefox/addon/darkreader) for `Firefox`, simply run the following command:

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

## DEPENDENCIES

-   clap
-   reqwest
-   rust-ini
-   serde
-   serde_json
-   thiserror
-   tokio

## LICENSE

This software is licensed under the [GPL v3.0 License](https://www.gnu.org/licenses/gpl-3.0.en.html).

## CONTRIBUTION

Everyone is welcome to contribute. Pull requests and issue reports are just one way of improving this project.

## LIVE DOCUMENTATION

As this repository grows, the documentation changes along with it - providing information that is up to date and relevant to the state of this project.

## A NOTE FROM THE AUTHOR

This is my first program written in `rust`. I am aware that a large portion of the code is in need of refactoring. I assure anyone that this program will continue to improve, along with it's code.

## INSPIRATION

This project was inspired by [Interfiber's `firefoxext` program written in `C`.](https://github.com/Interfiber/firefoxext). This program is it's spiritual successor. 🙂
