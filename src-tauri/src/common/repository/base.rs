pub mod tablens {
    /** Resource Database Namespace (aka table name) */
    pub const RESOURCE: &str = "resource";

    /** Category Database Namespace (aka table name) */
    pub const CATEGORY: &str = "category";

    /** Subject Database Namespace (aka table name) */
    pub const SUBJECT: &str = "subject";

    /** Subject Database Namespace (aka table name) */
    pub const TAG: &str = "tag";
}

pub mod relatens {
    /// entity is belong another entity
    /// Relation Namespace
    ///    
    /// ## Relation
    /// entity -> BELONG -> entity
    pub const BELONG: &str = "belong";

    /// Resource is belong Category
    /// Relation Namespace
    /// 
    /// ## Relation
    /// tag -> TAGGING -> resource
    pub const TAGGING: &str = "tagging";
}


pub mod env {

    use once_cell::sync::Lazy;
    use surrealdb::Surreal;
    use surrealdb::engine::remote::ws::Client;
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::opt::auth::Root;

    use crate::common::repository::PRE_DEFINED_REPOSITORY;
    pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

    pub async fn connent_db() -> Result<String, surrealdb::Error> {
        // Connect to the server
        DB.connect::<Ws>("localhost:8000").await?;

        // Signin as a namespace, database, or root user
        DB.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

        // Select a specific namespace / database
        DB.use_ns("test").use_db("test").await?;

        // pre defined sql function
        PRE_DEFINED_REPOSITORY.define_fns().await?;

        Ok(String::from("Success Connect to DB"))
    }
}
