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
in the file, with identical trees to the corresponding commit in nixpkgs. it
makes no attempt to sort the input file, though that would be a good idea. we
did not have time to do so.

because of this structure, any git tooling can just follow the first-parent
relations of all the commits to get only commits with hydra builds.

how to run:

```
$ cargo r -- -i ./data.csv -b refs/heads/meow -C ./nixpkgs
```

you would then have a branch "meow" that has a bunch of hydra update merge
commits from which you can bisect.

## todos

* support a mode to incrementally add more commits by scanning the input for
  the parent commit at the tip of the branch and then continuing from that
  point. we mean its idempotent so who cares, but it might get slow for very
  long histories.
* acquire a data source.

  we have a query for the production hydra database, and some stale test data
  committed to the repo, but it needs to get shoved in a cron job.

  the query as it stands was run like so:
  `psql $SOME_CONNECTION_DETAILS -v job=tested -v project=nixos -v jobset=trunk-combined --csv -f hydra-query.sql > data.csv`
* make her faster (read: instant rather than a few seconds). probably could use
  the commit-graph, as we *bet* that we are hitting absolutely pathological git
  database design cases. there is gitoxide support for the commit-graph, we
  just didn't bother using it. but tbh we don't care.
