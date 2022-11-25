use rusqlite::{Connection, Result};

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
            format_override TEXT
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
fn close_db(connection: Connection) {
    match connection.close() {
        Ok(_) => println!("[DB] Connection closed successfully."),
        Err(err) => println!("[DB] Failed to close connection! {}", err.1),
    }
}

pub fn init() -> Result<()> {
    let connection = open_db();

    for create_table in get_tables_to_create() {
        match connection.execute(create_table, ()) {
            Ok(updated) => println!("[DB] {} rows were updated", updated),
            Err(err) => println!("[DB] Update failed: {}", err),
        }
    }

    close_db(connection);

    Ok(())
}