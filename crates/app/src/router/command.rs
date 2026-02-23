use super::context::Context;
use super::error::RouterError;

pub trait Command {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, ctx: &mut Context) -> Result<(), RouterError>;
}