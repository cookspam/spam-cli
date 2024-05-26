use std::str::FromStr;

use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signer;

use crate::Miner;

impl Miner {
    pub async fn update_admin(&self, new_admin: String) {
        let signer = self.signer();
        let new_admin = Pubkey::from_str(new_admin.as_str()).unwrap();
        let ix = spam::instruction::update_admin(signer.pubkey(), new_admin);
        self.send_and_confirm(&[ix], false, false)
            .await
            .expect("Transaction failed");
    }
}
