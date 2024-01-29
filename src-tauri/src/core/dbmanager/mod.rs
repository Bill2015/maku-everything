use std::borrow::BorrowMut;
use std::process::{Child, Command};
use std::sync::Mutex;
use anyhow::anyhow;
use anyhow::Error as AError;
use tauri::api::process::Command as TCommand;
use log::info;
use log::error;

#[derive(thiserror::Error, Debug)]
pub enum DBManagerError {
    #[error("Already start the Database")]
    AlreadyStarted,

    #[error("Start the Database failed")]
    StartDBFailed(AError),

    #[error("Excuted the command failed")]
    ExcutedCommandFailed(AError),

    #[error("Kill the process failed")]
    KillTheProcessFailed(AError),

    #[error("Failed to restart the Database")]
    RestartDBFailed(AError),

    #[error("Failed to ternimated the Database")]
    TerminatedDBFailed(AError),
}

pub struct DatabaseManagerStatus {
    pub db_manager: Mutex<DatabaseManager>,
}
impl DatabaseManagerStatus {
    pub fn new() -> Self {
        let db_manager = match DatabaseManager::new() {
            Ok(val) => val,
            Err(err) => panic!("{}", err.to_string()),
        };
        Self { db_manager: Mutex::new(db_manager) }
    }
}

pub struct DatabaseManager {
    cmd: Command,
    child: Option<Child>,
    // api_process: Option<GroupChild>,
}

impl DatabaseManager {    
    pub fn new() -> Result<Self, DBManagerError> {
        info!("SurrealDB started");

        let tc = TCommand::new_sidecar("surreal")
            .map_err(|err| DBManagerError::ExcutedCommandFailed(anyhow!(err)))?
            .args(["start", "-A", "--user", "root", "--pass", "root"]);
        
        Ok(DatabaseManager {
            cmd: tc.into(),
            child: None,
        })
    }

    pub fn start_db(&mut self) -> Result<(), DBManagerError> {
        match self.child.borrow_mut() {
            Some(_) => {
                info!("Already start the database, create canceled.");
                Err(DBManagerError::AlreadyStarted)
            }
            None => {
                let child = self.cmd.spawn();
                match child {
                    Ok(v) => {
                        self.child = Some(v);
                        info!("Database Start successful");
                        Ok(())
                    }
                    Err(err) => {
                        error!("Database Start Failed");
                        Err(DBManagerError::StartDBFailed(anyhow!(err)))
                    }
                }
            }
        }     
    }

    pub fn terminate_db(&mut self) -> Result<(), DBManagerError> {
        match self.child.borrow_mut() {
            Some(child) => {
                // child.wait().expect("Some error happened when killing child process");
                child
                    .kill()
                    .map_err(|err| DBManagerError::KillTheProcessFailed(anyhow!(err)))?;
                self.child = None;
                info!("Terminated the started Database");
                Ok(())
            }
            _ => {
                info!("Database not started yet, no need to ternimated");
                Ok(())
            }
        }
    }

    pub fn restart_db(&mut self) -> Result<(), DBManagerError> {
        let terminate_result = self.terminate_db();
        match terminate_result {
            Ok(_) => {
                info!("Excuted the Database ternimated command");
                match self.start_db() {
                    Ok(_) => {
                        info!("Restart the Database Successful");
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to restart the Database");
                        return Err(DBManagerError::RestartDBFailed(e.into()));
                    }
                }
            }
            Err(e) => {
                error!("Failed to ternimated & restart the Database");
                return Err(DBManagerError::TerminatedDBFailed(e.into()));
            }
        }
    }
}
