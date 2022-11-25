use rusqlite::Connection;

fn get_tables_to_create() -> Vec<&'static str> {
    vec![
        "CREATE TABLE IF NOT EXISTS players (
            player_id INTEGER PRIMARY KEY ASC,
            discord_id TEXT,
            discord_name TEXT,
            shuffleit_name TEXT
        ) WITHOUT ROWID",
        "CREATE TABLE IF NOT EXISTS tournaments (
            tournament_id INTEGER PRIMARY KEY ASC,
            name TEXT,
            description TEXT,
            rules TEXT,
            format_override TEXT,
            start_date INTEGER,
            end_date INTEGER
        ) WITHOUT ROWID",
        "CREATE TABLE IF NOT EXISTS formats (
            format_type TEXT
        )",
        "CREATE TABLE IF NOT EXISTS stages (
            stage_id INTEGER PRIMARY KEY ASC,
            tournament_id INTEGER,
            format TEXT,
            position INTEGER,
            FOREIGN KEY(tournament_id) REFERENCES tournaments(tournament_id),
            FOREIGN KEY(format) REFERENCES formats(format_type)
        ) WITHOUT ROWID",
        "CREATE TABLE IF NOT EXISTS results (
            result_id INTEGER PRIMARY KEY ASC,
            stage_id INTEGER,
            player1_id INTEGER,
            player2_id INTEGER,
            player1_wins REAL,
            player2_wins REAL,
            FOREIGN KEY(stage_id) REFERENCES stages(stage_id),
            FOREIGN KEY(player1_id) REFERENCES players(player_id),
            FOREIGN KEY(player2_id) REFERENCES players(player_id)
        ) WITHOUT ROWID",
        "CREATE TABLE IF NOT EXISTS tournament_teams (
            team_id INTEGER PRIMARY KEY ASC,
            tournament_id INTEGER,
            FOREIGN KEY(tournament_id) REFERENCES tournaments(tournament_id)
        ) WITHOUT ROWID",
        "CREATE TABLE IF NOT EXISTS stage_groups (
            group_id INTEGER PRIMARY KEY ASC,
            stage_id INTEGER,
            FOREIGN KEY(stage_id) REFERENCES stages(stage_id)
        ) WITHOUT ROWID",
        "CREATE TABLE IF NOT EXISTS stage_seeds (
            seed_id INTEGER PRIMARY KEY ASC,
            bracket_number INTEGER,
            stage_id INTEGER,
            player_id INTEGER,
            team_id INTEGER,
            FOREIGN KEY(stage_id) REFERENCES stages(stage_id),
            FOREIGN KEY(player_id) REFERENCES players(player_id),
            FOREIGN KEY(team_id) REFERENCES tournament_teams(team_id)
        ) WITHOUT ROWID",
        "CREATE TABLE IF NOT EXISTS player_to_tournament (
            id INTEGER PRIMARY KEY ASC,
            tournament_id INTEGER,
            player_id INTEGER,
            FOREIGN KEY(tournament_id) REFERENCES tournaments(tournament_id),
            FOREIGN KEY(player_id) REFERENCES players(player_id)
        )",
        "CREATE TABLE IF NOT EXISTS player_to_team (
            id INTEGER PRIMARY KEY ASC,
            team_id INTEGER,
            player_id INTEGER,
            FOREIGN KEY(team_id) REFERENCES tournament_teams(team_id),
            FOREIGN KEY(player_id) REFERENCES players(player_id)
        )",
        "CREATE TABLE IF NOT EXISTS member_to_group (
            id INTEGER PRIMARY KEY ASC,
            group_id INTEGER,
            player_id INTEGER,
            team_id INTEGER,
            FOREIGN KEY(group_id) REFERENCES stage_groups(group_id),
            FOREIGN KEY(team_id) REFERENCES tournament_teams(team_id),
            FOREIGN KEY(player_id) REFERENCES players(player_id)
        )"
    ]
}

fn open_db() -> Connection {
    match Connection::open("./db/database.db3") {
        Ok(conn) => {println!("[DB] Connection opened successfully."); conn},
        Err(err) => panic!("[DB] Failed to open connection: {}", err),
    }
}

pub fn init() {
    let connection = open_db();

    for create_table in get_tables_to_create() {
        match connection.execute(create_table, ()) {
            Ok(updated) => println!("[DB] {} rows were updated", updated),
            Err(err) => println!("[DB] Update failed: {}", err),
        }
    }

    println!("[DB] Database initialised successfully!");
}

#[derive(Debug, Serialize)]
pub struct Player {
    id: i64,
    discord_id: String,
    discord_name: String,
    shuffleit_name: String
}

#[derive(Debug, Serialize)]
pub struct Tournament {
    id: i64,
    name: String,
    description: String,
    rules: Option<String>,
    format_override: Option<String>,
    start_date: Option<i64>,
    end_date: Option<i64>
}

pub fn get_players() -> Vec<Player> {
    let connection = open_db();

    let mut statement = connection.prepare("SELECT * FROM players").unwrap();
    let players: Vec<Player> = statement.query_map([], |row| {
        Ok(Player {
            id: row.get(0)?,
            discord_id: row.get(1)?,
            discord_name: row.get(2)?,
            shuffleit_name: row.get(3)?,
        })
    }).unwrap().map(|t| { t.unwrap() }).collect();

    players
}
pub fn get_tournaments() -> Vec<Tournament> {
    let connection = open_db();

    let mut statement = connection.prepare("SELECT * FROM tournaments").unwrap();
    let tournaments = statement.query_map([], |row| {
        Ok(Tournament {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            rules: row.get(3)?,
            format_override: row.get(4)?,
            start_date: row.get(5)?,
            end_date: row.get(6)?,
        })
    }).unwrap().map(|t| { t.unwrap() }).collect();

    tournaments
}