pub(crate) mod filter;

pub mod user {
    pub fn get_user(uid:i32) {
        if uid == 1 {
            println!("user is: {}", "zyxm");
        } else if uid == 2 {
            println!("user is: {}", "cole");
        } else {
            println!("user is: {}", "unknown");
        }
    }

    pub fn get_username(uid:i32) -> &'static str {
        // if uid == 1 {
        //     return  "zyxm";
        // } else if uid == 2 {
        //     return  "cole";
        // } else {
        //     return "unknown";
        // }

        let name = if uid == 1 {
            "zyxm"
        } else if uid == 2 {
            "cole"
        } else {
            "unknown"
        };
        name
    }

    fn add(i:i32) ->i32{
        i + 1
    }
}