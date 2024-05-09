use anyhow::Result;
use proto::*;

use crate::{net::PlayerSession, util};

pub async fn on_player_get_token_cs_req(
    session: &PlayerSession,
    _body: &PlayerGetTokenCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_GET_TOKEN_SC_RSP,
            PlayerGetTokenScRsp {
                retcode: 0,
                uid: session.player_uid(),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_player_login_cs_req(
    session: &PlayerSession,
    body: &PlayerLoginCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_LOGIN_SC_RSP,
            PlayerLoginScRsp {
                login_random: body.login_random,
                server_timestamp_ms: util::cur_timestamp_ms(),
                stamina: 240,
                basic_info: Some(PlayerBasicInfo {
                    nickname: String::from("JadeSR"),
                    level: 1,
                    stamina: 240,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await
}
