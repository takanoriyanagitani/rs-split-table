use tonic::Status;

/// Checks the table name to prevent unexpected sql(e.g, SQL injection).
pub trait TableChecker: Sync + Send + 'static {
    fn check(&self, table_name: &str) -> Result<(), Status>;
    fn to_checked(&self, table_name: String) -> Result<String, Status> {
        self.check(table_name.as_str())?;
        Ok(table_name)
    }
}

#[derive(Clone)]
pub struct CheckFn<F> {
    checker: F,
}

impl<F> TableChecker for CheckFn<F>
where
    F: Fn(&str) -> Result<(), Status> + Send + Sync + 'static,
{
    fn check(&self, table_name: &str) -> Result<(), Status> {
        (self.checker)(table_name)
    }
}

/// Creates a [`TableChecker`] from the specified function.
pub fn fn2checker<F>(checker: F) -> impl TableChecker + Clone
where
    F: Fn(&str) -> Result<(), Status> + Send + Sync + Clone + 'static,
{
    CheckFn { checker }
}

/// Creates a [`TableChecker`] from the specified functions(AND).
pub fn double_checker_new<A, B>(chk1: A, chk2: B) -> impl TableChecker
where
    A: Fn(&str) -> Result<(), Status> + Send + Sync + 'static,
    B: Fn(&str) -> Result<(), Status> + Send + Sync + 'static,
{
    let dchk = move |tabname: &str| {
        chk1(tabname)?;
        chk2(tabname)?;
        Ok(())
    };
    CheckFn { checker: dchk }
}
