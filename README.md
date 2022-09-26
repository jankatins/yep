# yep: yet another package manager for local cli commands

[THIS IS AN IDEA, written down to see if it has any merits and looks sane... Comments welcome!]

Install and update command line apps in your $HOME folder. 

Good for maintaining local cli commands, which you want to update regularly to the latest version available in the upstream release page/repo. Gives you the flexibility to install whatever you want. Minimizes the hassle of keeping them updated.

## Overview

Installing a package is a three-step process:

1. Expanding the given "recipe" with some defaults. Depends on the given `--type=xx` (Default: downloading from a 
   github release page and installing as a static binary). This basically makes a minimal example 
   `yep install cli/cli` do the right thing without removing flexibility.  
3. Getting the content of the package to the local disk, e.g., downloading and extracting a release zip or cloning a
   repository into a "package" folder or installing via rust/cargo.
3. Doing something with content, e.g., renaming a file in the package folder or symlinking a binary into $PATH or
   executing a Makefile in the folder to install the package

The "recipe" how to install a package is persisted and updates are basically reinstalls.

- Build in rust, just to learn the language
- Inspired by [zinit](https://github.com/zdharma-continuum/zinit) and how the download/install stuff works
- Extendable: add more arguments to influence what to do with package content or write a new source to get packages from
- Explicit: only the simplest (single cli command in a github release for the os+arch) case is covered by the defaults
- Smart enough: figuring out what to download and what to expose in the `$PATH` is build in
- Flexible: the arguments describe how to "install" a package
- Depends on other cli commands, e.g. for extraction of downloaded files or git

## Features in the happy path

- Can figure out the latest release of a project on github for a platform/arch and downloads the right file from that
  release
    - Does some smart decision which file to download (zinit has regexes, but maybe do some scoring or so...)
- Can extract the downloaded file into a local folder
    - Delegate to some local commands for the actual extraction
- Make some commands executable
    - E.g. based on "file $file"
- Add a symlink to binaries in a folder (`$HOME/.local/bin` on linux, user can overwrite)
    - based on the project name? Or all binaries?
    - user specified
- `$HOME/.local/yep/packages/<package>/.yep.v1.json`: persistent store for the "commands" to handle the package
- Update = install, by running the persisted commands
    - Actually make the initial command write the file and make install/update idempotent (remove everything but the
      persisted commands file, load file, run file)

## Invocation

- `yep install <source> <arguments>`: installs `<source>` according to what is specified in the arguments.
  The default arguments are `--type=static-binary --from='gh-r'` which expects `<source>` to be a
  github `org/repo` combination (see "happy path" above).
- `yep delete <package-name or source>`: remove a package and all files created outside of the package folder
- `yep setup`: creates the required folders and default config file and puts itself into the needed places for
  everything to work and to update itself. Idempotent.

## Arguments / commands:

- Executed in the order they are given
- Some arguments are "commands" (like "link this to there...") and can be given multiple times
- Every enabled command/argument can be disabled via `--no-<command>` which removes any such commands from the list up
  to that point

- `--id-as <name>`: the name of the package in the packages folder; defaults to something derived from the given
  url/package/repo slug by some magic conversion (todo: define it...); Can only be given once.
- `--from='{gh-r,gh,gl-r,gl,clone,download,nodejs}'`: uses the appropriate download method to download, maybe extract and
  finally fill a folder; can only be given once; Latest given argument wins.
- `--dl-pick=<regex>`/ `--dl-pick-os=<regex>`/ `--dl-pick-os-arch=<regex>`: influences the file which is picked by the
  download step
- `--dl-nodejs-package="<package-name>[,<package-name>]"`: Installs a/multiple node package(s) via npm into the package folder, using 
  `package-name` (fallback: `name`)
- `--dl-version=<str>`: the version to download, defaults to "latest release/HEAD" if not given; Can only be given once.
- `--mv='<regex>-><name>'`: move a file after extracting it; both parts relative to the package folder; Explicit arguments overwrite
   default ones; can be given multiple times.
- `--ln='<regex>-><name>'`: symlink a file after extracting it; both parts relative to the package folder; Explicit arguments
   overwrite default ones; can be given multiple times.
- `--exec=<command to exec>`: executes the given command in the folder; Explicit arguments overwrite default ones; can be given 
  multiple times.
- `--install-link='<regex>-><name>'`: Symlinks the file specified by the first part in the package folder into the configured
  binaries folder; first part is relative to the package folder, second part is either relative to the binaries folder
  or absolute; if first part matches more than one file, will all symlink them (if a second part is given as a file,
  will error); if second part is given and ends in a slash (=folder), will add the symlink under the same name in that
  folder. If nothing is given, will symlink all executable files in the root folder of the package under the same name. Explicit 
  arguments overwrite default ones; can be given multiple times.
    - TODO: maybe make the default collect all executable files and add a `--install-link=!...`
      or `--no-install-link=<regex>` variant which excludes files again? But how would that work with extensions which
      get called once per specified argument instead of with all arguments?
- `--install-yep-extensions`: will put symlinks to any executable files matching `(type|command|from)_*` in the package
  folder into the extension folder (see below).
- `--install-shell-wrapper-node="<path/to/binary> [-> <name>]`: Install a shell script which wraps a node.js installation 
  into the configured binaries folder as `name` (fallback: filename of `<path/to/binary>`) ; (Similar things for python-pip/...)
- (Probably more, e.g. install man files or zsh completions)

TODO: think about how to overwrite/extend stuff from the template/default type: I can see both "overwrite it" (e.g. the
rust case below installs a different executable) and extend it (e.g. installs two executables").

Before actually doing the actions specified by the arguments, some string replacements are done:

- `%PACKAGEFOLDER%`: folder to the package

## Extending

Three cases:

- **Change default behavior/add a template**: add a `--type=<name>` which then can set the
  defaults: `$HOME/.local/yep/extensions/type_<type name>`:
  calling it with the `<source>` will output default options which are inserted into the start of the original
  cli: e.g.
    - `yep install cli/cli`  will use the default `static-binary` and
      calls `$HOME/.local/yep/packages/yep/yep type_static-binary cli/cli` (after checking that there is
      no `$HOME/.local/yep/extensions/type_static-binary`) which then outputs `--from='gh-r' --install-link` which
      downloads the latest release from github and symlinks all executables in the root folder of the resulting package
      folder.
    - `yep install whatever --type=rust` calls `$HOME/.local/yep/extensions/type_rust whatever` which might return something
      like `--from='rust' --install-link='bin/whatever -> whatever'` which then
      might install rust into the folder and then use cargo in that folder to install the
      `whatever` package and finally symlink the installed package into the binary folder
    - `yep install whatever --type=nodejs` calls `$HOME/.local/yep/extensions/type_nodejs whatever` which might 
      return something like `--from='nodejs' --install-shell-wrapper-node='node_module/.bin/whatever -> whatever'` which
      then might install nodejs into the folder, use that nodejs to install the `whatever` package and finally create 
      a shells cript wrapper arround `node_module/.bin/whatever` package into the binary folder.
    - `yep install org/repo --type=make` calls `$HOME/.local/yep/extensions/type_make org/repo` which
      might return `--from='gh' --exec='make install DESTDIR=%PACKAGEFOLDER% --install-link` which clones the org/repo
      from github, runs the specified make command and finally symlinks all executable files in the root folder of the
      package into the binary folder.
- **Add a new download method**: adds a command which handles getting content and putting it into the package folder
  `$HOME/.local/yep/extensions/from_<from name>`: `yep --from="git-clone" https://git.whatever.com/repo.git ...`
  will call
  `$HOME/.local/yep/extensions/from_git-clone https://git.whatever.com/repo.git <packages folder> -- <original arguments>`
  and expects that command to fill the package folder with the content of the
  repo; The `from` command should indicate via return code if the content of the packages
  was refreshed (=rerun the rest of the recipe, return code 0) or not (TODO: some special code, outside of the normal
  error codes).
- **Add new commands**: can add new commands by putting executable files
  into `$HOME/.local/yep/extensions/command_<new command>`
  which get called: `yep ... --whatever='x->y'`
  -> `exec $HOME/.local/yep/extensions/command_whatever {install,remove} 'x->y'`

Installation by a special argument/type (e.g. `yep --type=extension ... ` which will ln all files in the root folder
which fit the required file patterns into the extension folder (see `--install-yep-extensions`).

## Use cases

#### install yep

Download the appropriate file for your arch/os, make it executable and run `yep setup`. This will do:

1. Read a config file, if it already exists
2. Write an example config file, if it doesn't exist, which just lists (but not configures) options
3. Setup the base folder(s): `$HOME/.local/yep/{...}`
4. Put yep into its own "package" together with the right persisted commands file
5. Put a symlink to yep into `$HOME/.local/bin/` (=installs it)

TODO: how to persist this steps so update runs this again? probably nothing special, if really needed, add
appropriate arguments/ commands in the persisted file. Or put that into the "update" command as a special post-update
step.

(The command is idempotent, so can be rerun again without harming your installation or fixing it if something is broken)

#### Install ripgrep via `yep BurntSushi/ripgrep`

1. Downloads the appropriate os+arch file from the latest release from github
2. Extract the file and makes ripgrep executable
3. Creates a symlink to the ripgrep binary as  `$HOME/.local/bin/ripgrep`
## License
This project is proudly licensed under the MIT license (LICENSE or http://opensource.org/licenses/MIT).

yep can be distributed according to the MIT license. Contributions will be accepted under the same license.
