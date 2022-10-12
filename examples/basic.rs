use valve_server_query::Client;

fn main() {
    let client =
        Client::new("127.0.0.1:12345").expect("Connect to dedicated server running Valve game");

    let server = client.info().expect("Get general server information");
    let players = client.players().expect("Get server player information");
    let rules = client.rules().expect("Get server rules");

    // Server Information
    let server_name = server.name();
    let loaded_map = server.map();
    let max_players = server.player_max();
    let players_online = server.player_count();

    println!("Server Name:     {}", &server_name);
    println!("Map Loaded:      {}", &loaded_map);
    println!("Players Max:     {}", &max_players);
    println!("Players Online:  {}", &players_online);

    println!("");

    // Player Information
    for player in players.iter() {
        println!("Player: {:?}", player);
    }

    println!("");

    // Rules
    for (rule, setting) in rules.iter() {
        println!("Rule:    {}", rule);
        println!("Setting: {}", setting);
    }
}
