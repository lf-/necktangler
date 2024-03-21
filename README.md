<!--
SPDX-FileCopyrightText: 2024 jade lovelace

SPDX-License-Identifier: MPL-2.0
-->

# necktangler

some bad software that makes a merge-commit structure based on channel updates
such that you can just bisect nixpkgs natively with `git bisect --first-parent`
and call it a day (!)

this should be able to be used to replace
[hydrasect](https://git.qyliss.net/hydrasect/tree/) once we have a data source.

![screenshot of gitk showing some absolutely unhinged branch structure, but
there is a lineage of merge commits on the left side](./screenshot.png)

## howmst the heck

this tool generates one merge commit per hydra bump, in oldest to newest order
in the file. it makes no attempt to sort the file, though that would be a good
idea. we did not have time to do so.

## todos

* support a mode to incrementally add more commits by scanning the input for
  the parent commit at the tip of the branch and then continuing from that
  point. we mean its idempotent so who cares, but it might get slow for very
  long histories.
* acquire a data source. although this was developed against
  https://channels.nix.gsc.io for lack of motivation to solve this problem,
  that Web site does not have any data for the past year.

  solution: some hydra db query.
* make her faster (read: instant rather than a few seconds). probably could use
  the commit-graph, as we *bet* that we are hitting absolutely pathological git
  database design cases. there is gitoxide support for the commit-graph, we
  just didn't bother using it.
