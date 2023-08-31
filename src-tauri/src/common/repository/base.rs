use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

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
    /// Resource is belong Category \
    /// Relation Namespace
    /// 
    /// ## Relation
    /// resource -> RESOURCE_BELONG -> category
    pub const RESOURCE_BELONG: &str = "resource_belong";

    /// Subject is belong Category
    /// Relation Namespace
    ///    
    /// ## Relation
    /// subject -> SUBJECT_BELONG -> category
    pub const SUBJECT_BELONG: &str = "subject_belong";

    
    /// Tag is belong Subject
    /// Relation Namespace
    ///    
    /// ## Relation
    /// tag -> TAG_BELONG_SUBJECT -> subject
    pub const TAG_BELONG_SUBJECT: &str = "tag_belong_subject";

    /// Tag is belong Category
    /// Relation Namespace
    ///    
    /// ## Relation
    /// tag -> TAG_BELONG -> category
    pub const TAG_BELONG: &str = "tag_belong";

    /// Resource is belong Category
    /// Relation Namespace
    /// 
    /// ## Relation
    /// tag -> TAGGING -> resource
    pub const TAGGING: &str = "tagging";
}


pub mod env {
    pub static DB: Surreal<Client> = Surreal::init();

    use surrealdb::Surreal;
    use surrealdb::engine::remote::ws::Client;
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::opt::auth::Root;

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

        Ok(String::from("Success Connect to DB"))
    }
}
