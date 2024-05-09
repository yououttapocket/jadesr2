use crate::net::PlayerSession;

use super::globals;
use anyhow::Result;
use proto::*;

pub struct PlayerInfo {
    pub uid: u32,
    pub lineup: LineupInfo,
    pub gender: Gender,
    pub hero_basic_type: HeroBasicType,
}

impl PlayerInfo {
    pub fn new() -> Self {
        Self {
            uid: 1337,
            lineup: default_lineup(),
            gender: Gender::from_str_name(globals.hero_gender.as_str()).unwrap(),
            hero_basic_type: HeroBasicType::from_str_name(globals.hero_basic_type.as_str())
                .unwrap(),
        }
    }

    pub async fn sync_lineup(&self, session: &PlayerSession) -> Result<()> {
        session
            .send(
                CMD_SYNC_LINEUP_NOTIFY,
                SyncLineupNotify {
                    lineup: Some(self.lineup.clone()),
                    ..Default::default()
                },
            )
            .await
    }
}

fn default_lineup() -> LineupInfo {
    LineupInfo {
        plane_id: 10001,
        name: String::from("Lineup 1"),
        index: 0,
        leader_slot: 0,
        mp: 5,
        mp_max: 5,
        avatar_list: globals
            .lineup
            .iter()
            .enumerate()
            .map(|(idx, id)| LineupAvatar {
                id: *id,
                slot: idx as u32,
                hp: 10000,
                sp: Some(AmountInfo {
                    cur_amount: 10000,
                    max_amount: 10000,
                }),
                avatar_type: 3,
                satiety: 0,
            })
            .collect(),
        ..Default::default()
    }
}
