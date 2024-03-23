// SPDX-FileCopyrightText: 2024 jade lovelace
//
// SPDX-License-Identifier: MPL-2.0

//! Makes a funny branch that has a merge commit per hydra build, and
use std::path::PathBuf;

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
    advance_ts: i64,
    hydra_job: String,
    hydra_build_id: String,
}

#[derive(clap::Parser)]
struct Arghs {
    /// Input file of csv of sha,advance_ts,hydra_job,hydra_build_id format.
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

    let reader = csv::ReaderBuilder::new().from_path(arghs.input_file)?;
    let mut it = reader.into_deserialize::<Record>();

    let name = "necktangler";
    let email = "necktangler-bot@nixos.org";

    let base_commit = it.next().unwrap()?;
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
        let l = l?;
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
            sig(Time::new(l.advance_ts, 0)),
            sig(updated_to.committer()?.time),
            // for very silly reasons, FullName is not accepted as a valid
            // input for commit_as, due to the *infallibility* of the try_from
            // operator. incredible.
            arghs.refname.clone(),
            format!(
                "Hydra build {} of job {}\n\nLink: https://hydra.nixos.org/build/{}",
                l.hydra_build_id, l.hydra_job, l.hydra_build_id
            ),
            updated_to.tree_id()?,
            [o.id(), updated_to.id()],
        )?;
        Ok(id.object()?)
    });
    let tip = tip?;

    println!("ref {ref_name} updated to {tip:?}");

    Ok(())
}
