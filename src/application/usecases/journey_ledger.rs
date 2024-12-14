use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{
        journey_ledger::JourneyLedgerRepository, quest_viewing::QuestViewingRepository,
    },
    value_objects::{
        quest_adventurer_junction::MAX_ADVENTURERS_PER_QUEST, quest_statuses::QuestStatuses,
    },
};

pub struct JourneyLedgerUseCase<T1, T2>
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    journey_ledger_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> JourneyLedgerUseCase<T1, T2>
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(journey_ledger_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            journey_ledger_repository,
            quest_viewing_repository,
        }
    }

    pub async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurer_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        let conditions_to_update = (quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string())
            && adventurer_count > 0
            && adventurer_count <= MAX_ADVENTURERS_PER_QUEST;

        if !conditions_to_update {
            return Err(anyhow::anyhow!("Invalid condition to chage status"));
        }

        let result = self
            .journey_ledger_repository
            .in_journey(quest_id, guild_commander_id)
            .await?;

        Ok(result)
    }

    pub async fn to_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let condition_to_update = quest.status == QuestStatuses::InJourney.to_string();

        if !condition_to_update {
            return Err(anyhow::anyhow!("Invalid condition to change status"));
        }

        let result = self
            .journey_ledger_repository
            .to_completed(quest_id, guild_commander_id)
            .await?;

        Ok(result)
    }

    pub async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let condition_to_update = quest.status == QuestStatuses::InJourney.to_string();

        if !condition_to_update {
            return Err(anyhow::anyhow!("Invalid condition to change status"));
        }

        let result = self
            .journey_ledger_repository
            .to_failed(quest_id, guild_commander_id)
            .await?;

        Ok(result)
    }
}
