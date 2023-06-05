use anchor_lang::prelude::*;

#[account]
pub struct Box {
    pub owner: Pubkey,
    pub boxname: String,
    pub odd1: f32,
    pub amount1: u64,
    pub odd2: f32,
    pub amount2: u64,
    pub odd3: f32,
    pub amount3: u64,
    pub odd4: f32,
    pub amount4: u64,
    pub rolled_times: u8,
    pub bank: u64,
    pub box_bump: u8,
}

impl Box {
    pub fn space() -> usize {       // space = 
        8 +     //  Discriminator
        32 +    //  Owner (Pubkey)
        4 +     //  Box Name (String)
        4 +     //  Odd 1 (f32)
        8 +     //  Amount 1 (u64)
        4 +     //  Odd 2 (f32)
        8 +     //  Amount 2 (u64)
        4 +     //  Odd 3 (f32)
        8 +     //  Amount 3 (u64)
        4 +     //  Odd 4 (f32)
        8 +     //  Amount 4 (u64)
        1 +     //  Rolled Times (u8)
        8 +     //  Bank (u64)
        1       //  Box Bump (u8)
    }
}

#[account]
pub struct BoxInvoice {
    pub roll: u8,
    pub roller: Pubkey,
    pub box_id: Pubkey,
    pub odd_won: f32,
    pub amount_won: u64,
}

impl BoxInvoice {
    pub fn space() -> usize {       // space = 
        8 +     //  Discriminator
        1 +     //  Roll (u8)
        32 +    //  Roller (Pubkey)
        32 +    //  Box ID (Pubkey)
        4 +     //  Odd Won (f32)
        8       //  Amount Won (u64)

    }
}