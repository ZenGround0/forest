// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::handle_rpc_err;
use cid::Cid;
use rpc_client::mpool_ops;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum MpoolCommands {
    #[structopt(help = "Retrieve pending messages in mempool")]
    Pending {
        #[structopt(short, help = "a valid CID")]
        cid: String,
    },
}

impl MpoolCommands {
    pub async fn run(&self) {
        match self {
            Self::Pending { cid } => {
                let cid: Cid = cid.parse().unwrap();
                let messages = mpool_ops::pending(cid)
                    .await
                    .map_err(handle_rpc_err)
                    .unwrap();
                println!("{:#?}", messages);
            }
        }
    }
}
