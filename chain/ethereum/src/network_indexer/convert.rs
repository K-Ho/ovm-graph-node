use graph::prelude::*;

use super::*;

impl ToEntityId for Ommer {
    fn to_entity_id(&self) -> String {
        format!("{:x}", self.0.hash.unwrap())
    }
}

impl ToEntityKey for Ommer {
    fn to_entity_key(&self, subgraph_id: SubgraphDeploymentId) -> EntityKey {
        EntityKey {
            subgraph_id,
            entity_type: "Block".into(),
            entity_id: format!("{:x}", self.0.hash.unwrap()),
        }
    }
}

impl ToEntityId for BlockWithOmmers {
    fn to_entity_id(&self) -> String {
        (*self).block.block.hash.unwrap().to_entity_id()
    }
}

impl ToEntityKey for &BlockWithOmmers {
    fn to_entity_key(&self, subgraph_id: SubgraphDeploymentId) -> EntityKey {
        EntityKey {
            subgraph_id,
            entity_type: "Block".into(),
            entity_id: format!("{:x}", (*self).block.block.hash.unwrap()),
        }
    }
}

impl ToEntityId for Transaction {
    fn to_entity_id(&self) -> String {
        format!("{:x}", self.0.hash)
    }
}

impl ToEntityKey for &Transaction {
    fn to_entity_key(&self, subgraph_id: SubgraphDeploymentId) -> EntityKey {
        EntityKey {
            subgraph_id,
            entity_type: "Transaction".into(),
            entity_id: format!("{:x}", self.0.hash),
        }
    }
}

impl TryIntoEntity for Ommer {
    fn try_into_entity(self) -> Result<Entity, Error> {
        let inner = &self.0;

        Ok(Entity::from(vec![
            ("id", format!("{:x}", inner.hash.unwrap()).into()),
            ("number", inner.number.unwrap().into()),
            ("hash", inner.hash.unwrap().into()),
            ("parent", inner.parent_hash.to_entity_id().into()),
            (
                "nonce",
                inner.nonce.map_or(Value::Null, |nonce| nonce.into()),
            ),
            ("transactionsRoot", inner.transactions_root.into()),
            ("transactionCount", (inner.transactions.len() as i32).into()),
            ("stateRoot", inner.state_root.into()),
            ("receiptsRoot", inner.receipts_root.into()),
            ("extraData", inner.extra_data.clone().into()),
            ("gasLimit", inner.gas_limit.into()),
            ("gasUsed", inner.gas_used.into()),
            ("timestamp", inner.timestamp.into()),
            ("logsBloom", inner.logs_bloom.into()),
            ("mixHash", inner.mix_hash.into()),
            ("difficulty", inner.difficulty.into()),
            ("totalDifficulty", inner.total_difficulty.into()),
            ("isOmmer", true.into()),
            ("ommerCount", (inner.uncles.len() as i32).into()),
            (
                "ommers",
                inner
                    .uncles
                    .iter()
                    .map(|hash| hash.to_entity_id())
                    .collect::<Vec<_>>()
                    .into(),
            ),
            ("ommerHash", inner.uncles_hash.into()),
            (
                "transactions",
                (inner
                    .transactions
                    .iter()
                    .map(move |transaction| transaction.hash.to_entity_id())
                    .collect::<Vec<_>>()
                    .into()),
            ),
            ("size", inner.size.into()),
            ("sealFields", inner.seal_fields.clone().into()),
        ] as Vec<(_, Value)>))
    }
}

impl TryIntoEntity for &BlockWithOmmers {
    fn try_into_entity(self) -> Result<Entity, Error> {
        let inner = self.inner();

        Ok(Entity::from(vec![
            ("id", format!("{:x}", inner.hash.unwrap()).into()),
            ("number", inner.number.unwrap().into()),
            ("hash", inner.hash.unwrap().into()),
            ("parent", inner.parent_hash.to_entity_id().into()),
            (
                "nonce",
                inner.nonce.map_or(Value::Null, |nonce| nonce.into()),
            ),
            ("transactionsRoot", inner.transactions_root.into()),
            ("transactionCount", (inner.transactions.len() as i32).into()),
            (
                "transactions",
                (inner
                    .transactions
                    .iter()
                    .map(move |transaction| transaction.hash.to_entity_id())
                    .collect::<Vec<_>>()
                    .into()),
            ),
            ("stateRoot", inner.state_root.into()),
            ("receiptsRoot", inner.receipts_root.into()),
            ("extraData", inner.extra_data.clone().into()),
            ("gasLimit", inner.gas_limit.into()),
            ("gasUsed", inner.gas_used.into()),
            ("timestamp", inner.timestamp.into()),
            ("logsBloom", inner.logs_bloom.into()),
            ("mixHash", inner.mix_hash.into()),
            ("difficulty", inner.difficulty.into()),
            ("totalDifficulty", inner.total_difficulty.into()),
            ("isOmmer", false.into()),
            ("ommerCount", (self.ommers.len() as i32).into()),
            (
                "ommers",
                self.inner()
                    .uncles
                    .iter()
                    .map(|hash| hash.to_entity_id())
                    .collect::<Vec<_>>()
                    .into(),
            ),
            ("ommerHash", inner.uncles_hash.into()),
            (
                "transactions",
                (inner
                    .transactions
                    .iter()
                    .map(move |transaction| transaction.hash.to_entity_id())
                    .collect::<Vec<_>>()
                    .into()),
            ),
            ("size", inner.size.into()),
            ("sealFields", inner.seal_fields.clone().into()),
        ] as Vec<(_, Value)>))
    }
}

impl TryIntoEntity for &Transaction {
    fn try_into_entity(self) -> Result<Entity, Error> {
        let inner = &self.0;

        Ok(Entity::from(vec![
            ("id", inner.hash.to_entity_id().into()),
            ("hash", inner.hash.into()),
            ("nonce", inner.nonce.into()),
            ("index", inner.transaction_index.unwrap().into()),
            ("from", inner.from.into()),
            ("to", inner.to.map_or(Value::Null, |to| to.into())),
            ("value", inner.value.into()),
            ("gasPrice", inner.gas_price.into()),
            ("gas", inner.gas.into()),
            ("inputData", inner.input.clone().into()),
            ("block", format!("{:x}", inner.block_hash.unwrap()).into()),
        ] as Vec<(_, Value)>))
    }
}
