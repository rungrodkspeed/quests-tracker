use crate::domain::{
    repositories::{
        crew_switchboard::CrewSwitchBoardRepository, quest_viewing::QuestViewingRepository,
    },
    value_objects::{
        quest_adventurer_junction::{QuestAdventurerJunction, MAX_ADVENTURERS_PER_QUEST},
        quest_statuses::QuestStatuses,
    },
};
use anyhow::Result;
use std::sync::Arc;

pub struct CrewSwitchBoardUseCase<T1, T2>
where
    T1: CrewSwitchBoardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    crew_switchboard_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> CrewSwitchBoardUseCase<T1, T2>
where
    T1: CrewSwitchBoardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(crew_switchboard_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            crew_switchboard_repository,
            quest_viewing_repository,
        }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurer_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        let quest_status_condition = quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string();
        let adventurer_count_condition = adventurer_count < MAX_ADVENTURERS_PER_QUEST;

        if !adventurer_count_condition {
            return Err(anyhow::anyhow!("Quest is full"));
        }

        if !quest_status_condition {
            return Err(anyhow::anyhow!("Quest is not joinable"));
        }

        self.crew_switchboard_repository
            .join(QuestAdventurerJunction {
                quest_id,
                adventurer_id,
            })
            .await?;

        Ok(())
    }

    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let leaving_condition = quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string();

        if !leaving_condition {
            return Err(anyhow::anyhow!("Quest in not leavable"));
        }

        self.crew_switchboard_repository
            .leave(QuestAdventurerJunction {
                quest_id,
                adventurer_id,
            })
            .await?;

        Ok(())
    }
}
