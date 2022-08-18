use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn sendTweet(ctx: Context<SendTweet>, topic: String, content: String) -> ProgramResult {
        let tweet: &mut Account<'info, Tweet> = ctx.accounts.tweet;
        let author: &Signer = ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if topic.chars().count() > 50{
            return Err(ErrorCode::TopicTooLong.into());
        }

        if content.chars().count() > 280{
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet.author = *author.key;
        tweet.topic = topic;
        tweet.content = content;
        tweet.timestamp = clock.unix_timestamp;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode{
    #[msg("Provided topic is too long. Make sure it has <= 50 characters.")]
    TopicTooLong,
    #[msg("Provided content is too long. Make sure it has <= 280 characters.")]
    ContentTooLong,
}

#[derive(Accounts)]
pub struct SendTweet<'info>{
    #[account(init, payer = author, space = Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct Tweet{
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_PREFIX_LENGTH: usize = 4;
const MAX_TITLE_LENGTH: usize = 50 * 4;
const MAX_CONTENT_LENGTH: usize = 280 * 4;

impl Tweet{
    const LEN: usize = DISCRIMINATOR_LENGTH 
    + PUBLIC_KEY_LENGTH
    + TIMESTAMP_LENGTH
    + STRING_PREFIX_LENGTH
    + MAX_TITLE_LENGTH
    + MAX_TITLE_LENGTH;
}


