i3spin
======

i3spin is a small utility for i3 window manager that, when run, obtains a list
of windows open on currently focused workspace and switches focus to previous
or next window.

It is intended to replicate Alt+Tab-like behaviour from rich desktop
environments, including but not limited to Microsoft Windows.

There are two commands provided, `i3spin_fwd` and `i3spin_rwd`. The first
switches to the next window and the latter to the previous window. This split
represents the author's desire to avoid pulling in dependency-heavy crates like
`clap` for commandline flag parsing.

The order of windows is taken directly from the data fetched from i3 through a
depth-first search performed on the container tree. This means that if the
workspace is first split into 3 rows and then the second row is split
horizontally into 2 columns, then the order of windows should be as follows:
first row, second row first column, second row second column, third row.
