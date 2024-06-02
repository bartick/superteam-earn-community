pub struct EarnUrl {}

impl EarnUrl {
    const URL: &'static str = "https://earn.superteam.fun/api/listings/";
    const BOUNTY: &'static str = "bounty";
    const PROJECT: &'static str = "project";
    pub fn project_url(now: &str) -> String {
        format!("{}?category=bounties&type={}&deadline={}", Self::URL, Self::PROJECT, now)
    }
    pub fn bounty_url(now: &str) -> String {
        format!("{}?category=bounties&type={}&deadline={}", Self::URL, Self::BOUNTY, now)
    }
}