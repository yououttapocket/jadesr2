use crate::util;

use super::*;

pub async fn on_get_basic_info_cs_req(
    session: &PlayerSession,
    _body: &GetBasicInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_BASIC_INFO_SC_RSP,
            GetBasicInfoScRsp {
                retcode: 0,
                player_setting_info: Some(PlayerSettingInfo::default()),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_hero_basic_type_info_cs_req(
    session: &PlayerSession,
    _body: &GetHeroBasicTypeInfoCsReq,
) -> Result<()> {
    let player_info = session.player_info();

    session
        .send(
            CMD_GET_HERO_BASIC_TYPE_INFO_SC_RSP,
            GetHeroBasicTypeInfoScRsp {
                retcode: 0,
                gender: player_info.gender.into(),
                cur_basic_type: player_info.hero_basic_type.into(),
                basic_type_info_list: vec![HeroBasicTypeInfo {
                    basic_type: HeroBasicType::GirlShaman.into(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await
}

pub async fn on_player_heart_beat_cs_req(
    session: &PlayerSession,
    body: &PlayerHeartBeatCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_HEART_BEAT_SC_RSP,
            PlayerHeartBeatScRsp {
                retcode: 0,
                client_time_ms: body.client_time_ms,
                server_time_ms: util::cur_timestamp_ms(),
                download_data: Some(ClientDownloadData {
                    version: 51,
                    time: util::cur_timestamp_ms() as i64,
                    data: rbase64::decode("G0x1YVMBGZMNChoKBAQICHhWAAAAAAAAAAAAAAAod0ABTkBDOlxVc2Vyc1x4ZW9uZGV2XERvY3VtZW50c1xnaXRcaGtycGdfYnVpbGRfc2VjdXJpdHlcamFkZV9zZWN1cml0eV9tb2R1bGUubHVhAAAAAAAAAAAAAQQfAAAAJABAAClAQAApgEAAKcBAAFYAAQAsgAABHUBBAKSAQQDkAEAA6cDBAekAwgHpQMIBrAAAASyAAAAfwEKFJABAAClAQAApgEAAKcBAAFYAAwAsgAABHUBBAKSAQQDkAEAA6cDBAekAwgHpQMIBrAAAASyAAAAfQEOFGQCAAA4AAAAEA0NTBAxVbml0eUVuZ2luZQQLR2FtZU9iamVjdAQFRmluZAQpVUlSb290L0Fib3ZlRGlhbG9nL0JldGFIaW50RGlhbG9nKENsb25lKQQXR2V0Q29tcG9uZW50SW5DaGlsZHJlbgQHdHlwZW9mBARSUEcEB0NsaWVudAQOTG9jYWxpemVkVGV4dAQFdGV4dBQqSmFkZVNSIGlzIGEgZnJlZSBhbmQgb3BlbiBzb3VyY2Ugc29mdHdhcmUEDFZlcnNpb25UZXh0FC5WaXNpdCBkaXNjb3JkLmdnL3JldmVyc2Vkcm9vbXMgZm9yIG1vcmUgaW5mbyEBAAAAAQAAAAAAHwAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAIAAAACAAAAAgAAAAIAAAACAAAAAgAAAAIAAAACAAAAAgAAAAIAAAACAAAAAgAAAAIAAAACAAAAAgAAAAIAAAAAAAAAAQAAAAVfRU5W").unwrap()
                }),
            },
        )
        .await
}
