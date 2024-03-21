// SPDX-FileCopyrightText: 2024 jade lovelace
//
// SPDX-License-Identifier: MPL-2.0

//! Makes a funny branch that has a merge commit per hydra build, and
use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use gix::{
    actor::SignatureRef,
    bstr::BStr,
    date::Time,
    refs::{
        transaction::{Change, RefEdit, RefLog},
        FullName,
    },
    ObjectId,
};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Record {
    sha: String,
    commit_date: i64,
    advance_date: i64,
    // FIXME
    // hydra_job: String,
    // FIXME:
    // hydra_build_id: String,
}

impl FromStr for Record {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mkerror = || Into::<Self::Err>::into("Missing line element");
        let (sha, rest) = s.split_once(' ').ok_or_else(mkerror)?;
        let (commit_date, rest) = rest.split_once(' ').ok_or_else(mkerror)?;
        let commit_date = i64::from_str(commit_date)?;
        let advance_date = rest;
        let advance_date = i64::from_str(advance_date)?;
        Ok(Record {
            sha: sha.to_string(),
            commit_date,
            advance_date,
        })
    }
}

#[derive(clap::Parser)]
struct Arghs {
    /// Input file of channels.nix.gsc.io format:
    /// <https://channels.nix.gsc.io/nixos-unstable/README.txt>
    ///
    /// Expected to be sorted from oldest to newest, and no check is performed
    /// on this (FIXME).
    #[clap(long, short = 'i')]
    input_file: PathBuf,

    /// Full reference name, e.g. refs/heads/meow. This will be wiped and the
    /// contents regenerated. It should at least be the same result every time
    /// with the same input.
    #[clap(long, short = 'b')]
    refname: String,

    /// Path to a checkout of nixpkgs. This program works entirely in memory so
    /// only the specified reference will be tampered with.
    #[clap(long, short = 'C')]
    nixpkgs_checkout: PathBuf,
}

fn main() -> Result<(), Error> {
    let arghs = Arghs::parse();

    let lines = std::fs::read_to_string(arghs.input_file)?
        .lines()
        .map(|l| Record::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;

    let name = "necktangler";
    let email = "necktangler-bot@nixos.org";

    let mut it = lines.iter();
    let base_commit = it.next().unwrap();
    let repo = gix::open(arghs.nixpkgs_checkout)?;
    let obj = repo.find_object(ObjectId::from_hex(base_commit.sha.as_bytes())?)?;

    let ref_name: FullName = arghs.refname.clone().try_into()?;

    repo.edit_reference(RefEdit {
        change: Change::Delete {
            expected: gix::refs::transaction::PreviousValue::Any,
            log: RefLog::AndReference,
        },
        name: ref_name.clone(),
        deref: false,
    })?;

    let tip: Result<_, Error> = it.try_fold(obj, |o, l| {
        let o = o.into_commit();
        let sig = |time| SignatureRef {
            name: BStr::new(name.as_bytes()),
            email: BStr::new(email.as_bytes()),
            time,
        };
        let updated_to = repo
            .find_object(ObjectId::from_hex(l.sha.as_bytes()).unwrap())?
            .into_commit();
        let id = repo.commit_as(
            sig(Time::new(l.advance_date, 0)),
            sig(Time::new(l.commit_date, 0)),
            // for very silly reasons, FullName is not accepted as a valid
            // input for commit_as, due to the *infallibility* of the try_from
            // operator. incredible.
            arghs.refname.clone(),
            "Hydwa update",
            updated_to.tree_id()?,
            [o.id(), updated_to.id()],
        )?;
        Ok(id.object()?)
    });
    let tip = tip?;

    println!("ref {ref_name} updated to {tip:?}");

    Ok(())
}
