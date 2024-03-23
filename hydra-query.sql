-- SPDX-FileCopyrightText: 2024 jade lovelace
--
-- SPDX-License-Identifier: CC0-1.0

select
    --concat(j.project, ':', j.name, ':', b.job) as ref, b.job, b.id, b.finished, b.timestamp, to_timestamp(b.timestamp) at time zone 'utc' as ts_clean, b.jobset_id, j.name, jsei.name, jsei.revision, jsei.uri, jsei.name
    jsei.revision as sha, b.timestamp as advance_ts, concat(j.project, ':', j.name, ':', b.job) as hydra_job, b.id as hydra_build_id
    from jobsets j
        inner join builds_old b on j.id = b.jobset_id
        inner join jobsetevalmembers jsem on jsem.build = b.id
        inner join jobsetevals jse on jsem.eval = jse.id
        inner join jobsetevalinputs jsei on jsei.eval = jse.id
    where
    b.job = :'job' and
    j.project = :'project' and
    j.name = :'jobset' and
    jsei.name = 'nixpkgs' and
    b.finished = 1 and
    -- arbitraily chosen start in time; there are some bogus commit IDs from
    -- 2014 or so, which we would rather not deal with
    b.timestamp > extract(epoch from '2018-01-01T00:00:00Z'::timestamptz)
    order by b.timestamp asc
