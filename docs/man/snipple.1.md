---
title: snipple
section: 1
footer: snipple VERSION_PLACEHOLDER
header: snipple User's Manual
author: Daniel Schauenberg <d@unwiredcouch.com>
date: DATE_PLACEHOLDER
---

<!-- This is the sniple(1) man page, written in Markdown. -->
<!-- To generate the roff version, run `make man` -->

# NAME

snipple — a simple snippet manager


# SYNOPSIS

`snipple list [--alfred]`

`snipple get <snippet>`



# EXAMPLES

`snipple list`
: Lists all snippets in the snippet directory.

`snipple get rust/test.snippet`
: Prints the `rust/test.snippet` snippet from the snippet directory.


# DESCRIPTION

`snipple` is a command line utility to manage reusable snippets of text. The
core idea is that snippets should just be text files in a directory structure
that can be managed within a filesystem. This enables a flexible way of
interacting and managing snippets via e.g. syncing with cloud services or
tools like `rsync`, or tracking changes with version control tooling like
`git`.

# CONFIGURATION

In order to support configuration of behaviour `snipple` looks for
configuration in the locations `~/.config/snipple/config.yaml` and
`~/.snipple.yaml`. The config file can contain the following properties:

**base_dir:**
: the base location to look for snippets in

**snippet_suffix:**
: the suffix that is being used for snippet files


# META OPTIONS AND COMMANDS

`--help`
: Show list of command-line options.

`version`
: Show version of snipple.



# AUTHOR

snipple is maintained by mrtazz.

**Source code:** `https://github.com/mrtazz/snipple`

# REPORTING BUGS

- https://github.com/mrtazz/snipple/issues

# SEE ALSO

- https://github.com/mrtazz/vim-snipple
