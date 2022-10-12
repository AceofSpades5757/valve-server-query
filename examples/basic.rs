use valve_server_query::Server;

fn main() {
    let server =
        Server::new("127.0.0.1:12345").expect("Connect to dedicated server running Valve game");

    let info = server.info().expect("Get general server information");
    let players = server.players().expect("Get server player information");
    let rules = server.rules().expect("Get server rules");

    // Server Information
    let server_name = info.name();
    let loaded_map = info.map();
    let max_players = info.player_max();
    let players_online = info.player_count();

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
