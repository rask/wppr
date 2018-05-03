# rask/wppr

> WordPress Plugin Repofier

[![Build Status](https://travis-ci.org/rask/wppr.svg?branch=master)](https://travis-ci.org/rask/wppr) [![codecov](https://codecov.io/gh/rask/wppr/branch/master/graph/badge.svg)](https://codecov.io/gh/rask/wppr)


**Currently unstable and in development, please proceed with caution if you plan
to use this in production. This README states a future state on some parts so
following this README further than the _Installation_ steps may or may not
work as expected. Version 1.0.0 aims to fulfill the features outlined in this
README.**

Most paid, private, or otherwise custom plugins are rather difficult to manage
in a WordPress installation that is managed with [Composer][composer].

WPPR helps with that by taking the private plugins and making Composer friendly
Git repositories from them.

Instead of setting up complex setups in CI scripts or otherwise you can use
the plugins in a similar fashion as you would use WPackagist plugins.

>   **DISCLAIMER**: This tool does not fetch and install paid or private plugins
>   for you for free. You still need to buy and install them by hand for the
>   initial setup.
>
>   The authors of this tool are not responsible for issues that may arise from
>   using this tool in case you run into issues with plugin authors. Thanks
>   for understanding.

[composer]: https://getcomposer.org

## Features

-   Makes paid plugins such as Advanced Custom Fields PRO or GravityForms
    simpler to install with Composer
-   Automatically upgrades the plugins
-   Manages simple Git history for the plugins
-   Creates Git tags for the plugins during upgrades
-   Pushes upgraded plugins into a remote Git repository of your choosing

### Development status

A simple project base is currently ready with command and argument handling.
Current version supports plugin listing commands only. Active development of
upgrade, Composerization, and Gitification commands is underway.

## Installation

WPPR currently runs on `x86_64` compatible Linux systems (this includes most
popular distros like Ubuntu).

WPPR requires the following to be available in your system:

-   Git
-   WP-CLI
-   Access to remote Git repositories with push rights

WPPR is best run with a cron trigger, meaning you probably want to run it on
a server. Manual runs are possible as well but you lose the benefit of automated
releases for the plugins.

### Download a binary

The WPPR binaries are standalone binaries, meaning you can download and run them
as is. Download a binary into a location that is defined in your system's
`$PATH`, e.g. `/usr/bin/wppr`.

### From sources

Requirements:

-   Stable Rust compiler (`rustc` and Cargo)
-   Git
-   Linux environment

Clone this repository and compile the tool with

    $ git clone <repo> wppr
    $ cd wppr
    $ cargo test
    $ cargo build --release

After this an operable executable should be sitting in

    ./wppr/target/release/wppr

You should copy this binary to a location which has been defined in your
system's `$PATH`, e.g. `/usr/bin/wppr`. Executing it as follows

    $ wppr --help

should display a help screen with name and version information, if this is not
the case then the cloned version has a error in it most probably.

## Setup

Before you can start using WPPR you need to create a WordPress installation into
your system which hosts the plugins that WPPR manages. The installation has the
following requirements:

-   It is installed and configured like any regular WordPress installation
-   Plugins inside the installation can be managed with WP-CLI (i.e. you can
    update your plugins with `wp-cli plugin update <plugin>`).

The installation can (and probably should) be private, meaning it does not need
to be publicly accessible from outside your system.

## Configuration

WPPR requires a single configuration file. The file should reside inside the
same directory as or in a parent directory of the WordPress installation which
you installed according to the previous section (_Setup_). This means it should
be either located next to `wp-config.php` or in a parent directory above it.

The configuration file format is [TOML][toml] and in it you defined various
settings:

[toml]: https://github.com/toml-lang/toml

-   `binaries`: Which system binaries are used for various operations
-   `plugins`: The plugins you want to manage with WPPR
-   `pre_upgrade` Commands to run before running upgrades
-   `post_upgrade`: Commands to run after running upgrades

The configuration file can be called anything but this documentation assumes you
name your configuration files `wppr.toml`.

### Example configuration

    # /path/to/managed/wordpress/wppr.toml

    [binaries]
    git = "git"
    wpcli = "/home/johndoe/bin/wp"

    [git]
    user_name = "WPPR System"
    user_email = "wppr@mycompany.com"
    force_push = false

    [pre_upgrade]
    commands = [
        "echo 'upgrading!'"
    ]

    [post_upgrade]
    commands = [
        "curl https://mycompany.com/some-webhook",
        "echo 'upgraded!'"
    ]

    [[plugins]]
    index_file = "wp-content/plugins/my-plugin/my-plugin.php"
    package_name = "mycompany/myplugin"
    remote_repository = "git@github.com:mycompany/myplugin-mirror.git"

    [[plugins]]
    index_file = "wp-content/plugins/advanced-custom-fields-pro/acf.php"
    package_name = "mycompany/acf-pro"
    remote_repository = "git@git.mycompany.com:/opt/repositories/acfpro-mirror.git"

### Configuration values

#### `binaries`

In this section you define various binaries that are to be used when running
operations with the tool.

##### `git`

Which Git executable to use. Can be a `$PATH` accessible binary.

##### `wpcli`

Which WP-CLI executable to use. Can be a `$PATH` accessible binary.

#### `git`

Git configuration.

##### `user_name`

Git user name to use for authoring WPPR commits.

##### `user_email`

Git user email address to use for authoring WPPR commits.

##### `force_push`

Whether to use `force` when pushing plugin changes to remote repositories.

#### `pre_upgrade` and `post_upgrade`

`commands` contains a list of shell commands to run before and after
running the `run` command. Only shell (e.g. Bash) commands are supported.

#### `plugins`

A collection of plugins to manage with WPPR. You can have as many `[[plugins]]`
sections as you need. One is OK too.

##### `index_file`

Relative path to the plugin's "index file", which contains the plugin file
header with name, version, description, etc.

##### `package_name`

When creating a new Composer package configuration for the plugin, this will
be the Composer package name, and to install the mirrored plugin you would use
this value as the `require` command value.

##### `remote_repository`

Git-compatible URL to a remote repository to which changes in plugins are
pushed. Your system and user needs to have push rights to this repository.

## Usage

Assuming you have a WordPress installation up and running and you have created
a `wppr.toml` configuration file you can now start using WPPR.

You can display a generic help message with:

    $ wppr --help

### Listing plugins WPPR is managing

    $ wppr --configuration /path/to/wppr.toml list

The `list` command lists all the plugins defined in `wppr.toml`. It also tells
you whether the defined plugins are valid for management or not.

### Run upgrades, git tags, and git pushes

    $ wppr --configuration /path/to/wppr.toml run

The `run` command does the real work in this tool.

1.  It first validates the plugins
2.  It initializes new plugins that have not been managed before
3.  It creates backups of plugin history and state
4.  It runs plugins upgrades using WP-CLI
5.  It checks if anything has changed (files, versions, etc.)
6.  If there are changes it commits the changes to the plugin Git history and
    creates a new tag with the value of the plugin's index file's `Version`
7.  Lastly it pushes the new changes and the Git tag to the configured remote
    repository.

Step 3 creates a backup, and in case any of the steps after that fail the backup
is restored to prevent malformed plugins from appearing into your repositories
later on.

## Automation with cron triggers

You can make the plugin "Composerization" automatic with cronjobs.

You can edit your cron configuration with

    $ crontab -e

To run the tool every hour create the following cronjob:

    0 0 * * * wppr --configuration /path/to/wppr.toml run > /dev/null 2>&1

You can also redirect the cronjob output to a script in case you want to work
with it after each run (e.g. send notifications or something):

    0 0 * * * wppr --configuration /path/to/wppr.toml run > /path/to/script.py 2>&1

Or if you prefer to just dump the output into a file:

    0 0 * * * wppr --configuration /path/to/wppr.toml run > /home/user/wppr.out 2>&1

With the cron definitions above you should receive automated updates into your
plugin mirror repositories every hour if there are upgrades available.

## Q&A

#### Does this work with themes as well?

No. Only plugins are supported at the moment.

#### It does not work with plugin _`plugin name here`_ for some reason!

Some plugins have really twisted upgrade procedures that bypass the regular
WordPress update procedures. In those cases either the plugin must be managed
in some other way or then we may be able to introduce a hook to let the plugin
be managed.

Create an issue for the plugin and we can see what to do if anything can be
done. Make sure to search first in case someone else has already raised an issue
about the plugin.

## Contributing

Pull requests, code reviews, issues, ideas, and other support very welcome.

When contributing code please follow these guideslines:

-   Maintainers with push rights should follow [_OneFlow_][oneflow], no pushing
    feature branches unless absolutely necessary
-   Pull requests should be made against the `master` branch
-   Format your code with `cargo fmt` (check changes to be made with
    `cargo fmt -- --write-mode=diff`)
-   Run tests with `cargo test`
-   Write tests for new features and changes that you implement, you can check
    coverage using the steps below

[oneflow]: http://endoflineblog.com/oneflow-a-git-branching-model-and-workflow

### Code coverage

(This section is a bit verbose as coverage testing Rust seems to be a relatively
new topic.)

In addition to `cargo test` this application is code coverage tested using a
nice little program called [`kcov`][kcov].

Provided in the project root is a script called `coverage.sh` which helps with
coverage reporting.

To generate coverage reports you first need to install `kcov`, and on
Debian-like Linux systems you can use

    $ ./coverage.sh install

to install it semi-automatically. Pull requests welcome to make it work on other
systems as well.

To generate a coverage report you need to follow these steps:

1.  Build the application using `cargo build`
2.  Test the code with `RUSTFLAGS='-C link-dead-code' cargo test` (the flag
    addition ensures we get 0% reporting for code which was not run at all)
3.  Generate coverage reports with `./coverage.sh run`

All code inside `./src` (apart from `main.rs`) is coverage reported. You can see
a neat HTML file for your report by opening the `./target/cov/merged/index.html`
file in a browser.

**NOTE**: doc-tests cannot be covered with kcov at this time. We need to rely
on the hope that doc-tests actually test the code properly.

[kcov]: https://github.com/SimonKagstrom/kcov

## License

WPPR is licensed with Apache-2.0. See [LICENSE.md][license].

[license]: ./LICENSE.md