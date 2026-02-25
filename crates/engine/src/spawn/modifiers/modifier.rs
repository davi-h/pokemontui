use crate::spawn::{context::SpawnContext, table::SpawnTable};

pub trait SpawnModifier {
    /// Quanto menor o número, mais cedo executa
    fn priority(&self) -> i32 {
        0
    }

    /// Aplica transformação na tabela
    fn modify(&self, ctx: &SpawnContext, table: &SpawnTable) -> SpawnTable;
}