use crate::entities::user::{User, UserId};
use crate::Result;
use mongodb::bson::Bson;
use mongodb::options::ReplaceOptions;
use mongodb::{bson::doc, Client, Collection};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct UserRepository {
    collection: Collection<User>,
}

static INSTANCE: OnceLock<UserRepository> = OnceLock::new();
impl UserRepository {
    const DATABASE_NAME: &'static str = "autorenfe";
    const COLLECTION_NAME: &'static str = "users";

    pub async fn instance() -> &'static Self {
        if INSTANCE.get().is_none() {
            INSTANCE.set(Self::new().await).unwrap();
        }
        INSTANCE.get().unwrap()
    }
    async fn new() -> Self {
        let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let client = Client::with_uri_str(&mongodb_uri)
            .await
            .expect("Error while creating mongodb client");
        let database = client.database(Self::DATABASE_NAME);
        let collection = database.collection(Self::COLLECTION_NAME);
        Self { collection }
    }

    pub async fn find_by_id(&self, id: UserId) -> Result<Option<User>> {
        let filter = doc! { "id": id };
        let user = self.collection.find_one(filter, None).await?;
        Ok(user)
    }

    pub async fn add(&self, user: User) -> Result<()> {
        self.collection.insert_one(user, None).await?;
        Ok(())
    }

    pub async fn update(&self, user: User) -> Result<()> {
        let filter = doc! { "id": user.id() };
        self.collection.replace_one(filter, user, None).await?;
        Ok(())
    }

    pub async fn delete(&self, id: UserId) -> Result<()> {
        let filter = doc! { "id": id };
        self.collection.delete_one(filter, None).await?;
        Ok(())
    }
    pub async fn add_or_update(&self, user: User) -> Result<()> {
        let filter = doc! { "id": user.id() };
        let options = ReplaceOptions::builder().upsert(true).build();
        self.collection.replace_one(filter, user, options).await?;
        Ok(())
    }
}

impl From<UserId> for Bson {
    fn from(id: UserId) -> Self {
        Bson::from(id.0)
    }
}
