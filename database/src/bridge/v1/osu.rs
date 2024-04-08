use bson::oid::ObjectId;
use chrono::Utc;

use vocolo_models::v1::osu::*;

use crate::{str_to_oid, FAKE_OID};

// General

impl From<crate::osu::GameMode> for GameMode {
    fn from(db_mode: crate::osu::GameMode) -> Self {
        match &db_mode {
            crate::osu::GameMode::Standard => GameMode::Standard,
            crate::osu::GameMode::Mania => GameMode::Mania,
        }
    }
}

impl From<GameMode> for crate::osu::GameMode {
    fn from(mode: GameMode) -> Self {
        match &mode {
            GameMode::Standard => crate::osu::GameMode::Standard,
            GameMode::Mania => crate::osu::GameMode::Mania,
        }
    }
}

//
// Tournament
//

impl From<crate::osu::Tournament> for Tournament {
    fn from(db_tournament: crate::osu::Tournament) -> Self {
        Self {
            id: db_tournament.id.unwrap_or(FAKE_OID),
            slug: db_tournament.slug,
            name: db_tournament.name,
            mode: db_tournament.mode.into(),
            invite_only: db_tournament.invite_only,
            min_team_size: db_tournament.min_team_size,
            max_team_size: db_tournament.max_team_size,
            registration_start_date: db_tournament.registration_start_date,
            registration_end_date: db_tournament.registration_end_date,
        }
    }
}

impl From<crate::osu::TournamentTeam> for TournamentTeam {
    fn from(db_tournament_team: crate::osu::TournamentTeam) -> Self {
        Self {
            id: db_tournament_team.id,
            name: db_tournament_team.name,
            captain: db_tournament_team.captain,
            players: db_tournament_team.players,
        }
    }
}

//
// Tournament DTO
//

impl From<CreateTournamentRequest> for crate::osu::Tournament {
    fn from(dto: CreateTournamentRequest) -> Self {
        let now = Utc::now();

        Self {
            id: None,
            slug: dto.slug.clone(),
            name: dto.name.clone(),
            mode: dto.mode.into(),
            invite_only: dto.invite_only.unwrap_or(false),
            min_team_size: dto.min_team_size,
            max_team_size: dto.max_team_size,
            registration_start_date: dto.registration_start_date.unwrap_or(now),
            registration_end_date: dto.registration_end_date.unwrap_or(now),
            teams: None,
        }
    }
}

impl From<UpdateTournamentRequest> for crate::osu::PartialTournament {
    fn from(dto: UpdateTournamentRequest) -> Self {
        Self {
            slug: dto.slug,
            name: dto.name,
            mode: dto.mode.map(|mode| mode.into()),
            invite_only: dto.invite_only,
            min_team_size: dto.min_team_size,
            max_team_size: dto.max_team_size,
            registration_start_date: dto.registration_start_date,
            registration_end_date: dto.registration_end_date,
            ..Default::default()
        }
    }
}

impl From<RegisterTournamentTeamRequest> for crate::osu::TournamentTeam {
    fn from(dto: RegisterTournamentTeamRequest) -> Self {
        Self {
            id: ObjectId::new(),
            name: dto.name.clone(),
            captain: 0,
            players: dto.players.unwrap_or(vec![]),
        }
    }
}

//
// Mappool
//

impl From<crate::osu::Mappool> for Mappool {
    fn from(db_mappool: crate::osu::Mappool) -> Self {
        Self {
            id: db_mappool.id.unwrap_or(FAKE_OID),
            mappack_link: db_mappool.mappack_link,
            maps: db_mappool.maps.into_iter().map(|map| map.into()).collect(),
        }
    }
}

impl From<crate::osu::MappoolMap> for MappoolMap {
    fn from(db_mappool_map: crate::osu::MappoolMap) -> Self {
        Self {
            beatmap_id: db_mappool_map.beatmap_id,
            modifiers: db_mappool_map.modifiers,
        }
    }
}

impl From<MappoolMap> for crate::osu::MappoolMap {
    fn from(mappool_map: MappoolMap) -> Self {
        Self {
            beatmap_id: mappool_map.beatmap_id,
            modifiers: mappool_map.modifiers,
        }
    }
}

//
// Mappool DTO
//

impl From<CreateMappoolRequest> for crate::osu::Mappool {
    fn from(dto: CreateMappoolRequest) -> Self {
        Self {
            id: None,
            tournament_id: str_to_oid(&dto.tournament_id),
            private: dto.private.unwrap(),
            mappack_link: "".to_string(),
            maps: vec![],
        }
    }
}

impl From<UpdateMappoolRequest> for crate::osu::PartialMappool {
    fn from(dto: UpdateMappoolRequest) -> Self {
        Self {
            mappack_link: dto.map_link,
            ..Default::default()
        }
    }
}

//
// Matchup
//

impl From<crate::osu::Matchup> for Matchup {
    fn from(db_matchup: crate::osu::Matchup) -> Self {
        Self {
            id: db_matchup.id.unwrap_or(FAKE_OID),
            tournament_id: db_matchup.tournament_id,
            date: db_matchup.date,
            team_red: db_matchup.team_red,
            team_blue: db_matchup.team_blue,
            maps: db_matchup.maps.into_iter().map(|map| map.into()).collect(),
        }
    }
}

impl From<crate::osu::MatchupMap> for MatchupMap {
    fn from(db_matchup_map: crate::osu::MatchupMap) -> Self {
        Self {
            map_id: db_matchup_map.map_id,
            map_type: db_matchup_map.map_type.into(),
            team: Default::default(),
            team_red_scores: db_matchup_map
                .team_red_scores
                .map(|v| v.into_iter().map(|score| score.into()).collect()),
            team_blue_scores: db_matchup_map
                .team_blue_scores
                .map(|v| v.into_iter().map(|score| score.into()).collect()),
        }
    }
}

impl From<crate::osu::MatchupMapType> for MatchupMapType {
    fn from(db_matchup_map_type: crate::osu::MatchupMapType) -> Self {
        match &db_matchup_map_type {
            crate::osu::MatchupMapType::Pick => MatchupMapType::Pick,
            crate::osu::MatchupMapType::Ban => MatchupMapType::Ban,
            crate::osu::MatchupMapType::Protect => MatchupMapType::Protect,
        }
    }
}

impl From<MatchupMapType> for crate::osu::MatchupMapType {
    fn from(matchup_map_type: MatchupMapType) -> Self {
        match &matchup_map_type {
            MatchupMapType::Pick => crate::osu::MatchupMapType::Pick,
            MatchupMapType::Ban => crate::osu::MatchupMapType::Ban,
            MatchupMapType::Protect => crate::osu::MatchupMapType::Protect,
        }
    }
}

impl From<crate::osu::MatchupMapScore> for MatchupMapScore {
    fn from(db_matchup_map_score: crate::osu::MatchupMapScore) -> Self {
        Self {
            player: db_matchup_map_score.player,
            mods: db_matchup_map_score.mods,
            score: db_matchup_map_score.score,
        }
    }
}

//
// Matchup DTO
//

impl From<CreateMatchupRequest> for crate::osu::Matchup {
    fn from(dto: CreateMatchupRequest) -> Self {
        Self {
            id: None,
            tournament_id: dto.tournament_id,
            date: dto.date,
            team_red: dto.team_red,
            team_blue: dto.team_blue,
            maps: vec![],
        }
    }
}

impl From<UpdateMatchupRequest> for crate::osu::PartialMatchup {
    fn from(dto: UpdateMatchupRequest) -> Self {
        Self {
            date: dto.date,
            ..Default::default()
        }
    }
}
