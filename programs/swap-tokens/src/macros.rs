// Log to Program Log with a prologue so transaction scraper knows following line is valid mango log
#[macro_export]
macro_rules! add_liquidity_emit {
    ($e:expr) => {
        msg!("liquidity added");
        emit!($e);
    };
}

#[macro_export]
macro_rules! swap_emit {
    ($e:expr) => {
        msg!("swapped");
        emit!($e);
    };
}

#[macro_export]
macro_rules! set_paused_emit {
    ($e:expr) => {
        msg!("set paused");
        emit!($e);
    };
}

#[macro_export]
macro_rules! set_swap_rate_emit {
    ($e:expr) => {
        msg!("set swap rate");
        emit!($e);
    };
}