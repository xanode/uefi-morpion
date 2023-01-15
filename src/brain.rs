extern crate std;

use crate::engine::{
    Engine, ItemSymbol,
};

fn evaluate(engine: &Engine, player: ItemSymbol) -> i32 {
    if let winning_symbol = engine.win() {
        if winning_symbol == player {
            return 1;
        } else if winning_symbol == ItemSymbol::Empty {
            return 0;
        } else {
            return -1;
        }
    }
}

fn compute_score(engine: &mut Engine, player: ItemSymbol, depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 || engine.win() != ItemSymbol::Empty {
        return evaluate(engine, ItemSymbol::X);
    }

    let mut best_score: i32 = if player == ItemSymbol::X { std::i32::MIN } else { std::i32::MAX };

    for index in engine.get_empty_items() {
        let mut engine_clone = engine.clone();
        clone.set_item_symbol(index, player);
        let score = compute_score(&mut engine_clone, match player {
            ItemSymbol::X => ItemSymbol::O,
            ItemSymbol::O => ItemSymbol::X,
        }, depth - 1, alpha, beta);

        if (player == ItemSymbol::X && score > best_score) || (player == ItemSymbol::O && score < best_score) {
            best_score = score;
        }

        if player == ItemSymbol::X {
            alpha = std::cmp::max(alpha, best_score);
        } else {
            beta = std::cmp::min(beta, best_score);
        }

        if beta <= alpha {
            break;
        }
    }
    best_score
}

pub fn compute_best_move(engine: &mut Engine, player: ItemSymbol, depth: u8) -> usize {
    let mut best_score = match player {
        ItemSymbol::X => std::i32::MIN,
        ItemSymbol::O => std::i32::MAX,
    };
    let mut best_move = 0;

    for index in engine.get_empty_items() {
        let mut engine_clone = engine.clone();
        engine_clone.set_item_symbol(index, player);
        let score = compute_score(&mut engine_clone, player, depth - 1, std::i32::MIN, std::i32::MAX);

        if (player == ItemSymbol::X && score > best_score) || (player == ItemSymbol::O && score < best_score) {
            best_score = score;
            best_move = index;
        }
    }
    best_move
}