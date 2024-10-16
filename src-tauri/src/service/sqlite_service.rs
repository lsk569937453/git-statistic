use rusqlite::{params, Connection};

pub fn clear_data(connection: &Connection) -> Result<(), anyhow::Error> {
    connection.execute_batch(
        "DROP TABLE IF EXISTS git_base_info;
            DROP TABLE IF EXISTS git_commit_info;
            DROP TABLE IF EXISTS git_author_info;
            DROP TABLE IF EXISTS git_file_info;
            DROP TABLE IF EXISTS git_line_info;

            DROP TABLE IF EXISTS git_tag_info;
            DROP TABLE IF EXISTS git_init_status;

        ",
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_init_status (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            current_tasks INTEGER NOT NULL, 
            total_tasks INTEGER NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_base_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            project_name TEXT NOT NULL, 
            generate_time TEXT NOT NULL,
            age    INTEGER NOT NULL, 
            active_days  INTEGER NOT NULL,
            total_files_count INTEGER NOT NULL,
            total_lines_count INTEGER NOT NULL,
            total_added_count INTEGER NOT NULL,
            total_deleted_count INTEGER NOT NULL,
            total_commits_count INTEGER NOT NULL,
            authors_count INTEGER NOT NULL,
            first_commit_time TEXT NOT NULL,
            last_commit_time TEXT NOT NULL
            )",
        params![],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_commit_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            quota_name TEXT NOT NULL, 
            quota_value TEXT NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_author_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            quota_name TEXT NOT NULL, 
            quota_value TEXT NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_file_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            quota_name TEXT NOT NULL, 
            quota_value TEXT NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_line_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            quota_name TEXT NOT NULL, 
            quota_value TEXT NOT NULL
            )",
        params![],
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS git_tag_info (
            id   INTEGER PRIMARY KEY AUTOINCREMENT, 
            quota_name TEXT NOT NULL, 
            quota_value TEXT NOT NULL
            )",
        params![],
    )?;

    Ok(())
}
