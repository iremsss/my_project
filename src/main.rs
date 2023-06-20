struct User {
    discord_username: String,
    email: String,
}

fn build_user(discord_username: String, email: String) -> User {
    User {
        discord_username,
        email,
    }
}

fn main() {

    let discord_username = String::from("karpuz#4188");
    let email = String::from("iremsaygin1997@gmail.com");
    let user = build_user(discord_username, email);

    println!("Username: {}, Email: {}", user.discord_username, user.email);
    //but we cant get the values username and email. becuse we changed their owner
    //println!("old discord_username: {} , old email: {}" , discord_username , email);
}
