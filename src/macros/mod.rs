#[macro_export]
macro_rules! maybe {
    ($it:expr, Option) => {
        match $it {
            Some(value) => value,
            None => return Ok(()),
        }
    };
    ($it:expr, Result) => {
        match $it {
            Ok(value) => value,
            Err(why) => {
                println!("{}", why.to_string());
                return Ok(());
            }
        }
    };
}

#[macro_export]
macro_rules! edit {
    ($http:expr, $message:expr, $content:expr) => {
        $message.edit($http, |it| it.content($content)).await
    };
}

#[macro_export]
macro_rules! config_for {
    ($id:expr, $handle:expr) => {
        $handle
            .get::<ServerConfigKey>()
            .unwrap()
            .clone()
            .get($id)
            .unwrap_or(&ServerConfig::new())
            .clone();
    };
}
