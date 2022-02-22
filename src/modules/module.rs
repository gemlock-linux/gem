use async_trait::async_trait;

#[async_trait]
pub trait Module {
    fn command(&self) -> clap::Command;
    async fn entry(&self, clap: &clap::ArgMatches);
}
