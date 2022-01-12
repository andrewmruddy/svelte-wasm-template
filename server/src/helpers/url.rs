pub enum Url{
    // Create
    CreateDatabase,
    CreateCollection,
    CreateCharter,
    CreateUser,
    CreateAction,
    CreateExpense,
    CreateFile,
    // Read
    ReadDatabase,
    ReadCollection,
    ReadCharter,
    ReadUser,
    ReadAction,
    ReadExpense,
    ReadFile,
    // Update
    UpdateCharter,
    UpdateUser,
    UpdateAction,
    UpdateExpense,
    UpdateFile,
    // Delete
    DeleteDatabase,
    DeleteCollection,
    DeleteCharter,
    DeleteUser,
    DeleteAction,
    DeleteExpense,
    DeleteFile,
}

impl Url{
    pub fn to_string(&self)-> String{
        match self{
            // Create
            Url::CreateDatabase=>return String::from("http://localhost:8529/_db/_system/create/database"),
            Url::CreateCollection=>return String::from("http://localhost:8529/_db/cms/create/collection"),
            Url::CreateUser=>return String::from("http://localhost:8529/_db/cms/create/master"),
            // Read
            Url::ReadDatabase=>return String::from("http://localhost:8529/_db/_system/read/database"),
            Url::ReadCollection=>return String::from("http://localhost:8529/_db/_system/read/database"),
            Url::ReadUser=>return String::from("http://localhost:8529/_db/_system/read/master"),
            // Update
            // Delete
            Url::DeleteDatabase=>return String::from("http://localhost:8529/_db/_system/delete/database"),
            Url::DeleteCollection=>return String::from("http://localhost:8529/_db/cms/delete/collection"),
            _=>String::from("")
        }
    }
}