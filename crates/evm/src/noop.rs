//! A no operation block executor implementation.

use reth_interfaces::{executor::BlockExecutionError, provider::ProviderError};
use reth_primitives::{BlockNumber, BlockWithSenders, PruneModes, Receipt};
use revm_primitives::db::Database;

use crate::execute::{
    BatchBlockExecutionOutput, BatchExecutor, BlockExecutionInput, BlockExecutionOutput,
    BlockExecutorProvider, Executor,
};

/// A [BlockExecutorProvider] implementation that does nothing.
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct NoopBlockExecutorProvider;

impl BlockExecutorProvider for NoopBlockExecutorProvider {
    type Executor<DB: Database<Error = ProviderError>> = Self;

    type BatchExecutor<DB: Database<Error = ProviderError>> = Self;

    fn executor<DB>(&self, _: DB) -> Self::Executor<DB>
    where
        DB: Database<Error = ProviderError>,
    {
        Self
    }

    fn batch_executor<DB>(&self, _: DB, _: PruneModes) -> Self::BatchExecutor<DB>
    where
        DB: Database<Error = ProviderError>,
    {
        Self
    }
}

impl<DB> Executor<DB> for NoopBlockExecutorProvider {
    type Input<'a> = BlockExecutionInput<'a, BlockWithSenders>;
    type Output = BlockExecutionOutput<Receipt>;
    type Error = BlockExecutionError;

    fn execute(self, _: Self::Input<'_>) -> Result<Self::Output, Self::Error> {
        Err(BlockExecutionError::UnavailableForNoop)
    }
}

impl<DB> BatchExecutor<DB> for NoopBlockExecutorProvider {
    type Input<'a> = BlockExecutionInput<'a, BlockWithSenders>;
    type Output = BatchBlockExecutionOutput;
    type Error = BlockExecutionError;

    fn execute_one(&mut self, _: Self::Input<'_>) -> Result<(), Self::Error> {
        Err(BlockExecutionError::UnavailableForNoop)
    }

    fn finalize(self) -> Self::Output {
        unreachable!()
    }

    fn set_tip(&mut self, _: BlockNumber) {}

    fn size_hint(&self) -> Option<usize> {
        None
    }
}