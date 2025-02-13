// Copyright © 2021 The Radicle Link Contributors
//
// This file is part of radicle-link, distributed under the GPLv3 with Radicle
// Linking Exception. For full terms see the included LICENSE file.

use thiserror::Error;

use librad::{
    git::{storage::Storage, tracking, Urn},
    paths::Paths,
    PeerId,
};

use crate::git::include;

#[derive(Debug, Error)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[error(transparent)]
    Include(#[from] include::Error),

    #[error(transparent)]
    Tracking(#[from] tracking::Error),
}

pub fn track(storage: &Storage, paths: &Paths, urn: &Urn, peer: PeerId) -> Result<(), Error> {
    let _tracked = tracking::track(storage, urn, peer)?;
    include::update(storage, paths, urn)?;
    Ok(())
}

pub fn untrack(storage: &Storage, paths: &Paths, urn: &Urn, peer: PeerId) -> Result<(), Error> {
    let _untracked = tracking::untrack(storage, urn, peer)?;
    include::update(storage, paths, urn)?;
    Ok(())
}
