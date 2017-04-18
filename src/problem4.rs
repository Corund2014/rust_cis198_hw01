use std::collections::HashMap;
/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to 
/// `dst`.
//Algorithm:
//For an even number of disks:
//1. make the legal move between pegs A and B (in either direction),
//2. make the legal move between pegs A and C (in either direction),
//3. make the legal move between pegs B and C (in either direction),
//4. repeat until complete.
//
//For an odd number of disks:
//1. make the legal move between pegs A and C (in either direction),
//2. make the legal move between pegs A and B (in either direction),
//3. make the legal move between pegs B and C (in either direction),
//4. repeat until complete.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut vectors = HashMap::new();
    vectors.insert(src,fill_src(num_discs));
    vectors.insert(aux,Vec::new());
    vectors.insert(dst,Vec::new());
    let is_even_pegs = if num_discs % 2 == 0 { true } else { false };
    let move1: Move = if is_even_pegs { (src, aux) } else { (src, dst) };
    let move2: Move = if is_even_pegs { (src, dst) } else { (src, aux) };
    let move3: Move = (aux, dst);
    let moves_array:[Move;3]=[move1,move2,move3];
    let mut moves_index=0;
    let mut next_move;

    //problem should be solved in (2^n)-1 steps
    for _ in 0..2u32.pow(num_discs)-1
    {
        next_move=moves_array[moves_index];
        
        //if move is illegal then switch pegs
        if !is_legal_move(&next_move,&vectors) {next_move=(next_move.1,next_move.0);}
        moves.push(next_move);
        let number=vectors.get_mut(&next_move.0).unwrap().pop().unwrap();
        vectors.get_mut(&next_move.1).unwrap().push(number);
        moves_index+=1;
        if moves_index>2 {moves_index=0;}
    }
    moves
}

///check possible move is it legal:
///1. starting Peg should not be empty
///2. you can not place large disk on small
fn is_legal_move(next_move: &Move,
                 vectors:&HashMap<Peg,Vec<u32>>)
                 -> bool 
{
    let vector_start=vectors.get(&next_move.0).unwrap();
    let vector_end=vectors.get(&next_move.1).unwrap();
    let number_start;
    let number_end;
    match vector_start.last() {
        Some(x) => number_start=x,
        None => return false,
    }
    match vector_end.last() {
        Some(x) => number_end=x,
        None => return true,
    }
    if number_start>number_end {return false;}
    true
}

///put disks at source Peg
fn fill_src(num_discs: u32) -> Vec<u32> {
    let mut src_peg: Vec<u32> = Vec::new();
    let iter = (1..num_discs + 1).rev();
    for i in iter {
        src_peg.push(i);
    }
    src_peg
}