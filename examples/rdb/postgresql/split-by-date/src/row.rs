use time::Date;

#[derive(Clone)]
pub struct Row {
    pub id: String,
    pub date: Date,
    pub msg: String,
}
