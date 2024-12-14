use minet_ai::*;
use std::io::{self, BufRead};
use rand::{seq::SliceRandom, thread_rng, Rng};

// 1.0 Confirm 
// 0.0 Deny

const GENERATIONS: usize = 500;
const NUM_PLAYERS: usize = 100;
const OPPONENTS_PER_PLAYER: usize = 75;

fn main() {
    let mut players: Vec<Player> = (0..NUM_PLAYERS)
        .map(|_| Player {
            strategy: 0.0,
            history: Vec::new(),
            brain: Minet::new(5,10,1), // Adjust parameters as needed for your network
            fitness: 0,
        })
        .collect();
    
    for gen in 0..GENERATIONS {
        println!("### Generation: {}", gen);
        
        // Reset each player's fitness and history at the start of the generation
        for p in &mut players {
            p.fitness = 0;
            p.history.clear();
        }

        let mut rng = rand::thread_rng();
        let mut best_10 = play_generation(&mut players);
        best_10.shuffle(&mut rng);
        let mut new_generation_brains: Vec<Minet> = Vec::new();
        let mut new_generation_players: Vec<Player> = Vec::new();
        for i in (0..10).step_by(2) {
            let (left, right) = best_10.split_at_mut(i + 1);
            let p1 = &mut left[i];
            let p2 = &mut right[0];
            for _ in 0..20 {
                let mut new = p1.brain.crossbreed(&p2.brain);
                new.mutate();
                new_generation_brains.push(new);
            }
        }
        
        for i in 0..NUM_PLAYERS {
            new_generation_players.push(Player {
                strategy: 0.0,
                history: Vec::new(),
                brain: new_generation_brains[i].clone(),
                fitness: 0,
            });
        }
        
        if gen == GENERATIONS - 1 {
            let top_10 = top_ten_players(players.clone());
            println!("### Top 10 Players:");
            for (i, player) in top_10.iter().enumerate() {
                game_display(player);
            }
        }
        
        players = new_generation_players;
    }
    let mut my_turn = players[0].clone();
    
    println!("\n\n");
    println!("BEST SCORING NEURAL NETWORK");
    println!("GENOME: \n ");
    my_turn.brain.display_genome();
    println!("");
    my_turn.brain.display();
    let file_save = "prisoner1.dot";
    println!("Visualized Network Saved to: {}", file_save);
    my_turn.brain.dot_to_file(file_save);
    
    println!("\n\n\n\n\n");
    
    play_against(&mut my_turn);
}

fn play_against (
    player2: &mut Player,
) {
    player2.history.clear();
    player2.strategy = 0.0;
    player2.fitness = 0;
    
    let mut player1 = Player {
        strategy: 0.0,
        history: Vec::new(),
        brain: Minet::new(1,1,1),
        fitness: 0,
    };
    
    for i in 0..5 { 
        player2.history.push(0.5);
        player1.history.push(0.5);
    }
    
    for i in 5..30 {
        println!("PRISONER'S DILEMMA GAME");
        println!("  play against the best neural network  \n\n ");
        
        print!("Player:         ");
        game_display(&player1);
        print!("Neural Network: ");
        game_display(&player2);
        if player2.strategy == 0.0 {
            println!("Neural Network has chosen to COOPERATE");
        } else if player2.strategy == 1.0 {
            println!("Neural Network has chosen to DEFECT");
        } else {
            println!("");
        }
        
        println!("Press 1 to CONFIRM    Press 2 to DENY");
        
        let mut stdin = io::stdin();
        let input = &mut String::new();
        input.clear();
        stdin.read_line(input);
        //println!("{}", input);
        
        if input.trim() == "1" {
            player1.strategy = 0.0;
            print!("\x1B[2J\x1B[1;1H");
        } else if input.trim() == "2" {
            player1.strategy = 1.0;
            print!("\x1B[2J\x1B[1;1H");
        }
        
        let input2 = vec![
            player1.history[i-1],
            player1.history[i-2],
            player1.history[i-3],
            player1.history[i-4],
            player1.history[i-5],
        ];
        
        let output2 = player2.brain.forward(input2)[0].round();
        player2.strategy = output2;
        
        play_round(&mut player1, player2);
    }

    

}



fn play_generation(players: &mut Vec<Player>) -> Vec<Player> {
    let mut rng = thread_rng();
    players.shuffle(&mut rng);    

    // Each player will face OPPONENTS_PER_PLAYER distinct opponents
    // To avoid double counting, we'll only play matches where opp > i
    for i in 0..players.len() {
        let mut possible_opponents: Vec<usize> = ((i+1)..players.len()).collect();
        possible_opponents.shuffle(&mut rng);
        
        for &opp_idx in possible_opponents.iter().take(OPPONENTS_PER_PLAYER) {
            // Carefully borrow mutable references to both players
            let (p1_slice, p2_slice) = if i < opp_idx {
                players.split_at_mut(opp_idx)
            } else {
                players.split_at_mut(i)
            };

            let (player1, player2) = if i < opp_idx {
                (&mut p1_slice[i], &mut p2_slice[0])
            } else {
                (&mut p2_slice[0], &mut p1_slice[opp_idx])
            };

            play_game(player1, player2);
        }
    }

    let top_10 = top_ten_players(players.clone());
    //game_display(&top_10[0]);

    let average: usize = players.iter().map(|p| p.fitness).sum();
    let weighted = average as f32 / OPPONENTS_PER_PLAYER as f32;
    println!("Generational Average: {}", weighted as f32 / players.len() as f32);

    top_10
}

#[derive(Clone)]
struct Player {
    strategy: f32,
    history: Vec<f32>,
    brain: Minet,
    fitness: usize, 
}

fn play_round(player1: &mut Player, player2: &mut Player) {
    let player1_move = player1.strategy;
    let player2_move = player2.strategy;
    
    player1.history.push(player1_move);
    player2.history.push(player2_move);
    
    let round_score = match (player1_move, player2_move) {
        (0.0, 0.0) => (3, 3), // Cooperate / Cooperate
        (0.0, 1.0) => (0, 5), // Cooperate / Defect
        (1.0, 0.0) => (5, 0), // Defect / Cooperate
        (1.0, 1.0) => (1, 1), // Defect / Defect
        _ => (0, 0),
    };   
    player1.fitness += round_score.0;
    player2.fitness += round_score.1;
}

fn play_game(player1: &mut Player, player2: &mut Player) { 
    
    player1.history.clear();
    player2.history.clear();
    // Initialize history with 3 rounds of abstaining moves (0.5 represents no decision yet)
    for _ in 0..5 {
        player1.history.push(0.0);
        player2.history.push(1.0);
    }

    for i in 5..25 {
        // Inputs for player1's neural net: last 3 moves of player2
        let input1 = vec![
            player2.history[i-1],
            player2.history[i-2],
            player2.history[i-3],
            player2.history[i-4],
            player2.history[i-5],
        ];
        
        // Inputs for player2's neural net: last 3 moves of player1
        let input2 = vec![
            player1.history[i-1],
            player1.history[i-2],
            player1.history[i-3],
            player1.history[i-4],
            player1.history[i-5],
        ];
        
        let output1 = player1.brain.forward(input1)[0].round();
        let output2 = player2.brain.forward(input2)[0].round();
        player1.strategy = output1;
        player2.strategy = output2;
        
        play_round(player1, player2);
    }


}

fn game_display(player1: &Player) {
    let player1_history = &player1.history;
    
    let mut output1 = String::new();
    for &move_val in player1_history.iter() {
        if move_val == 0.0 {
            output1.push_str(" ▔ ");
        } else if move_val == 1.0 {
            output1.push_str(" █ ");
        } else if (move_val - 0.5).abs() < f32::EPSILON {
            output1.push_str(" ? ");
        }
    }
    let score = (player1.fitness);
    output1.push_str(&score.to_string());
    println!("{}", output1);
}

fn top_ten_players(mut players: Vec<Player>) -> Vec<Player> {
    // Sort players in descending order by their fitness
    players.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    
    // Truncate the vector to keep only the top 10
    players.truncate(10);
    players
}
