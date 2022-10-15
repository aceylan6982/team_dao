use anchor_lang::prelude::*;
use anchor_spl::token::{self, Airdrop};

declare_id!("A2Cv3qRFTSS8MQztxX6ZxuRwVEPAzpwDeuhf2wwHKS6Q");

#[program]
pub mod tiamo {
    use super::*;

    // Lets create new team...
    pub fn create_team(ctx: Context<CreateTeam>, name:String, team_count:u32 ) -> Result<()> {
        let team : &mut Account<Team> = &mut ctx.accounts.team_account;
        team.bump= *ctx.bumps.get("team_account").unwrap();
        team.name=name;
        team.team_count=team_count;
        team.member= Vec::new();
        team.invited_member= Vec::new();
        team.autority=ctx.accounts.signer.key();
        team.pool=pool;

        team.member.push(ctx.accounts.signer.key());

        msg!("Team {} successfully created. Owner of the team: {} ", team.name, team.autority);

        Ok(())
    }
    // Hey Do you have any idea? Here is your key...
    pub fn create_proposal (ctx:Context<CreateProposal>, title:String,description:String)-> Result<()> {
        let proposal : &mut Account<Proposal> = &mut ctx.accounts.proposal_account;
        proposal.bump= *ctx.bumps.get("team_account").unwrap();
        proposal.title=title;
        proposal.description=description;
        proposal.vote=vote;
        proposal.vote_yes= 0;
        proposal.vote_no= 0;

         // corespoing vote count base on `vote`
        if vote {
            proposal.vote_yes += 1;
            proposal_account.vote.add(proposal.vote_yes);
        } else {
            proposal.vote_no += 1;
            proposal_account.vote.add(proposal.vote_no);
        }
        // Proposal is voting by team members one conditions yes > no should be bigger 3/2...
        if proposal.vote_yes > proposal.vote_no {
            msg!("This proposal is accepted by team voting")
        }
        msg!("Proposal { } successfully created. Owner of the proposal: { } ", proposal.title,ctx.accounts.signer.key());

        Ok(())
    }
    // I am inviting the new teammate. Where are you buddy??
    pub fn invite_member (ctx:Context<InviteMember>, invited_member:Pubkey)-> Result<()> {
        let team_account : &mut Account<Team> = &mut ctx.accounts.team_count;
        if team_account.invited_member.len() >=team_account.team_count.try_into().unwrap(){
            return err!(ErrorCode::TeamFullCount);
        }
        if team_account.member.contains (&invited_member){
            return err!(ErrorCode::PlayerAlreadyExists);
        }
        team.account.invited_member.push(invited_member);

        msg!(" Player { } invited to the team { }.", ctx.accounts.signer.key(), team_account.name);

        Ok(())
    }

    // Yep here is the new team, welcome to new family :)
    pub fn join_team (ctx:Context<JaoinTeam>)-> Result<()> {
        let team_account : &mut Account<Team> = &mut ctx.accounts.team_count;
        if team_account.member.len() >=team_account.team_count.try_into().unwrap(){
            return err!(ErrorCode::TeamFullCount);
        }
        if !team_account.invited_member.contains (&ctx.accounts.signer.key()){
            return err!(ErrorCode::MemberIsNotDemanded);
        }
        team.account.member.push(ctx.accounts.signer.key());

        msg!(" Player { } joined to the { } team",  ctx.accounts.signer.key() ,team_account.name);

        Ok(())
    }
    // If you wanna leave from team, here is the function...
    pub fn leave_team (ctx:Context<LeaveTeam>)-> Result<()> {
        let team_account : &mut Account<Team> = &mut ctx.accounts.team_count;
        if team_account.member.len() >=1 {
            return err!(ErrorCode::NotAllowedLeave);
        }
        if team_account.member.contains (&ctx.accounts.signer.key()){
            return err!(ErrorCode::CaptainNotLeaveThis);
        }
        team.account.member.remove(ctx.accounts.signer.key());

        msg!(" Player { } removed to the { } team",  ctx.accounts.signer.key() ,team_account.name);

        Ok(())
    }
    // Transfer the autority
    pub fn transfer_autority (ctx:Context<TransferAutority>, transfer_autority:Pubkey) -> Result <()>{
        let new_captain : &mut Account<Team> = &ctx.accounts.team_account;
        new_captain = ctx.accounts.signer.key();

        team.account.autority.add(ctx.accounts.signer.key());

        msg!(" Welcome to new Captain { } .",  ctx.accounts.signer.key());

        Ok(())
    }
    //Prizes divided 5 piecess..
    pub fn  share_prize (ctx:Context<SharePrize>, share_prize: U64, team_account:Pubkey){
        let  money : &mut Account<Team> = &mut ctx.accounts.team_account;
        money = solana airdrop u32;
        
        let team_member_account : u64 = money % 5; // %20 sharing money every team member...

        team_account.pool.push(ctx.accounts.signer.key())
        msg! ("Prizes { } share %20 { } to every single member." . , money, team_member_account)

        ok (())
    }

}

// ------ Instruction Accounts ------

#[derive(Accounts)]
#[instruction (name:String)]
pub struct CreateTeam {
    #[account(
        init, payer=signer, space=1000, seeds=["team_count".as_bytes(),name.as_bytes()],bump
    )]

    pub team_account : Account <'info, Team>,
    #[account (mut)]
    pub signer : Signer <'info>,
    pub system_program : Program <'info, System>,
}
#[derive(Accounts)]
#[instruction (title:String)]
pub struct CreateProposal <'info> {
    #[account( mut, seeds = ["team_account".as_bytes(),team_account.name.as_bytes()],bump)]

    pub team_account : Account <'info, Team>,
    pub proposal: Account<'info, Proposal>,
    pub vote: Account<'info, Vote>,
    #[account (
        init, payer=signer, space=1000, seeds=["proposal_count".as_bytes(),name.as_bytes()],bump,
        constraint=team_account.team_autority == signer.key()
    )]
    #[account]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
   
}
#[derive(Accounts)]
#[instruction (invite_member:Pubkey)]
pub struct InviteMember <'info> {
    #[account( mut, seeds = ["team_account".as_bytes(),team_account.name.as_bytes()],bump)]

    pub team_account : Account <'info, Team>,
    #[account (
        init, payer=signer, space=1000, seeds=["invite_team".as_bytes(),name.as_bytes()],bump,
        constraint=team_account.team_autority == signer.key()
    )]
    #[account]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]

pub struct JaoinTeam <'info> {
    #[account( mut, seeds = ["team_account".as_bytes(),team_account.name.as_bytes()],bump)]

    pub team_account : Account <'info, Team>,
    #[account (
        init, payer=signer, space=1000, seeds=["join_team".as_bytes(),name.as_bytes()],bump,
        constraint=team_account.team_autority == signer.key()
    )]
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]

pub struct LeaveTeam <'info> {
    #[account( mut, seeds = ["team_account".as_bytes(),team_account.name.as_bytes()],bump)]

    pub team_account : Account <'info, Team>,
    #[account (
        init, payer=signer, space=1000, seeds=["leave_team".as_bytes(),name.as_bytes()],bump,
        constraint=team_account.team_autority == signer.key()
    )]
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction (transfer_autority: Pubkey)]
pub struct TransferAutority <'info> {
    #[account( mut, seeds = ["team_account".as_bytes(),team_account.name.as_bytes()],bump)]

    pub team_account : Account <'info, Team>,
    pub new_captain : Account <'info, Team>,
    #[account (
        init, payer=signer, space=1000, seeds=["transfer_autority".as_bytes(),name.as_bytes()],bump,
        constraint=team_account.team_autority == signer.key();
    )]
    #[account]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction (share_prize: U64, team_account:Pubkey)]
pub struct SharePrize <'info> {
    #[account( mut, seeds = ["team_account".as_bytes(),team_account.name.as_bytes()],bump)]

    pub team_account : Account <'info, Team>,
    #[account (
        init, payer=signer, space=1000, seeds=["share_prize".as_bytes(),name.as_bytes()],bump,
        constraint=team_account.team_autority == signer.key()
    )]
    #[account(mut)]
    pub money : Money <'info>,
    pub team_member_account : TeamMemberAccount <'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// ------ Account States (Data) ------


#[account]
pub struct Team {
    pub name : String,
    pub team_count : u32,
    pub member : Vec <Pubkey>,
    pub autority : Pubkey,
    pub bump : u8,
    pub invited_member : Vec <Pubkey>,
    pub pool : u64,
}
impl Team {
    const LEN : usize =8 // disciramantor
    + 32 //name
    + 5 // team member=5 person
    + 5 * 32 // team members capacity
    + 32 //autority pubkey
    + 1 // bump
    + 5 * 32 // invited member capacity
    + 1 ;//pool of team
}
#[account]
pub struct Proposal {
    pub title : String,
    pub description : String,
    pub autority : Pubkey,
    pub vote : u64,
    pub vote_yes : u64,
    pub bump : u8,
    pub vote_no : u64
}
#[error_code]
pub enum ErrorCode {
    #[msg("Team capacity is full.")]
    TeamFullCount,
    #[msg("Player already in the team.")]
    PlayerAlreadyExists,
    #[msh("Team is full now")]
    TeamFullCount,
    #[msg("Player is not in the invited list.")]
    MemberIsNotDemanded,
    #[msg("Team has one member. Where are you going?")]
    NotAllowedLeave,
    #[msg("Captain doesnt leave by this way")]
    CaptainNotLeaveThis
}

